use std::{
    thread,
    sync::{Arc, Mutex, Condvar},
    marker::PhantomData,
    cell::Cell,
    collections::VecDeque,
    time::Duration,
};

fn main() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    println!("Hello from main thread");

    t1.join().unwrap();
    t2.join().unwrap();

    pass_closure_to_thread();
    get_a_value_from_thread();
    scoped_threads();
    using_arc();
    mutex_in_practice();
    thread_parking();
}

fn f() {
    println!("Hello from another thread");

    let id = thread::current().id();
    println!("This is my thread id {id:?}")
}

fn pass_closure_to_thread() {
    let numbers = vec![1, 2, 3];

    // Pass a closure into a thread
    thread::spawn(move || {
        for n in &numbers {
            println!("{n}");
        }
    }).join().unwrap();
}

fn get_a_value_from_thread() {
    // Get a value from a thread
    let numbers = Vec::from_iter(0..=1000);

    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum: i32 = numbers.iter().sum();
        sum / len as i32
    });

    let average: i32 = t.join().unwrap();

    println!("Average of numbers is {average}")
}

fn scoped_threads() {
    let numbers = vec![1, 2, 3];

    thread::scope(|s| {
        s.spawn(|| {
            println!("Length: {}", numbers.len());
        });
        s.spawn(|| {
            for n in &numbers {
                println!("{n}")
            }
        });
    })
}

// Can't use Rc with threads, must use Arc
fn using_arc() {
    let a = Arc::new([1, 2, 3]);

    thread::spawn({
        // using 'let' inside of this scope allows us to keep
        // the code clean and not litter it with clone statements
        // and variable definitions
        let a = a.clone();
        move || {
            println!("Using arc {:?}", a);
            dbg!(a)
        }
    }).join().unwrap();
}

#[allow(dead_code)]
struct X {
    handle: i32,
    // X is not Sync anymore, because PhantomData is not sync
    _not_sync: PhantomData<Cell<()>>,
}

fn mutex_in_practice() {
    let n = Mutex::new(0);

    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }
            });
        }
    });

    assert_eq!(n.into_inner().unwrap(), 1000);
}

fn thread_parking() {
    let queue = Mutex::new(VecDeque::new());
    let not_empty = Condvar::new();

    thread::scope(|s| {
        s.spawn(|| {
            loop {
                let mut q = queue.lock().unwrap();
                let item = loop {
                    if let Some(item) = q.pop_front() {
                        break item;
                    } else {
                        q = not_empty.wait(q).unwrap();
                    }
                };
                drop(q);
                dbg!(item);
            }
        });

        // Producing thread
        for i in 0..10 {
            queue.lock().unwrap().push_back(i);
            not_empty.notify_one();
            thread::sleep(Duration::from_secs(1));
        }
    })
}
