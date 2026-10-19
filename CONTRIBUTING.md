# Contributing to World Kernel

World Kernel is a measured experiment, not a product. Its value is that a recorded claim can be
re-run and found wrong. A useful contribution makes the evidence clearer, repairs a demonstrated
problem, or reproduces a result. It does not need to make the project larger, and "this would be a
nice capability" is not a reason to start.

## Which part of the repository you are changing

The repository separates what ships from what measures, and a contribution belongs in exactly one of
them. Getting this wrong is the most common way to spend a maintainer's time.

| Location | What it is | Is it shipped? | A contribution here needs |
|---|---|---|---|
| `src/` | The library surface, the crate `world_kernel` | Yes | A failing contract test before the fix |
| `tests/` | The test suite, plus support code that is deliberately not shipped | No | The same gate as `src/`, plus a reason in the change |
| `examples/` | The runnable reproductions of recorded experiments | No | A result record that the example can regenerate |
| `experiments/` | Corpora, fixtures and machine records for each pre-registered experiment | No | A protocol written down before the measurement |

Two consequences catch people out:

- The measured transfer planner is **not** in `src/`. It lives in `tests/support/transfer_core.rs`
  because `docs/ADR-005-reduce-to-the-representation.md` moved it out of the shipped surface on
  purpose. The only shipped part of that work is `src/experience.rs`. Do not move the planner back.
- `experiments/consumer-transfer` is a **separate crate**, not a target of the root package. The root
  package depends on it as a dev-dependency, which is a dev-dependency cycle and is legal in cargo.
  That cycle is the point: it makes the consumer external, so the crate can reach
  `world_kernel::experience` and nothing else. Do not move it under `src/`, do not make it a normal
  dependency, and do not give it a path into `tests/`.

## Before you start

Read these in order. They are not background, they are the contract you would be changing.

1. [README.md](README.md) for the public overview and current status.
2. [AGENTS.md](AGENTS.md) for the working rules and the full verification gate.
3. [docs/SPEC.md](docs/SPEC.md) for the normative contract and the authorization state.
4. [docs/BOUNDARIES.md](docs/BOUNDARIES.md) for ownership, trust assumptions and the WK invariants.
5. [docs/EXPERIMENT.md](docs/EXPERIMENT.md) for the research question and the stop conditions.

**The current milestone sequence is complete and no new capability is authorised.**
[docs/SPEC.md](docs/SPEC.md) states it as: "No M6, and no new capability, is authorised." Open an
issue before starting a feature or protocol change.

## Set up

You need Git, a current stable Rust toolchain, and `jq` for the schema checks. No external database
or service is needed; SQLite is built through the bundled `rusqlite` feature.

```bash
git clone https://github.com/memolabs-dev/world-kernel.git
cd world-kernel
cargo test
```

The one ignored test, the live UNI contract test, is the exception. It needs a local UNI checkout and
an absolute `UNI_BIN` path, as documented in [docs/SPEC.md](docs/SPEC.md). It is ignored by default
so the default suite stays hermetic.

## The real gate

