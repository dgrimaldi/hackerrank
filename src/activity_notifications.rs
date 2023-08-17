pub mod activity_notifications_mod {
    use std::sync::{Arc};
    use std::sync::atomic::{AtomicI32, Ordering};
    use std::thread;
    use std::time::{Duration};

    // Implementing a Trait on a Type
    pub trait Activity  {
        fn activity_notifications(&mut self) -> i32;
    }
    pub struct Notification{
        pub d: i32,
        pub n: i32,
        pub expenditure: Vec<i32>
    }

    #[derive(Debug)]
    enum Median {
        I32(i32),
        F32(f32)
    }


    impl Activity for Notification {
        fn activity_notifications(&mut self) -> i32 {
            let d = self.d;
            let n = self.n;
            let expenditure = &mut self.expenditure;

            let sort = |sliced_notification: &mut [i32]| sliced_notification.sort();

            let number_of_notification = Arc::new(AtomicI32::new(0));

            let is_even = (d % 2) == 0;
            let median_index = if is_even {
                ((d / 2) as usize, (d / 2) as usize + 1)
            } else {
                (((d + 1) as usize / 2), 0usize)
            };

            let start = d as usize;
            let end = n as usize;


            for _ in 0..(end - start) {

                let mut exp = expenditure.clone();

                let number_of_notification = number_of_notification.clone();

                thread::spawn(move || {
                    let sliced_notification = &mut exp[0..start];
                    sort(sliced_notification);

                    let median = if is_even {
                        Median::F32((sliced_notification[median_index.0 - 1] + sliced_notification[median_index.1 - 1]) as f32 / 2.0)
                    } else {
                        Median::I32(sliced_notification[median_index.0 - 1])
                    };

                    match median {
                        Median::I32(mediam_value) => if mediam_value * 2 <= exp[start] {  // let mut num = number_of_notification.lock().unwrap(); // *num += 1
                            number_of_notification.fetch_add(1, Ordering::SeqCst);
                        }
                        Median::F32(mediam_value) => if mediam_value * 2.0 <= exp[start] as f32 {  // let mut num = number_of_notification.lock().unwrap(); // *num += 1;
                            number_of_notification.fetch_add(1, Ordering::SeqCst);
                        }
                    }
                });
                thread::sleep(Duration::from_nanos(1));
                expenditure.remove(0);
            }



            number_of_notification.load(Ordering::SeqCst)
        }
    }
}