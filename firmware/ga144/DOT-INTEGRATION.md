# DOT embodied-model integration

This document binds Emotional Toys firmware semantics to the DOT00-00a embodied model maintained in the sibling manufacturing repository. DOT remains at concept intake; this is an interface agreement for firmware development, not a claim of a released PCB, selected parts, or approved safety limits.

The source references are:

- `ai-forth/manufacturing/boards/DOT00-00a/project/INTERACTION-BRIEF.md`
- `ai-forth/manufacturing/boards/DOT00-00a/project/BEHAVIOUR-DESIGN.md`
- `ai-forth/manufacturing/boards/DOT00-00a/project/ENTITY-NUCLEUS-FRAME.md`
- `ai-forth/manufacturing/boards/DOT00-00a/project/BOM-DRAFT.md`

## One embodied contract

```text
DOT sensory face                 Entity Nucleus                Collectible record
-----------------                --------------                ------------------
two ambient-light eyes ──►       bounded state estimate ──►     reviewed derivative
quiet/privacy control ────►      interaction policy ──────►     public-safe metadata
LEDs + sound path ◄───────       GA144/Volatco firmware ◄──     provenance marker
external ring (optional) ◄─      haptic extension contract
```

The Entity Nucleus is the serviceable core. DOT's first fit path is a selected Volatco carrier; a custom GA144 nucleus is explicitly a later, separately controlled path. The same event vocabulary can therefore run on a GA144 prototype without asserting that it is already the release controller for DOT.

## First Signal mapping

| First Signal marker | DOT meaning | Implementation boundary |
| --- | --- | --- |
| `0x1e101` | Success/acknowledgement visual accent | Two DOT light channels behind the sensory-face diffuser. Pattern/cadence must carry meaning beyond colour. |
| `0x1e102` | Reserved haptic request | Not a base DOT actuator. It may drive a later external ring only after its mass, power, mechanical load, wiring, and safe operating limits are reviewed. |
| `0x1e103` | Success/acknowledgement tone | DOT acoustic path. Suppress immediately in quiet/privacy mode and rate-limit it. |

The marker sequence is an internal, deterministic event trace. It is not an emotional diagnosis, an identity signal, or material to publish raw in NFT metadata.

## Firmware rules inherited from DOT

- Photoresistors are ambient-light inputs, not people sensors. Sample with LEDs off or compensate for self-illumination.
- Quiet/privacy mode must suppress sound immediately; it cannot be learned away.
- Brightness, duty cycle, sound level, current, thermal, and battery limits are fixed constraints, not adaptive behavior.
- A missing, blocked, saturated, or contradictory sensor must lead to a deterministic diagnostic/fallback state.
- Visual meaning must remain intelligible through cadence and position, not colour alone.
- Raw inputs stay private. Only a human-reviewed, consent-safe derivative may become collectible media.

## Required design evidence before hardware binding

1. Select the exact Entity Nucleus/Volatco revision or document the custom GA144 prototype path.
2. Define GA144 node coordinates, pins, voltage domains, driver circuits, and connector/harness assignments for each channel.
3. Select and document the DOT light, acoustic, sensor, quiet-mode, and power components in the controlled BOM.
4. Confirm the sensory-face optical baffles, diffuser, speaker path, service access, and any external-ring interface with a fit prototype.
5. Add focused simulator tests for normal acknowledgement, quiet mode, blocked-light sensor, and fault fallback before a board load.

Until those evidence items exist, `first-signal.aforth` remains a serial event prototype only.
