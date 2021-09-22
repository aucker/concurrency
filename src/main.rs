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

/*use std::thread;
use std::sync::mpsc;
use std::time::Duration;*/
//creating Multiple Producers by Cloning the Transmitter
//through clone
/*fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("fr"),
            String::from("da"),
            String::from("fs"),
            String::from("sa"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("sa"),
            String::from("df"),
            String::from("fg"),
            String::from("cv"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(2000));
        }
    });

    for received in rx {
        println!("got {}", received);
    }
}*/

//mutex: mutual exclusion: a mutex allows only one thread to access some data at any given time
/*use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}*/

//use std::rc::Rc;    //the trait `Send` is not implemented for `Rc<Mutex<i32>>`
use std::sync::{Mutex, Arc};
use std::thread;

/*fn main() {
    let counter = Mutex::new(0);    //new(): association function
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}*/
fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("result is {}", *counter.lock().unwrap());
}