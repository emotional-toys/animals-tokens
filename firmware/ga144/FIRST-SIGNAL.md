# First Signal firmware baseline

`first-signal.aforth` is the first simulator-backed Emotional Toys firmware artifact. It is intentionally a serial event sequence, not a hardware driver.

## What it does

The program runs at GA144 node 708, uses the aForth CLI's `708serial.aforth` support library, then emits these 18-bit event identifiers in order:

| Order | Identifier | Meaning |
| --- | --- | --- |
| 1 | `0x1e101` | DOT visual acknowledgement begins. |
| 2 | `0x1e102` | Reserved request for a separately reviewed haptic external ring. |
| 3 | `0x1e103` | DOT audible acknowledgement begins. |

They are stable control/provenance markers only. They contain no emotional classification, raw audio, biometric data, or wallet information.

## Simulation

Run it from the `ga144-aforth-cli` checkout so the bundled serial library resolves:

```sh
cd /home/cartheur/ame/aiventure/aiventure-github/ai-forth/ga144-aforth-cli
./ga /absolute/path/to/toys/animals-tokens/firmware/ga144/first-signal.aforth
./ga --run /absolute/path/to/toys/animals-tokens/firmware/ga144/first-signal.aforth
```

The source compiles into a GA144 bootstream, and its bounded `--run` simulation completes. This validates the event sequence, not its physical interpretation: the simulator never models real Volatco wiring or proves that a physical actuator responds.

## Hardware binding required

Before loading this onto a board, document all of the following in a follow-up change:

- the board revision and GA144 node coordinates;
- pins and electrical interface for the light, motor driver, and chirp transducer;
- voltage/current limits and any external driver circuitry;
- the selected bootstream topology; and
- a recovery image.

Only after that review should `signal-light` and `signal-chirp` be connected to DOT base routines. `signal-haptic` requires a separately reviewed external-ring electrical and mechanical interface.
