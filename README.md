# serde_dyn
Rust crate that assists with automatically identifying types in serialized data.

This allows you to use UUID values to select deserialization instructions to use at runtime,
rather than being forced to pick a type to deserialize at compile time.

This crate was designed to solve a problem in the [Amethyst game engine](https://amethyst.rs)
but I hope you'll find it useful outside of that context too.
