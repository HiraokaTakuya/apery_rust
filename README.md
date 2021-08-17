# Apery

Apery is a free USI shogi engine derived from Stockfish and Apery(C++ version).
Apery was rewritten in Rust.
Apery requires a USI-compatible GUI (e.g. Shogidokoro, ShogiGUI, MyShogi).

## Usage

Apery requires the evaluation function binaries as a submodule.
Use this command If you have not have the evaluation function binaries at apery_rust/eval/.
```bash
cd apery_rust
git submodule init
git submodule update
```

If you have evaluation function binaries, Apery can run.
The following is a sample command to run Apery.
```bash
cd apery_rust
cargo build --release
./target/release/apery <<EOF
isready
go byoyomi 5000
wait
EOF
```
See USI protocol on the web for details.

## Rust toolchain

stable

## Build

An execute binary file is generated at apery_rust/target/release/apery
```bash
cargo build --release
```

If you do not use the evaluation file, build with "material" feature instead of "kppt" feature.
```bash
cargo build --release --no-default-features --features "material"
```

## Install

```bash
cargo install --git https://github.com/HiraokaTakuya/apery_rust.git
```

## Uninstall

```bash
cargo uninstall apery
```

## Profile

The following is a sample of how to use the profiler for Ubuntu.

- Install valgrind, kcachegrind
```bash
sudo apt install -y valgrind kcachegrind
```

- Add the following to apery_rust/Cargo.toml
```
[profile.release]
debug = true
```

- Do the following commands.
```bash
# Build and run apery.
cd apery_rust
cargo build --release
valgrind --tool=callgrind ./target/release/apery <<EOF
isready
go byoyomi 60000
wait
EOF
# Show the profiling result.
kcachegrind callgrind.out.???? # ???? is some number.
```

## License

Apery is free, and distributed under the GNU General Public License version 3 (GPL v3).

See the file named "LICENSE" for details.
