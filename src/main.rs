use crossbeam_channel::{unbounded, Select, Receiver, Sender};

use std::{fmt::Debug, any::Any};

pub type BoxMsgAny = Box<dyn Any + Send>;

#[derive(Debug)]
struct MsgSum {
    v: i32,
}

#[derive(Debug)]
struct MsgReqSum;

#[derive(Debug)]
struct MsgRspSum {
    sum: i128,
}


#[derive(Debug)]
struct MsgReqAddWorker {
    worker: Box<dyn Worker>,
    rsp_tx: Sender<BoxMsgAny>
}

#[derive(Debug)]
struct MsgRspAddWorker {
    tx: Sender<BoxMsgAny>,
    rx: Receiver<BoxMsgAny>,
}

#[derive(Debug)]
struct MsgDone;

trait Worker: Send + Debug {
    fn do_work(&mut self, rsp_tx: &Sender<BoxMsgAny>, msg: BoxMsgAny);
}

#[derive(Debug)]
struct Summer {
    sum: i128,
}

impl Worker for Summer {
    fn do_work(&mut self, rsp_tx: &Sender<BoxMsgAny>, msg_any: BoxMsgAny) {
        if let Some(msg) = msg_any.downcast_ref::<MsgSum>() {
            self.sum += msg.v as i128;
        } else if msg_any.downcast_ref::<MsgReqSum>().is_some() {
            rsp_tx.send(Box::new(MsgRspSum { sum: self.sum })).unwrap();
        } else {
            println!("Summer.do_work: unknown msg_any, expected MsgSum");
        }
    }
}

fn worker() {
    println!("worker:+");

    let (ws, wr) = unbounded::<BoxMsgAny>();

    std::thread::scope(|scope| {
        println!("worker outer thread:+");

        scope.spawn(|| {
            println!("worker_t1:+");

            let mut workers: Vec<Box<dyn Worker>> = Vec::new();
            let mut worker_our_receivers: Vec<Receiver<BoxMsgAny>> = Vec::new();
            let mut worker_our_senders: Vec<Sender<BoxMsgAny>> = Vec::new();

            let mut sel = Select::new();
            let worker_t1 = sel.recv(&wr);

            loop {
                // Get Worker's
                let oper = sel.select();
                match oper.index() {
                    i if i == worker_t1 => {
                        let msg_any = oper.recv(&wr).unwrap();
                        if let Some(msg) = msg_any.downcast_ref::<MsgReqAddWorker>() {
                            println!("worker_t1: msg={msg:?}");
                            let msg = msg_any.downcast::<MsgReqAddWorker>().unwrap();
                            let (our_tx, their_rx) = unbounded::<BoxMsgAny>();
                            let (their_tx, our_rx) = unbounded::<BoxMsgAny>();
                            let msg_rsp = Box::new(MsgRspAddWorker { tx: their_tx, rx: their_rx });
                            msg.rsp_tx.send(msg_rsp).unwrap();
                            workers.push(msg.worker);
                            worker_our_senders.push(our_tx);
                            worker_our_receivers.push(our_rx);
                            //sel.recv(worker_our_receivers.last().unwrap());
                        } else if let Some(msg) = msg_any.downcast_ref::<MsgDone>() {
                            println!("worker_t1: msg={msg:?}");
                            break;
                        } else {
                            println!("worker_t1: ignoring unreconized msg, expecting MsgAddWorker or MsgDone");
                        }
                    }
                    i if (i - 1) < workers.len() && (i - 1) < worker_our_receivers.len() => {
                        assert!(i > 0);
                        let worker_idx = i - 1;
                        let msg_any = oper.recv(&worker_our_receivers[worker_idx]).unwrap();
                        let worker = &mut workers[worker_idx];
                        println!("worker_t1: call worker[{i}].do_work");
                        worker.do_work(&worker_our_senders[worker_idx], msg_any);
                        println!("worker_t1: retf worker[{i}].do_work");
                    }
                    _ => {
                        unreachable!();
                    }
                }
            }

            println!("worker_t1:-");
        });

        let (rsp_tx, rsp_rx) = unbounded::<BoxMsgAny>();

        let s = Box::new(Summer { sum: 0 });
        let msg_add_worker = Box::new(MsgReqAddWorker { worker: s, rsp_tx });
        ws.send(msg_add_worker).unwrap();
        let msg_any = rsp_rx.recv().unwrap();
        let msg = msg_any.downcast::<MsgRspAddWorker>().unwrap();

        let summer_tx = msg.tx.clone();
        let summer_rx = msg.rx.clone();

        let msg_sum = Box::new(MsgSum { v: 2 });
        summer_tx.send(msg_sum).unwrap();

        let msg_sum = Box::new(MsgReqSum);
        summer_tx.send(msg_sum).unwrap();

        println!("worker outer thread: waiting msg_rsp_sum");
        let msg_any = summer_rx.recv().unwrap();
        let msg_rsp_sum= msg_any.downcast::<MsgRspSum>().unwrap();
        println!("worker outer thread: received msg_rsp_sum={msg_rsp_sum:?}");
        assert_eq!(msg_rsp_sum.sum, 2);

        let msg_done = Box::new(MsgDone);
        ws.send(msg_done).unwrap();
        println!("worker outer thread:-");
    });

    println!("worker:-");
}

fn main() {
    println!("main:+");
    worker();
    println!("main:-");
}
