

use defmt::info;
use heapless::Vec;
use core::fmt::Write;

fn main() {

    let mut  boy = "Sdsasd";
    
    let boy = boy.as_bytes();
    info!("{:?}",defmt::Debug2Format(&boy));
    
    let boy = boy.utf8_chunks();
    info!("{:?}",defmt::Debug2Format(&boy));
    
    // task is to have a vector and turn it into bytes and read the bytes back
    
    let nothing = " ";
    let numb1= "0323";
    let numb2 = "133";
    let username = "Bobbby";
    
    let mut buf: Vec<_, 16> = Vec::new();
    
    buf.extend_from_slice(numb1.as_bytes());
    buf.extend_from_slice(nothing.as_bytes());
    buf.extend_from_slice(numb2.as_bytes());
    buf.extend_from_slice(nothing.as_bytes());
    
    
    buf.extend_from_slice(username.as_bytes());
    
    // info!("{:?}",defmt::Debug2Format(buf.utf8_chunks()));
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
    
    info!("{}", first_number);
    let mut first_number :i32 = first_number.parse().unwrap();
    
    first_number += 1;
    
    info!("{}",username);
    
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
    
    info!("{}",middle_number);
    info!("normal ==> {:?} \n utf8 ==> {:?}",defmt::Debug2Format(&buf), defmt::Debug2Format(&buf.utf8_chunks()));
    
    use heapless::String;
    let mut first_number_str: String<16> = String::new();
    write!(first_number_str, "{}", first_number).unwrap();
    
    let mut buf: Vec<_, 16> = Vec::new();
    buf.extend_from_slice(first_number_str.as_bytes());
    buf.extend_from_slice(nothing.as_bytes());
    buf.extend_from_slice(middle_number.as_bytes());
    buf.extend_from_slice(nothing.as_bytes());
    buf.extend_from_slice(username.as_bytes());
    
    info!("normal ==> {:?} \n utf8 ==> {:?}",defmt::Debug2Format(&buf), defmt::Debug2Format(&buf.utf8_chunks()));
    }
    