# World Kernel

[![CI](https://github.com/memolabs-dev/world-kernel/actions/workflows/ci.yml/badge.svg)](https://github.com/memolabs-dev/world-kernel/actions/workflows/ci.yml)
[![License: MIT or Apache-2.0](https://img.shields.io/badge/license-MIT%20or%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![Status: experimental](https://img.shields.io/badge/status-experimental-orange.svg)](docs/SPEC.md)

World Kernel is a Rust research prototype for admitting and recording grounded changes inside a
declared digital scope. A change connects an exact candidate, the observations used to prepare it,
an assurance result, current authority, an atomic commit and a replayable receipt.

The project asks a narrow question: can another actor accept, reject or resume an exact change from
public records without trusting the first actor's conversation or private reasoning?

World Kernel is not an agent framework, a model router, a policy engine or a universal source of
truth. Producers propose. Assurance providers evaluate. The Kernel admits and records a change inside
the World it owns.

## Status

This repository is experimental. It has no release, is not published to crates.io and makes no
production-readiness claim. The current pre-registered research sequence is complete and no further
milestone is pre-registered. The code, tests, protocols and recorded results remain here so the claims
can be inspected and reproduced.

The main result is mixed by design. Several capabilities were established, but comparisons against
competent application-specific baselines often tied. One later 96-case corpus found 26 cases that the
portable transfer model could represent and the frozen baseline could not. That measures a capability
difference, not user value or a reason to expand the project.

The exact status and stop conditions live in [docs/SPEC.md](docs/SPEC.md). The shorter experimental
record is in [docs/EXPERIMENT.md](docs/EXPERIMENT.md).

## Run the vertical slice

You need a current stable Rust toolchain. `jq` is only required for the schema checks in the full
verification gate.

```bash
git clone https://github.com/memolabs-dev/world-kernel.git
cd world-kernel
cargo run --example vertical_slice
```

The example creates an in-memory World, observes a versioned document, binds a candidate to a simulated
assurance result, submits one grounded change, prints the receipt and verifies that replay reconstructs
the live snapshot.

The central mutation seam is:

```rust
kernel.submit(&grounded_change, &authority)
```

It returns either a typed rejection or a receipt. `Kernel::snapshot` reads the current projection.
`Kernel::replay` rebuilds it from recorded events without invoking a model or an external effect.

This repository is meant to be cloned and studied as an experiment. If you want to exercise the
library from another local crate, use a path dependency:

```toml
[dependencies]
world-kernel = { path = "../world-kernel" }
```

## What is implemented

- Admission of an exact candidate against current authority, trusted assurance, current revisions,
  declared context coverage, patch preconditions and idempotency.
- Atomic SQLite publication of the projection, event and stored outcome.
- Replayable history and portable continuation records.
- An impact engine that keeps impact, truth, assurance, authority and coverage separate.
- A constrained branch-convergence experiment that evaluates the reconstructed candidate against the
  target's own rules.
- A portable experience representation. The measured transfer planner remains test support rather than
  part of the shipped library surface.
- Read-only integration seams for UNI and Kollio that preserve ownership in those products.

## Recorded experiments

Every result below has a checked-in machine record, a human-readable account and a runnable example or
test. The CI workflow reruns the examples and rejects drift in recorded artifacts.

| Experiment | Recorded finding | Entry point |
|---|---|---|
| Admission | 80 cases, no decision difference between the Kernel and two competent application baselines | [result](experiments/admission-benchmark/RESULTS.md) |
| Portable continuity | 24 pre-registered sequences covered for the Kernel; no baseline cost comparison was possible | [result](experiments/continuation/RESULTS.md) |
| Incremental revision | 23 of 24 stories covered and one partial; the equal-information comparison tied on observable results | [result](experiments/incremental/RESULTS.md) |
| Branch convergence | A composition of two individually valid branches is blocked when the reconstructed candidate violates the target rule | [result](experiments/branch-convergence/RESULTS.md) |
| Transfer benchmark | 30 closed cases, zero false direct transfers and a tie with the baseline | [result](experiments/transfer-benchmark/RESULTS.md) |
| External consumer check | 16 cases, three independent measurements and no reversal of the reduction decision | [result](experiments/consumer-transfer/RESULTS.md) |
| Wide transfer corpus | 96 cases, 26 separations where the baseline model could not carry the case, and agreement on all 33 cases both models could express | [result](experiments/wide-corpus/RESULTS.md) |

Run an individual experiment with its matching example:

```bash
cargo run --example admission_benchmark
cargo run --example incumbent_comparison
cargo run --example m4_branch
cargo run --example transfer_benchmark
cargo run --example consumer_check
cargo run --example wide_corpus
```

## Scope limits

The prototype has no external effect dispatcher, signatures, distributed authority, negative-query
read sets or generic merge protocol. `AuthoritySource` is checked synchronously at admission but is not
a versioned authority ledger. The UNI collector detects ordinary mutation and post-verification
substitution, but it does not provide an immutable filesystem snapshot against a concurrent A-B-A
mutation. The host process and configured UNI binary remain trusted.

These are experimental limits, not hidden guarantees. See
[docs/BOUNDARIES.md](docs/BOUNDARIES.md) for the ownership and trust model.

## Documentation

- [docs/SPEC.md](docs/SPEC.md) is the normative product and protocol specification.
- [docs/EXPERIMENT.md](docs/EXPERIMENT.md) records the question, comparisons and stop conditions.
- [docs/BOUNDARIES.md](docs/BOUNDARIES.md) records ownership, trust assumptions and proof limits.
- [docs/IMPACT-CONTRACT.md](docs/IMPACT-CONTRACT.md) states the impact rules bound to the engine by tests.
- [docs/TRANSFER-CONTRACT.md](docs/TRANSFER-CONTRACT.md) states the transfer rules retained after the
  planner moved out of the library surface.
- [docs/M4-PROTOCOL.md](docs/M4-PROTOCOL.md) and [docs/M5-PROTOCOL.md](docs/M5-PROTOCOL.md) preserve the
  pre-registered scopes for branch convergence and transferable experience.
- [CONTRIBUTING.md](CONTRIBUTING.md) explains the contribution and verification path.
- [SECURITY.md](SECURITY.md) states the trust boundary and how to report a flaw in it.
- [AGENTS.md](AGENTS.md) is the repository contract for coding agents.

## Contributing

This is a measured experiment with a closed current milestone sequence. Bug reports, documentation
repairs, reproduction reports and narrowly scoped fixes are welcome. New capabilities need evidence
that they answer an existing stop or reversal condition before implementation starts.

Read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a pull request.

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache License 2.0](LICENSE-APACHE), at your option.
