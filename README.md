# Apery

Apery is a free USI shogi engine derived from Stockfish and Apery(C++ version).
Apery was rewritten in Rust.
This requires a USI-compatible GUI (e.g. Shogidokoro, ShogiGUI, MyShogi).

## Usage

This requires the evaluation function binaries as a submodule.
Use this command If you have not have the evaluation function binaries at apery/eval/.
```bash
cd apery
git submodule init
git submodule update
```

If you have evaluation function binaries, Apery can run.
The following is a sample command to run Apery.
```bash
cd apery
cargo run --release
isready
go byoyomi 5000
quit
```
See USI protocol on the web for details.

## Rust toolchain

nightly only

## Install

cargo install --path .

## Uninstall

cargo uninstall apery

## License

Apery is free, and distributed under the GNU General Public License version 3 (GPL v3).

See the file named "LICENSE" for details.
