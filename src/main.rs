use std::env;
fn main() {
    // pi to the nth digit
    // I know that pi = radius/2r
    let mut args = env::args();
    let args_test: Vec<String> = env::args().collect();
    println!("{:?}", args_test);

    // Consume default argument, which is the path of the executable itself
    args.next();

    let string_num = match args.next() {
        Some(arg) => arg,
        None => panic!("no number provided"),
    };

    dbg!(&string_num);

    let mut num: u32 = string_num.parse::<u32>().unwrap();
    let res = Pi::pi();
    if num > 1 {
        num +=1;
    }
    let out = &res.to_string()[..(num as usize)];
    println!("{}", out);
}

pub struct Pi;

impl Pi {
    pub fn gregory_leibniz(place: u32) -> f64 {
        // Find number of iterations that we need to do
        // Based off of Alternate series estimation theorem that `R_{n}<=a_{n+1}`
        // place+1 instead of place 'cause something is bugged...
        let n = (10_u32.pow(place + 1) - 3) / 2 + 1;

        // For code for the iterator below
        // let mut sum = 0_f64;
        // for n in 0..=n {
        //     sum += 1_f64 / (2 * n + 1) as f64 * (-1_i32).pow(n) as f64;
        // }
        // sum * 4_f64

        (0..=n).into_iter().fold(0_f64, |acc, n| {
            acc + 1_f64 / (2 * n + 1) as f64 * (-1_i32).pow(n) as f64
        }) * 4_f64
    }
    // Is faster than gregory-leibniz series, but in order to solve the amount of
    // places we'll need to solve a polynomial, which I don't know how to implement
    //
    // Well, we could just use the gregory-leibniz n instead, since nilakantha is guranteed to be better, but the number isn't linear
    // so we still end up with an overflow issue
    pub fn nikalantha(place: u32) -> f64 {
        let n = (10_u32.pow(place) - 3) / 2 + 1;

        // (0..=n).into_iter().fold(0_f64, |acc, n| {
        //     acc + 1_f64 / ((n + 2) as f64 * (n + 3) as f64 * (n + 4) as f64)
        //         * (-1_i32).pow(n) as f64
        // })
        // * 4_f64
        //     + 3_f64

        let mut sum = 0_f64;
        for n in 0..=n {
            sum += 1_f64 / ((2 * n + 2) * (2 * n + 3) * (2 * n + 4)) as f64 * (-1_i32).pow(n) as f64
        }
        sum * 4_f64 + 3_f64
    }
    // In theory this is better than the previous methods
    pub fn machin() -> f64 {
        4_f64 * (4_f64 * (0.2_f64).atan() - (1_f64 / 239_f64).atan())
    }
    
    // The lazy solution and probably best takeaway in this case
    pub fn pi() -> f64 {
        std::f64::consts::PI
    }
}
