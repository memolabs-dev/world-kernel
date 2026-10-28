//! The identity of a consumed projection: typed, versioned, and stable.
//!
//! Step 2 of the epistemic differential computation programme asks for a content
//! identity on the nodes rather than `hash(data)`: the same bytes must invalidate
//! when the legitimate way of interpreting them has moved. The identity this
//! engine already compares on every incremental pass is the fingerprint of the
//! consumed projection, so this file holds the properties that make it an identity
//! rather than a byte digest, and the oracle that step names: the same corpus
//! produces the same identity, with no agent and no model involved.
//!
//! The projection is typed by the payload's declared type (`Facet`), the
//! evaluator, its version and the engine are the derivation policy a record was
//! produced under, and a definition that moved is revalidated rather than
//! compared. `docs/ADR-002-revisable-work.md` states all three; this file is the
//! test that says the engine does them.

mod support;

use std::collections::BTreeMap;

use serde_json::json;
use support::impact_fixture as fixture;
use world_kernel::impact::{
    Disposition, EvaluationRecord, Facet, NodeVersion, Revised, Snapshot, full_recompute, revise,
};

/// The projection definition that shipped before the declared type was part of it.
///
/// Kept as a literal because it is a historical fact rather than a constant: a
/// record computed under it must be revalidated, never compared as if the
/// definition had not moved.
const PREVIOUS_DEFINITION: &str = "facet-canonical-json/1";

fn records_at(snapshot: &Snapshot) -> BTreeMap<String, EvaluationRecord> {
    let targets: Vec<String> = fixture::DERIVED.iter().map(|id| (*id).to_owned()).collect();
    fixture::oracle(snapshot, &targets)
        .expect("the oracle evaluates the fixture")
        .into_iter()
        .map(|(id, evaluated)| (id, evaluated.record))
        .collect()
}

fn revise_from(
    base: &Snapshot,
    target: &Snapshot,
    records: &BTreeMap<String, EvaluationRecord>,
) -> Revised {
    let targets: Vec<String> = fixture::DERIVED.iter().map(|id| (*id).to_owned()).collect();
    revise(
        base,
        target,
        records,
        &targets,
        &fixture::evaluators(),
        &fixture::trust(),
        fixture::limits(),
    )
    .expect("the incremental pass runs")
}

/// The limit node keeps its exact bytes and its version, and only the declared
/// type it is read under moves. Nothing any evaluator compares by value changed,
/// so a projection that ignored the declared type would reuse every consumer.
#[test]
fn a_byte_identical_payload_under_another_declared_type_is_not_the_same_consumption() {
    let s0 = fixture::snapshot(0, 100, 80, 110, "draft");
    let records = records_at(&s0);

    let mut s1 = s0.clone();
    s1.revision = 1;
    s1.nodes.insert(
        fixture::LIMIT.to_owned(),
        NodeVersion::new(1, "money/2", json!({"value": 100}), "producer"),
    );

    let revised = revise_from(&s0, &s1, &records);

    assert_eq!(
        revised.findings[fixture::ELIGIBLE_A].disposition,
        Disposition::RecomputedSame,
        "the projection is typed by the declared type, so the consumption moved and the consumer is not reused"
    );
    // The same bytes still mean the same number, so the arithmetic is unchanged.
    assert_eq!(
        revised.findings[fixture::ELIGIBLE_A].output,
        Some(json!({"eligible": true})),
        "a moved interpretation does not change the value it produces"
    );
    assert_eq!(
        revised.findings[fixture::SUPPORT].disposition,
        Disposition::ReusedAfterCheck,
        "a consumer that never read limit is not concerned with how limit is interpreted"
    );
}

/// Nothing consumed moved and no evaluator version changed. The only thing that
/// moved is the engine that produced the record, and the engine is part of the
/// derivation policy: the record is revalidated, never carried forward as if its
/// definition had not moved.
#[test]
fn a_record_produced_by_another_engine_is_revalidated_and_the_trail_names_it() {
    let s0 = fixture::snapshot(0, 100, 80, 110, "draft");
    let mut records = records_at(&s0);
    records
        .get_mut(fixture::ELIGIBLE_A)
        .expect("the fixture records eligible_a")
        .engine = "world-impact/sequential/1".to_owned();

    let revised = revise_from(&s0, &s0, &records);
    let finding = &revised.findings[fixture::ELIGIBLE_A];

    assert_eq!(
        finding.disposition,
        Disposition::RecomputedSame,
        "the definition moved, so the record is re-run instead of reused"
    );
    assert_eq!(
        finding.explanation.trigger.as_deref(),
        Some("engine world-impact/sequential/1"),
        "the trail says which part of the definition moved"
    );
    assert_eq!(
        revised.findings[fixture::ELIGIBLE_B].disposition,
        Disposition::UnchangedInScope,
        "only the record that was altered is revalidated"
    );
    assert_eq!(revised.evaluations, 1);
    assert_eq!(revised.reused, 2);
}

