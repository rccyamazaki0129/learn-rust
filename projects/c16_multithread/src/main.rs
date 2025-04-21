use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
            String::from("!"),
        ];
        
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });


    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("message"),
            String::from("for"),
            String::from("you"),
            String::from("..."),
        ];
        
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    let mut counter = 0;
    loop {
        let receive = rx.try_recv();
        match receive {
            Ok(msg) => {
                println!("Got: {}", msg);
                counter += 1;
                
            }
            Err(_) => {
                println!("waiting...");
                thread::sleep(Duration::from_millis(100));
            }
        };
        if (10 <= counter) {
            break;
        }
    }
}

fn no_longer_main() {
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

    {

        let v = vec![
            1, 2, 3
        ];

        let handle = thread::spawn(move || {
            println!("here's a vector: {:?}", v);
        });

        handle.join().unwrap();
    }
}
