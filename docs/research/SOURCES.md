# Research sources and provenance

Access date for this initial register: **2026-08-05**.

Access date for the source-quality policy checkpoint: **2026-08-23**.

Only public primary sources are used below. “Adopt” means adopting a problem
boundary or locally designed behavior, not claiming compatibility. This
checkpoint uses factual paraphrases rather than translated NASA implementation
code. Any future source reuse must record provenance and licence obligations.

## SRC-NASA-CFS — Core Flight System bundle v7.0.1

- Title: Core Flight System — BUNDLE
- Organisation: NASA Goddard Space Flight Center Flight Software Systems Branch
- Source: <https://github.com/nasa/cFS/blob/088b2fa828db9ff7e00733f1908e0eeb59f66ce3/README.md>
- Version: official v7.0.1 tag, commit `088b2fa828db9ff7e00733f1908e0eeb59f66ce3`
- Informed: bundle composition; distinction between framework, mission-selected
  applications, lab examples, and a flight distribution; NASA's warning that
  the bundle is a starting point and mission verification remains necessary.
- Local treatment: adopt cautious scope and responsibility analysis; reject
  parity, operational-readiness, endorsement, and distribution-equivalence
  claims.

## SRC-NASA-CFE — cFE Application Developers Guide v7.0.1

- Title: cFE Application Developer's Guide
- Organisation: NASA
- Source: <https://github.com/nasa/cFE/blob/c5fb2b4d540bd55eb6c3707da7dd13eee679d4dd/docs/cFE%20Application%20Developers%20Guide.md>
- Version: cFE v7.0.1 tag, commit `c5fb2b4d540bd55eb6c3707da7dd13eee679d4dd`
- Informed: Executive, Software Bus, Event, Table, Time, and File Service
  responsibilities; application identity/lifecycle; published message
  interfaces; bounded pipe concepts; staged table validation/activation; time
  as a service.
- Local treatment: adopt the responsibility questions and separation concerns;
  independently design Rust behavior and reject API/source translation or
  implied behavioral compatibility.

## SRC-NASA-OSAL — Operating System Abstraction Layer v7.0.1

- Title: Core Flight System: Operating System Abstraction Layer
- Organisation: NASA
- Source: <https://github.com/nasa/osal/blob/d2d877a69cff47452bcca274b309147d48e6c16f/README.md>
- Version: OSAL v7.0.1 tag, commit `d2d877a69cff47452bcca274b309147d48e6c16f`
- Informed: OS services are a distinct abstraction responsibility in cFS.
- Local treatment: retain conceptual separation between framework policy and
  host OS mechanisms; reject an empty OSAL-shaped crate before a second backend
  or concrete port requires it.

## SRC-NASA-PSP — Platform Support Package v7.0.1

- Title: Core Flight System: Platform Support Package
- Organisation: NASA
- Source: <https://github.com/nasa/PSP/blob/c4b3b0b65b119e106481ad8e20976ae4d7f554e3/README.md>
- Version: PSP v7.0.1 tag, commit `c4b3b0b65b119e106481ad8e20976ae4d7f554e3`
- Informed: platform-specific functionality is distinct from the executive and
  OS abstractions.
- Local treatment: keep future platform effects behind narrow evidence-driven
  interfaces; reject claiming PSP equivalence or creating speculative hardware
  layers.

## SRC-RUST-UNSAFE — The Rust Reference: `unsafe`

- Title: The `unsafe` keyword
- Organisation: The Rust Project
- Source: <https://doc.rust-lang.org/stable/reference/unsafe-keyword.html>
- Version: stable documentation accessed 2026-08-05
- Informed: unsafe code introduces proof obligations not verified by the
  compiler.
- Local treatment: ordinary local framework crates should forbid unsafe code;
  this policy does not inspect or constrain dependency internals. Any future
  local exception requires isolation, invariants, evidence, an ADR, and human
  review.

