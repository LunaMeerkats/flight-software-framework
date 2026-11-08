# Retain subscriber output on a shared writer

## Objective and acceptance

Extend the command-session writer policy under
[ADR-0039](docs/adr/0039-runnable-command-session.md) and RFF-LIB-REQ-014
revision 3. A short write followed by a writer error currently drains the
subscriber's typed output and loses its retry position. Reuse the result
presenter for one subscriber line and keep the selected slot occupied until
full acceptance. The peer can stage output in its own slot while the active
presenter holds the only mutable borrow of the writer. No additional queue
or arbitration is needed for one serial host.

## Verification and stopping point

Input `785c6610260c05c66b3c52207487084eb085a715` matches refreshed origin.
The locked local baseline passes. Independent Codex inspection confirms
input hosted run 42375973171 passed all 20 steps, 258 workspace tests and
the 222-line sample.

Exercise short progress, a returned error, exact suffix retry, peer staging,
selected-gate release and ordered lines on the same writer. Preserve the
result error regression and the 24-line successful transcript. Check, lint,
test and run identical source in an independent Cargo binary. Compare source
and transcript bytes, inspect metadata, and review source, rendered documents
and the complete diff. Publish the reviewed increment on `main` and inspect
exact hosted CI.

Stop at bounded subscriber presentation. The script exits on error. A serial
host retaining the presenter can choose another explicit turn. If verification
fails, preserve the evidence and withdraw only this run's incomplete changes.
