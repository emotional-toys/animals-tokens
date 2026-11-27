# Forest Creature Token series

The Forest Creature Tokens are collectible milestones in DOT's lifetime. Each creature represents a distinct, bounded behavioral pathway; it is not a score of a person, an emotional diagnosis, or an automatic mint trigger.

## Pathways

| Token | DOT pathway | Meaning | Required eligibility before proposing a mint |
| --- | --- | --- | --- |
| Owl | Attentive observation | DOT is ready, quietly observing, and has a stable ambient-light baseline. | Calibration completes without a sensor fault; quiet mode may be active. |
| Fox | Acknowledged connection | DOT gives a bounded acknowledgement and a person deliberately responds. | A rate-limited acknowledgement completes; the response is explicit and non-identifying. |
| Squirrel | Focused work | DOT completes a bounded working sequence. | Working pattern starts and finishes without a diagnostic or safety fallback. |
| Rabbit | Considered attention | DOT makes a restrained request for attention. | Cue is rate-limited; visual-only fallback remains available; it is not minted merely because the cue repeats. |
| Hedgehog | Chosen boundary | A user selects quiet/privacy mode or DOT takes a safe fallback. | A quiet-mode change is explicit, or a diagnostic is reviewed. Never mint a fault automatically. |

## Minting policy

1. Firmware records an internal event with a private event ID and a reviewed, non-sensitive summary.
2. A local policy checks the pathway's eligibility and a per-pathway cooldown.
3. A human chooses whether the milestone merits a collectible and prepares public-safe derivative media.
4. The human mints through OpenSea Studio and records chain references in `metadata/items/`.

Raw ambient readings, audio, images of people, precise location, wallet material, and inferred personal emotional state never form part of public NFT metadata.

## Common traits

Every series item uses these traits:

| Trait | Value |
| --- | --- |
| Series | Forest Creature Tokens |
| Entity | DOT |
| Interaction model | Bounded lifetime pathway |
| Provenance | Consent-safe reviewed derivative |
| Edition | 1 of 1 |

Add the animal, pathway, and only verified event references to each individual mint package.