## SRC-RUST-CHANNEL — Rust standard-library synchronous channel

- Title: `std::sync::mpsc::sync_channel`
- Organisation: The Rust Project
- Source: <https://doc.rust-lang.org/std/sync/mpsc/fn.sync_channel.html>
- Version: stable documentation accessed 2026-08-05
- Informed: stable Rust provides a bounded channel whose full-buffer behavior is
  blocking for `send`, with one receiver and cloneable senders.
- Local treatment: retain it as one future implementation option; reject using
  library defaults as the bus overflow contract or mistaking one-receiver
  channels for publish/subscribe fan-out.

## SRC-RUST-TIME — Rust standard-library time types

- Title: `std::time::Instant` and `std::time::SystemTime`
- Organisation: The Rust Project
- Sources: <https://doc.rust-lang.org/std/time/struct.Instant.html> and
  <https://doc.rust-lang.org/std/time/struct.SystemTime.html>
- Version: stable documentation accessed 2026-08-05
- Informed: monotonic and non-monotonic system-clock behavior have different
  semantics and platform considerations; wall-clock access is not a
  deterministic test clock.
- Local treatment: inject framework time and use manual simulated time in tests;
  do not infer real-time guarantees.

## SRC-RUST-CARGO — Cargo workspaces

- Title: Workspaces — The Cargo Book
- Organisation: The Rust Project
- Source: <https://doc.rust-lang.org/cargo/reference/workspaces.html>
- Version: stable documentation accessed 2026-08-05
- Informed: workspaces share a lockfile and target directory and can centralise
  package metadata and lint declarations; each member must opt into workspace
  lints explicitly.
- Local treatment: use the smallest workspace needed once a crate boundary is
  proven, require each member to inherit workspace lints, and do not create a
  multi-crate topology in this checkpoint.

## SRC-RUST-CARGO-LICENCE — Cargo licence metadata

- Title: The Manifest Format — licence and licence-file fields
- Organisation: The Rust Project
- Source: <https://doc.rust-lang.org/cargo/reference/manifest.html#the-license-and-license-file-fields>
- Version: stable documentation accessed 2026-08-26
- Informed: Cargo uses SPDX expressions, and `OR` permits recipients to choose
  either listed licence.
- Local treatment: use `MIT OR Apache-2.0` as the Cargo expression now that the
  holder and files are complete; keep publication disabled as a separate policy.

## SRC-OSI-MIT — The MIT License

- Title: The MIT License
- Organisation: Open Source Initiative
- Source: <https://opensource.org/license/mit>
- Version: SPDX identifier `MIT`, accessed 2026-08-26
- Informed: permissive use and redistribution terms, required notice retention,
  and warranty/liability disclaimer.
- Local treatment: apply as one recipient-selectable branch of the repository's
  dual licence, using the exact confirmed copyright notice.

## SRC-ASF-APACHE-2.0 — Apache License, Version 2.0

- Title: Apache License, Version 2.0
- Organisation: Apache Software Foundation
- Source: <https://www.apache.org/licenses/LICENSE-2.0.txt>
- Version: Apache License 2.0, January 2004; accessed 2026-08-26
- Informed: copyright and patent grants, redistribution conditions, trademark
  limitation, and warranty/liability disclaimer.
- Local treatment: apply the unmodified terms as one recipient-selectable branch
  of the repository's dual licence.

## SRC-RUST-STYLE — Rust Style Guide

- Title: The Rust Style Guide
- Organisation: The Rust Project
- Source: <https://doc.rust-lang.org/style-guide/>
- Version: stable documentation accessed 2026-08-23
- Informed: default Rust formatting uses a 100-character maximum line width;
  comment-only source lines should use the smaller of an 80-character limit or
  the enclosing maximum; rustfmt follows this guide but does not make every
  project-specific source-quality decision.
