Common types found in the Stellare ecosystem, it can be used standalone.

Some of the most common types share overlap with many other crates. Namely the `math` module contains many types that are present in many other crates such as [glam](https://github.com/bitshifter/glam-rs), [cgmath](https://github.com/rustgd/cgmath), [euclid](https://github.com/servo/euclid), and [many](https://github.com/bitshifter/mathbench-rs) others. 

# Why?

The main reason this crate exist is to provide an interface for the Stellare crates that adapts to the design needs of the rest of the crates.

As a standalone crate; the purpose of it is not to "compete" with them (some crates inside the ecosystem might use some of said crates directly or indirectly!).

Exposing a dependency's type to the public API can be problematic as well. Implementing these types as part of the ecosystem allows for more control over versioning. 

This crate differentiates itself from other math libraries through strict yet user-friendly type safety. While glam prioritizes performance, nalgebra emphasizes mathematical completeness, and uom provides full-featured unit systems, this crate strikes a middle ground, balancing performance, type safety, and compile-time efficiency in that order.

In an effort to integrate well with other crates that have overlapping functionality, this crate provides feature flags to implement From and Into, allowing conversion to and from other crates' types when applicable. This is done on a best-effort basis. (WIP)
