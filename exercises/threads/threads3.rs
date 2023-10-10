// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.

use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Arc<Queue>, tx: Arc<mpsc::Sender<u32>>) -> Result<(), mpsc::SendError<u32>> {
    // let qc = Arc::new(q);
    // let qc1 = Arc::clone(&qc);
    // let qc2 = Arc::clone(&qc);

    let q1 = Arc::clone(&q);
    let q2 = Arc::clone(&q);
    let tx1 = Arc::clone(&tx);
    let tx2 = Arc::clone(&tx);

    let t1 = thread::spawn(move || {
        for val in &q1.first_half {
            println!("sending {:?}", val);
            tx1.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let t2 = thread::spawn(move || {
        for val in &q2.second_half {
            println!("sending {:?}", val);
            tx2.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();
    Ok(())
}

fn main() {
    let (tx, rx) = mpsc::channel();
    // let queue = Queue::new();
    let queue = Arc::new(Queue::new());
    let queue_length = queue.length;

    let tx = Arc::new(tx); // Convert tx to Arc<T> for thread sharing
    if let Err(err) = send_tx(Arc::clone(&queue), Arc::clone(&tx)) {
        eprintln!("Error sending data: {:?}", err);
    }

    let mut total_received: u32 = 0;
    drop(tx);
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}
