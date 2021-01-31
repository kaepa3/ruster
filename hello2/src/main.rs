use std::sync::{mpsc, Arc, Mutex};
use std::thread;

fn main() {
    tra();
    jen();
    ke();
    thd();
    mem();
    ch();
    th();
}
fn th() {
    let mut handles = Vec::new();
    let mut data = vec![1; 10];
    let mut snd_channeles = Vec::new();
    let mut rcv_channeles = Vec::new();

    for _ in 0..10 {
        let (snd_tx, snd_rx) = mpsc::channel();
        let (rcv_tx, rcv_rx) = mpsc::channel();

        snd_channeles.push(snd_tx);
        rcv_channeles.push(rcv_rx);

        handles.push(thread::spawn(move || {
            let mut data = snd_rx.recv().unwrap();
            data += 1;
            let _ = rcv_tx.send(data);
        }));
    }
    for x in 0..10 {
        let _ = snd_channeles[x].send(data[x]);
    }
    for x in 0..10 {
        data[x] = rcv_channeles[x].recv().unwrap();
    }
    for handle in handles {
        let _ = handle.join();
    }
    dbg!(data);
}
fn ch() {
    println!("ch");
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let data = rx.recv().unwrap();
        println!("{}", data);
    });
    let _ = tx.send("hell");
    let _ = handle.join();
}
fn mem() {
    let mut handles = Vec::new();
    let mut data = Arc::new(Mutex::new(vec![1; 10]));

    for x in 0..10 {
        let data_ref = data.clone();
        handles.push(thread::spawn(move || {
            let mut data = data_ref.lock().unwrap();
            data[x] += 1;
        }));
    }

    for handle in handles {
        let _ = handle.join();
    }
    dbg!(data);
}
fn thd() {
    let handle = thread::spawn(|| {
        println!("hello wold");
    });
    let r = handle.join();
    println!("{:?}", r);
    let mut handles = Vec::new();

    for x in 0..10 {
        handles.push(thread::spawn(move || {
            println!("hello{}", x);
        }));
    }
}
fn ke() {
    let important_data = "hoge".to_string();
    calc_data(&important_data);
    println!("{}", important_data);
}

fn calc_data(data: &String) {
    println!("{}", data);
}
fn jen() {
    println!("jen");
    fn make_tuple<T, S>(t: T, s: S) -> (T, S) {
        (t, s)
    }
    let t1 = make_tuple(1, 2);
    println!("{:?}", t1);
    let t2 = make_tuple("hello", "world");
    println!("{:?}", t2);
    let t3 = make_tuple("close", 2);
    println!("{:?}", t3);
}
fn tra() {
    trait Tweet {
        fn tweet(&self);
        fn tweet_twice(&self) {
            self.tweet();
            self.tweet();
        }
        fn shout(&self) {
            println!("Uooooooooohh!!!");
        }
    };
    struct Dove;
    struct Duck;
    impl Tweet for Dove {
        fn tweet(&self) {
            println!("Coo!");
        }
    }
    impl Tweet for Duck {
        fn tweet(&self) {
            println!("Quack!");
        }
    }
    let dove = Dove {};
    dove.tweet();
    dove.tweet_twice();
    dove.shout();

    let duck = Duck {};
    duck.tweet();
    duck.tweet_twice();
    duck.shout();
}
