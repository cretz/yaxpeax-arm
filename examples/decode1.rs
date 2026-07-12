// Throwaway: decode a few explicit 4-byte thumb encodings (given as hex on the
// command line, little-endian byte order as `aabbccdd`) and print the display text.
use yaxpeax_arch::{Decoder, U8Reader};
use yaxpeax_arm::armv7::InstDecoder;

fn main() {
    let decoder = InstDecoder::default().with_thumb_mode(true);
    for arg in std::env::args().skip(1) {
        let v = u32::from_str_radix(arg.trim_start_matches("0x"), 16).unwrap();
        let bytes = v.to_be_bytes(); // arg gives bytes in memory order b0b1b2b3
        let mut reader = U8Reader::new(&bytes[..]);
        match decoder.decode(&mut reader) {
            Ok(inst) => println!("{}  ->  {}", arg, inst),
            Err(e) => println!("{}  ->  ERR {}", arg, e),
        }
    }
}
