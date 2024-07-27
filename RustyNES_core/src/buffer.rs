pub struct Buffer {
    pub data: Vec<u8>,
    pub index: usize,
}

impl Buffer {
    pub fn new_buffer() -> Self {
        Self {
            data: Vec::new(),
            index: 0,
        }
    }

    pub fn new_from_bytes(bytes: Vec<u8>) -> Self {
        Self {
            data: bytes,
            index: 0,
        }
    }

    // write functions //////////////////////////////////////////
    pub fn write_bool(&mut self, val: bool) {
        self.data.push(val as u8);
    }

    pub fn write_u8(&mut self, val: u8) {
        self.data.push(val);
    }

    pub fn write_u16(&mut self, val: u16) {
        self.data.extend_from_slice(&val.to_le_bytes());
    }

    pub fn write_u32(&mut self, val: u32) {
        self.data.extend_from_slice(&val.to_le_bytes());
    }

    pub fn write_u64(&mut self, val: u64) {
        self.data.extend_from_slice(&val.to_le_bytes());
    }

    pub fn write_u8_arr(&mut self, arr: &[u8]) {
        for a in arr {
            self.write_u8(*a)
        }
    }

    pub fn write_u32_arr(&mut self, arr: &[u32]) {
        for a in arr {
            self.write_u32(*a);
        }
    }

    // read functions //////////////////////////////////////////
    pub fn read_bool(&mut self) -> bool {
        self.read_u8() != 0
    }

    pub fn read_u8(&mut self) -> u8 {
        let val = self.data[self.index];
        self.index += 1;
        val
    }

    pub fn read_u16(&mut self) -> u16 {
        let mut bytes = [0; 2];
        self.read_u8_arr(&mut bytes);
        u16::from_le_bytes(bytes)
    }

    pub fn read_u32(&mut self) -> u32 {
        let mut bytes = [0; 4];
        self.read_u8_arr(&mut bytes);
        u32::from_le_bytes(bytes)
    }

    pub fn read_u64(&mut self) -> u64 {
        let mut bytes = [0; 8];
        self.read_u8_arr(&mut bytes);
        u64::from_le_bytes(bytes)
    }

    pub fn read_u8_arr(&mut self, arr: &mut [u8]) {
        for a in arr {
            *a = self.read_u8();
        }
    }

    pub fn read_u32_arr(&mut self, arr: &mut [u32]) {
        for a in arr {
            *a = self.read_u32();
        }
    }
}
