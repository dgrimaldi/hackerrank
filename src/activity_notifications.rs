pub mod activity_notifications_mod {
    pub trait Activity {
        fn activity_notifications(&self) -> i32;
    }
    pub struct Notification<'a>{
        pub d: i32,
        pub n: i32,
        pub expenditure: &'a[i32]
    }

    impl Notification<'_>  {
        pub fn new(&self) -> &Notification {
            self
        }
    }

    impl Activity for Notification<'_>  {
        fn activity_notifications(&self) -> i32 {
            let mut number_of_notification = 0;
            let is_even = (self.d % 2) == 0;
            let median_index = if is_even {
                ((self.d / 2)as usize, (self.d / 2 ) as usize + 1)
            } else {
                (((self.d + 1) as usize / 2), 0usize)
            };

            println!("self.expenditure {:?}",self.expenditure);


            for i in 0..(self.n -self.d) {
                let mut sliced_notification = Vec::new();
                for val in &self.expenditure[i as usize..(self.d as usize + i as usize)] {
                    sliced_notification.push(val)
                }
                sliced_notification.sort();


                let median: u32 = if is_even {
                    ((*sliced_notification[median_index.0 - 1] + *sliced_notification[median_index.1 - 1]) as u32 / 2).try_into().unwrap()
                } else {
                    (*sliced_notification[median_index.0 - 1] as u32).try_into().unwrap()
                };


                println!("sliced_notification {:?}", sliced_notification);
                println!("median {:?}", median_index);
                println!("self.expenditure {:?}",self.expenditure[self.d as usize + i as usize]);
                println!();

                if median*2 > self.expenditure[self.d as usize + i as usize] as u32 {
                    number_of_notification += 1;
                }

            }
            // const LENGTH: usize = &n.n as usize;
            //
            // let mut number_of_notification = 0;
            // let number_of_control_days = n.n - n.d;
            // for i in 0..number_of_control_days {
            //     let end_index = (n.d + i) as usize;
            //     let start_index = i as usize;
            //
            //     let notification =  n.expenditure as [i32; LENGTH];
            //
            //     let mut numbers = notification[start_index..end_index];
            //     numbers.sort();
            //     let mid = numbers.len() / 2;
            //     let median = numbers[mid];
            //
            //     if median*2 > notification[end_index] {
            //         number_of_notification += 1;
            //     }
            // }
            // number_of_notification
            number_of_notification
        }
    }
}