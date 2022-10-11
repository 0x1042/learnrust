#[derive(Debug)]
pub struct MagicMessage {
    pub magic: u64,
    pub id: u32,
    pub len: u64,
    pub extra: String,
}

impl MagicMessage {
    pub fn new(magic: u64, id: u32, len: u64) -> Self {
        Self {
            magic,
            id,
            len,
            extra: Default::default(),
        }
    }
}
