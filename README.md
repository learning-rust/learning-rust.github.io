# Learning Rust

### Basics
* [Why Rust](source/docs/a1.why_rust.md)
* [Installation](source/docs/a2.installation.md)
* [Hello World](source/docs/a3.hello_world.md)
* [Cargo,crates and basic project structure](source/docs/a4.cargo,crates_and_basic_project_structure.md)
* [Comments and documenting the code](source/docs/a5.comments_and_documenting_the_code.md)
* [Variable bindings, constants and statics](source/docs/a6.variable_bindings,constants_and_statics.md)
* [Functions](source/docs/a7.functions.md)
* [Primitive data types](source/docs/a8.primitive_data_types.md)
* [Operators](source/docs/a9.operators.md)
* [Control flows](source/docs/a10.control_flows.md)

### Beyond The Basics
* [Vectors](source/docs/b1.vectors.md)
* [Structs](source/docs/b2.structs.md)
* [Enums](source/docs/b3.enums.md)
* [Generics](source/docs/b4.generics.md)
* [Impls and traits](source/docs/b5.impls_and_traits.md)

### The Tough Part
* [Ownership](source/docs/c1.ownership.md)
* [Borrowing](source/docs/c2.borrowing.md)
* [Lifetimes](source/docs/c3.lifetimes.md)
    
### Lets Get It Started
* [Code organization](source/docs/d1.code_organization.md)
* [Functions](source/docs/d2.functions.md)
* [Modules](source/docs/d3.modules.md)
* [Crates](source/docs/d4.crates.md)
* [Workspaces](source/docs/d5.workspaces.md)
* [use](source/docs/d6.use.md)
* [std, primitives and preludes](source/docs/d7.std_primitives_and_preludes.md)

### Error Handling
* [Smart Compiler](source/docs/e1.smart_compiler.md)
* [Panicking](source/docs/e2.panicking.md)
* [Option and Result](source/docs/e3.option_and_result.md)
* [Unwrap and Expect](source/docs/e4.unwrap_and_expect.md)
* [Error and None Propagation](source/docs/e5.error_and_none_propagation.md)
* [Combinators](source/docs/e6.combinators.md)
* [Custom Error Types](source/docs/e7.custom_error_types.md)

---
* Site : http://learning-rust.github.io
* Medium: https://medium.com/learning-rust

> 🐣 I am a **Sri Lankan** 🇱🇰 Web Developer who lives in **Vietnam** 🇻🇳. So I am not a native English speaker and just learning Rust, If you found any mistake or something need to be changed, even a spelling or a grammar mistake, feel free to create a pull request. Thanks.

---

# learning-rust.github.io(source)

The website is built with [Hexo](https://hexo.io/) Nodejs blog framework. You can see the generated files at [learning-rust/learning-rust.github.io](https://github.com/learning-rust/learning-rust.github.io) repository.

## Getting started

Install dependencies:

``` bash
$ git clone git@github.com:learning-rust/site.git
$ cd site
$ npm install
$ npm install hexo-cli -g
```

Generate:

``` bash
$ hexo generate
```

Run server:

``` bash
$ hexo server
```