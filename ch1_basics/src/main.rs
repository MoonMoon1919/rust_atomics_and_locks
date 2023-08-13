use std::thread;

fn main() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    println!("Hello from main thread");

    t1.join().unwrap();
    t2.join().unwrap();

    pass_closure_to_thread();
    get_a_value_from_thread();
    scoped_threads();
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
