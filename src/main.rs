/*use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
*/

/*use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    //thread create concurrency
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        //println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got {}", received);
}*/

use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();


    thread::spawn(move || {
        let vals = vec![
            String::from("fr"),
            String::from("da"),
            String::from("fs"),
            String::from("sa"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("got {}", received);
    }
}