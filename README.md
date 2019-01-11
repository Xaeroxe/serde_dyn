# serde_dyn [![Crates.io](https://img.shields.io/crates/v/serde_dyn.svg)](https://crates.io/crates/serde_dyn)

Rust crate that assists with automatically identifying types in serialized data.

This allows you to use UUID values to select deserialization instructions to use at runtime,
rather than being forced to pick a type to deserialize at compile time.

This crate was designed to solve a problem in the [Amethyst game engine](https://amethyst.rs)
but I hope you'll find it useful outside of that context too.

# Contributing

I welcome contributions and alterations to this project! [Here's some info to help you get started.](https://help.github.com/articles/about-pull-requests/)

- If you would consider your change "substantial" open an issue on the issues tab so we can discuss it before you build it.
- If you're fixing a bug please provide a unit test for the bug fixed so we don't do it again.
- If applicable, new features should have some basic unit tests added too.
- Please format your code with the most recent stable release of rustfmt before submitting your PR.
- I don't have a regular release schedule, if you want something you've added put on crates.io right away make sure to
bump the version number for the project in your pull request.
