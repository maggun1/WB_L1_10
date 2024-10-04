use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let n: usize = 10;
    let numbers: Vec<usize> = (1..=n).collect();

    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    let mut handles = vec![];

    handles.push(thread::spawn(move || {
        loop {
            match rx1.recv() {
                Ok(num) => {
                    let square = num * num;
                    tx2.send(square).unwrap();
                }
                Err(_) => {
                    break;
                }
            }
        }
    }));

    handles.push(thread::spawn(move || {
        loop {
            match rx2.recv() {
                Ok(square) => {
                    println!("Получено число: {}", square);
                }
                Err(_) => {
                    break;
                }
            }
        }
    }));

    for num in numbers {
        tx1.send(num).unwrap();
        thread::sleep(Duration::from_secs(1));
    }

    drop(tx1);

    for handle in handles {
        handle.join().unwrap();
    }
}
