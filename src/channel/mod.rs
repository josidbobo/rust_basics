#![allow(unused)]
use std::env;
use std::io::Write;
use std::sync::mpsc::channel;
use std::{io, thread};

fn main() {
    let (tx, rx) = channel();

    let sender = thread::spawn(move || {
        let args : Vec<String> = env::args().collect();
        
        for i in args.iter(){
        tx.send(i.to_owned())
            .expect("Unable to send on channel");}
    });

    let receiver = thread::spawn(move || {
        let value = rx.recv().expect("Unable to receive from channel");
        println!("{value}");
    });

    sender.join().expect("The sender thread has panicked");
    receiver.join().expect("The receiver thread has panicked");
}
