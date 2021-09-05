fn main() {
    let bytes = std::fs::read(std::env::args().nth(1).unwrap()).unwrap();

    sjis_strings(&bytes, 6)
}

fn sjis_strings(bytes: &[u8], minimum_bytes: usize) {
    let mut state = State::Initial;
    let mut string = vec![];
    for &byte in bytes {
        match (state, byte) {
            | (State::Initial, 0x81..=0x9f)
            | (State::Initial, 0xe0..=0xef)
            => state = State::SecondByte(byte),

            | (State::Initial, 0x20..=0x7e)
            | (State::Initial, 0xa1..=0xdf)
            => string.push(byte),

            | (State::SecondByte(first), 0x40..=0x9e) if first % 2 == 1 && byte != 0x7f
            => {
                string.push(first);
                string.push(byte);
                state = State::Initial;
            },

            | (State::SecondByte(first), 0x9f..=0xfc) if first % 2 == 0
            => {
                string.push(first);
                string.push(byte);
                state = State::Initial;
            },

            _ => {
                if string.len() >= minimum_bytes {
                    let (decoded, _, _) = encoding_rs::SHIFT_JIS.decode(&string);
                    println!("{}", decoded);
                }
                string.clear();
                state = State::Initial;
            },
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum State { Initial, SecondByte(u8) }
