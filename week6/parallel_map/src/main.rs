use crossbeam_channel;
use std::{thread, time};

fn parallel_map<T, U, F>(mut input_vec: Vec<T>, num_threads: usize, f: F) -> Vec<U>
where
    F: FnOnce(T) -> U + Send + Copy + 'static,
    T: Send + 'static,
    U: Send + 'static + Default,
{
    let mut output_vec: Vec<U> = Vec::with_capacity(input_vec.len());
    for _ in 0..input_vec.len() {
        output_vec.push(Default::default());
    }
    let (s, r): (
        crossbeam_channel::Sender<(usize, T)>,
        crossbeam_channel::Receiver<(usize, T)>,
    ) = crossbeam_channel::unbounded();
    let (os, or) = crossbeam_channel::unbounded();
    for _ in 0..num_threads {
        let r_ = r.clone();
        let os_ = os.clone();
        thread::spawn(move || {
            loop {
                match r_.recv() {
                    Ok(tuple) => {
                        let tuple_ = (tuple.0, f(tuple.1));
                        os_.send(tuple_).unwrap();
                    }
                    Err(_) => break,
                }
            }
            drop(os_);
        });
    }
    // input_vec.into_iter().enumerate().for_each(|tuple| {
    //     s.send(tuple).unwrap();
    // });
    while input_vec.len() != 0 {
        let tuple = (input_vec.len() - 1, input_vec.pop().unwrap());
        s.send(tuple).unwrap();
    }
    drop(s);
    drop(os);
    loop {
        match or.recv() {
            Ok(tuple) => {
                output_vec[tuple.0] = tuple.1;
            }
            Err(_) => break,
        }
    }
    output_vec
}

fn main() {
    let v = vec![6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 12, 18, 11, 5, 20];
    let squares = parallel_map(v, 10, |num| {
        println!("{} squared is {}", num, num * num);
        thread::sleep(time::Duration::from_millis(500));
        num * num
    });
    println!("squares: {:?}", squares);
}
