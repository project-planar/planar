pub const MAGIC: &[u8; 4] = b"PDLA";
pub const VERSION: u32 = 1;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Header {
    pub magic: [u8; 4],
    pub version: u32,
    pub checksum: u64,
}

impl Header {
    pub fn new(payload_checksum: u64) -> Self {
        Self {
            magic: *MAGIC,
            version: VERSION,
            checksum: payload_checksum,
        }
    }

    pub fn as_bytes(&self) -> [u8; 16] {
        unsafe { std::mem::transmute(*self) }
    }

    pub fn from_bytes(bytes: &[u8; 16]) -> Self {
        unsafe { std::mem::transmute(*bytes) }
    }
}