use defmt::info;
use esp_storage::FlashStorage;
use embedded_storage::{ReadStorage, Storage};
use heapless::Vec;
use core::fmt::Write;

const STORAGE_OFFSET: u32 = 0x9000;

#[embassy_executor::task]
pub async fn get_and_increment() {
    let mut buffer = [0u8; 4];
        
    let current_count = match FlashStorage::new().read(STORAGE_OFFSET, &mut buffer) {
        Ok(_) => {
            let count = u32::from_le_bytes(buffer);
            
            // Check if flash is uninitialized (all 0xFF bytes)
            if count == 0xFFFFFFFF || count == 4294967295 {
                info!("Flash memory uninitialized, starting from 0");
                
                // Initialize flash with 0
                let initial_value = 0u32;
                if let Err(e) = FlashStorage::new().write(STORAGE_OFFSET, &initial_value.to_le_bytes()) {
                    defmt::error!("Failed to initialize flash: {:?}", defmt::Debug2Format(&e));
                }
                0
            } else {
                info!("Read existing counter from flash: {}", count);
                count
            }
        },
        Err(_) => {
            info!("Flash read failed, starting from 0");
            0
        }
    };
    
    
    let nothing = " ";
    let numb2 = "400";
    let username = "Bobbby";
    
    use heapless::String;
    let mut numb1_str: String<16> = String::new();
    write!(numb1_str, "{}", current_count).unwrap();
    
    let mut buf: Vec<_, 32> = Vec::new(); 
    
    buf.extend_from_slice(numb1_str.as_bytes());
    buf.extend_from_slice(nothing.as_bytes());
    buf.extend_from_slice(numb2.as_bytes());
    buf.extend_from_slice(nothing.as_bytes());
    buf.extend_from_slice(username.as_bytes());
    
    info!("Built buffer with persistent counter: {:?}", defmt::Debug2Format(&buf));
    
   
    let mut start = 0;
    let mut spaces = 0;
    for (i, &byte) in buf.iter().enumerate() {
        if byte == b' ' { spaces += 1; if spaces == 2 { start = i + 1; } }
        if spaces == 2 && byte != b' ' && (i + 1 == buf.len() || buf[i + 1] == b' ') { 
            let username = core::str::from_utf8(&buf[start..=i]).unwrap(); 
            info!("Extracted username: {}", username);
            break; 
        }
    }
    
    let mut end = 0;
    for (i, &byte) in buf.iter().enumerate() {
        if byte == b' ' { end = i; break; }
    }
    let first_number = core::str::from_utf8(&buf[0..end]).unwrap();
    
    info!("Extracted first number: {}", first_number);
    let mut first_number: i32 = first_number.parse().unwrap();
    if  first_number > 50000{
        let first_number = 0;
    }
    first_number += 1;
    info!("Incremented first number: {}", first_number);
    
    let mut space_count = 0;
    let mut first = 0;
    let mut second = 0;
    for (i, &byte) in buf.iter().enumerate() {
        if byte == b' ' {
            space_count += 1;
            if space_count == 1 {
                first = i;
            }
            if space_count == 2 {
                second = i;
                break;
            }
        }
    }
    let middle_number = core::str::from_utf8(&buf[first+1..second]).unwrap();
    
    info!("Extracted middle number: {}", middle_number);
    info!("Original buffer ==> {:?} \n utf8 ==> {:?}", defmt::Debug2Format(&buf), defmt::Debug2Format(&buf.utf8_chunks()));
    
    let mut first_number_str: String<16> = String::new();
    write!(first_number_str, "{}", first_number).unwrap();
    
    let mut buf: Vec<_, 32> = Vec::new();
    buf.extend_from_slice(first_number_str.as_bytes());
    buf.extend_from_slice(nothing.as_bytes());
    buf.extend_from_slice(middle_number.as_bytes());
    buf.extend_from_slice(nothing.as_bytes());
    buf.extend_from_slice(username.as_bytes());
    
    info!("Updated buffer ==> {:?} \n utf8 ==> {:?}", defmt::Debug2Format(&buf), defmt::Debug2Format(&buf.utf8_chunks()));
    
    if let Err(e) = FlashStorage::new().write(STORAGE_OFFSET, &(first_number as u32).to_le_bytes()) {
        defmt::error!("Failed to write counter to flash: {:?}", defmt::Debug2Format(&e));
    } else {
        info!("Successfully saved counter {} to flash", first_number);
    }
}
