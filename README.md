# Experiment dynamic crossbeam_channel select

Experiment with dynamically adding/remove receivers
from crossbeam_channel select. At the moment I can't
get this to work by incrementally creating receivers. But
if I preallocate receivers I can incrementally add them
to a crossbeam_channel Select object.

But, I believe it's possible as shown by the "working" test in
mutable_receivers_vec. In there I declare `receivers: Rc<RefCell<Vec<Receiver<i32>>>>`
and incrementally add receivers and show that the address, `ref_rx`,
are the same as the addresses printed after the loop.
```
wink@3900x 23-02-23T20:00:49.951Z:~/prgs/rust/myrepos/exper_dynamic_crossbeam_channel_select (Simple-example-showing-compile-error)
$ cargo run
   Compiling exper_dynamic_crossbeam_channel_select v0.1.0 (/home/wink/prgs/rust/myrepos/exper_dynamic_crossbeam_channel_select)
    Finished dev [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/exper_dynamic_crossbeam_channel_select`

add_rx_from_immutable_rcvrs: ref_rx=0x55df667baae0
add_rx_from_immutable_rcvrs: ref_rx=0x55df667baaf0
ok: idx=0 ref_rx=0x55df667bac30
ok: idx=1 ref_rx=0x55df667bac40
ok: &(*receivers).borrow()[0]=0x55df667bac30
ok: &(*receivers).borrow()[1]=0x55df667bac40
```

I've also used "cfg" features cerr1, cerr2, cerr3 and cerr4 so
the errors output by the different code can be seen:
> Note you can run 2 or more cfgs by listing them; `cargo build --features cerr1,cerr2,cerr3,cerr4`
```
wink@3900x 23-02-23T19:21:49.151Z:~/prgs/rust/myrepos/exper_dynamic_crossbeam_channel_select (Simple-example-showing-compile-error)
$ cargo build --features cerr1
   Compiling exper_dynamic_crossbeam_channel_select v0.1.0 (/home/wink/prgs/rust/myrepos/exper_dynamic_crossbeam_channel_select)
error[E0716]: temporary value dropped while borrowed
  --> src/main.rs:70:27
   |
70 |             let ref_rx = &(*receivers).borrow()[idx];
   |                           ^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
71 |             println!("cerr1: idx={idx} ref_rx={ref_rx:p}");
72 |             sel.recv(ref_rx);
   |             ---------------- borrow later used here
73 |         }
   |         - temporary value is freed at the end of this statement
   |
   = note: consider using a `let` binding to create a longer lived value

For more information about this error, try `rustc --explain E0716`.
error: could not compile `exper_dynamic_crossbeam_channel_select` due to previous error
wink@3900x 23-02-23T19:26:58.945Z:~/prgs/rust/myrepos/exper_dynamic_crossbeam_channel_select (Simple-example-showing-compile-error)
$ cargo build --features cerr2
   Compiling exper_dynamic_crossbeam_channel_select v0.1.0 (/home/wink/prgs/rust/myrepos/exper_dynamic_crossbeam_channel_select)
error[E0716]: temporary value dropped while borrowed
  --> src/main.rs:84:45
   |
84 |             add_rx_mut_rcvrs(&mut sel, &mut receivers.borrow_mut(), rx);
   |                              --------       ^^^^^^^^^^^^^^^^^^^^^^     - temporary value is freed at the end of this statement
   |                              |              |
   |                              |              creates a temporary value which is freed while still in use
   |                              borrow later used here
   |
   = note: consider using a `let` binding to create a longer lived value

For more information about this error, try `rustc --explain E0716`.
error: could not compile `exper_dynamic_crossbeam_channel_select` due to previous error
wink@3900x 23-02-23T19:27:01.281Z:~/prgs/rust/myrepos/exper_dynamic_crossbeam_channel_select (Simple-example-showing-compile-error)
$ cargo build --features cerr3
   Compiling exper_dynamic_crossbeam_channel_select v0.1.0 (/home/wink/prgs/rust/myrepos/exper_dynamic_crossbeam_channel_select)
error[E0716]: temporary value dropped while borrowed
   --> src/main.rs:99:45
    |
99  |             add_rx_mut_rcvrs(&mut sel, &mut receivers.borrow_mut(), rx1);
    |                                             ^^^^^^^^^^^^^^^^^^^^^^      - temporary value is freed at the end of this statement
    |                                             |
    |                                             creates a temporary value which is freed while still in use
100 |             add_rx_mut_rcvrs(&mut sel, &mut receivers.borrow_mut(), rx2);
    |                              -------- borrow later used here
    |
    = note: consider using a `let` binding to create a longer lived value

For more information about this error, try `rustc --explain E0716`.
error: could not compile `exper_dynamic_crossbeam_channel_select` due to previous error
wink@3900x 23-02-23T19:27:02.719Z:~/prgs/rust/myrepos/exper_dynamic_crossbeam_channel_select (Simple-example-showing-compile-error)
$ cargo build --features cerr4
   Compiling exper_dynamic_crossbeam_channel_select v0.1.0 (/home/wink/prgs/rust/myrepos/exper_dynamic_crossbeam_channel_select)
error[E0716]: temporary value dropped while borrowed
   --> src/main.rs:117:30
    |
117 |                 let r_rx1 = &(*receivers).borrow()[0];
    |                              ^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
...
121 |             }
    |             - temporary value is freed at the end of this statement
...
127 |                 sel.recv(r_rx2); // If enabled error[E0716] on line 23
    |                 --------------- borrow later used here
    |
    = note: consider using a `let` binding to create a longer lived value

For more information about this error, try `rustc --explain E0716`.
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
