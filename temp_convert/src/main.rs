use std::io;

fn main() {
    let mut input_temperature;
    let mut input_symbol;

    println!("Temperature converter!");
    input_temperature = get_temperature();
    input_symbol = get_temperature_symbol();

    if "f" == input_symbol || "F" == input_symbol {
        input_temperature = convert_f_to_c(input_temperature);
        input_symbol = "c".to_string();
    } else {
        input_temperature = convert_c_to_f(input_temperature);
        input_symbol = "f".to_string();
    }
    
    println!("Your temperature is {}{}", input_temperature, input_symbol);
}

fn get_temperature() -> f32 {
    let message = "What is the temperature you want to convert?".to_string();
    let temperature = get_user_input(message);
    let temperature = temperature.parse::<f32>()
        .expect("please give me a number next time :(");

    temperature
}

fn get_temperature_symbol() -> std::string::String {
    let message = "Is this a temperature in c or f?".to_string();
    let temperature_symbol = get_user_input(message);

    if "f" == temperature_symbol || "F" == temperature_symbol || "c" == temperature_symbol || "C" == temperature_symbol {
        temperature_symbol
    } else {
        panic!("Please enter C or F");
    }
}

fn get_user_input(message: std::string::String) -> std::string::String {
    let mut user_input = String::new();
    
    println!("{}", message);
    io::stdin().read_line(&mut user_input)
        .expect("I failed to read the line, please try again");

    user_input.trim().to_string()
}

fn convert_f_to_c(temperature: f32) -> f32 {
    (temperature - 32.0) * (5.0/9.0)
}

fn convert_c_to_f(temperature: f32) -> f32 {
    (temperature * (9.0 / 5.0)) + 32.0
}