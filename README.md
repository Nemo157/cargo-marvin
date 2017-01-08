# cargo-[marvin][]

[Inspired by ebrythil on Reddit][inspiration].

<pre>
&gt; cargo install --git https://github.com/Nemo157/cargo-marvin
&gt; cargo marvin build --example moved_value
<span style="color:green;"></span><span style="color:green;font-weight:bold;">   Compiling</span> cargo-marvin v0.1.0 (file:///Users/Nemo157/sources/cargo-marvin)
<span style="color:red;font-weight:bold;">error</span><span style="color:red;font-weight:bold;">[E0382]</span><span style="font-weight:bold;">: </span><span style="font-weight:bold;">use of moved value: `foo`. Again. There goes my hope you learned that by now. Serously. It's not that hard...</span>
 <span style="color:blue;font-weight:bold;">    --&gt;</span> examples/moved_value.rs:7:9
      <span style="color:blue;font-weight:bold;">|</span>
 <span style="color:blue;font-weight:bold;">   6</span> <span style="color:blue;font-weight:bold;">|</span>     bar(<span style="color:blue;font-weight:bold;">foo</span>);
      <span style="color:blue;font-weight:bold;">|</span>         <span style="color:blue;font-weight:bold;">---</span> <span style="color:blue;font-weight:bold;">value moved here</span>
 <span style="color:blue;font-weight:bold;">   7</span> <span style="color:blue;font-weight:bold;">|</span>     bar(<span style="color:red;font-weight:bold;">foo</span>);
      <span style="color:blue;font-weight:bold;">|</span>         <span style="color:red;font-weight:bold;">---</span> <span style="color:red;font-weight:bold;">value used here after move</span>
      <span style="color:blue;font-weight:bold;">|</span>
      <span style="color:blue;font-weight:bold;">=</span> <span style="font-weight:bold;">note</span>: move occurs because `foo` has type `std::string::String`, which does not implement the `Copy` trait

<span style="color:red;font-weight:bold;">error</span><span style="color:red;font-weight:bold;"></span><span style="font-weight:bold;">: </span><span style="font-weight:bold;">aborting due to previous error. Maybe try a garbage-collected language?</span>
<span style="color:red;"></span><span style="color:red;font-weight:bold;">error:</span> Could not compile `cargo-marvin`.

To learn more, run the command again with --verbose.

<span style="color:red;"></span><span style="color:red;font-weight:bold;">error:</span> Really, you think I'm just gonna ignore all those errors above?
</pre>

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
