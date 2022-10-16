# Learning Rust

This website is built using [Hugo](https://gohugo.io/) and [Docura](https://docura.github.io/).

## Local setup
- [Install the extended version of Hugo](https://gohugo.io/getting-started/installing/)
- Clone the repository and start Hugo server
    ```
    $ git clone --depth 1 https://github.com/learning-rust/site.git
    $ cd site
    $ git submodule update --init --recursive
    $ hugo server
    ```

## Content

### Basics
* [Why Rust](content/en/docs/a1.why-rust.md)
* [Installation](content/en/docs/a2.installation.md)
* [Hello World](content/en/docs/a3.hello-world.md)
* [Cargo,crates and basic project structure](content/en/docs/a4.cargo-crates-and-basic-project-structure.md)
* [Comments and documenting the code](content/en/docs/a5.comments-and-documenting-the-code.md)
* [Variable bindings, constants and statics](content/en/docs/a6.variable-bindings-constants-and-statics.md)
* [Functions](content/en/docs/a7.functions.md)
* [Primitive data types](content/en/docs/a8.primitive-data-types.md)
* [Operators](content/en/docs/a9.operators.md)
* [Control flows](content/en/docs/a10.control-flows.md)

### Beyond The Basics
* [Vectors](content/en/docs/b1.vectors.md)
* [Structs](content/en/docs/b2.structs.md)
* [Enums](content/en/docs/b3.enums.md)
* [Generics](content/en/docs/b4.generics.md)
* [Impls and traits](content/en/docs/b5.impls-and-traits.md)

### The Tough Part
* [Ownership](content/en/docs/c1.ownership.md)
* [Borrowing](content/en/docs/c2.borrowing.md)
* [Lifetimes](content/en/docs/c3.lifetimes.md)

### Lets Get It Started
* [Code organization](content/en/docs/d1.code-organization.md)
* [Functions](content/en/docs/d2.functions.md)
* [Modules](content/en/docs/d3.modules.md)
* [Crates](content/en/docs/d4.crates.md)
* [Workspaces](content/en/docs/d5.workspaces.md)
* [use](content/en/docs/d6.use.md)
* [std, primitives and preludes](content/en/docs/d7.std-primitives-and-preludes.md)

### Error Handling
* [Smart Compiler](content/en/docs/e1.smart-compiler.md)
* [Panicking](content/en/docs/e2.panicking.md)
* [Option and Result](content/en/docs/e3.option-and-result.md)
* [Unwrap and Expect](content/en/docs/e4.unwrap-and-expect.md)
* [Error and None Propagation](content/en/docs/e5.error-and-none-propagation.md)
* [Combinators](content/en/docs/e6.combinators.md)
* [Custom Error Types](content/en/docs/e7.custom-error-types.md)

---
* Site : http://learning-rust.github.io
* Medium: https://medium.com/learning-rust

> 🐣 I am a **Sri Lankan**🇱🇰 Web Developer who lives in **Singapore**🇸🇬. I am not a native English speaker and just practicing Rust in my leisure time, while working as a Golang/Devops developer. So, if you found any mistake or something I need to be changed, even a spelling/ grammar mistake, feel free to buzz me.
