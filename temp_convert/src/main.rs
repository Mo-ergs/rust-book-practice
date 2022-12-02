use std::io;

fn main() {
    loop {
        println!("Please enter a temperature in Fahrenehit");

        let mut temperature = String::new();

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: u32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let temperature = (temperature - 32) * 5 / 9;

        println!("the temperature in celcius is {temperature}");
    }
}
