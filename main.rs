use std::io;
use rand::Rng; // rng stands for Random number generators
use std::cmp::Ordering; 

fn main() {
    println!("Guessing Game!");

    let s_no = rand::thread_rng().gen_range(0..=100);
    //println!("The secret no. is: {s_no}");

    loop{
        println!("Enter your guess.");
        let mut guess =String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read");

        let guess:u32=match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("you guessed :{guess}");

        match guess.cmp(&s_no){
            Ordering::Less => println!("Greater than  !!"),
            Ordering::Greater => println!("less than !!"),
            Ordering::Equal => {
                println!("You win !!");
                break;
            }
        }
    }

}
