

fn main() {

let mut  boy = "Sdsasd";

let boy = boy.as_bytes();
println!("{:?}",boy);

let boy = boy.utf8_chunks();
println!("{:?}",boy);

// task is to have a vector and turn it into bytes and read the bytes back

let nothing = " ";
let numb1= "0323";
let numb2 = "133";
let username = "Bobbby";

let mut buf: Vec<u8> = Vec::new();

buf.extend(numb1.as_bytes());
buf.extend(nothing.as_bytes());
buf.extend(numb2.as_bytes());
buf.extend(nothing.as_bytes());


buf.extend(username.as_bytes());

println!("{:?}",buf.utf8_chunks());
let mut start = 0;
let mut spaces = 0;
for (i, &byte) in buf.iter().enumerate() {
    if byte == b' ' { spaces += 1; if spaces == 2 { start = i + 1; } }
    if spaces == 2 && byte != b' ' && (i + 1 == buf.len() || buf[i + 1] == b' ') { 
        let username = core::str::from_utf8(&buf[start..=i]).unwrap(); break; }
}
let mut end = 0;
for (i, &byte) in buf.iter().enumerate() {
    if byte == b' ' { end = i; break; }
}
let first_number = core::str::from_utf8(&buf[0..end]).unwrap();

println!("{}", first_number);
let mut first_number :i32 = first_number.parse().unwrap();

first_number += 1;

println!("{}",username);

let mut space_count = 0;
let mut first = 0;
let mut secound = 0;
for (i, &byte) in buf.iter().enumerate(){
    if byte == b' ' {
        space_count += 1;
        if space_count == 1{
            first = i;
        }
        if space_count == 2{
            secound = i;
            break;
        }
    }


}
let middle_number = core::str::from_utf8(&buf[first+1..secound]).unwrap();

println!("{}",middle_number);
println!("normal ==> {:?} \n utf8 ==> {:?}",buf, buf.utf8_chunks());

let first_number = first_number.to_string();

let mut buf: Vec<u8> = Vec::new();
buf.extend(first_number.as_bytes());
buf.extend(nothing.as_bytes());
buf.extend(middle_number.as_bytes());
buf.extend(nothing.as_bytes());
buf.extend(username.as_bytes());

println!("normal ==> {:?} \n utf8 ==> {:?}",buf, buf.utf8_chunks());
}
