use crossbeam_channel::{bounded, unbounded, Receiver, Select, Sender};

#[allow(unused)]
fn simple() {
    println!("simple:+");

    std::thread::scope(|scope| {
        println!("outer thread:+");

        let (cmd_tx, cmd_rx) = unbounded::<i32>();
        let (rsp_tx, rsp_rx) = unbounded::<Sender<i32>>();
        let (done_tx, done_rx) = bounded::<()>(0);

        let mut receiver: Vec<Receiver<i32>> = Vec::new();

        scope.spawn(move || {
            println!("t1:+");

            let mut sel = Select::new();

            // From cmd_rx get two i32's
            let oper1 = sel.recv(&cmd_rx);
            for li in 1..=2 {
                println!("t1: TOL1");
                let oper = sel.select();
                match oper.index() {
                    i if i == oper1 => {
                        let oper1_v = oper.recv(&cmd_rx).unwrap();
                        println!("t1: oper1_v={oper1_v}");
                        assert_eq!(li, oper1_v);
                    }
                    _ => unreachable!(),
                }
            }

            let (tx, rx) = unbounded::<i32>();
            rsp_tx.send(tx);
            receiver.push(rx);
            let oper2 = sel.recv(&receiver[0]);

            // Add r2 and get 2 messages
            for _ in 3..=4 {
                println!("t1: TOL2");
                let oper = sel.select();
                match oper.index() {
                    i if i == oper1 => {
                        let oper1_v = oper.recv(&cmd_rx).unwrap();
                        println!("t1: oper1_v={oper1_v}");
                        assert_eq!(oper1_v, 3);
                    }
                    i if i == oper2 => {
                        let oper2_v = oper.recv(&receiver[0]).unwrap();
                        println!("t1: oper2_v={oper2_v}");
                        assert_eq!(oper2_v, 4);
                    }
                    _ => unreachable!(),
                }
            }

            done_tx.send(());

            println!("t1:-");
        });

        cmd_tx.send(1).unwrap();
        cmd_tx.send(2).unwrap();
        cmd_tx.send(3).unwrap();

        println!("outer thread: waiting for rev other_cmd_tx");
        let other_cmd_tx = rsp_rx.recv().unwrap();
        println!("outer thread:    received rev other_cmd_tx");
        other_cmd_tx.send(4).unwrap();
        //other_cmd_tx.send(5).unwrap();

        done_rx.recv().unwrap();
        println!("outer thread:-");
    });

    println!("simple:-");
}

fn main() {
    println!("main:+");
    simple();
    println!("main:-");
}
