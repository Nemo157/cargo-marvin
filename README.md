# [cargo-marvin][marvin]

[Inspired by ebrythil on Reddit][inspiration].

```
> cargo install --git https://github.com/Nemo157/cargo-marvin
> cargo marvin build --example moved_value
   Compiling cargo-marvin v0.1.0 (file:///Users/Nemo157/sources/cargo-marvin)
error[E0382]: use of moved value: `foo`. Again. There goes my hope you learned that by now. Serously. It's not that hard...
     --> examples/moved_value.rs:7:9
      |
    6 |     bar(foo);
      |         --- value moved here
    7 |     bar(foo);
      |         --- value used here after move
      |
      = note: move occurs because `foo` has type `std::string::String`, which does not implement the `Copy` trait

error: aborting due to previous error. Maybe try a garbage-collected language?
error: Could not compile `cargo-marvin`.

To learn more, run the command again with --verbose.

error: Really, you think I'm just gonna ignore all those errors above?
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.

[marvin]: https://en.wikipedia.org/wiki/Marvin_(character)
[inspiration]: https://www.reddit.com/r/rust/comments/5mlxps/rust_makes_implicit_invariants_explicit/dc57wd3/