- Local treatment: adopt stable 100-column formatting and an 80-column
  comment-prose review default; adapt physical-line enforcement to review
  because URLs, literals, macros, and generated material require explicit
  policy rather than a blanket textual rule.

## SRC-RUSTFMT-CONFIG — rustfmt configuration

- Title: Configuring Rustfmt
- Organisation: The Rust Project
- Source: <https://rust-lang.github.io/rustfmt/>
- Version: rustfmt 1.9.0-stable, used and accessed 2026-08-23
- Informed: repository or parent `rustfmt.toml` files configure rustfmt; stable
  options are usable on stable toolchains, while unstable options require a
  nightly toolchain and explicit opt-in.
- Local treatment: adopt only the stable `max_width = 100` option in this
  checkpoint; reject nightly-only formatting options and do not treat rustfmt
  success as proof that every physical line fits the policy.

## SRC-RUST-CLIPPY-LINES — Clippy function-length configuration

- Titles: Clippy lint configuration; `too_many_lines` lint
- Organisation: The Rust Project
- Sources:
  <https://doc.rust-lang.org/clippy/lint_configuration.html#too-many-lines-threshold>
  and
  <https://rust-lang.github.io/rust-clippy/rust-1.96.0/index.html#too_many_lines>
- Version: Clippy 0.1.96 / Rust 1.96, used and accessed 2026-08-23
- Informed: Clippy exposes a configurable function-line threshold whose default
  is 100 and applies it through the allowed-by-default `too_many_lines` lint.
- Local treatment: adapt the threshold to the locally selected 60-line review
  trigger and deny the individual lint for all targets. Retain narrow reasoned
  expectations for cohesive chronological tests. Reject treating this line
  count as a complexity, correctness, certification, or agency-compliance
  metric.

## SRC-RUST-CARGO-LAYOUT — Cargo package layout

- Title: Package Layout — The Cargo Book
- Organisation: The Rust Project
- Source: <https://doc.rust-lang.org/cargo/guide/project-layout.html>
- Version: stable Cargo documentation accessed 2026-08-23
- Informed: Cargo assigns conventional locations to library, binary, example,
  benchmark, and integration-test targets and documents target/module naming
  conventions.
- Local treatment: adopt the standard single-package layout already present;
  add no crate or directory until an independently coherent boundary requires
  it.

## SRC-RUST-API-GUIDELINES — Rust API naming and documentation guidance

- Titles: Rust API Guidelines — Naming; Documentation
- Organisation: Rust library-team authors and Rust project contributors
- Sources: <https://rust-lang.github.io/api-guidelines/naming.html> and
  <https://rust-lang.github.io/api-guidelines/documentation.html>
- Version: public guidelines accessed 2026-08-23
- Informed: conventional casing and names distinguish modules, types, values,
  and constants; public fallible behavior should document error conditions.
- Local treatment: adopt the naming vocabulary and explicit public-behavior
  documentation principles; apply examples and metadata proportionately to
  this unpublished pre-v0.1 crate rather than treating the checklist as a
  mandatory lint group.

## SRC-JPL-POWER-TEN — The Power of Ten

- Title: The Power of Ten — Rules for Developing Safety Critical Code
- Author: Gerard J. Holzmann, NASA/JPL Laboratory for Reliable Software
- Source: <https://spinroot.com/gerard/pdf/P10.pdf>
- Version: IEEE Computer, June 2006; public author copy accessed 2026-08-23
- Informed: the paper's fourth rule uses one printed page, typically about 60 C
  lines, as a function-size bound intended to improve unit-level understanding
  and analysis; the paper explicitly describes rules primarily targeting C and
  safety-critical software.
- Local treatment: adapt 60 Clippy-counted lines as a review threshold for this
  experimental Rust repository. Reject direct applicability, a universal NASA
  mandate, line compression, artificial helper extraction, and any claim of
  JPL or NASA compliance.

