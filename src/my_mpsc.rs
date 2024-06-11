use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

pub fn test_channels() {
    let (transmitter, receiver): (Sender<u8>, Receiver<u8>) = mpsc::channel();

    let send_result = transmitter.send(100);
    println!("Send Status: {}", send_result.is_ok());
    let send_result = transmitter.send(105);

    let mut failed_count = 0u8;

    let processor_code = move || {
        println!("Starting Processor thread: ");
        loop {
            println!("Attempting to receive message from channel");
            let receive_result = receiver.recv_timeout(Duration::from_millis(800));

            if receive_result.is_ok() {
                println!("Received message: {}", receive_result.unwrap());
            }
            else{
                failed_count+=1;
            }
            if failed_count>10{
                println!("Aborting processor thread.. no work available");
                break;
            }
        }
    };

 for x in 1..=6{
  let send_result = transmitter.send(x);
  println!("Send status: {}", send_result.is_ok());
     std::thread::sleep(Duration::from_millis(1400));
 }

 std::thread::spawn(processor_code).join();
}
