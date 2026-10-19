# ADR 006: the identity of a consumed projection is typed and versioned

Date: 2026-10-17
Status: accepted
Relates to: [ADR-002](ADR-002-revisable-work.md), [ADR-003](ADR-003-measure-before-building.md),
[ADR-004](ADR-004-the-facet-advantage-scales.md), [M3-PROTOCOL.md](M3-PROTOCOL.md),
[IMPACT-CONTRACT.md](IMPACT-CONTRACT.md)

## What was found

ADR-002 states three rules about the identity of a consumption, and the engine implemented one of them.

> A narrower comparison than the whole canonical value requires a deterministic, **typed**, **versioned**
> projection from the trust registry. If the comparator's identity or version changes, the affected uses
> are revalidated; an old fingerprint is never reused as if its definition had not moved.

`NodeVersion.payload_type` is declared, documented as the source of `Facet::Field`'s typing, and read
by nothing in `src`, `tests` or `examples`. `Facet::fingerprint` hashed the comparator version, the facet
and the value, so two byte-identical payloads under two declared types produced one fingerprint and every
consumer of the second was reused. The section header of the constant was the only thing about the
definition that was versioned: `check_read` compared `read.comparator` against
`FACET_COMPARATOR_VERSION`, and the fingerprint did not change with the value it was projected from.

The derivation policy had two implementations and they disagreed. `directly_concerned`, which decides
what the frontier reaches, tested the record's engine **and** its target version;
`revise`'s reuse gate and the frontier's local predicate tested the evaluator's name and version
**only**. A record produced by another engine was therefore reached by the frontier and, in the same
pass, carried forward by the reuse gate with `consumed_facets_unchanged`.

Both are falsified from the closed arithmetic fixture in `tests/m3_projection_identity.rs`, and the two
cases fail on the parent commit:

- `limit` keeps its version and its exact bytes and only its declared type moves. `eligible_a` is reused.
- A record's `engine` is set to another string, with every consumed value unmoved. It is reused.

## The decision

**The projection is typed.** `Facet::fingerprint(payload_type, value)` folds the declared type of the
payload the value was read from. The same bytes under another declared type are a different consumption:
`check_read` returns `Changed`, and the consumer is re-evaluated rather than reused. `None` is for a
produced value with no declared node type behind it, and it is a value like any other rather than a
wildcard, so absence of a declaration is itself an ingredient.

**The projection is versioned as a whole.** `FACET_COMPARATOR_VERSION` moves to
`facet-canonical-json/2`. A record computed under `/1` is `DefinitionMoved` and revalidated rather than
compared, which is the behaviour ADR-002 already prescribed and this change is what makes it reachable.

**One predicate answers the question.** `definition_moved(record, evaluator)` covers the evaluator, its
version and the engine, and both the reuse gate and `directly_concerned` call it, so the two answers
cannot part company again. The trail gains `definition_trigger`, which names which part of the policy
moved: `evaluator name@version`, or `engine world-impact/sequential/0`.

**A write into a working snapshot does not re-declare a payload type.** The engine recomputes a node's
payload and keeps the declared type the snapshot gave it, and the application baseline in test support
does the same with the type of the node it already has. The parent commit rewrote both, which was
invisible while the type was unused and would otherwise create a second identity for a node inside one
pass, so that a read recorded against it could not be reproduced from the snapshot a later pass compares
against.

## Why the identity is not `hash(data)`

The question a revision asks is "is this the same consumption?", and `hash(data)` answers "are these the
same bytes". Those differ in both directions and both directions have a cost. Two payloads can be
byte-identical and mean different things, because the declared type names the interpretation: this is
the defect above, and a byte digest is structurally unable to see it. Two payloads can differ in a byte
nobody consumed and mean the same thing, which is the facet advantage [ADR-004](ADR-004-the-facet-advantage-scales.md)
was reversed for, and a node digest would destroy it.

So no second identity is introduced. The identity is the one the reuse decision already compared: the
projection of what was consumed, under the declaration it was read under, versioned as a definition. A
record carries exactly one fingerprint per read, and it is the fingerprint that decides reuse.

## What this refuses

- **No node-level content hash.** A `content_identity()` on the node would be a second identity in the
  crate with no consumer: the reuse gate compares reads, not nodes. Adding one would repeat the failure
  ADR-003 recorded, a capability nothing measured and nothing reads.
- **No similarity or embedding comparator**, which ADR-002 already refuses, and no change to the rule
  that a pinned read whose version moved still falls back to the current node. A dependency is what was
  consumed; the recorded artifacts depend on that rule and it is not touched.
