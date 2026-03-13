use crate::external_h2w::{
    ExternalH2W66BitCircuit, ACTIVE_ROWS_PER_REP, ADVICE_COLS, FIXED_COLS, INSTANCE_COLS,
    LIN_CONSTRAINTS_PER_REP, LOOKUP_CELLS_PER_REP, MUL_CONSTRAINTS_PER_REP,
};
use crate::integration::{
    prove_and_verify_real_circuit_with_instances, IntegrationField, RealProofMetrics,
};
use halo2_proofs::dev::MockProver;

pub const HALO2WRONG_STYLE_RECOMMENDED_K: usize = 13;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Halo2WrongAdapterMetadata {
    pub recommended_k: usize,
    pub active_rows_per_rep: usize,
    pub logical_lookup_cells_per_rep: usize,
    pub logical_mul_constraints_per_rep: usize,
    pub logical_lin_constraints_per_rep: usize,
    pub advice_cols: usize,
    pub fixed_cols: usize,
    pub instance_cols: usize,
}

pub fn halo2wrong_adapter_metadata() -> Halo2WrongAdapterMetadata {
    Halo2WrongAdapterMetadata {
        recommended_k: HALO2WRONG_STYLE_RECOMMENDED_K,
        active_rows_per_rep: ACTIVE_ROWS_PER_REP,
        logical_lookup_cells_per_rep: LOOKUP_CELLS_PER_REP,
        logical_mul_constraints_per_rep: MUL_CONSTRAINTS_PER_REP,
        logical_lin_constraints_per_rep: LIN_CONSTRAINTS_PER_REP,
        advice_cols: ADVICE_COLS,
        fixed_cols: FIXED_COLS,
        instance_cols: INSTANCE_COLS,
    }
}

pub fn build_halo2wrong_style_circuit(
    repetitions: usize,
) -> ExternalH2W66BitCircuit<IntegrationField> {
    ExternalH2W66BitCircuit::new(repetitions)
}

pub fn verify_halo2wrong_style_mock(repetitions: usize) -> Result<(), String> {
    let circuit = build_halo2wrong_style_circuit(repetitions);
    let prover = MockProver::run(
        HALO2WRONG_STYLE_RECOMMENDED_K as u32,
        &circuit,
        vec![vec![]],
    )
    .map_err(|err| format!("MockProver construction failed: {err:?}"))?;
    prover
        .verify()
        .map_err(|failures| format!("Verification failed: {failures:?}"))
}

pub fn prove_and_verify_halo2wrong_style_real(
    repetitions: usize,
    k_run: Option<u32>,
) -> Result<RealProofMetrics, String> {
    let circuit = build_halo2wrong_style_circuit(repetitions);
    prove_and_verify_real_circuit_with_instances(
        circuit,
        k_run.unwrap_or(HALO2WRONG_STYLE_RECOMMENDED_K as u32),
        &[&[&[]]],
    )
}
