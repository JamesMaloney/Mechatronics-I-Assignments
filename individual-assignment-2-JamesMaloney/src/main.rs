use rand::distributions::{Distribution, Uniform};

fn main(){
    let range = Uniform::new(0, 4);
    let mut rng = rand::thread_rng();
    let error_val = range.sample(&mut rng);
    enum ErrorVal {
        Zero = 0,
        One = 1,
        Two = 2,
        Three = 3,
    }
    impl ErrorVal {
        fn from_u32(value: u32) -> ErrorVal {
            match value {
                1 => ErrorVal::One,
                2 => ErrorVal::Two,
                3 => ErrorVal::Three,
                _ => ErrorVal::Zero,
            }
        }
    }

    let error_enum = ErrorVal::from_u32(error_val);

    match error_enum {
        ErrorVal::Zero => {
            println!("UNEXPECTED VALUE");
            panic!(error_val);
        }
        ErrorVal::One => println!("User got a one"),
        ErrorVal::Two => println!("user got a two"),
        ErrorVal::Three => println!("user got a three"),
    }
}