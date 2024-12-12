# Some things to take into account

The sources from where this document takes its content are:

[5 deadly Rust anti-patterns to avoid](https://www.youtube.com/watch?v=SWwTD2neodE)

[8 deadly mistakes beginner Rust developers make](https://www.youtube.com/watch?v=PbR4ECFIckg)

[Common Newbie Mistakes and Bad Practices in Rust](https://adventures.michaelfbryan.com/posts/rust-best-practices/bad-habits/)


## Indirection

Understand inmutability and borrowing rules!.

Avoid unnecessary indirection. Pay attention and decide when to use owned, slices or references (&)

## Slice indexing

Don't use loops and indexes ( `xxxxxx[n]` ).

Use iterators and functional operators: map(), filter(), reduce(), flatten(), zip(),...

## Option values

Don't use sentinel values (like -1, "", null,...) to indicate something special.

Use `Option` type values { Some, None }.

## Enum and pattern matching

When working with data types with only a limited set of possible values. If you use Enums, you can use pattern matching.

Use pattern matching whenewer you can.

Use specialized (concrete, definite, with semantic meaning) data types whenewer possible.

## Error handling

Use `Result` type values.

Use `?` operator to propagate errors.

All custom errors must implement `Error` trait. 

You can use `#[derive(Error, Debug)]` macro to implement.

## Standard Library

### use the traits provided

`Default`, to initialize.

`From` or `TryFrom`, to convert from other types.

`FromStr`, to convert from strings.

### use the macros provided

`todo!`, to compile with unfinished code ("to do" code).

`concat!` or `format!`, to compose strings.

### tooling

`cargo fmt`, to format the code compliying with the standard style guidelines.

`cargo clippy`, to linter.

## General advice

Don't overuse `Rc::`, `Box::`, `&mut`,... to overrule borrowing checks.

Structure your code good enough, so you are not holding long live refereces to objects.

[Rust Koans](https://users.rust-lang.org/t/rust-koans/2408)

[The Rustonomicon](https://doc.rust-lang.org/nightly/nomicon/)
