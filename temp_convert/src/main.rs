use std::io;
use std::ops::Deref;

enum Temperature {
    Farenheit(f32),
    Celsius(f32),
}

fn main() {
    println!("Temperature converter!");
    let mut input_temperature = get_temperature();
    let mut input_symbol = get_temperature_symbol();

    match input_symbol {
        'f' => {
            input_temperature = convert_f_to_c(input_temperature);
            input_symbol = 'c';
            },
        'c' => {
            input_temperature = convert_c_to_f(input_temperature);
            input_symbol = 'f';
            },
        _ => unreachable!(),
    }
    
    println!("Your temperature is {}{}", input_temperature, input_symbol);
}

fn get_temperature() -> f32 {
    let message = "What is the temperature you want to convert?";
    let temperature = get_user_input(message);
    temperature.parse::<f32>()
        .expect("please give me a number next time :(")    
}

fn get_temperature_symbol() -> char {
    let message = "Is this a temperature in c or f?";
    let temperature_symbol = get_user_input(message);

    match temperature_symbol.deref() {
        "f" | "F" => 'f',
        "c" | "C" => 'c',
        _ => panic!("Please enter C or F"),
    }
}

fn get_user_input(message: &str) -> String {
    let mut user_input = String::new();
    println!("{}", message);
    io::stdin().read_line(&mut user_input)
        .expect("I failed to read the line, please try again");

    String::from(user_input.trim())
}

fn convert_f_to_c(temperature: f32) -> f32 {
    (temperature - 32.0) * (5.0/9.0)
}

fn convert_c_to_f(temperature: f32) -> f32 {
    (temperature * (9.0 / 5.0)) + 32.0
}