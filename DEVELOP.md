# Developer Guide

This document covers how to build, test, and contribute to `ooga-booga-fuzz`.

## Repository Layout

| Path | Purpose |
|------|---------|
| `driver.cpp` / `driver.h` | libFuzzer harness and target dispatcher |
| `main.cpp` | Entry point; links modules and starts fuzzing |
| `include/bitcoinfuzz/` | Core headers and base module interface |
| `modules/<name>/` | Individual differential-fuzzing modules (one per upstream project) |
| `custommutator/` | Custom libFuzzer mutators |
| `external/` | Git submodules for upstream dependencies |
| `scripts/` | Build helpers (`auto_build.py`, `build-all.sh`) |
| `helpers/` | JVM loader and shared utilities |

## Prerequisites

### macOS

Apple Clang does **not** ship libFuzzer. Install Homebrew LLVM first:

```bash
make setup-macos
```

The root `Makefile` auto-detects Homebrew LLVM at:
- `/opt/homebrew/opt/llvm/bin/clang++` (Apple Silicon)
- `/usr/local/opt/llvm/bin/clang++` (Intel)

### Linux

Install Clang, LLD, and LLVM dev libraries:

```bash
sudo apt-get install clang lld llvm-dev
```

Some modules need additional system packages:

```bash
sudo apt-get install libsodium-dev libboost-all-dev npm nodejs quickjs libquickjs
```

Language runtimes required by various modules:
- **Rust** — nightly toolchain (for sanitizers)
- **Go** — 1.22+
- **Java** — JDK 21
- **.NET** — SDK 10.0.x
- **Python** — 3.11+ with `pip`

### Submodules

Many modules depend on code in `external/`:

```bash
git submodule update --init --recursive
```

## Building

### Quick build (one module)

```bash
make clean
CXXFLAGS="-DENTROPYLAB" make bitcoinfuzz
```

Run a target:

```bash
FUZZ=bip32_master_keygen ./bitcoinfuzz -max_total_time=30
```

### Auto-build script

`scripts/auto_build.py` builds exactly the modules named in `CXXFLAGS` and handles cleaning automatically:

```bash
# Build selected modules
CXXFLAGS="-DBITCOIN_CORE -DRUST_BITCOIN" ./scripts/auto_build.py

# Full clean before building
CLEAN_BUILD="FULL" CXXFLAGS="-DBITCOIN_CORE -DRUST_BITCOIN" ./scripts/auto_build.py

# Clean specific modules only
CLEAN_BUILD="-DSECP256K1" CXXFLAGS="-DBITCOIN_CORE" ./scripts/auto_build.py
```

### Build every present module

The `scripts/build-all.sh` script iterates over `modules/*/` and builds anything with a `Makefile`. CI uses this for commit-by-commit builds.

```bash
bash scripts/build-all.sh
```

### Docker (recommended for reproducibility)

```bash
# Quick run via just
just run descriptor_parse

# Or via docker compose directly
docker compose up descriptor_parse --build --force-recreate
```

See [`RUNNING.md`](./RUNNING.md) for Docker options, environment variables, and crash reproduction.

## Testing

### Custom mutator tests

```bash
cd custommutator && make test_onion
./custommutator/test_onion
```

### Entropylab smoke-test

Run all Entropylab targets for a short duration:

```bash
make run-entropylab
```

Override per-target fuzz time:

```bash
FUZZ_TIME=5 make run-entropylab
```

## Code Formatting

Format C++ code before committing:

```bash
make format
```

Check formatting in CI style (returns non-zero on violations):

```bash
make check-format-all
```

The CI workflow (`.github/workflows/format-check.yml`) also checks Rust, Go, Java, and Python formatting.

## Running CI Locally with `act`

