use std::io;
use std::process::exit;

fn main() {
    let mut input_temperature;
    let mut input_symbol;

    println!("Temperature converter!");
    input_temperature = get_temperature();
    input_symbol = get_temperature_symbol();

    println!("input temp: {} {}", input_temperature, input_symbol);
}

fn get_temperature() -> f32 {
    let message = "What is the temperature you want to convert?".to_string();
    let temperature = get_user_input(message);
    let temperature: f32 = temperature.parse()
        .expect("please give me a number next time :(");

    temperature
}

fn get_temperature_symbol() -> std::string::String {
    let message = "Is this a temperature in c or f?".to_string();
    let temperature_symbol = get_user_input(message);

    if "f" == temperature_symbol {
        temperature_symbol
    } else if "c" == temperature_symbol {
        temperature_symbol
    } else {
        exit(1);
    }
}

fn get_user_input(message: std::string::String) -> std::string::String {
    let mut user_input = String::new();
    
    println!("{}", message);
    io::stdin().read_line(&mut user_input)
        .expect("I failed to read the line, please try again");

    user_input.trim().to_string()
}