/// A record computed under the definition that shipped before this one is
/// revalidated once, not compared as if the definition had not moved.
#[test]
fn a_record_computed_under_another_definition_is_revalidated() {
    assert_ne!(
        world_kernel::impact::FACET_COMPARATOR_VERSION,
        PREVIOUS_DEFINITION,
        "if the definition had not moved this test would be about nothing"
    );

    let s0 = fixture::snapshot(0, 100, 80, 110, "draft");
    let mut records = records_at(&s0);
    for read in &mut records
        .get_mut(fixture::ELIGIBLE_A)
        .expect("the fixture records eligible_a")
        .reads
    {
        read.comparator = PREVIOUS_DEFINITION.to_owned();
    }

    let revised = revise_from(&s0, &s0, &records);

    assert_eq!(
        revised.findings[fixture::ELIGIBLE_A].disposition,
        Disposition::RecomputedSame,
        "an old fingerprint is never reused as if its definition had not moved"
    );
}

/// The facet advantage survives the typed projection: a field nobody consumed can
/// change without invalidating anybody. This is the property the reduction in
/// `docs/ADR-003-measure-before-building.md` was reversed for, and a projection
/// that folded the whole payload into its identity would lose it.
#[test]
fn an_unconsumed_field_of_a_consumed_node_still_reuses_every_consumer() {
    let s0 = fixture::snapshot(0, 100, 80, 110, "draft");
    let s1 = fixture::snapshot_with_note(1, 100, "nothing reads this");

    let revised = revise_from(&s0, &s1, &records_at(&s0));

    assert_eq!(
        revised.evaluations, 0,
        "no evaluator reads the note, so none may run"
    );
    assert_eq!(revised.reused, 3);
    for id in fixture::DERIVED {
        assert_eq!(
            revised.findings[id].disposition,
            Disposition::UnchangedInScope,
            "{id}: an unconsumed field is not a consumption"
        );
    }
}

/// The definition itself, held at the unit level: the identity of a projection
/// moves with the facet, the declared type, the value and its absence, and with
/// nothing else. This is the boundary step 2 asks to place between content and
/// procedure: an unconsumed field of the payload is not an ingredient, so the
/// facet advantage survives.
#[test]
fn the_identity_of_a_projection_moves_with_its_ingredients_and_with_nothing_else() {
    let field = Facet::Field("value".to_owned());
    let whole = Facet::Whole;
    let money = json!(100);

    assert_ne!(
        field.fingerprint(Some("money/1"), Some(&money)),
        field.fingerprint(Some("money/2"), Some(&money)),
        "the declared type is how the bytes are interpreted, so it is an ingredient"
    );
    assert_ne!(
        field.fingerprint(Some("money/1"), Some(&money)),
        field.fingerprint(None, Some(&money)),
        "no declared type is not the same as a declared type"
    );
    assert_ne!(
        field.fingerprint(Some("money/1"), Some(&money)),
        field.fingerprint(Some("money/1"), Some(&json!(101))),
        "the value is an ingredient"
    );
    assert_ne!(
        field.fingerprint(Some("money/1"), Some(&money)),
        whole.fingerprint(Some("money/1"), Some(&money)),
        "the facet is an ingredient"
    );
    assert_ne!(
        field.fingerprint(Some("money/1"), Some(&money)),
        field.fingerprint(Some("money/1"), None),
        "a recorded absence is a value of its own, not a missing argument"
    );

    // Nothing else is. The same corpus produces the same identity, and the key
    // order of an object is not part of a canonical value.
    assert_eq!(
        whole.fingerprint(Some("money/1"), Some(&json!({"value": 100, "note": "x"}))),
        whole.fingerprint(Some("money/1"), Some(&json!({"value": 100, "note": "x"}))),
        "the identity is a function of its ingredients"
    );
    assert_eq!(
        whole.fingerprint(Some("money/1"), Some(&json!({"value": 100, "note": "x"}))),
        whole.fingerprint(Some("money/1"), Some(&json!({"note": "x", "value": 100}))),
        "a canonical value does not depend on the order the keys were written in"
    );
}

/// The oracle step 2 names: the same corpus produces the same identity.
///
/// Both routes that can reach a record (the incremental pass and the full
/// recompute it is measured against) must record the same identity for the same
/// data, and two independent runs of the same corpus must agree. An identity that
/// depended on the route or on the run would move while the data did not, which is
/// the stop condition attached to step 2.
#[test]
fn the_same_corpus_produces_the_same_identity_on_both_routes() {
    let s0 = fixture::snapshot(0, 100, 80, 110, "draft");
    let s1 = fixture::snapshot(1, 90, 80, 110, "draft");
    let targets: Vec<String> = fixture::DERIVED.iter().map(|id| (*id).to_owned()).collect();
    let recompute = || {
        full_recompute(
            &s1,
            &targets,
            &fixture::evaluators(),
            &fixture::trust(),
            fixture::limits(),
        )
        .expect("the oracle evaluates the fixture")
    };

    let first = recompute();
    let second = recompute();
    for id in &targets {
        assert_eq!(
            first[id].record, second[id].record,
            "{id}: two runs over the same corpus recorded different identities"
        );
    }

    // The limit moved from 100 to 90, so both comparisons are re-run and the
    // support consumer, which reads only their two booleans, is not.
    let revised = revise_from(&s0, &s1, &records_at(&s0));
    let mut compared = 0usize;
    for id in &targets {
        if !matches!(
            revised.findings[id].disposition,
            Disposition::RecomputedSame | Disposition::RecomputedChanged
        ) {
            continue;
        }
        assert_eq!(
            revised.records[id].reads, first[id].record.reads,
            "{id}: the incremental pass recorded another identity than the oracle for the same corpus"
        );
        compared += 1;
    }
    assert_eq!(
        compared, 2,
        "the corpus must recompute something for this comparison to be evidence"
    );
}
