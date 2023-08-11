
pub mod time_conversion_mod {

    pub enum Meridian {
        PM,
        AM
    }

    impl Meridian {
        pub fn from(input: &str) -> Meridian{
            if string_to_bytes(input).iter().any(|val| *val == (65)) {
                return Meridian::AM
            }
            Meridian::PM
        }
    }

    pub fn time_conversion_impl(meridian: Meridian, input: &str) -> String {
        let mut val: Vec<u8> = Vec::from(string_to_bytes(input));
        match meridian {
            Meridian::AM => {
                if val[0] == 49 && val[1] == 50 {
                    val[0] = 48u8;
                    val[1] = 48u8;
                }
            },
            Meridian::PM => {
                if val[0] == 49 && val[1] == 50 {
                } else if val[1] == 56 {
                    val[0] = 50u8;
                    val[1] = 48u8;
                } else if val[1] == 57 {
                    val[0] = 50u8;
                    val[1] = 49u8;
                }
                else {
                    val[0] += 1u8;
                    val[1] += 2u8
                }
            }
        }
        format_output(String::from_utf8(val).unwrap())
    }

    fn string_to_bytes(input: &str) -> &[u8] {
        input.as_bytes()
    }

    fn format_output(input: String) -> String {
        String::from(&input[..input.len()-2])
    }

}