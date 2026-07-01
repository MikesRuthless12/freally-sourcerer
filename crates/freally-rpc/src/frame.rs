//! Length-prefixed framing.
//!
//! Every frame is a `u32` big-endian length followed by exactly that
//! many bytes of UTF-8 JSON. The length excludes itself. Frames larger
//! than `MAX_FRAME_BYTES` are rejected so a hostile peer cannot OOM the
//! server with a single 4-GiB length prefix.

use std::io;

use thiserror::Error;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

/// Hard cap on a single frame's payload (16 MiB). The largest legitimate
/// frame on this transport is a query result batch: ~10 000 hits × ~1 KiB
/// of metadata each ≈ 10 MiB. 16 MiB leaves headroom; a hostile peer
/// announcing 17 MiB gets the connection dropped without allocating.
pub const MAX_FRAME_BYTES: usize = 16 * 1024 * 1024;

#[derive(Debug, Error)]
pub enum FrameError {
    #[error("io: {0}")]
    Io(#[from] io::Error),

    #[error("frame too large: {len} bytes (max {max})")]
    TooLarge { len: usize, max: usize },

    #[error("invalid utf-8 in frame")]
    InvalidUtf8,

    #[error("peer closed connection mid-frame")]
    UnexpectedEof,
}

/// Reads length-prefixed frames from any `AsyncRead`.
pub struct FrameReader<R> {
    inner: R,
    buf: Vec<u8>,
}

impl<R: AsyncRead + Unpin> FrameReader<R> {
    pub fn new(inner: R) -> Self {
        Self {
            inner,
            buf: Vec::new(),
        }
    }

    pub fn into_inner(self) -> R {
        self.inner
    }

    /// Read the next frame. Returns `Ok(None)` on a clean EOF before any
    /// bytes of a new frame have been read. Returns `UnexpectedEof` if
    /// the peer hangs up mid-length-prefix or mid-payload.
    pub async fn read_frame(&mut self) -> Result<Option<String>, FrameError> {
        let mut len_buf = [0_u8; 4];
        match self.inner.read_exact(&mut len_buf).await {
            Ok(_) => {}
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => return Ok(None),
            Err(e) => return Err(FrameError::Io(e)),
        }
        let len = u32::from_be_bytes(len_buf) as usize;
        if len > MAX_FRAME_BYTES {
            return Err(FrameError::TooLarge {
                len,
                max: MAX_FRAME_BYTES,
            });
        }
        self.buf.clear();
        self.buf.resize(len, 0);
        self.inner.read_exact(&mut self.buf).await.map_err(|e| {
            if e.kind() == io::ErrorKind::UnexpectedEof {
                FrameError::UnexpectedEof
            } else {
                FrameError::Io(e)
            }
        })?;
        let s = std::str::from_utf8(&self.buf).map_err(|_| FrameError::InvalidUtf8)?;
        Ok(Some(s.to_string()))
    }
}

/// Writes length-prefixed frames to any `AsyncWrite`.
pub struct FrameWriter<W> {
    inner: W,
}

impl<W: AsyncWrite + Unpin> FrameWriter<W> {
    pub fn new(inner: W) -> Self {
        Self { inner }
    }

    pub fn into_inner(self) -> W {
        self.inner
    }

    pub async fn write_frame(&mut self, payload: &str) -> Result<(), FrameError> {
        let bytes = payload.as_bytes();
        if bytes.len() > MAX_FRAME_BYTES {
            return Err(FrameError::TooLarge {
                len: bytes.len(),
                max: MAX_FRAME_BYTES,
            });
        }
        let len = (bytes.len() as u32).to_be_bytes();
        self.inner.write_all(&len).await?;
        self.inner.write_all(bytes).await?;
        self.inner.flush().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{duplex, split};

    #[tokio::test]
    async fn round_trip_small_frame() {
        let (a, b) = duplex(64);
        let (ra, wa) = split(a);
        let (rb, wb) = split(b);
        let mut writer = FrameWriter::new(wa);
        let mut reader = FrameReader::new(rb);
        writer.write_frame(r#"{"hello":"world"}"#).await.unwrap();
        let got = reader.read_frame().await.unwrap().unwrap();
        assert_eq!(got, r#"{"hello":"world"}"#);
        drop(writer);
        drop(ra);
        drop(wb);
    }

    #[tokio::test]
    async fn rejects_oversized_announcement() {
        let (a, b) = duplex(8);
        let (_ra, mut wa) = split(a);
        let (rb, _wb) = split(b);
        let mut reader = FrameReader::new(rb);
        let mut prefix = ((MAX_FRAME_BYTES + 1) as u32).to_be_bytes().to_vec();
        prefix.extend_from_slice(b"junk");
        wa.write_all(&prefix).await.unwrap();
        let err = reader.read_frame().await.unwrap_err();
        assert!(matches!(err, FrameError::TooLarge { .. }));
    }

    #[tokio::test]
    async fn clean_eof_returns_none() {
        // Drop ALL of `a` (both halves) so b's read side reliably sees
        // EOF — tokio's DuplexStream signals EOF only when both halves
        // of the other side are gone.
        let (a, b) = duplex(8);
        let (rb, _wb) = split(b);
        let mut reader = FrameReader::new(rb);
        drop(a);
        let got = reader.read_frame().await.unwrap();
        assert!(got.is_none());
    }
}
