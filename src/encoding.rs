use encoding::all::ASCII;
use encoding::types::RawEncoder;
use encoding::{ByteWriter, DecoderTrap, EncoderTrap, Encoding};

fn escape_unicode(_encoder: &mut dyn RawEncoder, input: &str, output: &mut dyn ByteWriter) -> bool {
    let escapes: Vec<String> = input
        .chars()
        .map(|ch| format!("{:x}", ch as isize))
        .collect();
    let escapes = escapes.concat();
    output.write_bytes(escapes.as_bytes());
    true
}

static UNICODE_ESCAPE: EncoderTrap = EncoderTrap::Call(escape_unicode);

fn encode(data: &str) -> Vec<u8> {
    if let Ok(encoded) = ASCII.encode(data, UNICODE_ESCAPE) {
        encoded
    } else {
        println!("Couldn't encode!");
        std::process::exit(1);
    }
}

fn decode(data: &[u8]) -> String {
    if let Ok(decoded) = ASCII.decode(data, DecoderTrap::Strict) {
        decoded
    } else {
        println!("Couldn't decode!");
        std::process::exit(1);
    }
}

pub fn to_ascii(data: &str) -> String {
    decode(&encode(data))
}
