# ADR-0002: Project name and licensing intent

- Status: Accepted; licence application pending copyright-holder confirmation
- Date: 2026-08-05
- Scope: Repository identity and intended open-source licence

## Context

The initial charter used “Rust Flight Framework” as a working title and left
licensing for human review. A public name and licence affect project identity,
redistribution rights, contribution expectations, and Cargo metadata, so they
must not be selected by automation alone.

## Decision

- The project name is **Rust Flight Framework** and the short internal label is
  **RFF**.
- Branding remains text-only for now and must not resemble NASA or cFS branding
  or imply flight readiness, certification, endorsement, or compatibility.
- No namespace or trademark clearance is claimed. Perform current searches
  before public release or registration of public package names.
- The approved licensing intent is **MIT OR Apache-2.0**, allowing recipients to
  choose either licence once the licence is applied.
- Until the exact copyright holder is confirmed and both licence texts are
  added, the repository remains unpublished, Cargo metadata omits a licence
  expression, and no redistribution permission is implied.
- Initial Cargo packages use `publish = false`.

## Alternatives considered

- Retain the working title indefinitely: rejected after explicit human approval
  of the project name.
- Apache-2.0 only: credible and includes an express patent grant, but offers less
  choice to Rust ecosystem users than the approved dual licence.
- MPL-2.0: provides file-level reciprocity, but that additional obligation was
  not selected.
- Remain unlicensed permanently: incompatible with the intended open Rust
  project, though it remains the accurate temporary state until licence files
  are complete.

## Evidence

- Cargo defines `OR` in an SPDX expression as allowing the recipient to choose
  either licence. (`SRC-RUST-CARGO-LICENCE`)
- The canonical MIT and Apache-2.0 terms and obligations are recorded in the
  source register. (`SRC-OSI-MIT`, `SRC-OSI-APACHE-2.0`)

## Consequences and follow-up

- README, crate documentation, and generated user-facing material use the
  approved name while retaining the safety and non-affiliation notices.
- The exact copyright holder remains the only blocker to applying the approved
  licence files and Cargo expression.
- Changing the name or licence requires a new human-approved decision record.
