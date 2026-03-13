use halo2_exactness_tooling::halo2wrong_adapter::{
    halo2wrong_adapter_metadata, verify_halo2wrong_style_mock, HALO2WRONG_STYLE_RECOMMENDED_K,
};

#[test]
fn test_halo2wrong_adapter_metadata_contract() {
    let meta = halo2wrong_adapter_metadata();
    assert_eq!(meta.recommended_k, HALO2WRONG_STYLE_RECOMMENDED_K);
    assert!(meta.active_rows_per_rep > 0);
    assert!(meta.logical_mul_constraints_per_rep > 0);
}

#[test]
fn test_halo2wrong_adapter_mock_path() {
    verify_halo2wrong_style_mock(1)
        .expect("halo2wrong-style adapter should verify under MockProver from outside the crate");
}
