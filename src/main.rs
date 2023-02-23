use std::cell::RefCell;

use crossbeam_channel::{unbounded, Receiver, Select};

fn add_to_sel<'a>(sel: &'a mut Select<'a>, rx: &'a Receiver<i32>) {
    println!("add_to_sel: rx={:p}", rx);
    sel.recv(rx);
}

fn main() {
    let mut sel = Select::new();

    let receivers: RefCell<Vec<Receiver<i32>>> = RefCell::new(Vec::new());

    let (_tx1, rx1) = unbounded::<i32>();
    let (_tx2, rx2) = unbounded::<i32>();

    receivers.borrow_mut().push(rx1);
    receivers.borrow_mut().push(rx2);

    let r_rx1 = &receivers.borrow()[0];
    let r_rx2 = &receivers.borrow()[1];

    println!("&*r_rx1={:p}", &*r_rx1);
    println!("&*r_rx1={:p}", &r_rx1);
    println!("&*r_rx2={:p}", &*r_rx2);

    add_to_sel(&'a mut sel, r_rx1);
    //sel.recv(r_rx1);
    sel.recv(r_rx2);

    //// Allocate first receiver
    //let (_tx1, rx1) = unbounded::<i32>();
    //receivers.borrow_mut().push(rx1);
    //let r_rx1 = &receivers.borrow()[0];
    //println!("&*r_rx1={:p}", &*r_rx1);
    //sel.recv(&*r_rx1);

    //// Allocate second receiver
    //let (_tx2, rx2) = unbounded::<i32>();
    //receivers.borrow_mut().push(rx2);
    //let r_rx2 = &receivers.borrow()[1];
    //println!("&*r_rx2={:p}", &*r_rx2);
    //sel.recv(&*r_rx2);

    //let lcl_i32 = 1;
    //println!("&lcl_i32={:p} lcl_i32={}", &lcl_i32, lcl_i32);

    //let box_i32 = Box::new(1);
    //println!("&box_i32={:p} &*box_i32={:p} box_i32={}", &box_i32, &*box_i32, box_i32);

    //let lcl_string = "astring".to_string();
    //println!("&lcl_string={:p} &*lcl_string={:p} lcl_string={}", &lcl_string, &*lcl_string, lcl_string);

    //let box_string = Box::new("astring".to_string());
    //println!("&box_string={:p} &*box_string={:p} box_string={}", &box_string, &*box_string, box_string);

    //// Allocating two receivers in a loop does not compile!
    //for i in 0..1 {
    //    // Allocate next receiver sometime in the future
    //    let (_tx2, rx2) = unbounded::<i32>();
    //    receivers.borrow_mut().push(rx2);

    //    println!("{i}: {:?}", receivers.borrow().len());
    //    println!("&receivers={:p} receivers={:?}", &receivers, receivers);
    //    println!("&receivers.borrow()={:p} &*receivers.borrow()={:p} &receivers.borrow()[0]={:p}", &receivers.borrow(), &*receivers.borrow(), &receivers.borrow()[0]);

    //    for i in 0..receivers.borrow().len() {
    //        let x = receivers.borrow();
    //        println!("receivers.borrow()[{i}] {:p}={:?}", &x[i], x[i]);
    //    }
    //    let rx = &receivers.borrow()[i];
    //    println!("{i}: &rx={:p} &*rx={:p} {:?}", &rx, &*rx, rx);
    //    sel.recv(&*rx);
    //}
}
