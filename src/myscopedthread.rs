use std::thread;
use std::thread::scope;

struct Person {
    first_name: String,
}

pub fn test_thread_variables() {
    let age = 34;
    let person_01 = Person {
        first_name: "Adarsh".to_string(),
    };

    let print_age = || {
        println!("Child closure: Your age is {age}");
        println!(
            "Child closure: The person name is {}",
            &person_01.first_name
        )
    };

    thread::scope(|scope| {
        scope.spawn(print_age);
    });

    // let _result = thread::spawn(print_age).join();
    //
    println!("Your age is {age}");
    println!("The person name is {}", &person_01.first_name);

    println!("finished printing age")
}
