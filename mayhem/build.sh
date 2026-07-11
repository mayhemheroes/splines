#!/usr/bin/env bash
#
# mayhem/build.sh — build this repo's cargo-fuzz target(s) as sanitized libFuzzer
# binaries (OSS-Fuzz Rust path: cargo-fuzz + ASan via RUSTFLAGS). EDIT per repo.
#
# Runs inside the commit image (RUST mayhem/Dockerfile) as `mayhem` in /mayhem.
# The Rust toolchain + cargo registry live at $CARGO_HOME=/opt/toolchains/rust/cargo
# (pinned by the Dockerfile ENV — absolute, $HOME-independent).
#
# AIR-GAPPED CONTRACT (SPEC §6.5): the PATCH tier re-runs THIS script OFFLINE.
#   - This FIRST build (in CI, online) populates the cargo registry under $CARGO_HOME.
#   - The PATCH re-run resolves crates from that cache. The rlenv runtime exports
#     CARGO_NET_OFFLINE=true for the re-run so cargo won't try to refresh the
#     crates.io index over the (absent) network — so do NOT hard-code `--offline`
#     here (it would break this first, online build).
#   - For a FULLY self-contained image (no runtime flag needed) instead vendor:
#       cargo vendor --versioned-dirs vendor   # commit vendor/ + a .cargo/config.toml
#     with [source.crates-io] replace-with = "vendored-sources".
set -euo pipefail

# clang rejects SOURCE_DATE_EPOCH='' — must be unset or a valid integer.
[ -n "${SOURCE_DATE_EPOCH:-}" ] || unset SOURCE_DATE_EPOCH

: "${MAYHEM_JOBS:=$(nproc)}"
# cargo-fuzz has no --jobs flag; cargo reads parallelism from CARGO_BUILD_JOBS.
export CARGO_BUILD_JOBS="$MAYHEM_JOBS"

cd "$SRC"

# OSS-Fuzz Rust libFuzzer+ASan flags. For Rust, ASan is applied via RUSTFLAGS
# (-Zsanitizer=address), NOT via the clang-oriented $SANITIZER_FLAGS (rustc ignores
# those). We reference $SANITIZER_FLAGS to honor the contract: if the override drops
# 'address' we drop the rust sanitizer to match. --cfg fuzzing matches libfuzzer-sys.
SANITIZER_FLAGS="${SANITIZER_FLAGS-}"
case "$SANITIZER_FLAGS" in
  *address*|"") RUST_SANITIZER="-Zsanitizer=address" ;;  # default (ASan on, halting)
  *)            RUST_SANITIZER="" ;;                      # explicit no-sanitizer override
esac

# ── DWARF < 4 enforcement (§6.2 item 10) ───────────────────────────────────────
# Mayhem's triage can't read DWARF >= 4, and the nightly's bundled LLVM defaults to
# DWARF 5. Thread $RUST_DEBUG_FLAGS (the rust arm of the DEBUG_FLAGS contract) to pin
# DWARF < 4 for our Rust code, and force the C/C++ CUs (libFuzzer, compiled via the cc
# crate) with -gdwarf-3.
export RUST_DEBUG_FLAGS="${RUST_DEBUG_FLAGS:--C debuginfo=2 -C force-frame-pointers=yes -C llvm-args=--dwarf-version=3}"
export CFLAGS="${CFLAGS:+$CFLAGS }-gdwarf-3"
export CXXFLAGS="${CXXFLAGS:+$CXXFLAGS }-gdwarf-3"

# Rust's ASan runtime archive (librustc-nightly_rt.asan.a) is built with the bundled
# LLVM (DWARF 5) and is linked BEFORE the project code, so its CU would land first in
# .debug_info and fail the check. Strip its debug sections once (idempotent; the
# stripped .a is baked into the image, so the offline PATCH re-run reproduces it).
ASAN_RT="$(find "$RUSTUP_HOME/toolchains" -name 'librustc-*_rt.asan.a' 2>/dev/null | head -1)"
if [ -n "$ASAN_RT" ] && [ -f "$ASAN_RT" ]; then
    echo "Stripping debug info from Rust ASan runtime to enforce DWARF < 4: $ASAN_RT"
    objcopy --strip-debug "$ASAN_RT" || true
fi

export RUSTFLAGS="${RUSTFLAGS:-} --cfg fuzzing ${RUST_SANITIZER} ${RUST_DEBUG_FLAGS}"

# EDIT: the cargo-fuzz crate directory. Use upstream's own fuzz/ when it builds on
# the pinned nightly; otherwise add an ADDITIVE mayhem/fuzz/ crate (leaves upstream
# untouched) and point --fuzz-dir at it.
FUZZ_DIR="mayhem/fuzz"
TRIPLE="x86_64-unknown-linux-gnu"

# Discover every target from the crate's fuzz_targets/ dir (one binary per target).
FUZZ_TARGETS=()
for f in "$FUZZ_DIR"/fuzz_targets/*.rs; do
  FUZZ_TARGETS+=("$(basename "${f%.*}")")
done
[ "${#FUZZ_TARGETS[@]}" -gt 0 ] || { echo "ERROR: no fuzz targets under $FUZZ_DIR/fuzz_targets/" >&2; exit 1; }

echo "=== cargo fuzz build (image nightly, ASan via RUSTFLAGS) ==="
echo "RUSTFLAGS=$RUSTFLAGS"
echo "targets: ${FUZZ_TARGETS[*]}"

# Use the image's DEFAULT toolchain (the Dockerfile pinned it). A `+toolchain`
# override would make rustup try to install another channel into the locked /opt/rust.
for t in "${FUZZ_TARGETS[@]}"; do
  echo "--- building fuzz target: $t ---"
  cargo fuzz build --fuzz-dir "$FUZZ_DIR" -O --debug-assertions "$t"
  bin="$SRC/$FUZZ_DIR/target/$TRIPLE/release/$t"
  [ -x "$bin" ] || { echo "ERROR: expected fuzz binary not found at $bin" >&2; exit 1; }
  cp "$bin" "/mayhem/$t"     # EDIT the output path/name to match your Mayhemfile target:
  echo "built /mayhem/$t"
done

echo "=== building test suite (normal flags) ==="
env -u RUSTFLAGS cargo test --no-run --tests
echo "build.sh complete"
