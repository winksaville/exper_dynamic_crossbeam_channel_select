use std::cell::RefCell;

use crossbeam_channel::{unbounded, Receiver, Select};

fn main() {
    let mut sel = Select::new();
    let receivers: RefCell<Vec<Receiver<i32>>> = RefCell::new(Vec::new());

    // Allocate first receiver
    let (_tx1, rx1) = unbounded::<i32>();
    receivers.borrow_mut().push(rx1);
    let rx = &receivers.borrow()[0];
    sel.recv(rx);

    // Allocate next receiver sometime in the future
    let (_tx2, rx2) = unbounded::<i32>();
    receivers.borrow_mut().push(rx2);
    let rx = &receivers.borrow()[1];
    sel.recv(rx);
}
