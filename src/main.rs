use std::{env, fs, io};
use std::time::SystemTime;
use hackerrank_rust::activity_notifications::activity_notifications_mod::{Activity, Notification};
use hackerrank_rust::mini_max_sum::mini_max_sum_mod::{mini_max_sum, Value};
use hackerrank_rust::time_conversion::time_conversion_mod::{Meridian, time_conversion_impl};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");


    let sys_time = SystemTime::now();

    match input.trim() {
        "1" => {print!("output {:?} ", mini_max_sum_fn())}
        "2" => {print!("output{:?} ", time_conversion_fn())}
        "3" => {print!("output {:?} ", activity_notifications_fn())}
        _ => {}
    }
    println!("in {:?} ", SystemTime::now().duration_since(sys_time).unwrap());

    fn mini_max_sum_fn() -> (i128, i128) {
        let arr = Value::new([396285104,573261094,759641832,819230764,364801279]);
        mini_max_sum(arr)
    }
    fn time_conversion_fn() -> String {
        let input =  "11:05:45PM";
        time_conversion_impl(Meridian::new(input), input)
    }


    fn activity_notifications_fn() -> i32 {
        let args: Vec<String> = env::args().collect();

        let file_path = &args[1];
        let expenditure_a: Vec<i32> = fs::read_to_string(file_path)
            .unwrap()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();

        let expenditure_b: Vec<i32> = Vec::from([2,3,4,2,3,6,8,4,5]);

        let mut notification = Notification {expenditure: expenditure_b, d: 5, n: 9};
        notification.activity_notifications();

        let mut notification = Notification {expenditure: expenditure_a, d: 10000, n: 200000};
        notification.activity_notifications()
    }

}
