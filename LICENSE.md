# Freally — All Rights Reserved

**Copyright © 2026 Mike Weaver. All rights reserved.**

This software, its source code, its design, its documentation, and every asset
in this repository (collectively, the "Work") are the proprietary property of
Mike Weaver.

## No license is granted

No right or licence — express or implied, by estoppel or otherwise — is granted
to any person under any patent, copyright, trade-secret, trade-mark, or other
intellectual-property right held by Mike Weaver, except as specifically and
explicitly granted in a separate written agreement signed by Mike Weaver.

## What you may NOT do

Without prior written permission from Mike Weaver, you may not:

- Copy, reproduce, or duplicate the Work, in whole or in part, in any form.
- Distribute, publish, sublicense, sell, lease, lend, rent, or transfer the
  Work or any copy of it, including via any public, private, or peer-to-peer
  network or repository.
- Modify, adapt, translate, port, or create derivative works of the Work.
- Reverse-engineer, decompile, disassemble, or attempt to derive the source
  code of any compiled portion of the Work, except where applicable law
  expressly forbids that restriction.
- Re-host the Work or any portion of it on any other repository, mirror,
  archive, package registry, file-sharing service, or content-delivery network.
- Remove, obscure, or alter any copyright, trade-mark, attribution, or licence
  notice that appears in or with the Work.
- Use the Work, or any portion of it, as input to train, fine-tune, evaluate,
  or otherwise develop any machine-learning model, large-language model, or
  generative-AI system.
- Use the Work in any commercial product, service, or workflow.
- Exploit the Work for any purpose competitive with Mike Weaver.

## What you may do

The only rights granted are those expressly granted by Mike Weaver in writing.

If you are reading this file because GitHub (or another forge) has displayed
the repository to you, that display does not constitute a grant of any right
beyond reading this notice.

## Contributions

Mike Weaver is not currently accepting external contributions to the Work.
Any contribution submitted (in any form, by any means) is offered to Mike
Weaver under a perpetual, irrevocable, worldwide, royalty-free, fully
sublicensable assignment of all rights, with no obligation on Mike Weaver to
incorporate, attribute, or compensate the contributor. If you do not consent
to this assignment, do not submit contributions.

## Third-party dependencies

Compiled binaries of the Work include third-party libraries distributed under
their own permissive licences (MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause,
ISC, CC0-1.0, Unlicense, Unicode-DFS-2016, Unicode-3.0, Zlib, MPL-2.0). Those
notices are reproduced in `THIRD-PARTY-NOTICES.md`. The third-party licences
apply only to the third-party code; they do not extend any rights over the
Work itself.

`cargo-deny` enforces this dependency policy in CI. AGPL, GPL, SSPL, BUSL,
CC-BY-NC, and CC-BY-SA dependencies are hard-banned.

## Trade-marks

"Freally" and the Freally magnifying-glass mark are trade-marks of Mike
Weaver. No right to use any of Mike Weaver's trade-marks is granted by this
notice.

## Warranty disclaimer

THE WORK IS PROVIDED "AS IS," WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE, TITLE, AND NON-INFRINGEMENT. IN NO EVENT
SHALL MIKE WEAVER BE LIABLE FOR ANY CLAIM, DAMAGES, OR OTHER LIABILITY,
WHETHER IN AN ACTION OF CONTRACT, TORT, OR OTHERWISE, ARISING FROM, OUT OF,
OR IN CONNECTION WITH THE WORK OR THE USE OR OTHER DEALINGS IN THE WORK.

## Termination

Any breach of this notice immediately and automatically terminates any
permission you may have had to access or use the Work. Upon termination
you must destroy all copies of the Work in your possession or control.

## Governing law

This notice is governed by the laws of the jurisdiction in which Mike Weaver
is domiciled, without regard to conflict-of-law rules.

## Contact

Permission requests, licensing enquiries, and trade-mark questions:
`mythodikalone@gmail.com` — subject line `LICENSE: Freally`.

---

**Note for Mike:** retain a software-IP lawyer before tagging v0.19.84.
Freally ships on three operating systems and indexes user data — broader
exposure than the Win-only Crash File. Pay extra attention to (a) whether
index data on disk constitutes personal data under GDPR / CCPA when shipped
at scale, (b) whether the OS-journal subscription pattern triggers any
platform-vendor TOS quirks (especially Apple's user-data-access framing),
and (c) whether the "Everything-style" framing creates any trade-mark
exposure with voidtools. None should be blocking, all should be reviewed.
