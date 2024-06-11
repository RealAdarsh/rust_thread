mod my_mpsc;
mod myscopedthread;
mod mythread;

fn main() {
    // mythread::test_thread();
    // mythread::spawn_thread();
    // myscopedthread::test_thread_variables();
    my_mpsc::test_channels();
}
