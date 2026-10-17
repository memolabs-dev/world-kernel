# World Kernel agent contract

This repository is the independent World Kernel experiment. It is not part of IntentLane, UNI or
Kollio.

## Read order

1. Read `docs/SPEC.md` before planning or changing behavior. It is the normative product and protocol
   specification, and it names the active milestone. Read `docs/ADR-001-portable-continuation.md` and
   `docs/M2-PROTOCOL.md` before working on continuation, and `docs/ADR-002-revisable-work.md` and
   `docs/M3-PROTOCOL.md` before working on impact, and `docs/ADR-003-measure-before-building.md` plus
   `docs/ADR-004-the-facet-advantage-scales.md` before adding any capability, because the reduce-or-stop
   decision was reversed and M4 is authorised to tranche 1. `docs/M4-PROTOCOL.md` is pre-registered and
   must be read before working on branches. `docs/M5-PROTOCOL.md` is pre-registered and must be read
   before working on transfer, together with `docs/ADR-005-reduce-to-the-representation.md`, which moved
   the planner out of the crate. The impact rules are written down in `docs/IMPACT-CONTRACT.md` and the
   transfer rules in `docs/TRANSFER-CONTRACT.md`.
2. Read `docs/BOUNDARIES.md` when work touches ownership, trust, UNI, Kollio or external state.
3. Read `docs/EXPERIMENT.md` when work touches benchmarks, falsification or claims about value.
4. Inspect the code and tests before relying on a document's description of current behavior. Running
   code wins when a status line has drifted.

## Current checkpoint

- Baseline commit: `c24b3fc`. The admitted-change slice, UNI exact-candidate collector, SQLite rollback
  and restart recovery are implemented.
- The M1 equal-information admission benchmark is implemented and recorded at `5003e90`: an 80-case
  versioned corpus, three equal-information systems, three test-only input ablations, and a checked-in
  result in `experiments/admission-benchmark/`.
- All ten M1 acceptance criteria pass. A, B and C reach the same decision in all 80 cases.
- The M1 continuation decision was taken on 2026-09-27 in `docs/ADR-001-portable-continuation.md`:
  continue on portable continuity rather than on admission superiority. The M1 negative result stands
  unchanged.
- M2 is implemented and recorded at `bc29a22`: `docs/M2-PROTOCOL.md` and `experiments/continuation/`.
  All 24 pre-registered sequences are covered for C2, 4 of them for C2 only because A2 and B2 have no
  export of their own yet, so no M2 cost comparison exists.
- M3 is recorded at `docs/M3-PROTOCOL.md` and `experiments/incremental/`. 23 of the 24 stories are
  covered and 1 is partial: M3-08 stays partial because the impact engine carries no assurance channel
  and granting it one would fold assurance into authority. H3-Transfert is measured at `tests/m3_second_domain.rs`
  against a frozen `src/impact`; H3-Utilite is unreachable, because it needs a consented human observation.
- The equal-information comparison ran on 2026-09-27 and is recorded in
  `experiments/incumbent-comparison/`, produced by `cargo run --example incumbent_comparison`. R3, A3, B3
  and C3 reached the same observable result on all 7 scenarios. A3 and B3 avoided the same work as C3 on
  6 of them; on the 7th the core ran 0 evaluators where the application ran 2, because a consumed facet is
  a narrower cache key than a whole value.
- The reduce-or-stop decision is recorded in `docs/ADR-003-measure-before-building.md`, which recommends
  that there be no M4. Two of the three pre-registered clauses are met; the clause "the core saves nothing the application could
  not already do" is false and stays recorded as false. The reduction is recommended on the narrower ground
  that every other capability the core produces is unconsumed by the fixture. Its reversal case is stated
  with it: on a graph of wide nodes the facet saving grows, and one example runner re-measures it.
- The reduction decided in ADR-003 was applied and then **reversed** on 2026-09-27 by ADR-004, because the
  reversal case ADR-003 itself required fires: the consumed-facet advantage is linear in the number of
  consumers and unbounded, 10 000 avoided evaluations at a fan-out of 10 000 against 1630 lines. One
  unconsumed field is enough. `src/impact` is restored; the Kernel is admission, history, portable
  continuity, the impact engine and the adapters. The rules are in `docs/IMPACT-CONTRACT.md` and
  `the_contract_document_and_the_measured_codes_agree` still binds that document to the engine.
