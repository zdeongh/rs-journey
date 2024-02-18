fn main() {

    let mut num_one : i32 = 0;
    let mut num_two : i32 = 1;
    println!("Curr fib_num is: {num_one}");
    println!("Curr fib_num is: {num_two}");

    loop {
        let num_current : i32 = num_one + num_two;
        num_one = num_two;
        num_two = num_current;
        println!("Curr fib_num is: {num_current}");

        //Note this programs panicks with an overflow because we reach i32 overflow
    }
}

