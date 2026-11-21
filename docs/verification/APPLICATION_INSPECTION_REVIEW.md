# Shared application inspection review

- Date: 2026-11-22 Australia/Sydney
- Reviewer: Codex; independent Codex review recorded below when complete
- Scope: RFF-LIB-REQ-018 revision 1 and
  [ADR-0043](../adr/0043-shared-application-inspection.md)

## Input and selected need

Clean main `304570ec5341dfa65b8ab1da7a3e2b786afadb01` matched refreshed
origin/main. Exact input hosted run
[44746983137](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/44746983137),
job 136324722525, completed all 20 steps successfully. The prior retained
admission-priority checkpoint is complete; no unfinished implementation was
absorbed. Plain owned application observations were inaccessible after
registration, requiring external observation slots in existing consumers.

Pre-edit local baseline passed all six commands below and 285 tests.
Local versions remain rustc/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy
0.1.98. Existing max_width 100, comment width 80, individual too_many_lines
threshold 60 and unsafe-code prohibition remain unchanged. No dependency,
lint, CI or toolchain change is selected.

## Commands and evidence boundary

Required baseline, before and after implementation:

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
git diff --check
```

PowerShell sets `$env:RUSTDOCFLAGS = '-D warnings'` before rustdoc.
Raw run logs and temporary review aids are under ignored
`target/review-2026-11-22-application-access/`. They are local evidence;
this tracked record retains the reproducible procedure and actual result.
Local checks alone establish no hosted result.

## Implementation and focused result

Both generic runtime owners now return a shared application reference through
the existing record lookup/error semantics, including Failed records.
Lookup invokes no callback and touches no inbox or lifecycle state. It adds
no mutable accessor, service, storage or dependency. Application-defined
shared-reference effects and runtime-local equal-slot aliases remain explicit.

The seven tests in `tests/application_inspection.rs` pass under
`cargo test --locked --test application_inspection`; focused Clippy with
`cargo clippy --locked --test application_inspection -- -D warnings` passes.
The [traceability rows](TRACEABILITY.md#stage-5-shared-application-inspection--2026-11-22)
name each history: ordinary owned observations, every lifecycle state,
work/message returned failure, exact selected clearing, retained peer FIFO,
unknown slots and default/configured owners without an Application bound.
No historical consumer source changes.

`cargo test --locked --doc` passes four examples: two positive and two
compile-fail. Explicitly extracted negative probes fail only with E0596
(ordinary mutable application method) and E0502 (overlapping live inspection
and mutable owner use). Corrected counterparts compile successfully. The
positive E0596 counterpart reads the shared Vec; the E0502 counterpart moves
the last application use before the owner call. Diagnostics were read, not
inferred from rustdoc success. After `cargo build --locked --lib`, each probe
used `rustc --edition=2024 --emit=metadata`, the explicit
`target/debug/librust_flight_framework.rlib` extern, dependency search path
`target/debug/deps` and the corresponding extracted source/output directory
under `target/review-2026-11-22-application-access/probes/`.

To reproduce from a clean checkout, extract the contents of each
`compile_fail,E0596` / `compile_fail,E0502` fence from the respective Rust
source, remove each line's rustdoc `///` prefix and its one following space,
and wrap the whole snippet inside `fn main() { ... }`. Save as
`E0596_negative.rs` and `E0502_negative.rs` in the probe directory. Copy each
to its `_positive.rs` counterpart and apply the substitutions above. The
generic helper remains inside main; rustc still type-checks its body.
Wrapping is necessary for the otherwise statement-only Vec example and
avoids an unrelated missing-main diagnostic. Create the probe directory,
build the library, then compile all four files with the recorded arguments:

```powershell
$framework = 'E:/Projects/flight software framework'
$probeDirectory = "$framework/target/review-2026-11-22-application-access/probes"
New-Item -ItemType Directory -Path $probeDirectory -Force | Out-Null
cargo build --locked --lib
foreach ($name in 'E0596_negative', 'E0596_positive', 'E0502_negative', 'E0502_positive') {
    $probe = "$probeDirectory/$name.rs"
    rustc --edition=2024 --emit=metadata --extern "rust_flight_framework=$framework/target/debug/librust_flight_framework.rlib" -L "dependency=$framework/target/debug/deps" --out-dir $probeDirectory $probe
    Write-Output "$name exit=$LASTEXITCODE"
}
```

Inspect negative diagnostics for exactly the named error, exit 1, and
positive results for exit 0; a generic compile failure is insufficient.

Exact orchestration commands were
`python target/review-2026-11-22-application-access/verify_probes.py` and
`python target/review-2026-11-22-application-access/verify_independent.py`.
Their saved source, compiler arguments, diagnostics and JSON results retain
the actual local invocations; the API docs retain the negative sources.

## Independent consumer

Executed the Cargo commands and source/metadata comparisons from the
[guide recipe](../APPLICATION_INSPECTION.md#verify-the-consumer-outside-the-workspace)
at `C:/Users/drrsm/AppData/Local/Temp/rff-application-inspection-778690b6a11f4c0d94a325c01998a9ba`.
Offline locked all-target check, warnings-denied Clippy, all seven consumer
tests and metadata pass. Metadata confirms the independent workspace root,
one edition-2024 library member, two graph packages and the sole dependency
pointing to this framework checkout. Test source is exactly 12,886 bytes,
byte-identical, with SHA256
`677bce9068c30717d02a7a04054eb8437d3f6da724d84d5829790c8a48a24a06`.
No maintained consumer crate was added.

## Final local baseline and environment event

The required locked format/check/Clippy baseline passed. The first final
workspace-test invocation exited 101 because Windows Application Control
blocked `target/debug/deps/configuration_table-b4add3f84aa90cd7.exe` before
execution (OS error 4551). Read-only CodeIntegrity events 3033, 3077 and 3118
at 09:54:06 local confirm the environment block. No assertion failure was
reported for that executable. No security setting, file name or execution
path was changed. One identical workspace-test retry completed successfully:
296 tests, comprising 292 ordinary tests and four rustdoc examples. Final
warnings-denied rustdoc and whitespace passed afterwards. The failed attempt
and accepted retry logs are both retained; no baseline command was omitted.

## Review and publication boundary

Root Codex source review and independent Codex source review accept blobs:

- Runtime: `11b692a3a64e70d77007621d188a709ab1089d10`
- MessagingRuntime: `400e5418ca887ffe82f7a5f743fdc537faacb476`
- Consumer: `4a094251091bba07aa4b5df17e853bb5dcc82a35`

Source review covers ownership/lifetimes, all-state lookup, error preservation,
generic bounds, absence of callback/queue mutation, resource behavior and
meaningful independent histories. Root and independent Codex complete
source/document/diff review accepts all 15 changed/new files with no remaining
findings. Whole-tree review covers 58 Rust files, 28,166 physical lines and
1,511 comment-only lines, zero width findings and three unchanged waiver
sites. All 763 relative paths, 85 fragments and 256 exact test references in
124 Markdown documents resolve. All 12 changed Markdown documents pass
rendered content/structure review; browser pixel QA was not performed.
No publication is claimed yet; exact hosted evidence remains a later gate.
Shared inspection is neither arbitrary-code containment, output draining,
origin authentication nor recovery. These controlled consumers establish no
external adoption, operational suitability or API stability.
