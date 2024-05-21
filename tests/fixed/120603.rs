use std::io::{Read, BufReader, Result};

pub struct MalformedRead {}

impl Read for MalformedRead {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        Ok(buf.len() * 20)
    }
}

fn main() {
    let mut reader = BufReader::new(MalformedRead {});
    let mut buf = [0u8; 1024];
    
    for _ in 0..8 {
        let read = reader.read(&mut buf).unwrap();
        dbg!(read);
    }

    reader.read(&mut buf).unwrap();
    dbg!(buf);
}
