# Experiment dynamic crossbeam_channel select

Experiment with dynamically adding/remove receivers
from crossbeam_channel select. This devolved into a
compile error. This is a relatively simple example
of the compile error.

If line 33 is commented out there is no error:
```
$ cargo run
   Compiling exper_dynamic_crossbeam_channel_select v0.1.0 (/home/wink/prgs/rust/myrepos/exper_dynamic_crossbeam_channel_select)
    Finished dev [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/exper_dynamic_crossbeam_channel_select`
main:+
main:-
```

If line 33 is present then we have an error:
```

```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
