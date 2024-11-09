# Aspen Halls (a video game)

> WARNING: git commits may be broken until v1.0
>
> eventually further projects details can be found at the [page](https://hellzbellz123.github.io/AspenHalls/)

Took me 3 years too get around to updating this, time to finally get started i guess.

funny story, this was originally started as 3d zelda clone in unity.
However i gave up and did not touch it for a really long time

[![No Maintenance Intended](http://unmaintained.tech/badge.svg)](http://unmaintained.tech/)
[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)
![Repo Size](https://img.shields.io/github/repo-size/hellzbellz123/AspenHalls?color=2948ff&label=Repo%20Size&style=flat-square)

<p align="center">
    <a href="https://github.com/Hellzbellz123/AspenHalls/releases"><img alt="GitHub release (latest by date)" src="https://img.shields.io/github/v/release/Hellzbellz123/AspenHalls?label=download&style=flat-square"></a>
</p>

## Platforms

- Library (The game can be used as a library for porting too other platforms or using other init strategys)
- Native (MacOs, Linux & Windows, a single launcher built for each target)
- Web (Wasm)
- Mobile
  - Android
  - iOS (⚠️ Soon)

## Requirements

- Rust
- Cargo
- [Cargo Make](https://github.com/sagiegurari/cargo-make) (general make targets for each platform and a package workflow)
- [Cargo Xwin](https://github.com/rust-cross/cargo-xwin) (required for windows development)
- [Cargo Apk](https://github.com/rust-mobile/cargo-apk) (required for android development)
- [Trunk](https://trunkrs.dev) (required for web development)

## Development Guide

- Run `cargo make run-native` for run desktop dev mode
- Run `cargo make run-mobile` too build and install on connected adb device
- Run `cargo make run-web` too start webserver and host wasm game there
- Run `cargo make` for all available tasks

## Usage as Library

why would you use this as a library?

- create ports too new platforms
- maybe mods?

## Build/Compile Time Benchmarks

Host Specs:

- cpu: Ryzen 5 5600X
- ram: 32gb 3600mhz
- os: Archlinux
- Compiler info
  - Rust Version: nightly-2023-11-20

Benchmarks:

- hyperfine 'RUSTFLAGS="-Z threads=1" cargo clippy'
  - Time (mean ± σ): 40.598 s ± 0.501 s [User: 218.603 s, System: 25.618 s]
  - Range (min … max): 40.094 s … 41.304 s 5 runs
- hyperfine 'RUSTFLAGS="-Z threads=8" cargo clippy' --prepare 'cargo clean' --runs=3 --warmup=2
  - Time (mean ± σ): 40.703 s ± 0.738 s [User: 219.198 s, System: 26.808 s]
  - Range (min … max): 39.491 s … 41.354 s 5 runs
- hyperfine 'cargo clippy' --prepare 'cargo clean' --runs=3 --warmup=2
  - Time (mean ± σ): 38.928 s ± 0.467 s [User: 217.681 s, System: 25.882 s]
  - Range (min … max): 38.531 s … 39.443 s 3 runs
