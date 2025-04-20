use std::io;

use colored::{self, Colorize};
use random_str as random;

fn main() {
// my to do is a coinflip so it says heads or tails

let chances: i32 = 5;
println!("Your chances {}",chances);
let _random_bool: bool = random::get_bool(); // generates me a random boolean

println!("Game Started");
println!("Random boolean: {}", _random_bool);  // debug  

// tails is true
// heads are false

println!("Enter Heads Or Tails");

let mut user_input = String::new();

    io::stdin().read_line(&mut  user_input).expect("Failed to read the message");
println!("{}",user_input);


if user_input.trim() == "Tails" || user_input.trim() == "Heads"{ // no idea but works with trim  


if  _random_bool == true && user_input != "Tails" { // as i see it generates it always false at first so i have to change it to true to make it work since nromaly nothing there and its false
    println!("{}","You lost".bright_magenta());
} else{
    println!("{}","You won".bright_green());
}
}

else{
  println!("{}","You cana't continue ".bright_red());
  return;
}
 



}
