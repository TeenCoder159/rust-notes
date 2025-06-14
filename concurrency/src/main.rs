use std::thread;
use std::time::Duration;

fn main() {
    // runs both pieces of code at the same time

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
    //.join() blocks the thread currently running to allow the other thread to run
}

/* Output:

hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!

*/

fn _main2() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    // drop(v);
    // v is no longer vaild but the thread might still be running, thus requiring `move`
    /*
    let handle = thread::spawn(|| {
            println!("Here's a vector: {v:?}");
        });
    */

    handle.join().unwrap();
}

/*------Message passing------*/

use std::sync::mpsc;

fn _message_passing() {
    // mpsc = multiple producer
    //mpsc::channel() creates new channel, AKA one way communicating platform
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}

/*------Shared-state concurrency------*/

// use std::rc::Rc;
use std::sync::{Arc, Mutex};
// use std::thread;

fn _mutex() {
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

    println!("Result: {}", *counter.lock().unwrap());
}
