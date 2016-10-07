enum SRecord<'a>
{
    Header,
    Record{address: u32, data: &'a [u8]},
    Termination(u32),
}

struct Checksum(u8);

impl Checksum {
    fn new(length: u8, address: u32) -> Checksum {
        let mut chk = Checksum(length);
        chk.add32(address);
        chk
    }
    fn add8(&mut self, byte: u8) {
        self.0 = self.0.wrapping_add(byte)
    }
    fn add16(&mut self, word: u16) {
        self.0 = self.0.wrapping_add(word as u8).wrapping_add((word >> 8) as u8)
    }
    fn add32(&mut self, long: u32) {
        self.0 = self.0
            .wrapping_add(long as u8)
            .wrapping_add((long >> 8) as u8)
            .wrapping_add((long >> 16) as u8)
            .wrapping_add((long >> 24) as u8)
    }
    fn calculate(&self) -> u8 {
        0xffu8 - self.0
    }
}

use std::fmt;
impl<'a> fmt::Display for SRecord<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SRecord::Header => { 
                let r68k = 0x7236386B; // "r68k" in ASCII
                let check = Checksum::new(7, r68k);
                write!(f, "S0070000{:08X}{:02X}", r68k, check.calculate())
            },
            SRecord::Record{address, data} => {
                let mut check = Checksum::new(4 + data.len() as u8, address);
                write!(f, "S2{:02X}{:06X}", 4 + data.len(), address).unwrap();
                for i in data {
                    write!(f, "{:02X}", i).unwrap();
                    check.add8(*i);
                };
                write!(f, "{:02X}", check.calculate())
            },
            SRecord::Termination(entrypoint) => {
                let check = Checksum::new(4, entrypoint);
                write!(f, "S804{:06X}{:02X}", entrypoint, check.calculate())
            },
        }
    } 
}

use std::io::Write;

pub fn write_s68(writer: &mut Write, data: Vec<(u32,Vec<u8>)>, entrypoint: u32) {
    writeln!(writer, "{}", SRecord::Header).unwrap();
    for (offset, mem) in data {
        let chunk_size = 34;
        for (i, chunk) in mem.chunks(chunk_size).enumerate() {
            writeln!(writer, "{}", SRecord::Record { address: offset + (i*chunk_size) as u32, data: chunk }).unwrap();
        };
    };
    writeln!(writer, "{}", SRecord::Termination(entrypoint)).unwrap();
}

#[cfg(test)]
mod tests {
    use super::{write_s68, Checksum, SRecord};
    use std::io::LineWriter;

    #[test]
    fn can_print_to_vec() {
        let mut lw = LineWriter::new(vec![]);
        let data: Vec<u8> = (0u8 .. 0xA0u8).collect();

        write_s68(&mut lw, vec![(2000, data)], 2000);
        assert!(lw.into_inner().unwrap().len() > 0);
    }

    #[test]
    fn checksum_matches() {
        // S105089E082C20 = S1 05 bytes, address 089E, data is 082C, checksum 20
        let mut chk = Checksum::new(5, 0x089E);
        chk.add32(0x082C);
        assert_eq!(0x20, chk.calculate());
    }

    #[test]
    fn checksum_matches_on_header() {
        // S00700007236386BAD is the r68k Header record
        let mut chk = Checksum::new(7, 0);
        chk.add16(0x7236);
        chk.add16(0x386B);
        assert_eq!(0xAD, chk.calculate());
    }
    #[test]
    fn checksum_matches_on_header_16bit() {
        // S00700007236386BAD is the r68k Header record
        let mut chk = Checksum::new(7, 0);
        chk.add8(0x72);
        chk.add8(0x36);
        chk.add8(0x38);
        chk.add8(0x6B);
        assert_eq!(0xAD, chk.calculate());
    }
    #[test]
    fn checksum_matches_on_header_8bit() {
        // S00700007236386BAD is the r68k Header record
        let mut chk = Checksum::new(7, 0);
        chk.add32(0x7236386B);
        assert_eq!(0xAD, chk.calculate());
    }
    
    #[test]
    fn checksum_matches_on_terminator() {
        // S804002016C5 is a sample r68k Termination record
        let chk = Checksum::new(04, 0x002016);
        assert_eq!(0xC5, chk.calculate());
    }

    #[test]
    fn checksum_matches_on_record() {
        let example = "S2243232406578616D706C6520646174612068657265206A75737420617320616E20657861A6";
        // S2 24 bytes, address 323240 data is 6578616D706C6520646174612068657265206A75737420617320616E20657861, checksum A6
        let data: Vec<u8> = vec![0x65, 0x78, 0x61, 0x6D, 0x70, 0x6C, 0x65, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x68, 0x65, 0x72, 0x65, 0x20, 0x6A, 0x75, 0x73, 0x74, 0x20, 0x61, 0x73, 0x20, 0x61, 0x6E, 0x20, 0x65, 0x78, 0x61];
        let rec = SRecord::Record { address: 0x323240, data: &data};
        let generated = format!("{}", rec);

        assert_eq!(example, generated);
    }
}