Install [`act`](https://github.com/nektos/act) to run GitHub Actions workflows on your machine.

### Quick start

Because the workflow performs `git fetch` commands against GitHub, pass a token so HTTPS auth works inside the container:

```bash
act -s GITHUB_TOKEN="$(gh auth token)"
```

If you don't have the `gh` CLI, create a classic token at https://github.com/settings/tokens with `repo` scope and pass it explicitly.

### Run a specific workflow

```bash
# Format check only
act -W .github/workflows/format-check.yml -s GITHUB_TOKEN="$(gh auth token)"

# Tests only
act -W .github/workflows/test.yml -s GITHUB_TOKEN="$(gh auth token)"

# Commit-by-commit build (the workflow we fixed for act compatibility)
act -W .github/workflows/commit-by-commit-build.yml -s GITHUB_TOKEN="$(gh auth token)"
```

### Run a specific job

```bash
act -j build -s GITHUB_TOKEN="$(gh auth token)"
```

### Troubleshooting `act`

| Symptom | Fix |
|---------|-----|
| `Permission denied (publickey)` on `git fetch` | Pass `-s GITHUB_TOKEN=...` as shown above. The workflow rewrites the origin URL to HTTPS when the token is present. |
| `Invalid JSON: unexpected end of JSON input` in matrix | This cascades from the `git fetch` failure. Fix the auth issue first. |
| Submodules fail to fetch | Some submodules use SSH. Either forward your SSH agent (`act --env SSH_AUTH_SOCK=$SSH_AUTH_SOCK`) or rewrite those submodule URLs to HTTPS in `.gitmodules`. |
| Large images / slow builds | Use a smaller image: `act -P ubuntu-24.04=catthehacker/ubuntu:act-latest` |

## Adding a New Fuzz Target

Targets live in `driver.cpp`. To add one:

1. **Declare the interface** in `include/bitcoinfuzz/basemodule.h`
2. **Implement the driver loop** in `driver.cpp` (`Driver::<Target>()`)
3. **Wire it into `Run()`** in `driver.cpp`
4. **Implement in each module** that should support it (`modules/<name>/module.cpp`)

For an **Entropylab-only** target, the edit surface is smaller:

1. `modules/entropylab/bitcoinfuzz_wrappers.rs` — Rust FFI shim
2. `modules/entropylab/entropylab_fuzz_lib/entropylab_fuzz_lib.h` — C declaration
3. `modules/entropylab/module.h` — `BaseModule` override declaration
4. `modules/entropylab/module.cpp` — C++ implementation
5. `Makefile` — add to `run-entropylab` loop if doing smoke tests

If the target is **new to bitcoinfuzz core**, also edit:
- `include/bitcoinfuzz/basemodule.h` — add virtual method
- `driver.cpp` — add `Driver::*Target()` method + `Run()` branch

See [`OOGA-BOOGA.md`](./OOGA-BOOGA.md) for the Entropylab coverage roadmap and gap analysis.

## Adding a New Module

1. Create `modules/<name>/` with a `Makefile` that builds `module.a`
2. Implement `module.cpp` and `module.h` inheriting from `BaseModule`
3. Add a `-D<NAME>` flag to the root `Makefile` (follow existing patterns)
4. Add a `README.md` inside `modules/<name>/` with build instructions and dependencies
5. If the module needs a git submodule, add it under `external/` and update `.gitmodules`

## Environment Variables

| Variable | Effect |
|----------|--------|
| `FUZZ` | Selects the fuzz target at runtime |
| `CXXFLAGS` | Enables modules at **build time** via `-D<MODULE>` |
| `MODULES` | Filters which compiled modules to load at runtime (comma-separated) |
| `LOG_OUTPUTS=1` | Prints every module's response (very noisy) |
| `FUZZ_TIME` | Per-target duration for `make run-entropylab` (default: 10 seconds) |

Copy `.env.example` to `.env` to set libFuzzer flags such as `LIBFUZZ_RUNS` and `LIBFUZZ_TIMEOUT` when using Docker.

## Selective Module Loading

You can load only specific modules without recompiling:

```bash
MODULES="BITCOIN_CORE,RUST_BITCOIN" FUZZ=bip32_master_keygen ./bitcoinfuzz
```

Useful for narrowing down which implementation disagrees in a differential mismatch.

## Reproducing Crashes

See the **Reproducing Crashes** section in [`RUNNING.md`](./RUNNING.md) for:
- Replay commands with `--entrypoint`
- Using `LOG_OUTPUTS=1` to see module disagreements
- Minimizing crashes with `-minimize_crash=1`
