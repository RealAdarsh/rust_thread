use std::thread::spawn;

pub(crate) fn test_thread() {
    let mut x = 0u128;
    for i in 1..500_000_000 {
        x += i;
    }
    println!("Completed the test thread");
}

pub fn spawn_thread() {
    let thread_fn = || {
        let mut x = 0u128;
        for i in 1..500_000_000 {
            x += i;
        }
        println!("{x}");
    };

    println!("Starting new worker thread");
    let handle = spawn(thread_fn);
    let handle2 = spawn(thread_fn);
    println!("Worker thread completed");

    loop {
        test_thread();
        if handle.is_finished() && handle2.is_finished() {
            println!("All the thread is finished");
            break;
        }
    }
}
