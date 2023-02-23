use crossbeam_channel::{unbounded, Receiver, Select};

fn immutable_receivers_vec() {
    let mut sel = Select::new();

    let (_tx1, rx1) = unbounded::<i32>();
    let (_tx2, rx2) = unbounded::<i32>();

    // If rcvrs is immutable all is well
    {
        // Pre allocate the rcvrs
        let rcvrs = vec![rx1, rx2];

        // Incrementally add them to sel
        add_rx_from_immutable_rcvrs(&mut sel, &rcvrs, 0);
        add_rx_from_immutable_rcvrs(&mut sel, &rcvrs, 1);
    }

    fn add_rx_from_immutable_rcvrs<'a>(
        sel: &mut Select<'a>,
        rcvrs: &'a [Receiver<i32>],
        idx: usize,
    ) {
        let ref_rx = &rcvrs[idx];
        println!("add_rx_from_immutable_rcvrs: ref_rx={ref_rx:p}");
        sel.recv(ref_rx);
    }
}

/// Various techniques showing you can't use a mutable array of receivers.
fn mutable_receivers_vec() {
    #[allow(unused)]
    use std::{cell::RefCell, rc::Rc};

    // This proves to me that "cerr1" should work as it's identical to this
    // except without the `let mut sel = Select::new();` and `sel.recv(ref_rx);`.
    // The "proof" is that `Receiver<i32>` addresses printed in the loop are the
    // same as seen after the loop is completed. Thus "proving" the `Receiver<i32>`s
    // weren't move, released or dropped.
    {
        let receivers: Rc<RefCell<Vec<Receiver<i32>>>> = Rc::new(RefCell::new(Vec::new()));

        for _ in 0..=1 {
            let (_, rx) = unbounded::<i32>();
            {
                receivers.borrow_mut().push(rx);
            }
            let idx = receivers.borrow().len() - 1;
            let ref_rx = &((*receivers).borrow()[idx]);
            println!("ok: idx={idx} ref_rx={ref_rx:p}");
        }
        println!("ok: &(*receivers).borrow()[0]={:p}", &(*receivers).borrow()[0]);
        println!("ok: &(*receivers).borrow()[1]={:p}", &(*receivers).borrow()[1]);
    }

    #[cfg(feature = "cerr1")]
    {
        let mut sel = Select::new();
        let receivers: Rc<RefCell<Vec<Receiver<i32>>>> = Rc::new(RefCell::new(Vec::new()));

        // Won't compile error[E0716]
        for _ in 0..=1 {
            let (_, rx) = unbounded::<i32>();
            {
                receivers.borrow_mut().push(rx);
            }
            let idx = receivers.borrow().len() - 1;
            let ref_rx = &(*receivers).borrow()[idx];
            println!("cerr1: idx={idx} ref_rx={ref_rx:p}");
            sel.recv(ref_rx);
        }
    }

    #[cfg(feature = "cerr2")]
    {
        let mut sel = Select::new();
        let receivers: Rc<RefCell<Vec<Receiver<i32>>>> = Rc::new(RefCell::new(Vec::new()));

        // Won't compile error[E0716]
        for _ in 0..=1 {
            let (_, rx) = unbounded::<i32>();
            add_rx_mut_rcvrs(&mut sel, &mut receivers.borrow_mut(), rx);
        }
    }

    #[cfg(feature = "cerr3")]
    {
        let mut sel = Select::new();

        let (_tx1, rx1) = unbounded::<i32>();
        let (_tx2, rx2) = unbounded::<i32>();

        let receivers: Rc<RefCell<Vec<Receiver<i32>>>> = Rc::new(RefCell::new(Vec::new()));

        // Won't compile error[E0716]
        {
            add_rx_mut_rcvrs(&mut sel, &mut receivers.borrow_mut(), rx1);
            add_rx_mut_rcvrs(&mut sel, &mut receivers.borrow_mut(), rx2);
        }
    }

    #[cfg(feature = "cerr4")]
    {
        let mut sel = Select::new();

        let (_tx1, rx1) = unbounded::<i32>();
        let (_tx2, rx2) = unbounded::<i32>();

        let receivers: Rc<RefCell<Vec<Receiver<i32>>>> = Rc::new(RefCell::new(Vec::new()));

        // Won't compile error[E0716]
        {
            {
                receivers.borrow_mut().push(rx1);
                let r_rx1 = &(*receivers).borrow()[0];
                println!("&*r_rx1={:p}", r_rx1);
                sel.recv(&*r_rx1);
                println!("after sel.recv(r_rx1)");
            }

            {
                receivers.borrow_mut().push(rx2);
                let r_rx2 = &(*receivers).borrow()[1];
                println!("&*r_rx2={:p}", &*r_rx2);
                sel.recv(r_rx2); // If enabled error[E0716] on line 23
                println!("after sel.recv(r_rx2)");
            }
        }
    }

    #[allow(unused)]
    fn add_rx_mut_rcvrs<'a>(
        sel: &mut Select<'a>,
        mut_rcvrs: &'a mut Vec<Receiver<i32>>,
        rx: Receiver<i32>,
    ) {
        let idx = mut_rcvrs.len();
        mut_rcvrs.push(rx);
        let ref_rx = &mut_rcvrs[idx];
        sel.recv(ref_rx);
    }

}

fn main() {
    immutable_receivers_vec();
    mutable_receivers_vec();
}
