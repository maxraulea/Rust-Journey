use std::io;

fn fibonacci_sequence(num: u64)-> u64{
    let mut memorize: Vec<u64>= Vec::new();
    memorize.push(1);
    memorize.push(1);

    for i in 2..num{
        memorize.push(memorize[i as usize -1] + memorize[i as usize - 2]);
    }
    let outcome = match memorize.pop() {
        Some(top) => top,
        None => {
            println!("something went wrong");
            return 0;
        }
    };
    return outcome;
}
fn main(){

    println!("which fibonacci number do you want to get?");

    let mut input: String = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("something went wrong");

    let input: u64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("wrong input");
            return;
        },};

        if input == 0{
            println!("please fill in a number larger than 0");
            return;
        }

    let output: String = fibonacci_sequence(input).to_string();
    if output == "0" {
        return;
    }
    println!("that number is {output}")
}