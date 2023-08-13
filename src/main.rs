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
        let notification = Notification::new(&Notification {expenditure: &[1,2,3,4,4], d: 4, n: 5});
        notification.activity_notifications()
    }
    //     let mut number_of_notification = 0;
    //     let is_even = (d % 2) == 0;
    //     let median_index = if is_even {
    //          (d/2, (d / 2) + 1)
    //     } else {
    //         (((d + 1) / 2), 0)
    //     };
    //
    //
    //     for i in 0..(n - d) {
    //         let end_index = d + i;
    //         let start_index = i;
    //
    //         let sliced_notification=  slice_notifiaction::<5,9>(notification, start_index, end_index);
    //         // let numbers= ;
    //         let sliced_notification_sorted = sort_arr(sliced_notification);
    //         // println!("{:?}", numbers)
    //
    //         println!("sliced_notification {:?}", sliced_notification);
    //         println!("notification {:?}", sliced_notification_sorted);
    //         // println!("{:?}", end_index);
    //
    //         let median: i32;
    //          if is_even {
    //              median = ((sliced_notification_sorted[median_index.0] - sliced_notification_sorted[median_index.1]) / 2) as i32
    //          } else {
    //              median = sliced_notification_sorted[median_index.0] as i32
    //          }
    //
    //         if median*2 > notification[end_index] as i32 {
    //             number_of_notification += 1;
    //         }
    //     }
    //     number_of_notification
    // }
    //
    // fn slice_notifiaction<const A: usize, const T: usize>(arr: &[usize; T], start: usize, end: usize) -> [usize;A] {
    //     <[usize; A]>::try_from(&arr[start..end]).unwrap()
    // }
    //
    //     // let input = activity_notifications_a(&[2, 3, 4, 2, 3, 6, 8, 4, 5], 5, 9);
    //
    // fn sort_arr<const T: usize>(arr: [usize; T]) -> [usize; T] {
    //     let mut new_array: [usize; T] = arr;
    //     let mut i:usize=0;
    //     let mut j:usize=0;
    //
    //     let mut min:usize=0;
    //     let mut temp:usize=0;
    //
    //     while i <= 4 {
    //         min = i;
    //         j   = i + 1;
    //
    //         while j <= 4
    //         {
    //             if arr[j] < arr[min] {
    //                 min = j;
    //             }
    //             j=j+1;
    //         }
    //         temp = arr[i];
    //         new_array[i] = arr[min];
    //         new_array[min] = temp;
    //
    //         i=i+1;
    //     }
    //     new_array
    // }

}
