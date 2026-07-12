// Throwaway: decode a raw code blob and print each instruction's opcode +
// operands (Debug), so the transpiler author can see exact yaxpeax operand
// shapes. Usage: dump <file> <thumb|arm> [base_hex]
use std::io::Read;
use yaxpeax_arch::{Decoder, U8Reader};
use yaxpeax_arm::armv7::InstDecoder;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut bytes = Vec::new();
    std::fs::File::open(&args[1])
        .unwrap()
        .read_to_end(&mut bytes)
        .unwrap();
    let thumb = args.get(2).map(|s| s == "thumb").unwrap_or(true);
    let base = args
        .get(3)
        .map(|s| u32::from_str_radix(s.trim_start_matches("0x"), 16).unwrap())
        .unwrap_or(0);
    let decoder = InstDecoder::default().with_thumb_mode(thumb);
    let mut off = 0usize;
    while off + 2 <= bytes.len() {
        let mut reader = U8Reader::new(&bytes[off..]);
        match decoder.decode(&mut reader) {
            Ok(inst) => {
                // width: 2 or 4 bytes for thumb
                let consumed = if inst.wide || !thumb { 4 } else { 2 };
                println!(
                    "{:08x}: {:>10} cond={:?} s={} ops={:?}",
                    base + off as u32,
                    format!("{:?}", inst.opcode),
                    inst.condition,
                    inst.s,
                    &inst.operands,
                );
                off += consumed;
            }
            Err(e) => {
                println!("{:08x}: <decode err {:?}>", base + off as u32, e);
                off += if thumb { 2 } else { 4 };
            }
        }
    }
}
