use std::sync::mpsc;
use std::thread;
use std::time::Duration;


pub fn start<F: 'static>(secs: u32, cb: F)
    where F: Fn(u32) -> () + Send {
    if secs < 1 {
        panic!("secs error! (secs > 1) is good.");
    }

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in (0..=secs).rev() {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move ||{
        for recv in rx {
            cb(recv);
        }
    });
}
