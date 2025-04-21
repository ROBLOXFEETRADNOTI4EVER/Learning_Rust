use std::io;
use colored::{self,Colorize};
fn main() {
   // to do make a simple calculator rn
loop {
    

println!("{}","Chose An operation you want to preform".on_truecolor(3 , 15, 235));
println!("{}","\n 1: abstract \n 2: substract \n 3: multiply \n 4: devide \n 5: sqaureroot \n".on_truecolor(3 , 143, 40));
let mut _first_input :String  = String::new();
    io::stdin().read_line(&mut  _first_input).expect("Failed to read msg");
// println!("{}",_first_input);


let  input = _first_input.trim().parse::<i32>().unwrap();
if input == 1 {
    println!("Enter first Number you want to add");
    let mut __first_num_ :String  = String::new();
    io::stdin().read_line(&mut  __first_num_).expect("Failed to read msg");
    let  _f1 = __first_num_.trim().parse::<u32>().unwrap();

    println!("Enter Secound Number you want to add");
    let mut _sec_num :String  = String::new();
    io::stdin().read_line(&mut  _sec_num).expect("Failed to read msg");
    let  _f2 = _sec_num.trim().parse::<u32>().unwrap();
    
    let result:u32  = add(_f1  ,_f2);

    println!("{}",result)
    
} else if input == 2 {
    println!("Enter first Number you want to add");
    let mut __first_num_ :String  = String::new();
    io::stdin().read_line(&mut  __first_num_).expect("Failed to read msg");
    let  _f1: i32 = __first_num_.trim().parse::<i32>().unwrap();

    println!("Enter Secound Number you want to add");
    let mut _sec_num :String  = String::new();
    io::stdin().read_line(&mut  _sec_num).expect("Failed to read msg");
    let  _f2: i32 = _sec_num.trim().parse::<i32>().unwrap();
    
    let result:i32  = substact(_f1  ,_f2);

    println!("{}",result)

} else if input == 3 {
    println!("Enter first Number you want to add");
    let mut __first_num_ :String  = String::new();
    io::stdin().read_line(&mut  __first_num_).expect("Failed to read msg");
    let  _f1: i32 = __first_num_.trim().parse::<i32>().unwrap();

    println!("Enter Secound Number you want to add");
    let mut _sec_num :String  = String::new();
    io::stdin().read_line(&mut  _sec_num).expect("Failed to read msg");
    let  _f2: i32 = _sec_num.trim().parse::<i32>().unwrap();
    
    let result:i32  = multiply_together(_f1  ,_f2);

    println!("{}",result)

} else if input == 4 {
    println!("Enter first Number you want to add");
    let mut __first_num_ :String  = String::new();
    io::stdin().read_line(&mut  __first_num_).expect("Failed to read msg");
    let  _f1: i32 = __first_num_.trim().parse::<i32>().unwrap();

    println!("Enter Secound Number you want to add");
    let mut _sec_num :String  = String::new();
    io::stdin().read_line(&mut  _sec_num).expect("Failed to read msg");
    let  _f2: i32 = _sec_num.trim().parse::<i32>().unwrap();
    
    let result:i32  = devide(_f1  ,_f2);

    println!("{}",result)
    
} else if input == 5 {
    println!("Enter first Number you want to add");
    let mut __first_num_ :String  = String::new();
    io::stdin().read_line(&mut  __first_num_).expect("Failed to read msg");
    let  _f1: i32 = __first_num_.trim().parse::<i32>().unwrap();

    println!("Enter Secound Number you want to add");
    let mut _sec_num :String  = String::new();
    io::stdin().read_line(&mut  _sec_num).expect("Failed to read msg");
    let  _f2: i32 = _sec_num.trim().parse::<i32>().unwrap();
    
    let result:i32  = squareroot(_f1  ,_f2);

    println!("{}",result)
    
}  else {
    println!("You Fucked up");

}
}

//    let add = add(44,44);
//    println!("{}",add);
//    let _substract = substact(50, 15);
//    println!("{}",_substract);
//    let _multiply_together = multiply_together(50, 15);
//    println!("{}",_multiply_together);
//    let _devine = devide(50, 15);
//    println!("{}",_devine);
//    let _squareroot = squareroot(3, 3);
//    println!("{}",_squareroot);


}

fn add(x:u32,y:u32 ) -> u32{
    // println!("DEJBG");
    // println!("{}",x);
    // println!("{}",y);
    // println!("DEJBG");

    x + y
}

fn substact(x:i32,y:i32 ) -> i32{
    x - y
}
fn multiply_together(x:i32,y:i32 ) -> i32{
    x * y
}

fn devide(x:i32,y:i32 ) -> i32{
    x / y
}

fn squareroot(x:i32,y:i32 ) -> i32{
    x * (y*y) 
}