## SRC-NASA-SOURCE-QUALITY — NASA coding and assurance guidance

- Titles: NPR 7150.2D — NASA Software Engineering Requirements; SWE-061 —
  Coding Standards; NASA-STD-8739.8B — Software Assurance and Software Safety
  Standard
- Organisation: NASA
- Sources: <https://swehb.nasa.gov/spaces/SITE/pages/123601159/NPR+7150.2D>,
  <https://swehb.nasa.gov/spaces/7150/pages/16450283/SWE-061+-+Coding+Standards>,
  and <https://standards.nasa.gov/standard/NASA/NASA-STD-87398>
- Versions: NPR 7150.2D; NASA-STD-8739.8B dated 2022-09-08; public pages
  accessed 2026-08-23
- Informed: NASA projects select, define, follow, and assess project coding
  methods and standards; official guidance discusses structure, module size,
  formatting, naming, comments, reviews, and static-analysis evidence.
- Local treatment: adopt the governance principles of an explicit local
  standard, tool-supported checks, review, and recorded evidence. Reject NASA
  applicability, assurance, endorsement, certification, and transplantation of
  language- or classification-specific requirements into this Rust experiment.

## SRC-ECSS-SOFTWARE-QUALITY — ECSS software engineering and assurance

- Titles: ECSS-E-ST-40C Rev.1 — Software; ECSS-Q-ST-80C Rev.2 — Software
  product assurance
- Organisation: European Cooperation for Space Standardization
- Sources:
  <https://ecss.nl/standard/ecss-e-st-40c-rev-1-software-30-april-2025/> and
  <https://ecss.nl/standard/ecss-q-st-80c-rev-2-software-product-assurance-30-april-2025/>
- Version: both dated 2025-04-30 and accessed 2026-08-23; each supersedes its
  preceding revision
- Informed: the standards cover software engineering and product assurance
  across the lifecycle and explicitly provide for tailoring to project
  characteristics and constraints.
- Local treatment: adapt lifecycle-wide review, verification, traceability, and
  project-tailoring principles. Reject applicability to this non-operational
  experiment, ECSS compliance claims, and any implication that ECSS prescribes
  this repository's Rust layout or 60-line threshold.

## SRC-JAXA-SOFTWARE — JAXA Software Development Standard

- Title: JERG-0-049D (E) — Software Development Standard
- Organisation: Japan Aerospace Exploration Agency
- Source: <https://sma.jaxa.jp/TechDoc/Docs/E_JAXA-JERG-0-049D.pdf>
- Version: JERG-0-049D English translation, 2023-03-30; accessed 2026-08-23
- Informed: section 5.3.8 calls for project-defined coding standards, source
  review, static analysis, recorded unit-test results, and module cyclomatic-
  complexity criteria within the standard's project classification scheme.
- Local treatment: adapt explicit coding rules, review, static analysis, and
  recorded evidence. Reject JAXA applicability or compliance and do not import
  its project-class thresholds or substitute Clippy's cognitive-complexity lint
  for a validated cyclomatic-complexity measure.

## SRC-AUS-SPACE-SOFTWARE — Australian public software guidance boundary

- Titles: Software developers and engineers; Australian endorsement of ECSS
  recommendations
- Organisation: Australian Space Agency
- Sources: <https://www.space.gov.au/software-developers-and-engineers> and
  <https://www.space.gov.au/news-and-media/new-recommendations-to-boost-global-space-opportunities-for-australia>
- Version: public pages accessed 2026-08-23
- Informed: the careers page describes software design, programming, testing,
  implementation, and maintenance; the later public notice describes Australian
  endorsement of ECSS recommendations. Neither page prescribes Rust naming,
  module layout, or a function-size threshold.
- Local treatment: record that the bounded public search found no independent
  Australian Space Agency code-style standard applicable to this repository.
  Do not invent one or imply Australian certification; evaluate the cited ECSS
  sources on their own terms.
