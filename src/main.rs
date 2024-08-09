// Part 1 of chapter 16- Concurrency

/*****************************************/
// Basic concurrency Implementationm
/*****************************************/


 // Handle is of type JoinHandle. It is an owned value that, when we call the join method on it, will wait for its thread to finish.
// use std::thread;
// use std::time::Duration;

// fn main() {
// //    
// //     let handle = thread::spawn(|| { 
// //         for i in 1..10 {
// //             println!("hi number {i} from the spawned thread!");
// //             thread::sleep(Duration::from_millis(1));
// //         }
// //     });

// //     for i in 1..5 {
// //         println!("hi number {i} from the main thread!");
// //         thread::sleep(Duration::from_millis(1));
// //     }

// //     handle.join().unwrap();
    // let v= vec![1,2,3];
    // let handle= thread::spawn(move||{
    //     println!("Here is a vector: {:?}", v);
    // });
//     handle.join().unwrap();
// }



/*****************************************/
//Message Passing
/*****************************************/


//mpsc: Multiple producer single consumer
// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

// fn main() {
    // let (tx, rx) = mpsc::channel(); //mpsc::channel returns a tuple with first elemet as sending end(tx) and second as receiving end(rx)

    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap(); //spwan thred owns tx and now ownership is given to send()
    //     //println!("val is {val}"); // error because tx doesn't have the ownership now
    // });
    // let received = rx.recv().unwrap();
    // println!("Got: {received}");


    // sending multiple values 
    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];

    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // for received in rx {
    //     println!("Got: {received}");
    // }


    //multiple sender(Creating Multiple Producers by Cloning the Transmitter)

//     let (tx, rx) = mpsc::channel();

//     let tx1 = tx.clone();
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread"),
//         ];

//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     thread::spawn(move || {
//         let vals = vec![
//             String::from("more"),
//             String::from("messages"),
//             String::from("for"),
//             String::from("you"),
//         ];

//         for val in vals {
//             tx1.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     for received in rx {
//         println!("Got: {received}");
//     }
// }


/*****************************************/
//Shared-State Concurrency
/*****************************************/

// ########### Purpose: multiple threads to access the same shared data ############

// Mutex = mutual exclusions
// You must attempt to acquire the lock before using the data.
// When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.
// (Ex: Panelist)


//  ###### Single Threaded ##########
// use std::sync::Mutex;

// fn main() {
//     let m = Mutex::new(5);

//     {
//         let mut num = m.lock().unwrap();
//         *num = 6;
//     }

//     println!("m = {:?}",m);
// }


//  ###### Multiple Threaded ##########

// ounter is immutable but we could get a mutable reference to the value inside it; this means Mutex<T> provides interior mutability, as the Cell family does. In the same way we used RefCell<T>
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // let counter = Mutex::new(0);
    // cannot use rf because it isnt safe with Mutex<> ounter is immutable but we could get a mutable reference to the value inside it; this means Mutex<T> provides interior mutability, as the Cell family does. In the same way we used RefCell<T>
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