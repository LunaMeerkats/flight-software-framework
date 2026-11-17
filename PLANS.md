# Continue periodic peer work after a returned failure

## Objective and acceptance

Give a downstream serial host a runnable example of continuing recurring
work after a selected application returns a typed error. Existing tests
demonstrate peer availability and timer disposal. This slice joins repeated
peer releases and a shared diagnostic FIFO with the direct runtime's work
error and event attempt. Implement RFF-LIB-REQ-017 in one test-enabled example.

Verify the exact failure, only the selected cursor being disabled, the peer's
retained release, repeated heartbeat callbacks before and after dequeue, and
preservation of an older queue record when the failure event is rejected.

## Verification and stopping point

Run the locked baseline, focused example and identical independent binary.
Review the whole diff, source form, links and rendered documents. Record local
evidence and verify the exact hosted head after an ordinary main push.
Stop at two cooperative applications with one terminal failure. If the change
cannot pass, retain the research and withdraw only this run's incomplete edits.
