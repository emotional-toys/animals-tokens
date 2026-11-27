# GA144 integration boundary

The live toy runs on GA144 Volatco hardware using arrayForth/polyForth. This directory documents the boundary between the device and the publishing process; it is not a claim that a particular image or metadata field was calculated on-chain.

The local development path is documented in [TOOLCHAIN.md](TOOLCHAIN.md). Simulation is required before a candidate is loaded onto hardware.

The first simulator-backed artifact is [first-signal.aforth](first-signal.aforth). Its current event-only boundary and the information needed to bind it to hardware are documented in [FIRST-SIGNAL.md](FIRST-SIGNAL.md).

[DOT-INTEGRATION.md](DOT-INTEGRATION.md) maps that firmware vocabulary onto DOT's sensory face, Entity Nucleus, quiet-mode, and external-ring model.

## Device event envelope

Produce one local manifest per candidate interaction. Keep it private when it contains person-linked data.

```json
{
  "event_id": "et-YYYYMMDD-sequence",
  "recorded_at": "2026-11-27T12:34:56Z",
  "device": { "model": "GA144", "firmware": "arrayforth-build-id" },
  "interaction": {
    "inputs": ["anonymised-sensor-summary"],
    "outputs": ["light-pattern-id", "motor-pattern-id", "chirp-pattern-id"]
  },
  "consent": "recorded | not-required | withheld",
  "source_hashes": { "derived_media_sha256": "hex digest" }
}
```

## Rules

- `event_id` must be unique and must not contain a person’s name.
- Hash the final public derivative, not raw private recordings, in the public workflow.
- Keep wall-clock time only when it does not reveal sensitive context; a date or sequence may be enough for public metadata.
- Do not embed wallet secrets, API keys, or recovery material in device firmware, manifests, or serial logs.
- The owner reviews the event before creating an NFT. No automatic minting path is permitted.
