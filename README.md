# Halo2 Exactness Tooling

This MIT-licensed repository is the standalone developer-facing tooling surface
for the BN254 deferred-quotient exactness repair work.

It exists to give Halo2-style developers and auditors a clean repo for:

- reusable circuit constructors for the released `A_secure` and `B_note` paths,
- a small library-facing integration API,
- real `create_proof` / `verify_proof` adapter helpers,
- benchmark and parity rerun scripts,
- checker inputs and supporting verification notes.

This repository is intentionally separate from the manuscript companion and
benchmark-evidence archive. The paper PDF, DOI snapshot, and manuscript-pinned
artifact history remain in the companion repository:

- `https://github.com/junbyjun1238/zk2-bench-b1-lite`

## Start Here

If you want the library-facing tooling surface, begin with:

- `src/integration.rs`
- `src/halo2wrong_adapter.rs`
- `examples/halo2_integration_demo.rs`
- `examples/halo2wrong_adapter_demo.rs`
- `docs/halo2_integration_example.md`
- `docs/halo2wrong_adapter_example.md`

If you want the rerun and verification harness:

- `scripts/run_ab_bench.py`
- `scripts/local_repeat_bench.py`
- `scripts/local_fixedk_fullbench.py`
- `docs/security_equivalence_checklist.md`
- `docs/wrapper_level_parity_requirements.md`

## Quick Start

Build and test:

```bash
cargo test --quiet
```

Run the integration demo:

```bash
cargo run --example halo2_integration_demo --quiet
```

The demo performs both a `MockProver` check and a real proof/verification
cycle, so it is intentionally slower than a pure smoke test.

Run the `halo2wrong` / `maingate`-style adapter demo:

```bash
cargo run --example halo2wrong_adapter_demo --quiet
```

This second demo shows that the standalone tooling surface now supports a
consumer-shaped `halo2wrong`-style path through the public crate API rather
than only through benchmark binaries.

## What This Repo Does

- exposes a minimal but real Halo2-facing integration path,
- exposes a minimal `halo2wrong` / `maingate`-style adapter path,
- keeps the public proof path reachable from library code,
- provides repeat/sweep runners for defended A/B comparisons,
- packages checker inputs and parity-facing status notes next to the code.

## What This Repo Does Not Claim

This repository does **not** by itself claim:

- backend closure,
- Fiat-Shamir closure,
- universal superiority across all wrapper realizations,
- full manuscript mechanization.

The safest reading remains:

- `A_secure` vs `B_note` under a bounded instantiated-family benchmark contract,
- plus a reusable early-stage Halo2 integration surface.

## Layout

- `src/`
  - released circuit code
  - public integration API
  - full-local proof binaries
- `examples/`
  - minimal integration demo
- `scripts/`
  - benchmark orchestration and checker helpers
- `certificates/`
  - current certificate/backend inputs used by the tooling
- `docs/`
  - integration and parity/security notes
- `benches/`
  - local generated outputs (kept untracked except `.gitkeep`)

## License

MIT. See `LICENSE`.
