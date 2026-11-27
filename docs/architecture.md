# Architecture

```text
GA144 hardware                         Publishing and ownership
--------------------                   ------------------------
arrayForth/polyForth                   human review
sensor / interaction event ───────►    curated media artifact
light / motor / chirp feedback         OpenSea-ready metadata
local provenance manifest ───────►     OpenSea Studio mint
                                       public collectible
```

## Design rules

- The GA144 is authoritative for the immediate physical interaction, not for wallet operations.
- A collectible represents a curated interpretation of an event, not an automatic claim about a person's emotional state.
- Keep raw sensor data and any identifiable audio private by default. Publish a derivative only when consent and rights are clear.
- The owner wallet performs the mint. Firmware never stores wallet material or signs marketplace transactions.
- The OpenSea collection is the publishing surface. A bespoke contract is deferred until the collection needs capabilities Studio cannot supply.

## Provenance chain

Each item has two linked records:

1. A private device manifest captures source, time, firmware version, anonymised inputs, and SHA-256 hashes of source/derived artifacts.
2. The public NFT metadata includes the creative result and selected non-sensitive attributes, then receives its chain, contract address, token ID, and OpenSea URL after minting.

This makes the work explainable without placing private inputs or secrets on-chain.
