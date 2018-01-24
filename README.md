# Ook

A compile-time Ook! interpreter with Rust macro.

Based on [The Ook! Example](https://danielkeep.github.io/tlborm/book/aeg-ook.html) of [The Little Book of Rust Macros](https://danielkeep.github.io/tlborm/).

## Example

Here is the a + b example of this interpreter.
Numbers are represented with the number of `@`'s.

```rust
Ook!(((@ @) (@ @ @));
        Ook. Ook!
        Ook. Ook?  Ook. Ook!
        Ook! Ook?
        Ook! Ook!  Ook? Ook.  Ook. Ook.  Ook. Ook?
        Ook? Ook!
        Ook? Ook.
        Ook! Ook.
    );
```
Build with cargo
```shell
cargo build --example a_plus_b
```
You will get the output below:
```
( @ @ @ @ @ )
```