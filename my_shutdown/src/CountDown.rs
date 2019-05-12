use std::sync::mpsc;
use std::thread;
use std::time::Duration;


pub fn start(secs: u32, cb: impl Fn(u32) -> ()) {
    if secs < 1 {
        panic!("secs error! (secs > 1) is good.");
    }

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in (1..=secs).rev() {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recv in rx {
        cb(recv);
    }
}
