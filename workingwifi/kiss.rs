use defmt::info;
use esp_storage::FlashStorage;
use embedded_storage::{ReadStorage, Storage};
const STORAGE_OFFSET: u32 = 0x9000;



#[embassy_executor::task]


pub async  fn get_and_increment(){
let mut buffer = [0u8; 4];

let current_count = match FlashStorage::new().read(STORAGE_OFFSET, &mut buffer) {
    Ok(_) => u32::from_le_bytes(buffer),
    Err(_) => 0 // first boot so it will be 0 
};

    // increment and save 
    let new_count = current_count + 1;
    if let Err(e) = FlashStorage::new().write(STORAGE_OFFSET,&new_count.to_le_bytes()){
        defmt::error!("Failed to write realod coutnt : {:?}", defmt::Debug2Format(&e));
    }
    info!("new count is {}", new_count);

}