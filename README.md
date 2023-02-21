# Experiment dynamic crossbeam_channel select

Experiment with dynamically adding/remove receivers
from crossbeam_channel select.

This runs but a new receiver is not added so we hang and
have to Ctrl-C out of the app:
```
wink@3900x 23-02-21T22:00:14.778Z:~/prgs/rust/myrepos/exper_dynamic_crossbeam_channel_select (main|REBASE 2/3)
$ cargo run
   Compiling exper_dynamic_crossbeam_channel_select v0.1.0 (/home/wink/prgs/rust/myrepos/exper_dynamic_crossbeam_channel_select)
    Finished dev [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/exper_dynamic_crossbeam_channel_select`
main:+
worker:+
worker outer thread:+
worker_t1:+
worker_t1: msg=MsgReqAddWorker { worker: Summer { sum: 0 }, rsp_tx: Sender { .. } }
worker outer thread: waiting msg_rsp_sum
^C
```

If I uncomment line 91 then I get a compile error:
```
wink@3900x 23-02-21T22:05:08.677Z:~/prgs/rust/myrepos/exper_dynamic_crossbeam_channel_select (main|REBASE 2/3)
$ cargo build
   Compiling exper_dynamic_crossbeam_channel_select v0.1.0 (/home/wink/prgs/rust/myrepos/exper_dynamic_crossbeam_channel_select)
error[E0502]: cannot borrow `worker_our_receivers` as mutable because it is also borrowed as immutable
  --> src/main.rs:90:29
   |
90 | ...                   worker_our_receivers.push(our_rx);
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
91 | ...                   sel.recv(worker_our_receivers.last().unwrap());
   |                       ----------------------------------------------
   |                       |        |
   |                       |        immutable borrow occurs here
   |                       immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `exper_dynamic_crossbeam_channel_select` due to previous error
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
