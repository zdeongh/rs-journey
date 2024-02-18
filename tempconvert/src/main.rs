use std::io;

fn main() {

    println!("This program converts a Temperature (Fahrenheit) into a Temperature (Celcius");
    println!("Please provide the Temperature in Fahrenheit you want to be converted");

    let mut input_temp_f = String::new();

     io::stdin()
        .read_line(&mut input_temp_f)
        .expect("Failed to read line");

    let _input_temp_f: f32 = input_temp_f
        .trim()
        .parse()
        .expect("Temperature is not a valid float");

    let _output_temp_c = convert_temp_to_c(_input_temp_f);

    println!("Your provided Temperature in Fahrenheit converts to {_output_temp_c} in Celcius");

}

fn convert_temp_to_c(input_f : f32) -> f32
{
    (input_f - 32.0) * (5.0/9.0)
}
