extern crate zmq;

use std::thread;
use std::time::Duration;

fn main() {
    let ctx = zmq::Context::new();

    let mut socket = ctx.socket(zmq::REP).unwrap();
    assert!(socket.bind("tcp://*:12346").is_ok(), "Unable to open port 12346");

    loop {
        let msg = socket.recv_msg(0).unwrap();
        println!("Received {}", msg.as_str().unwrap());
        thread::sleep(Duration::from_millis(1000));
        socket.send("hello from rust".to_string().as_bytes(), 0).unwrap();
    }
}
