# Experiment dynamic crossbeam_channel select

Experiment with dynamically adding/remove receivers
from crossbeam_channel select.

Here is a solution, but I pre-allocate the receivers as I did
in the simple example:
```
wink@3900x 23-02-21T22:26:13.158Z:~/prgs/rust/myrepos/exper_dynamic_crossbeam_channel_select (main)
$ cargo run
   Compiling exper_dynamic_crossbeam_channel_select v0.1.0 (/home/wink/prgs/rust/myrepos/exper_dynamic_crossbeam_channel_select)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/exper_dynamic_crossbeam_channel_select`
main:+
worker:+
worker outer thread:+
worker_t1:+
worker_t1: worker_idx=0 msg=MsgReqAddWorker { worker: Summer { sum: 0 }, rsp_tx: Sender { .. } }
worker outer thread: waiting msg_rsp_sum
Summer.do_work:+
Summer.do_work: before self.sum=0 msg=MsgSum { v: 2 } after self.sum=2
Summer.do_work:+
Summer.do_work: msg=MsgReqSum msg_rsp=MsgRspSum { sum: 2 }
worker outer thread: received msg_rsp_sum=MsgRspSum { sum: 2 }
worker outer thread:-
worker_t1: msg=MsgDone
worker_t1:-
worker:-
main:-
```

Here is the pre-allocation:
```
    74              let mut worker_their_receivers: Vec<Receiver<BoxMsgAny>> = Vec::new();
    75              let mut worker_their_senders: Vec<Sender<BoxMsgAny>> = Vec::new();
    76              let mut worker_our_receivers: Vec<Receiver<BoxMsgAny>> = Vec::new();
    77              let mut worker_our_senders: Vec<Sender<BoxMsgAny>> = Vec::new();
    78              for _ in 1..=10 {
    79                  let (our_tx, their_rx) = unbounded::<BoxMsgAny>();
    80                  let (their_tx, our_rx) = unbounded::<BoxMsgAny>();
    81                  worker_our_senders.push(our_tx);
    82                  worker_our_receivers.push(our_rx);
    83                  worker_their_senders.push(their_tx);
    84                  worker_their_receivers.push(their_rx);```
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
