use esp_storage::FlashStorage;
use embedded_storage::{ReadStorage, Storage};
const STORAGE_OFFSET: u32 = 0x9000;
pub struct ReloadCounter {
    storage: FlashStorage,
}
impl ReloadCounter {
    pub fn new() -> Self {
        Self {
            storage: FlashStorage::new(),
        }
    }

    pub async fn get_and_increment(&mut self) -> u32 {
        let mut buffer = [0u8; 4];
        
        // Read current count
        let current_count = match self.storage.read(STORAGE_OFFSET, &mut buffer) {
            Ok(_) => u32::from_le_bytes(buffer),
            Err(_) => 0, // First boot
        };
        
        // Increment and save
        let new_count = current_count + 1;
        if let Err(e) = self.storage.write(STORAGE_OFFSET, &new_count.to_le_bytes()) {
            defmt::error!("Failed to write reload count: {:?}",defmt::Debug2Format(&e));
        }
        
        new_count
    }
}
