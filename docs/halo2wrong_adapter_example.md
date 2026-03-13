# halo2wrong-Style Adapter Example

This document explains the narrow integration surface added for a
`halo2wrong` / `maingate`-style consumer path.

## What this example proves

The repository no longer exposes only:
- internal benchmark binaries, or
- a `B_note`-only integration demo.

It now also exposes:
- a public `halo2wrong`-style adapter module,
- a runnable external-style example,
- and an external integration test that uses the public crate surface.

The new public entry points are:
- `src/halo2wrong_adapter.rs`
- `examples/halo2wrong_adapter_demo.rs`
- `tests/halo2wrong_adapter.rs`

## Why this matters

The purpose is not to claim broad third-party adoption. The purpose is to show
that the tooling repo can participate in a recognizable Halo2 workflow shaped
around `maingate`-style circuits, rather than only in its own benchmark runners.

This narrows the gap between:
- \"artifact bundle with code\"

and:
- \"early-stage Halo2-facing developer tooling\"

## Public adapter surface

The adapter module exposes:

- `halo2wrong_adapter_metadata()`
- `build_halo2wrong_style_circuit(...)`
- `verify_halo2wrong_style_mock(...)`
- `prove_and_verify_halo2wrong_style_real(...)`

It also relies on the generic public real proof helper:

- `integration::prove_and_verify_real_circuit_with_instances(...)`

That helper is important because it lets a consumer route both:
- the released `B_note` circuit, and
- the `halo2wrong`-style example circuit

through the same public proof API without calling the internal benchmark bins.

## Run the example

```bash
cargo run --example halo2wrong_adapter_demo --quiet
```

The example:
- verifies the `halo2wrong`-style path under `MockProver`,
- runs a real prove/verify cycle for the `halo2wrong`-style circuit,
- runs the released `B_note` circuit through the same generic public real-proof helper,
- prints both metric bundles.

## External test coverage

The public adapter path is also covered by:

```bash
cargo test --quiet --test halo2wrong_adapter
```

This matters because the test lives outside the crate boundary and therefore
confirms that the public API is sufficient for a consumer-shaped path.

## Claim boundary

This example demonstrates:
- a real Halo2-facing adapter surface for a `maingate` / `halo2wrong`-style path,
- a shared public prove/verify helper,
- and a minimal external-style consumption proof.

It does **not** by itself establish:
- compatibility with every `halo2wrong` gadget family,
- third-party adoption,
- backend closure,
- full wrapper-level parity,
- PCS / Fiat-Shamir soundness.
