use std::io;

fn convert(num: f64)-> f64{
    return num * 1.8 + 32.0
}

fn main(){
    println!("please enter a °C value");

    let mut celcius: String = String::new();

    io::stdin()
    .read_line(&mut celcius)
    .expect("failed to read line");

    let celcius_num: f64 = match celcius.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("wrong input");
            return},
    };

    let fahrenheit: String = convert(celcius_num).to_string(); 
    print!(" so, {celcius} °C equals {fahrenheit} °F")

    
}