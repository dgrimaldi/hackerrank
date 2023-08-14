use std::io;
use std::os::unix::raw::nlink_t;
use hackerrank_rust::activity_notifications::activity_notifications_mod::{Activity, Notification};
use hackerrank_rust::mini_max_sum::mini_max_sum_mod::{mini_max_sum, Value};
use hackerrank_rust::time_conversion::time_conversion_mod::{Meridian, time_conversion_impl};

fn main() {

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim() {
        "1" => {println!("{:?}", mini_max_sum_fn())}
        "2" => {println!("{:?}", time_conversion_fn())}
        "3" => {println!("output {:?}", activity_notifications_fn())}
        _ => {}
    }
    fn mini_max_sum_fn() -> (i128, i128) {
        let arr = Value::new([396285104,573261094,759641832,819230764,364801279]);
        mini_max_sum(arr)
    }
    fn time_conversion_fn() -> String {
        let input =  "11:05:45PM";
        time_conversion_impl(Meridian::new(input), input)
    }


    fn activity_notifications_fn() -> i32 {
        let notification = Notification::new(&Notification {expenditure: &[2,3,4,2,3,6,8,4,5], d: 5, n: 9});
        notification.activity_notifications()
    }

}
