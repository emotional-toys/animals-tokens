# GA144 development and simulation

Two sibling repositories provide the active local toolchain. They intentionally remain external dependencies: this repository contains Emotional Toys firmware source, tests, and provenance—not copies of the compilers.

| Repository | Use here | Verified capability |
| --- | --- | --- |
| `ga144-aforth-cli` | Primary aForth/arrayForth-like workflow and non-interactive simulation | `./ga --run examples/lucas-series.aforth` completes in the simulator. |
| `ga144-assembler-toolchain` | Python assembler, bootstream generation, disassembly, and hardware loading | `python3 tests/test.py` passes all 29 tests. |

## Source dialect decision

New Emotional Toys firmware should start as `.aforth` and use `ga144-aforth-cli` for simulation. Use the Python assembler's `.ga` dialect only when an assembly-level feature, its established examples, or its serial-loader path is needed. The dialects are similar but are **not** source-compatible; do not compile one toolchain's program with the other by assumption.

## Simulation-first loop

Set these paths for a local shell session:

```sh
AFORTH_CLI=/home/cartheur/ame/aiventure/aiventure-github/ai-forth/ga144-aforth-cli
ASM_TOOLCHAIN=/home/cartheur/ame/aiventure/aiventure-github/ai-forth/ga144-assembler-toolchain
```

1. Write the program under `firmware/ga144/` using its declared dialect.
2. Compile and inspect it from the aForth CLI checkout so its include paths resolve:

   ```sh
   cd "$AFORTH_CLI"
   ./ga --pretty /absolute/path/to/program.aforth
   ./ga --run /absolute/path/to/program.aforth
   ```

3. Record the source revision, dialect, bootstream type, intended node map, and simulator result in the private device manifest.
4. Generate and inspect a bootstream before hardware loading:

   ```sh
   cd "$AFORTH_CLI"
   ./ga --bootstream /absolute/path/to/program.aforth
   ```

5. Only then connect the board and load deliberately, with the actual serial device and expected bootstream topology:

   ```sh
   cd "$AFORTH_CLI"
   ./ga-load /absolute/path/to/program.aforth /dev/ttyUSB0 --speed 460800
   ```

Never run the final command against an unverified device path. Serial loading changes real hardware state.

## Current test status

- The primary CLI’s known-good simulator smoke test is `./ga --run examples/lucas-series.aforth` from its own checkout.
- Its full `./ga --test-all` suite currently fails one compiler-layout assertion: the node-205 forward-reference/tail-jump case. This is a known limitation recorded in that repository's `CURRENT-STATUS.md`; it is not evidence that every simulation fails.
- `examples/hmm-sim.aforth` currently fails on an unsupported `org` directive, so it is not an appropriate starting point until repaired.
- The Python assembler toolchain’s `python3 tests/test.py` completed successfully with 29 tests in this environment. Its own README states that it does not include a simulator; use it for assembly/bootstream/hardware tasks, not simulation.

## Hardware gate

Before loading a new program onto a Volatco board, require all of the following:

- a passing focused simulator run for the program's intended behavior;
- reviewed node and pin allocation;
- the correct bootstream type (`async`/`708`, `target`/`708-300`, or `2wire`/`300`);
- a confirmed serial port that belongs to the intended board; and
- a rollback bootstream or known-good recovery image.

The board's physical response—light pattern, motor behavior, and chirp sequence—is the final acceptance test. Capture only non-sensitive derived evidence for the collectible provenance record.
