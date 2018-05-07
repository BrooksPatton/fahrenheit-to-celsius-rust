use std::io::{self, Write};
use std::fmt::{self, Display};

// I think it makes sense to derive Copy for this,
// since it's only a f32 and the tag and thus cheap to copy
#[derive(Clone, Copy, Debug)]
enum Temperature {
    Fahrenheit(f32),
    Celsius(f32),
}

impl Temperature {
    fn convert_in_place(&mut self) {
        use Temperature::*; // Bring the variants of Temperature into the local scope to simplify matching
        match *self {
            // Here we change `self` in place and assign it the new Temperature value
            Fahrenheit(t) => *self = Celsius((t - 32.0) / 1.8),
            Celsius(t) => *self = Fahrenheit(t * 1.8 + 32.0),
        }
    }

    fn convert_to_new(&self) -> Self { // `Self` is a shorthand for the type we are currently writing the impl for
        use Temperature::*;
        match *self {
            Fahrenheit(t) => Celsius((t - 32.0) / 1.8),
            Celsius(t) => Fahrenheit(t * 1.8 + 32.0),
        }
    }

    fn is_celsius(&self) -> bool {
        if let Temperature::Celsius(_) = *self {
            true
        } else {
            false
        }
    }

    fn is_fahrenheit(&self) -> bool {
        !self.is_celsius()
    }
}

// We'll want to implement the Display trait for our enum,
// so that we can print it using `{}` and get the temperature nicely formatted
impl Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Temperature::*;
        match *self {
            // The `write!()` macro works similarly to the `print!()` macro, except that
            // it takes a target to write to. In our case this is a mutable reference to a std::fmt::Formatter
            Fahrenheit(t) => write!(f, "{}°F", t),
            Celsius(t) => write!(f, "{}°C", t),
        }
    }
}

fn main() {
    println!("Temperature converter!");
    let temperature = get_temperature();
    println!("Your input temperature is {}", temperature);
    println!("After converting your temperature is {}", temperature.convert_to_new());
}

fn get_temperature() -> Temperature {
    let mut buf = String::new(); // We'll reuse this buffer for all our input
    get_user_input("What is the temperature you want to convert?\n> ", &mut buf);

    let temp = buf.trim().parse()
        .expect("please give me a number next time :(");

    get_user_input("Is this a temperature in c or f?\n> ", &mut buf);
    match buf.trim() {
        "f" | "F" => Temperature::Fahrenheit(temp),
        "c" | "C" => Temperature::Celsius(temp),
        _ => panic!("Please enter C or F"),
    }
}

// Providing a buffer for `read_line()` to read into let's us reuse the buffer and save some allocations
fn get_user_input(message: &str, buffer: &mut String) {
    buffer.clear(); // Remove any old content in the buffer
    
    // We'll use `print!()` instead of `println!()` so we can have a message that doesn't end with a newline
    // Because `print!()` doesn't automatically flush Stdout we'll need to do this manually. Otherwise the message
    // might not get printed before we read from Stdin
    print!("{}", message);
    io::stdout().flush().expect("could not flush Stdout");
    
    io::stdin().read_line(buffer)
        .expect("I failed to read the line, please try again");
}