- The identity of a consumption is typed and versioned, and one predicate decides it, recorded in
  `docs/ADR-006-typed-projection-identity.md`. `Facet::fingerprint` folds the declared type of the payload a
  value was read from, `FACET_COMPARATOR_VERSION` moved to `facet-canonical-json/2` so a record computed
  under the previous definition is revalidated rather than compared, and `definition_moved` covers the
  evaluator, its version and the engine for both the reuse gate and `directly_concerned`. Two defects were
  found in the code and falsified in `tests/m3_projection_identity.rs` before the fix: a byte-identical
  payload under a moved declared type was reused, and a record produced by another engine was carried
  forward while the frontier considered it reached. No execution count and no reuse decision in a recorded
  run moves, and the facet advantage is unchanged; the engine and application surfaces re-measure at 1702
  and 272 lines from 1630 and 257, and `docs/ADR-003` and `docs/ADR-004` keep the figures recorded at their
  own decisions.
- M5 tranche 1 is implemented in `src/experience.rs`, with `tests/transfer_plan.rs`,
  `tests/transfer_benchmark.rs`, `tests/transfer_contract.rs` and a checked-in
  `experiments/transfer-benchmark/` result. 30 closed cases, zero false direct transfers, and a **tie**
  with a competent non-Kernel baseline.
  `schemas/experience-capsule-v0.experimental.schema.json` is strict and a test binds it to the types.
- The M5 continuation gate is scored, not asserted. Condition 5 is scored on complexity under Amendment 1
  in `docs/M5-PROTOCOL.md`, which leaves the pre-registered table untouched: `kernelSurface` is 1007 lines
  and `baselineA` is 515, a ratio of 1.96, and the brief nominates that measure twice. The gate reads
  **three met, two not met, two unreachable**, so **the brief's reduction trigger fires and M5 is to be
  reduced to the smallest useful portable experience representation.** Tranche 2 is not funded. Nothing
  measured on this corpus separated the Kernel from the baseline on any outcome.
- That reduction is applied by `docs/ADR-005-reduce-to-the-representation.md`: the planner moved from
  `src/transfer.rs` to `tests/support/transfer_core.rs`, `pub mod transfer` is gone, and the shipped M5
  surface is `src/experience.rs` alone at 323 lines. The moved file was not rewritten, so the recorded
  artifact still reproduces 1007 and no score was re-run. `docs/TRANSFER-CONTRACT.md` carries the rules a
  consumer would otherwise have to read 684 lines of test support for, and
  `the_transfer_contract_and_the_moved_implementation_agree` binds it to both enums it describes.
- `tests/transfer_artifact.rs` binds the rendered `RESULTS.md` to the recorded `results.json`: every
  condition the JSON calls met is rendered as met, an unmet condition is never rendered as met,
  unreachable is kept distinct from not met, and the verdict line's three counts must cover all seven
  conditions. Run `cargo run --example transfer_benchmark -- --render-only` after changing the renderer.
- M4 tranche 1 is implemented in `src/branch.rs` and `tests/branch_convergence.rs`. A combination of two
  individually valid branches is blocked by a constraint check on the reconstructed candidate, and a
  composition of exactly the limit is admissible. The oracle in `tests/support/branch_fixture.rs` is a
  separate program over integers that reads no expected value.
- The first ADR-005 reversal condition was executed on 2026-09-28 and did not fire. The pre-registration is
  `docs/CONSUMER-CHECK-PROTOCOL.md`, the record is `experiments/consumer-transfer/RESULTS.md`, and the
  consumer is the separate crate `experiments/consumer-transfer`, which reaches `world_kernel::experience`
  and nothing else. 16 cases, three independent measurements, the consumer and `plan_transfer` both
  matching the declared expectation on all 16, zero reversals. `tests/consumer_check.rs` fails the build if
  the consumer reads anything it was not given, and `the_comparison_detects_a_wrong_consumer` proves the
  comparison can fail.