This is the gate, not a summary of it. It matches [AGENTS.md](AGENTS.md) and CI, and a pull request
that has not run all of it will be asked to.

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
cargo clippy --manifest-path experiments/consumer-transfer/Cargo.toml --all-targets -- -D warnings
cargo fmt --manifest-path experiments/consumer-transfer/Cargo.toml --check
```

The last two are separate on purpose. `experiments/consumer-transfer` is not a root target, so
`cargo clippy --all-targets` never sees it. Skip them and a change to the consumer ships unchecked.

**A clean `git status` after the gate is part of the proof.** Several examples rewrite their
checked-in records. If a record changed, regenerate it and include it, and say why it changed. If it
changed and you did not expect it to, that is the finding.

One rewrite is expected and is not a finding. `experiments/admission-benchmark/` records timings,
and timings move on every run, so its files come back modified after the gate even with no code
change of your own. Three `results.json` files under `experiments/` (admission-benchmark,
continuation, incremental) also carry an environment block naming the machine the measurement was
recorded on, and that block rewrites on a machine of a different arch or OS. CI restores the
benchmark directory and compares the JSON records with the environment identity stripped, because
the environment is provenance, not measurement. Do the same locally: restore the benchmark
directory with `git checkout -- experiments/admission-benchmark/`, leave the environment blocks
alone, and treat every other changed record as the finding.

## Invariants that tests enforce

These are not conventions. Each one is checked by a named test, and the test fails when the
invariant breaks. Read the test before you change the code it guards.

| Invariant | Enforced by |
|---|---|
| The transfer contract document and the moved implementation name the same variants | `tests/transfer_contract.rs` |
| The impact contract document and the measured code codes agree | `tests/m3_comparison.rs` |
| A recorded result file matches a fresh measurement, so a record cannot drift from the run | `tests/m3_comparison.rs`, `tests/wide_corpus.rs`, `tests/consumer_check.rs` |
| The rendered `RESULTS.md` agrees with the recorded JSON, and an unmet condition is never rendered as met | `tests/transfer_artifact.rs`, `tests/wide_corpus.rs`, `tests/consumer_check.rs` |
| The comparison systems are the files the protocol froze, by hash, so a separation cannot be manufactured by extending the thing being separated | `tests/wide_corpus.rs` |
| The consumer is external and uninformed, and the comparison can still fail | `tests/consumer_check.rs` |
| The corpus is the declared shape and the decisive subset is powered | `tests/wide_corpus.rs` |
| The default test suite needs no external service | the single ignored test is the live UNI contract test |

`tests/wide_corpus.rs` and `tests/consumer_check.rs` also each contain a test that proves their own
comparison can fail. If you make a comparison pass by weakening it, one of those is what catches you.

## Invariants that are only written down

These carry the same weight, and no test enforces them for you. They are in
[AGENTS.md](AGENTS.md) and [docs/BOUNDARIES.md](docs/BOUNDARIES.md).

- Keep the Kernel independent. Integrate UNI, Kollio and IntentLane through their public surfaces.
  Never edit their repositories as part of World Kernel work.
- Keep domain policy in adapters or experiment fixtures. The Kernel owns generic admission and
  history mechanics only.
- Preserve the distinction between impact and truth, assurance and authority, and internal commit and
  external effect. No part of the model may collapse them into one validity flag.
- A manifest or an evaluator may never award itself a stronger trust profile than the configuration
  grants, and a reconstruction may never bootstrap its own trust.
- A continuation package may never supply its own trust configuration or an expected head.
- A recorded human decision is never recomputed. Produce an obligation and change nothing.
- Adaptability is declared and defaults to false. A difference never becomes bridgeable because it
  looks bridgeable.
- No external effect dispatcher. The M1 gate was decided; adding one is not a contribution.

## Proposing new work

The project reduces or stops, and it records why. A proposal has to fit that shape or it will be
declined for a structural reason rather than a quality one.

A proposal should name:

1. **Which pre-registered condition it answers.** A stop or reversal condition already written down in
   [docs/EXPERIMENT.md](docs/EXPERIMENT.md) or one of the ADRs. If none exists, say so; that is a
   different kind of proposal and a slower one.
2. **The smallest testable change.** Usually a tranche, usually with a stop signal attached.
3. **The result that would stop the work.** A pre-registered stop condition, not a judgement call
   afterwards.

`docs/ADR-003-measure-before-building.md` recommended reducing the project away, and
`docs/ADR-004-the-facet-advantage-scales.md` reversed that because the reduction's own reversal case
fired. `docs/ADR-005-reduce-to-the-representation.md` then reduced again. Those are the pattern: a
pre-registered condition decides, and being wrong about it is recorded rather than quietly fixed.

## Pull request checklist

- Describe the observed problem and the evidence for it, not the intended design.
- Say which row of the table above your change belongs in.
- Name every gate command you ran and its outcome.
- Include regenerated records, and explain any that changed.
- Update [docs/SPEC.md](docs/SPEC.md) when a durable requirement or the authorization state changes.
- Update [docs/BOUNDARIES.md](docs/BOUNDARIES.md) when a proved guarantee or a trust assumption
  changes.
- Do not create another architecture overview. The existing documents are the overview.
- Call out any claim you could not verify.

By contributing, you agree that your contribution is licensed under either of
[MIT](LICENSE-MIT) or [Apache License 2.0](LICENSE-APACHE), at your option.
