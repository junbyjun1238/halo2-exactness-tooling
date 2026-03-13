use halo2_exactness_tooling::halo2wrong_adapter::{
    build_halo2wrong_style_circuit, halo2wrong_adapter_metadata, verify_halo2wrong_style_mock,
};
use halo2_exactness_tooling::integration::{
    build_b_note_circuit, prove_and_verify_real_circuit,
    prove_and_verify_real_circuit_with_instances,
};
use halo2_exactness_tooling::shared_inputs::InputProfile;

fn main() {
    let metadata = halo2wrong_adapter_metadata();
    println!("halo2wrong-style adapter demo");
    println!("recommended_k={}", metadata.recommended_k);
    println!("active_rows_per_rep={}", metadata.active_rows_per_rep);
    println!(
        "logical_counts={{lookups:{}, mul:{}, lin:{}}}",
        metadata.logical_lookup_cells_per_rep,
        metadata.logical_mul_constraints_per_rep,
        metadata.logical_lin_constraints_per_rep
    );

    verify_halo2wrong_style_mock(1)
        .expect("halo2wrong-style adapter demo should verify under MockProver");

    let external = build_halo2wrong_style_circuit(1);
    let external_metrics = prove_and_verify_real_circuit_with_instances(external.clone(), 13, &[&[&[]]])
        .expect("halo2wrong-style adapter demo should verify under the shared public real prove/verify path");

    // Also show that the same library-level real helper can be used against the released
    // B_note circuit without shelling out to benchmark binaries.
    let b_note_metrics =
        prove_and_verify_real_circuit(build_b_note_circuit(1, InputProfile::Boundary), 17)
            .expect("B_note should run through the same public real-proof helper");

    println!("mock_verification=ok");
    println!(
        "external_metrics={{k_run:{}, prove_ms:{:.2}, verify_ms:{:.2}, proof_bytes:{}}}",
        external_metrics.k_run,
        external_metrics.prove_ms,
        external_metrics.verify_ms,
        external_metrics.proof_bytes
    );
    println!(
        "b_note_metrics={{k_run:{}, prove_ms:{:.2}, verify_ms:{:.2}, proof_bytes:{}}}",
        b_note_metrics.k_run,
        b_note_metrics.prove_ms,
        b_note_metrics.verify_ms,
        b_note_metrics.proof_bytes
    );
    println!(
        "claim_boundary=this demonstrates a shared public proof helper across a halo2wrong-style consumer path and the released B_note circuit surface"
    );
}
