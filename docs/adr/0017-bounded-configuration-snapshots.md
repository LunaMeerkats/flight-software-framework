# ADR-0017: Bounded configuration snapshot lifecycle core

- Status: Accepted and implemented as the standalone ADR-0005 core
- Date: 2026-08-31
- Scope: Standalone in-memory validation, revision assignment, and rollback

## Context

At this decision checkpoint, Stage 2 was complete and ADR-0005 already defined
RFF-REQ-006 revision and rollback rules, but work callbacks had no configuration
access. Combining storage, a general application context, runtime ownership,
and a host parser in one increment would have made several unrelated API choices
at once.

The core must preserve immutable accepted values and bounded history without
requiring a generic value's interior mutability or heap use to be trusted.

## Decision

`ConfigurationTable<E, MAX_BYTES>` owns inline byte snapshots and retains one
`fn(&[u8]) -> Result<(), E>` mission validator. `ConfigurationSnapshot` exposes
only its original acceptance revision and immutable accepted byte slice. No
schema, encoding, file format, protocol, or typed application view is selected.
The table cannot be cloned; caller-owned snapshot clones are permitted.

The same function validates initial and replacement content. Its identity is
fixed by construction, but its purity, termination, external state, and error
storage are mission responsibilities. No callback panic containment is added.

Operations follow this order:

1. Reject input longer than `MAX_BYTES` before invoking mission validation.
2. Copy bounded bytes into a private candidate, then validate that content.
3. On replacement, check the `u64` revision high-water mark for exhaustion.
4. Only then replace active content, retain its predecessor in the sole
   rollback slot, and advance high-water. Initial acceptance is revision 1.

`ConfigurationError<E>` distinguishes oversize input, the exact concrete
validation rejection, revision exhaustion, and absent rollback. Each operation
documents which variants it can return. Validation rejection takes precedence
over revision exhaustion and changes none of the table's retained state.

Rollback takes the retained snapshot without revalidation, discards the
displaced active value, and consumes the slot. It preserves the restored
snapshot's original revision without lowering high-water. Equal content is a
new acceptance. Exhaustion does not prevent an available rollback, but rolling
back does not recover unused revision numbers. Revisions carry no table origin.

A table retains at most two snapshots, each with inline capacity `MAX_BYTES`;
replacement can also hold a bounded candidate. This is a content/storage-shape
bound, not an exact stack-usage guarantee: compiler moves and temporaries,
caller inputs and clones, and validator effects/errors are separate. The table
itself performs no heap allocation. The const bound must fit the host's resource
budget. A zero bound permits an empty value only if the validator accepts it.

## Alternatives considered

- **Generic value with borrowed access:** fewer byte-specific choices, but a
  generic value can contain shared mutable state and unbounded allocations.
  Defer until a concrete typed consumer establishes an enforceable contract.
- **Validator supplied on every operation:** smaller owner representation but
  permits accidental policy changes during replacement. Retain one function.
- **Stateful validator trait or closure owner:** can carry mission context but
  adds ownership and mutation semantics not needed by this core. Defer.
- **Runtime-wide configuration generic or wrapper now:** could begin proving
  application visibility, but requires a work/context design. Keep it separate.
- **Mutable active values, toggle rollback, or caller revisions:** reject
  because they violate ADR-0005's already accepted snapshot identity rules.

## Evidence and consequences

The authoritative inputs are local ADR-0005, RFF-REQ-006, the Stage 3 roadmap,
and the existing inline-bounded message representation. No fresh external
research or source reuse is needed; this decision adds no upstream claim.

Public tests cover initial validation, copied ownership, byte bounds, rejection
atomicity, repeated and equal-content acceptance, replacement of older history,
consume-once rollback, revision non-reuse, and concrete errors. Private tests
exercise final revision assignment, checked exhaustion, rejection precedence,
and callback suppression without exposing revision or validator overrides.

This ADR alone proves standalone transitions only. ADR-0018 later integrates
runtime ownership, application-visible safe points, restart retention, and no
automatic rollback following a returned application error. Their combined
evidence verifies RFF-REQ-006. This core still adds no persistence, schema
migration, host loading, redo, automatic rollback, or event emission.

## Risks and revisit conditions

Large inline capacities can exhaust stack resources. A validation function can
depend on changing external state or fail to return; an accepted snapshot is
not revalidated on rollback. Snapshot revision equality across tables does not
establish shared identity. These are documented limits, not containment claims.

Revisit when a concrete runtime consumer requires typed decoded values,
context-dependent validation, stronger origin identity, different bounded
storage, concurrency, or persistence. Preserve ADR-0005 behavior unless a
separate decision explicitly replaces it.
