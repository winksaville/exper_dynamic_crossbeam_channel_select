use crossbeam_channel::{unbounded, Select, Receiver, Sender};

use std::{fmt::Debug, any::Any};

#[allow(unused)]
fn simple() {
    println!("simple:+");

    let (s1, r1) = unbounded::<i32>();
    let (s2, r2) = unbounded::<i32>();

    std::thread::scope(|scope| {
        //println!("outer thread:+");

        scope.spawn(|| {
            //println!("t1:+");

            let mut sel = Select::new();

            // Add r1 and get 2 messages
            let oper1 = sel.recv(&r1);
            for li in 1..=2 {
                //println!("t1: TOL1");
                let oper = sel.select();
                match oper.index() {
                    i if i == oper1 => {
                        let oper1_v = oper.recv(&r1).unwrap();
                        //println!("t1: oper1_v={oper1_v}");
                        assert_eq!(li, oper1_v);
                    }
                    _ => unreachable!(),
                }
            }

            // Add r2 and get 2 messages
            let oper2 = sel.recv(&r2);
            for _ in 3..=4 {
                //println!("t1: TOL2");
                let oper = sel.select();
                match oper.index() {
                    i if i == oper1 => {
                        let oper1_v = oper.recv(&r1).unwrap();
                        //println!("t1: oper1_v={oper1_v}");
                        assert_eq!(oper1_v, 3);
                    }
                    i if i == oper2 => {
                        let oper2_v = oper.recv(&r2).unwrap();
                        //println!("t1: oper2_v={oper2_v}");
                        assert_eq!(oper2_v, 4);
                    }
                    _ => unreachable!(),
                }
            }

            // Remove r1 get one message on r2
            sel.remove(oper1);
            for _ in 5..=5 {
                //println!("t1: TOL3");
                let oper = sel.select();
                match oper.index() {
                    i if i == oper2 => {
                        let oper2_v = oper.recv(&r2).unwrap();
                        //println!("t1: oper2_v={oper2_v}");
                        assert_eq!(oper2_v, 5);
                    }
                    _ => unreachable!(),
                }
            }

            //println!("t1:-");
        });

        s1.send(1).unwrap();
        s1.send(2).unwrap();
        s1.send(3).unwrap();
        s2.send(4).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(500));
        s2.send(5).unwrap();

        //println!("outer thread:-");
    });

    println!("simple:-");
}


fn main() {
    println!("main:+");
    simple();
    println!("main:-");
}
