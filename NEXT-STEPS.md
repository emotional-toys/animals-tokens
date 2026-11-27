# Next steps

## Goal

Produce the first Emotional Toys collectible from a real GA144 interaction, with a simulator-tested firmware artifact and a reviewed OpenSea item record.

## 1. Establish the firmware baseline

- [ ] Create `firmware/ga144/first-signal.aforth`.
- [ ] Implement one deterministic interaction: a named light pattern, motor pattern, and chirp sequence.
- [ ] Add a short header recording source dialect, intended node/pin map, and bootstream type.
- [ ] Keep hardware-specific pin assignments explicit; do not guess them from examples.

## 2. Simulate before hardware

- [ ] Run the program with `ga144-aforth-cli` from its own checkout: `./ga --run /absolute/path/to/first-signal.aforth`.
- [ ] Inspect the compilation output and confirm the intended node allocation.
- [ ] Capture the firmware revision and simulator result in a private device manifest.
- [ ] Do not use `examples/hmm-sim.aforth` as the baseline: it currently fails on `org`.

## 3. Load and observe the Volatco board

- [ ] Generate and review the bootstream.
- [ ] Confirm the actual board serial device and bootstream topology.
- [ ] Preserve a known-good recovery bootstream before loading the new program.
- [ ] Load one board only, then record the observed lights, motors, chirps, and any serial output.

## 4. Prepare the first collectible

- [ ] Create a consent-safe visual, audio, or video derivative; keep raw inputs private.
- [ ] Copy `metadata/examples/emotional-toy.example.json` to `metadata/items/` and complete it.
- [ ] Compute and record the SHA-256 of the final public media.
- [ ] Review title, description, traits, rights, and provenance before upload.

## 5. Publish and record

- [ ] Mint through the Emotional Toys OpenSea collection using the owner wallet.
- [ ] Add the chain, contract address, token ID, and OpenSea URL to the metadata record.
- [ ] Check the public item page while signed out.
- [ ] Commit the reviewed public metadata; never commit wallet credentials, private keys, raw personal data, or source recordings.

## Exit criteria for the first signal

The first item is complete when the firmware has a focused passing simulator run, the physical sequence has been observed on the intended board, the public artifact is consent-safe, and the minted item's metadata records verifiable provenance.
