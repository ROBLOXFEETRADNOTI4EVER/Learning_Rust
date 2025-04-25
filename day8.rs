


fn main() {
 let x = 5;
 let _y = x;

// --------------------------

 // BOOROW CHECKER ERROR EXAMPLE

//  let s1 = String::from("Hi");
//  let _s2 = s1;

//  println!("{}",s1); // BOROOW CHACKER SAID FUCK YOU NOP NO NON NO NO  NO BOROOW CHECKER NO NO NO NO 

// --------------------------

// let s1 = String::from("Hi");
// let _s2 = s1.clone();

// println!("{}",s1); // good but double the memory aswell


// Owner ship in rust
// ------------------------------------------------------------------------------------------------------------------------------------------------------------
// let s = String::from("Hello");
//     // takes_ownership(s); // error one bad 
//     takes_ownership(s.clone()); // good one since i cloned
//     println!("{}",s);

//     let x = 5;
//     makes_copy(x);
//     println!("{}",x);
    

//     fn takes_ownership(some_string: String){
//         println!("{}",some_string);
//     }
//     fn makes_copy(some_string: i32){ // integers get copied  and no eror
//         println!("{}",some_string);
//     }


// -----

// let s1 = gives_ownership(); // we have s1 and the gives ownership return s1 value 
// let s2 = String::from("Hello");
// let s3 = _takes_and_gives_back(s2);

// println!("si => {}, s2 => {}",s1,s3);

// fn gives_ownership() -> String{
//     let some_string = String::from("hello");
//     some_string
// }

// fn _takes_and_gives_back(a_string: String) -> String{ // gets value from it and returns it back
//  a_string
// }

// -----

// ------------------------------------------------------------------------------------------------------------------------------------------------------------


// Refrences
// ------------------------------------------------------------------------------------------------------------------------------------------------------------

// let s1 = String::from("Hello");
// let (s2, len) = calculate(s1);
// println!("s2 {}, len {}",s2,len);

// fn calculate(s: String) -> (String,usize){
//     let lenghts = s.len();
//     (s, lenghts)
// }

// let s1: i32 = 20;
// let s2: i32 = 500;
// let s3: i32 = 0;

// let (_s1,_s2,s3) = _vigh_ferenc(s1,s2,s3);
// let _calc_it_bitch = sqre(s1, s2);

// println!("calculated {}",s3);
// println!("calculated {}",_calc_it_bitch);
// fn _vigh_ferenc(_s1:i32,_s2:i32,_s3:i32) -> (i32,i32,i32){
//     let caclulated: i32 = _s1 * _s2;
//     let varr: i32 = 4;
//     (varr,varr,caclulated)
// }

// fn sqre(a:i32,b:i32) -> i32{
//     a * b
// }


//

// let mut _s1 =  String::from("Hello");
//     change(&mut _s1);

// fn change(x:&mut String){
//     x.push_str(",World")
// }


//
//

// let   mut s = String::from("Hello"); // if i ad a mute it won't work since pointwr error and corrupt data

// let r1 = & s; // if theres a mutable it woN't work

// let r2 = & s;
// // let r3 = &mut s; // ERROR

// println!("{}, {}  ",r1,r2);

// let r3 = &mut s; // this works since r1 and r2 are out of scope
// println!("{}",r3);


//
//
// ------------------------------------------------------------------------------------------------------------------------------------------------------------

// let _refrence_to_nothing = dangle();
// // println!("{}",_refrence_to_nothing);

// }
// fn  dangle() -> &String{
//     let  s = String::from("Hello");
//     &s
// }


// refrences rules
// 1 At any given time, you can have either one mutable refrnce or  any number of immutable refrences

// Refrences must be always valid;

// let mut _s = String::from("hello world");

// let _hello = &s[..5];
// let _world = &s[..];
// let mut _s2 = "hello world";

// let _word = _first_word(&_s2);
// // s.clear(); // this mutates the string 
// // println!("{} ", _word);

// }

// fn _first_word(s: &str) ->  &str{
// let btyes = s.as_bytes();

// for (i, &item) in btyes.iter().enumerate(){
//     if item == b' ' {
//         return &s[..i];
//     }
// }
// &s[..]

let a = [1,2,3,4,5,6,7,8,9];
let mut _slice = &a[..2];
println!("{:?}",_slice); 
}
