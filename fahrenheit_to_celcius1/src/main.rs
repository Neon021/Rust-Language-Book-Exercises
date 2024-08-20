use std::io;

fn main() {
    loop {

    println!("Please enter a fahrenheit value");
    let mut fahrenheit = String::new();

    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read line");

    let fahrenheit: f32 = match fahrenheit.trim().parse(){
        Ok(fah) => fah,
        Err(_) => continue,
    };

    println!("fahrenheit as f32: {fahrenheit}");

    let five_over_nine: f32 = (5.0/9.0) as f32;
    let celcius = (fahrenheit - 32.0) * five_over_nine;
    println!("celcius value: {celcius}");
    }
}
