use std::sync::mpsc;
use std::thread;
use std::time::Duration;
// fn main() {
//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move || {
//         let val = String::from("HI");
//         tx.send(val).unwrap();
//         // println!("val is {}", val);
//     });

//     let received = rx.recv().unwrap();
//     println!("Got: {received}");
// }

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move || {
//         let vals: Vec<String> = vec![
//             String::from("Ola"),
//             String::from("Hello"),
//             String::from("Hola")
//         ];

//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
        
//     });

//     for received in rx {
//         println!("Got: {received}");
//     }
// }

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals: Vec<String> = vec![
            String::from("Ola1"),
            String::from("Hello2"),
            String::from("Hola3")
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        
    });

    thread::spawn(move || {
        let vals: Vec<String> = vec![
            String::from("Ola"),
            String::from("Hello"),
            String::from("Hola")
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