- **No propagation, no sensitivity match, no assurance channel.** Only identity is changed here.
- **Not a new capability.** This is a defect fix inside rules ADR-002 already calls normative: no new
  public type, no new dependency, no new code in `IMPACT-CONTRACT.md`'s list, no new external effect. The
  status line in `docs/SPEC.md` that forbids a new capability is not contradicted, because nothing is
  added to the surface a consumer can use; something the surface already promised is made true.

## The stop condition, and where it is held

The programme step this implements names its own oracle and its own stop condition: the same corpus
produces the same identity, and if the identity moves while the data does not, the boundary between
content and procedure has been placed wrong.

It is held by tests rather than by a benchmark, and the reason is worth recording. There is no threshold
and no competing system in it: an identity that depends on the route or on the run is a defect, not a
measurement. An `experiments/` record would need a protocol, a corpus and a reading, and would then
assert a property with a number.

- `the_same_corpus_produces_the_same_identity_on_both_routes`: for one corpus, every read of every
  recomputed node is identical between the incremental pass and the full recompute, and two independent
  full recomputes of the same corpus are equal.
- `the_identity_of_a_projection_moves_with_its_ingredients_and_with_nothing_else`: the fingerprint is a
  function of the facet, the declared type, the value and its absence, and of nothing else; a canonical
  value does not depend on the order its keys were written in.
- `an_unconsumed_field_of_a_consumed_node_still_reuses_every_consumer`: the property ADR-004 restored the
  engine for, restated against a projection that now folds in more.

## The failing-first evidence

`cargo test --test m3_projection_identity` on the parent commit: two pass, three fail. The three are
`a_byte_identical_payload_under_another_declared_type_is_not_the_same_consumption` (reused instead of
recomputed), `a_record_produced_by_another_engine_is_revalidated_and_the_trail_names_it` (reused instead
of recomputed) and `a_record_computed_under_another_definition_is_revalidated` (the definition had not
moved). The last one is written against the literal `facet-canonical-json/1`, which is a historical fact
and stays in the file: it asserts that a record from the previous definition is revalidated, and it is
what makes the bump load-bearing rather than cosmetic.

`cargo test --test m3_projection_identity` after the change: six pass. The tree runs 224 tests, all
green; the new file contributes six of them and no other target's count changed.

## What moves, and what does not

**No execution count and no reuse decision moves.** No scenario in `experiments/incremental`,
`experiments/incumbent-comparison` or `experiments/wide-corpus` changes a declared type or an engine
version, so nothing in a recorded run flips, and
`the_recorded_result_file_matches_a_fresh_measurement` and
`the_recorded_reversal_case_matches_a_fresh_measurement` re-measure green.

**The surface line counts re-measure, because the runners count physical lines of the code.** They are
reproduced by the same commands, not edited:

| Surface | Recorded before | Re-measured |
|---|---|---|
| M3 production, `src/impact` | 1662 | 1734 |
| The engine as the comparison sees it, C3 | 1630 | 1702 |
| The application mechanism as it sees itself, A3 and B3 | 257 | 272 |
| Engine lines per application line | 6.34 | 6.26 |

The comparison's clause "at a comparable price" is met on both figures: the same 2 evaluator runs are
still bought for slightly fewer engine lines per application line. `docs/ADR-003-measure-before-building.md`
and `docs/ADR-004-the-facet-advantage-scales.md` quote 1630 and 257 as the figures recorded at their own
decisions and are left as the records they are; `docs/SPEC.md` and `docs/EXPERIMENT.md` carry the current
figure.

The comparator version moving **does** invalidate records persisted by an older build. That is the
intended behaviour rather than a cost: an old fingerprint is never reused as if its definition had not
moved, and the alternative is a silent reuse of a projection nobody can reconstruct. The bump is also
what makes this change observable in an existing deployment, since no checked-in artifact carries a facet
fingerprint that would have moved.

The application baseline keeps the declared type of a node it writes back. On the parent commit the same
rewrite was invisible, because the type was unused; with a typed projection it would have cost the
baseline one evaluation on `mutation_7_unchanged_world`, where the recorded artifact says zero. The
mechanism under measurement is unchanged and the artifact's execution counts are unchanged; the passing
`a_change_nothing_consumed_costs_nothing_anywhere` is the check.

## Consequences

- [IMPACT-CONTRACT.md](IMPACT-CONTRACT.md) Rule 5 states the typed and versioned projection from the
  consumer's side. No code is added or renamed, so its list stays as it was.
- `docs/SPEC.md` section 8c records the rule as part of M3 rather than as a new milestone section.
- The frontier and the reuse gate now give one answer about a moved derivation policy. A record produced
  by another engine is revalidated in both places, which is a small amount of extra work in the one case
  where they used to disagree, and never in a case where the data did not move.
- The programme's later steps (propagation, sensitivity, decision comparison) are untouched and unfunded.
  This ADR closes step 2 as a rule the engine enforces, not as a new mechanism.
