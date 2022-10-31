/*
 * @Author: jimmy.zhao
 * @Date: 2022-10-31 12:54:31
 * @LastEditTime: 2022-10-31 13:15:15
 * @LastEditors: jimmy.zhao
 * @Description: 
 * 
 * 
 */

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("This Secret Number is {secret_number}");

    loop {
        println!("Please input your guess number!");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; 

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
    
}
