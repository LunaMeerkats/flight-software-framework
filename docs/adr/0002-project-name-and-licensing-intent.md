# ADR-0002: Project name and dual licensing

- Status: Accepted; licence applied 2026-08-26
- Date: 2026-08-05
- Scope: Repository identity and open-source licence

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
- Repository content is licensed under **MIT OR Apache-2.0**, allowing each
  recipient to choose either licence.
- Daniel Smith confirmed his authority to license all current repository
  content and approved the exact notice `Copyright 2026 Daniel Smith` on
  2026-08-26.
- The canonical terms are stored in [LICENSE-MIT](../../LICENSE-MIT) and
  [LICENSE-APACHE](../../LICENSE-APACHE), and Cargo records the SPDX expression
  `MIT OR Apache-2.0`.
- Cargo packages retain `publish = false`; applying the licence does not publish
  or authorize automation to publish the crate.

## Alternatives considered

- Retain the working title indefinitely: rejected after explicit human approval
  of the project name.
- Apache-2.0 only: credible and includes an express patent grant, but offers less
  choice to Rust ecosystem users than the approved dual licence.
- MPL-2.0: provides file-level reciprocity, but that additional obligation was
  not selected.
- Remain unlicensed permanently: rejected as incompatible with the intended open
  Rust project. This was the accurate temporary state until the holder and exact
  notice were confirmed.

## Evidence

- Cargo defines `OR` in an SPDX expression as allowing the recipient to choose
  either licence. (`SRC-RUST-CARGO-LICENCE`)
- The canonical MIT and Apache-2.0 terms and obligations are recorded in the
  source register. (`SRC-OSI-MIT`, `SRC-ASF-APACHE-2.0`)

## Consequences and follow-up

- README, crate documentation, and generated user-facing material use the
  approved name while retaining the safety and non-affiliation notices.
- The confirmed notice, both licence files, and Cargo expression resolve the
  temporary unlicensed state without changing `publish = false`.
- Changing the name, licence, or confirmed notice requires a new human-approved
  decision record.
