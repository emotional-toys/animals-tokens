# Next steps

## Goal

Produce the first Emotional Toys collectible from a real GA144 interaction, with a simulator-tested firmware artifact and a reviewed OpenSea item record.

## Next task: Owl — attentive observation

Implement the first mint-eligible DOT lifetime pathway: a quiet, ready state that establishes a stable ambient-light baseline without a sensor fault. Its candidate collectible is [Forest Creature Token — Owl](token/forest-creature-token-owl.MINTING.md).

- [ ] Add a focused `.aforth` simulation program for the attentive-observation pathway.
- [ ] Model a valid calibration, an LED-off sample window, and a blocked/saturated-sensor fallback; do not use raw readings as collectible data.
- [ ] Confirm quiet mode suppresses the chirp while retaining an accessible visual state.
- [ ] Run the focused simulation from `ga144-aforth-cli`, capture the source revision and result in a private device manifest, and review the node allocation.
- [ ] Define the selected Entity Nucleus/Volatco or GA144 prototype pin map, driver circuits, power limits, and bootstream topology before any board load.
- [ ] Produce a consent-safe owl derivative only after a qualifying, reviewed event; add final chain, contract, token, and OpenSea references in `metadata/items/` after minting.

## Follow-on pathways

- [ ] Fox — acknowledged connection.
- [ ] Squirrel — focused work.
- [ ] Rabbit — considered attention.
- [ ] Hedgehog — chosen boundary / quiet mode or human-reviewed safe fallback.

## Hardware validation

- [ ] Generate and review the bootstream.
- [ ] Confirm the actual board serial device and bootstream topology.
- [ ] Preserve a known-good recovery bootstream before loading the new program.
- [ ] Load one board only, then record the observed lights, motors, chirps, and any serial output.

## Prepare a collectible

- [ ] Create a consent-safe visual, audio, or video derivative; keep raw inputs private.
- [ ] Copy `metadata/examples/emotional-toy.example.json` to `metadata/items/` and complete it.
- [ ] Compute and record the SHA-256 of the final public media.
- [ ] Review title, description, traits, rights, and provenance before upload.

## Publish and record

- [ ] Mint through the Emotional Toys OpenSea collection using the owner wallet.
- [ ] Add the chain, contract address, token ID, and OpenSea URL to the metadata record.
- [ ] Check the public item page while signed out.
- [ ] Commit the reviewed public metadata; never commit wallet credentials, private keys, raw personal data, or source recordings.

## Exit criteria for Owl

The Owl item is complete when the attentive-observation pathway has a focused passing simulator run, the physical sequence has been observed on the intended board, the public artifact is consent-safe, and the minted item's metadata records verifiable provenance.
