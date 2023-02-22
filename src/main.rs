use crossbeam_channel::{unbounded, Receiver, Select};

fn main() {
    println!("main:+");
    type BoxMsgAny = Box<dyn std::any::Any + Send>;

    let (_cmd_tx, cmd_rx) = unbounded::<BoxMsgAny>();

    let mut sel = Select::new();
    let mut receivers: Vec<Receiver<BoxMsgAny>> = Vec::new();

    receivers.push(cmd_rx);
    sel.recv(&receivers[0]);

    let (_tx, rx) = unbounded::<BoxMsgAny>();

    // $ cargo run
    //    Compiling exper_dynamic_crossbeam_channel_select v0.1.0 (/home/wink/prgs/rust/myrepos/exper_dynamic_crossbeam_channel_select)
    // error[E0502]: cannot borrow `receivers` as mutable because it is also borrowed as immutable
    //   --> src/main.rs:32:5
    //    |
    // 13 |     sel.recv(&receivers[0]);
    //    |               --------- immutable borrow occurs here
    // ...
    // 32 |     receivers.push(rx);
    //    |     ^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
    // 33 |     sel.recv(&receivers[0]);
    //    |     ----------------------- immutable borrow later used here
    // 
    // For more information about this error, try `rustc --explain E0502`.
    // error: could not compile `exper_dynamic_crossbeam_channel_select` due to previous error
    receivers.push(rx);
    //sel.recv(&receivers[0]);

    println!("main:-");
}
