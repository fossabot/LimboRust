use bytes::BufMut;
use uuid::Uuid;

pub trait PacketBufferWrite {
    fn write_i8(&mut self, value: i8);

    fn write_u8(&mut self, value: u8);

    fn write_i16(&mut self, value: i16);

    fn write_u16(&mut self, value: u16);

    fn write_i32(&mut self, value: i32);

    fn write_i64(&mut self, value: i64);

    fn write_f32(&mut self, value: f32);

    fn write_f64(&mut self, value: f64);

    fn write_uuid(&mut self, uuid: Uuid);

    fn write_u8_array(&mut self, array: &[u8]);

    fn write_bool(&mut self, value: bool) {
        self.write_u8(if value { 0x01 } else { 0x00 });
    }

    fn write_string(&mut self, string: &String) {
        self.write_var_i32(string.len() as i32);
        self.write_u8_array(string.as_bytes());
    }

    // TODO: add chat struct
    //fn write_chat(&mut self, value: Chat);

    // TODO: add identifier struct
    //fn write_identifier(&mut self, value: Identifier);

    fn write_var_i32(&mut self, mut value: i32) {
        while value & -128i32 != 0 {
            self.write_u8(((value & 127i32) as u8) | 128u8);
            value = ((value as u32) >> 7) as i32;
        }
        self.write_u8(value as u8);
    }

    fn write_var_i64(&mut self, mut value: i64) {
        while value & -128i64 != 0 {
            self.write_u8(((value & 127i64) as u8) | 128u8);
            value = ((value as u64) >> 7) as i64;
        }
        self.write_u8(value as u8);
    }

    // TODO: add block_pos struct
    //fn read_block_pos(&mut self) -> Result<BlockPos>;
}

impl<T: BufMut> PacketBufferWrite for T {
    fn write_i8(&mut self, value: i8) {
        self.put_i8(value);
    }

    fn write_u8(&mut self, value: u8) {
        self.put_u8(value);
    }

    fn write_i16(&mut self, value: i16) {
        self.put_i16(value);
    }

    fn write_u16(&mut self, value: u16) {
        self.put_u16(value);
    }

    fn write_i32(&mut self, value: i32) {
        self.put_i32(value);
    }

    fn write_i64(&mut self, value: i64) {
        self.put_i64(value);
    }

    fn write_f32(&mut self, value: f32) {
        self.put_f32(value);
    }

    fn write_f64(&mut self, value: f64) {
        self.put_f64(value);
    }

    fn write_uuid(&mut self, uuid: Uuid) {
        self.put_u128(uuid.as_u128());
    }

    fn write_u8_array(&mut self, array: &[u8]) {
        self.put_slice(array);
    }
}
