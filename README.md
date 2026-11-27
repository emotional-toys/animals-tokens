# Emotional Toys

Emotional Toys turns a physical interaction into a collectible record. A GA144 device runs the immediate experience—lights, motors, chirps, and sensed state—using arrayForth/polyForth. The publishing layer turns a deliberately selected result into media and metadata for the live [Emotional Toys OpenSea collection](https://opensea.io/collection/emotional-toys).

## What is active

The current publishing surface is OpenSea Studio. This repository does **not** deploy a smart contract or handle wallet credentials. It is the source of truth for:

- the hardware-to-publishing provenance model;
- OpenSea-compatible item metadata;
- reproducible media and metadata review before a human mints; and
- the publishing checklist.

Read [the architecture](docs/architecture.md) before adding an item, then follow [the OpenSea publishing workflow](docs/opensea-publishing.md).

## Layout

| Path | Purpose |
| --- | --- |
| `firmware/ga144/` | arrayForth/polyForth integration contract and hardware notes. |
| `metadata/schema/` | Validation schema for item metadata. |
| `metadata/items/` | Reviewed, publishable metadata records. |
| `metadata/examples/` | A non-production example. |
| `docs/` | Architecture, provenance, and publishing process. |
| `legacy/near-prototype/` | Superseded NEAR/Rust tutorial work; not a deployable contract. |

## Safety boundary

Never put seed phrases, private keys, wallet-export files, OpenSea API keys, or personally identifying raw audio in this repository. Publishing must be a deliberate human action from the collection-owner wallet.

## Adding a collectible

1. Capture the device event and create a provenance manifest following `firmware/ga144/README.md`.
2. Render a public-safe image, video, or audio artifact. Do not publish raw personal recordings by default.
3. Copy `metadata/examples/emotional-toy.example.json` into `metadata/items/`, fill it out, and validate it against `metadata/schema/emotional-toy.schema.json`.
4. Review the description, rights, consent, and attributes.
5. Upload and mint through OpenSea Studio using the collection-owner wallet. Record the returned chain, contract address, token ID, and final item URL in the metadata record.
