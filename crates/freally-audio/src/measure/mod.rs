//! Measurement primitives — kept separate from the symphonia decode
//! loop so each can be unit-tested with a synthetic frame stream.

pub mod loudness;
pub mod silence;
