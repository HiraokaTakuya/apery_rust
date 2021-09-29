# Apery

Apery is a free USI shogi engine derived from [Stockfish](https://github.com/official-stockfish/Stockfish) and [Apery(C++ version)](https://github.com/HiraokaTakuya/apery).
Apery was rewritten in Rust.
Apery requires a USI-compatible GUI (e.g. [Shogidokoro](http://shogidokoro.starfree.jp/), [ShogiGUI](http://shogigui.siganus.com/), [MyShogi](https://github.com/yaneurao/MyShogi)).

## Usage

Apery requires the evaluation function binaries as a submodule.
The following are sample commands to clone, build and run Apery.
```bash
git clone https://github.com/HiraokaTakuya/apery_rust.git && \
cd apery_rust && \
git submodule init && \
git submodule update && \
cargo build --release && \
./target/release/apery <<EOF
isready
go byoyomi 5000
wait
EOF
```
See USI protocol on the web for details.

## Rust toolchain

stable

## Install

1. Install rustup and cargo

If you use macOS, Linux, or another Unix-like OS, run the following in your terminal.
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
If you use Windows, install [Visual Studio C++ Build tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) and download the [rustup installer](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe).

See detail.
[https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

2. Install apery

```bash
cargo install --git https://github.com/HiraokaTakuya/apery_rust.git
```

## Uninstall

```bash
cargo uninstall apery
```

## Build

An execute binary file is generated at apery_rust/target/release/apery
```bash
git clone https://github.com/HiraokaTakuya/apery_rust.git && \
cd apery_rust && \
git submodule init && \
git submodule update && \
cargo build --release
```

If you do not use the evaluation file, build with "material" feature instead of "kppt" feature.
```bash
cargo build --release --no-default-features --features "material"
```

## Documentation

Build the documentation in target/doc in rustdoc's usual format.
```bash
cargo doc --document-private-items --no-deps --open
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
