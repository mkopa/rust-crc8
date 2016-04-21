pub struct Crc8 {
        table : [u8; 256]
}

impl Crc8 {
        pub fn create_msb(polynomial : u8) -> Crc8 {
                let msbit : u8 = 0x80;
                let mut t : u8 = msbit;
                let mut tmp : u8;
                let mut i : u32 = 1;
                let mut idx : u8;
                let mut table : [u8; 256] = [0; 256];

                while i < 256 {
                        if t & msbit != 0 { tmp = polynomial; } else { tmp = 0; }
                        t = (t << 1) ^ tmp;
                        for j in 0..i {
                                idx = (i+j) as u8;
                                table[(idx) as usize] =  table[j as usize] ^ t;
                        }
                        i *= 2;
                }

                return Crc8{ table : table};
        }

        pub fn create_lsb(polynomial :u8) -> Crc8 {
                let mut i :u32 = 0xff;
                let mut j :u32;
                let mut t :u8 = 1;
                let mut tmp :u8;
                let mut idx :u8;
                let mut table : [u8; 256] = [0; 256];

                while i != 0 {
                        if t & 1 != 0 {
                                tmp = polynomial;
                        }
                        else {
                                tmp = 0;
                        }
                        t = (t >> 1) ^ tmp;
                        j = 0;
                        while j < 256 {
                                idx = (i+j) as u8;
                                table[idx as usize] = table[j as usize] ^ t;
                                j += 2 * i;
                        }
                        i >>= 1;
                }

                return Crc8{ table : table };
        }

        pub fn calc(&mut self, buffer : &[u8], length: i32, crc: u8) -> u8 {
                let mut i : i32 = 0;
                let mut crc_tmp = crc;

                while i < length {
                        crc_tmp = self.table[((crc_tmp ^ buffer[i as usize]) as u8) as usize];
                        i+=1;
                }
                return crc_tmp;
        }
}
