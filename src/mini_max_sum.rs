pub mod mini_max_sum_mod {
    pub struct Value {
        input : [i128; 5],
        output : [i128; 5]
    }
    impl Value {
        pub fn new(input: [i128;5]) -> Value {
            Value {
                input,
                output: [0,0,0,0,0]
            }
        }
    }

    pub fn mini_max_sum(mut arr: Value) -> (i128, i128) {
        let entire_array_sum: i128 = arr.input.iter().sum();
        for (index, val) in arr.input.iter().enumerate() {
            arr.output[index] = entire_array_sum - val
        }

        let max: i128 = arr.output.iter().max().copied().unwrap();
        let min: i128 = arr.output.iter().min().copied().unwrap();
        (min, max)
    }
}