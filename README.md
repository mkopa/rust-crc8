# rust-crc8
A CRC8 implementation

#### Installation

Use [cargo package](https://crates.io/crates/crc8).

#### Usage
```
extern crate crc8;
use crc8::*;

fn main() {
        let buff : [u8; 3] = [1, 2, 3];

        /* Init Crc8 module for given polynomial in regular bit order. */
        let mut crc8 = Crc8::create_lsb(130);

        /* calculate a crc8 over the given input data.
         * pbuffer: pointer to data buffer.
         * length: number of bytes in data buffer.
         * crc:	previous returned crc8 value.
        */
        let mut crc = crc8.calc(&buff, 3, 0);
        println!("crc8: {}", crc);

        /* Init Crc8 module for given polynomial in reverse bit order. */
        crc8 = Crc8::create_msb(130);
        crc = crc8.calc(&buff, 3, 0);
        println!("crc8: {}", crc);
}
```