- The second ADR-005 reversal condition was executed on 2026-09-28 and **did** fire. The pre-registration is
  `docs/WIDE-CORPUS-PROTOCOL.md`, amended once for composition and for no other reason, and the record is
  `experiments/wide-corpus/RESULTS.md`. 96 cases over the full `ConditionKind` alphabet, 26 separations from
  a threshold of 1, all 26 on cases the baseline's model cannot carry and none on the 33 both models
  express, with a new independent oracle siding with the planner on all 96. The recorded 30-case tie was
  scoped, not wrong. `tests/wide_corpus.rs` freezes the baseline and the planner by hash, so a separation
  cannot be manufactured by extending the thing being separated. Run 0, which drew one expressible case in
  96, is kept verbatim as `experiments/wide-corpus/results-run0-defect.json` and is not a result.
- Boundaries that survive: the Kernel stores digests and never object bytes, a reconstruction cannot
  bootstrap its own trust, and no validity flag exists anywhere in the M3 model.

## Working rules

- Keep the Kernel independent. Integrate UNI and Kollio through their public surfaces. Do not edit
  their repositories as part of World Kernel work.
- Keep domain policy in adapters or experiment fixtures. The Kernel owns generic admission and
  history mechanics only.
- Preserve the distinction between impact and truth, assurance and authority, internal commit and
  external effect.
- Add no external effect dispatcher. The M1 gate was decided; M2 is an experiment, not a platform.
- M4 is authorised to tranche 1 only, and only the stories in `docs/M4-PROTOCOL.md`. Tranches 2 to 8 and
  the 32-story matrix are not authorised. If the blocked composition ever needs a second impact engine,
  reduce rather than proceed; that is the pre-registered stop signal.
- Never let a branch absorb the whole source when only part was selected, and never write a source head
  in as an ancestor of a partial adoption.
- Never let a continuation package supply its own trust configuration or an expected head.
- Never collapse history, currency, business verdict, authority and coverage into one validity flag, and
  never let a manifest or an evaluator award itself a stronger trust profile than the configuration grants.
- Never recompute a recorded human decision. Produce an obligation and change nothing.
- Never collapse an unknown fact, an absent fact, an unobserved capability and an undeclared key into one
  state. Never let a source assurance become a target assurance, and never let a difference become
  bridgeable because it looks bridgeable: adaptability is declared and defaults to false.
- Use test-driven development for behavior changes. A failing contract test must precede the fix.
- Update `docs/SPEC.md` when a durable requirement or milestone status changes. Update
  `docs/BOUNDARIES.md` when a proved guarantee or trust assumption changes. Do not create another
  architecture overview.
- Keep unrelated user changes intact. Do not push or modify remote state without explicit approval.

## Verification

Run all of these before handing work back:

```bash
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check
cargo run --example vertical_slice
cargo run --example admission_benchmark
cargo run --example m2_continuation
cargo run --example m3_revision
cargo run --example incumbent_comparison
cargo run --example m4_branch
cargo run --example transfer_benchmark
cargo run --example consumer_check
cargo run --example wide_corpus
jq empty schemas/world-change-v0.experimental.schema.json
jq empty schemas/experience-capsule-v0.experimental.schema.json
```

`experiments/consumer-transfer` is a separate crate, not a target of the root package. Lint and format it
on its own, or a change to the consumer ships unchecked:

```bash
cargo clippy --manifest-path experiments/consumer-transfer/Cargo.toml --all-targets -- -D warnings
cargo fmt --manifest-path experiments/consumer-transfer/Cargo.toml --check
```

The root package reaches the consumer as a dev-dependency, which is a dev-dependency cycle and is legal in
cargo. It is what makes the consumer external: the crate can reach `world_kernel::experience` and nothing
else in this repository. Do not move `consumer-transfer` under `src/`, do not make it a normal dependency,
and do not give it a path into `tests/`.

Record `UNI_BIN` as an absolute path when running the live UNI contract test. The runner executes the
binary with the declared workspace as its working directory, so a relative path cannot resolve.

`incumbent_comparison` writes `experiments/incumbent-comparison/results.json` and `RESULTS.md`, and
`tests/m3_comparison.rs` asserts the checked-in result matches a fresh measurement, so the artifact cannot
drift from the run. If that test fails, re-run the example rather than editing the artifact.

When changing `ProcessUniRunner`, also build UNI and run the ignored live contract test as documented
in `docs/SPEC.md`.

