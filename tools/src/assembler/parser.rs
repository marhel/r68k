use pest::prelude::*;
use super::super::{OpcodeInstance, Size};
use operand::Operand;
impl_rdp! {
    grammar! {
        statement = { label? ~ instruction ~ comment? }
        instruction = { mnemonic ~ operands? }
        mnemonic = @{ letter+ ~ qualifier? }
        qualifier = _{ longsize | wordsize | bytesize }
        longsize = { [".L"] | [".l"] }
        wordsize = { [".W"] | [".w"] }
        bytesize = { [".B"] | [".b"] }
        comment = { [";"] ~ any* ~ eoi }
        operands = { operand ~ (comma ~ operand)* }
        comma = {[","]}
        operand = { drd | ard | api | apd | adi | aix | abs | pcd | pci | imm | ari }

        drd = @{ ["D"] ~ ['0'..'7'] }
        ard = @{ ["A"] ~ ['0'..'7'] }
        ari = { ["("] ~ ard ~ [")"] }
        api = { ["("] ~ ard ~ [")"] ~ ["+"] }
        apd = { ["-"] ~["("] ~ ard ~ [")"] }
        adi = { ["("] ~ displacement ~ [","] ~ ard ~ [")"] }
        aix = { ["("] ~ displacement ~ [","] ~ ard ~ [","] ~ (drd | ard) ~ [")"] }
        abs = @{ number ~ qualifier? }
        pcd = { ["("] ~ displacement ~ [","] ~ ["PC"] ~ [")"] }
        pci = { ["("] ~ displacement ~ [","] ~ ["PC"] ~ [","] ~ (drd | ard) ~ [")"] }
        imm = @{ ["#"] ~ number ~ qualifier? }

        displacement = _{ number }
        number = { hex | bin | dec | oct}
        hex = @{ ["$"] ~ ["-"]? ~ (['0'..'9'] | ['A'..'F'] | ['a'..'f'])+ }
        bin = @{ ["%"] ~ ["-"]? ~(['0'..'1'])+ }
        oct = @{ ["@"] ~ ["-"]? ~(['0'..'7'])+ }
        dec = @{ ["-"]? ~ ['0'..'9']+ }

        label = @{ name ~ [":"] }
        letter = _{ ['A'..'Z'] | ['a'..'z'] }
        digit = _{ ['0'..'9'] }
        name = @{ letter ~ (letter | digit)* }

        whitespace = _{ [" "] | ["\t"] }
    }

    process! {
        // process_instruction(&self) -> OpcodeInstance<'input> {
        //     (&mnemonic: mnemonic, operands: _operands()) => {
        //         OpcodeInstance {
        //             mnemonic: mnemonic,
        //             size: Size::Byte,
        //             operands: operands,
        //         }
        //     }
        // }
        process_operands(&self) -> Vec<Operand> {
            (_: operands, head: process_operand(), mut tail: process_remaining_operands()) => {
                tail.push(head);
                tail.reverse();
                tail
            },
            () => {
                Vec::new()
            }
        }
        process_remaining_operands(&self) -> Vec<Operand> {
            (_: comma, head: process_operand(), mut tail: process_remaining_operands()) => {
                tail.push(head);
                tail
            },
            () => {
                Vec::new()
            }
        }
        process_operand(&self) -> Operand {
            (_: operand, &reg: drd) => {
                Operand::DataRegisterDirect(reg[1..].parse().unwrap())
            },
            (_: operand, &reg: ard) => {
                Operand::AddressRegisterDirect(reg[1..].parse().unwrap())
            },
            (_: operand, _: ari, &reg: ard) => {
                Operand::AddressRegisterIndirect(reg[1..].parse().unwrap())
            },
            (_: operand, _: api, &reg: ard) => {
                Operand::AddressRegisterIndirectWithPostincrement(reg[1..].parse().unwrap())
            },
            (_: operand, _: apd, &reg: ard) => {
                Operand::AddressRegisterIndirectWithPredecrement(reg[1..].parse().unwrap())
            },
            (_: operand, _: adi, number: process_number(), &reg: ard) => {
                Operand::AddressRegisterIndirectWithDisplacement(reg[1..].parse().unwrap(), number as i16)
            },
            (_: operand, _: aix, number: process_number(), &reg: ard, &ireg: ard) => {
                Operand::AddressRegisterIndirectWithIndex(reg[1..].parse().unwrap(), 8u8+ireg[1..].parse::<u8>().unwrap(), number as i8)
            },
            (_: operand, _: aix, number: process_number(), &reg: ard, &ireg: drd) => {
                Operand::AddressRegisterIndirectWithIndex(reg[1..].parse().unwrap(), ireg[1..].parse().unwrap(), number as i8)
            },
            (_: operand, _: pcd, number: process_number()) => {
                Operand::PcWithDisplacement(number as i16)
            },
            (_: operand, _: pci, number: process_number(), &ireg: ard) => {
                Operand::PcWithIndex(8u8+ireg[1..].parse::<u8>().unwrap(), number as i8)
            },
            (_: operand, _: pci, number: process_number(), &ireg: drd) => {
                Operand::PcWithIndex(ireg[1..].parse().unwrap(), number as i8)
            },
            (_: operand, _: abs, number: process_number(), _: wordsize) => {
                Operand::AbsoluteWord(number as u16)
            },
            (_: operand, _: abs, number: process_number(), _: longsize) => {
                Operand::AbsoluteLong(number as u32)
            },
            (_: operand, _: abs, number: process_number()) => {
                Operand::AbsoluteWord(number as u16)
            },
            (_: operand, _: imm, number: process_number(), _: bytesize) => {
                Operand::Immediate(Size::Byte, number as u32)
            },
            (_: operand, _: imm, number: process_number(), _: wordsize) => {
                Operand::Immediate(Size::Word, number as u32)
            },
            (_: operand, _: imm, number: process_number(), _: longsize) => {
                Operand::Immediate(Size::Long, number as u32)
            },
            (_: operand, _: imm, number: process_number()) => {
                Operand::Immediate(Size::Unsized, number as u32)
            },
        }

        process_number(&self) -> i32 {
            (_: number, &dec: dec) => {
                dec.parse().unwrap()
            },
            (_: number, &hex: hex) => {
                i32::from_str_radix(&hex[1..], 16).unwrap()
            },
            (_: number, &oct: oct) => {
                i32::from_str_radix(&oct[1..], 8).unwrap()
            },
            (_: number, &bin: bin) => {
                i32::from_str_radix(&bin[1..], 2).unwrap()
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Rdp, Rule};
    use pest::prelude::*;
    extern crate rand;
    use operand::Operand;
    use super::super::super::{OpcodeInstance, Size};

    #[test]
    fn test_drd_operand() {
        process_operand("D0", &Operand::DataRegisterDirect(0));
        process_operand("D7", &Operand::DataRegisterDirect(7));
    }
    #[test]
    fn test_ard_operand() {
        process_operand("A0", &Operand::AddressRegisterDirect(0));
        process_operand("A7", &Operand::AddressRegisterDirect(7));
    }
    #[test]
    fn test_ari_operand() {
        process_operand("(A0)", &Operand::AddressRegisterIndirect(0));
        process_operand("(A7)", &Operand::AddressRegisterIndirect(7));
    }
    #[test]
    fn test_api_operand() {
        process_operand("(A0)+", &Operand::AddressRegisterIndirectWithPostincrement(0));
        process_operand("(A7)+", &Operand::AddressRegisterIndirectWithPostincrement(7));
    }
    #[test]
    fn test_apd_operand() {
        process_operand("-(A0)", &Operand::AddressRegisterIndirectWithPredecrement(0));
        process_operand("-(A7)", &Operand::AddressRegisterIndirectWithPredecrement(7));
    }
    #[test]
    fn test_adi_operand() {
        process_operand("($10,A0)", &Operand::AddressRegisterIndirectWithDisplacement(0, 16));
        process_operand("(%10,A7)", &Operand::AddressRegisterIndirectWithDisplacement(7, 2));
    }
    #[test]
    fn test_aix_operand() {
        process_operand("( 10,A0,D0)", &Operand::AddressRegisterIndirectWithIndex(0, 0, 10));
        process_operand("($10,A0,A1)", &Operand::AddressRegisterIndirectWithIndex(0, 9, 16));
        process_operand("(%10,A7,D7)", &Operand::AddressRegisterIndirectWithIndex(7, 7, 2));
        process_operand("(@10,A7,A6)", &Operand::AddressRegisterIndirectWithIndex(7, 14, 8));
    }
    #[test]
    fn test_abs_operand() {
        process_operand("100", &Operand::AbsoluteWord(100));
        process_operand("$100.B", &Operand::AbsoluteWord(256));
        process_operand("@100.W", &Operand::AbsoluteWord(64));
        process_operand("%100.L", &Operand::AbsoluteLong(4));
    }
    #[test]
    fn test_pcd_operand() {
        process_operand("(-10,PC)", &Operand::PcWithDisplacement(-10));
    }
    #[test]
    fn test_pci_operand() {
        process_operand("(10,PC,D0)", &Operand::PcWithIndex(0, 10));
        process_operand("(10,PC,A0)", &Operand::PcWithIndex(8, 10));
    }
    #[test]
    fn test_imm_operand() {
        process_operand("#%111", &Operand::Immediate(Size::Unsized, 7));
        process_operand("#%111.B", &Operand::Immediate(Size::Byte, 7));
        process_operand("#%111.W", &Operand::Immediate(Size::Word, 7));
        process_operand("#%111.l", &Operand::Immediate(Size::Long, 7));
    }

    fn process_operand(input: &str, expected: &Operand) {
        let mut parser = Rdp::new(StringInput::new(input));
        if !parser.operand() || !parser.end() {
            let qc = parser.queue_with_captures();
            panic!("{} => {:?}", input, qc);
        }
        assert_eq!(*expected, parser.process_operand());
    }
    #[test]
    fn test_random_operand() {
        for o1 in 1..15 {
            let input = format!("{}", operand(o1, true));
            let mut parser = Rdp::new(StringInput::new(input.trim()));
            if !parser.operand() || !parser.end() {
                let qc = parser.queue_with_captures();
                panic!("{} => {:?}", input.trim(), qc);
            }
            parser.process_operand();
        }
    }
    #[test]
    fn test_imm_operands() {
        process_operands("#%111,(A7)", &vec![Operand::Immediate(Size::Unsized, 7), Operand::AddressRegisterIndirect(7)]);
        process_operands("-(A0),(8,PC)", &vec![Operand::AddressRegisterIndirectWithPredecrement(0), Operand::PcWithDisplacement(8)]);
        process_operands("D0,D1,D2,D3,D4", &(0..5).map(|i|Operand::DataRegisterDirect(i)).collect::<Vec<Operand>>());
    }

    fn process_operands(input: &str, expected: &Vec<Operand>) {
        let mut parser = Rdp::new(StringInput::new(input));
        if !parser.operands() || !parser.end() {
            let qc = parser.queue_with_captures();
            panic!("{} => {:?}", input, qc);
        }
        let qc = parser.queue_with_captures();
        println!("{} => {:?}", input.trim(), qc);
        assert_eq!(*expected, parser.process_operands());
    }
    #[test]
    fn test_random_operands() {
        for o1 in 1..15 {
            for o2 in 1..15 {
                let input = format!("{}{}", operand(o1, true), operand(o2, false));
                let mut parser = Rdp::new(StringInput::new(input.trim()));
                if !parser.operands() || !parser.end() {
                    let qc = parser.queue_with_captures();
                    panic!("{} => {:?}", input.trim(), qc);
                }
                let qc = parser.queue_with_captures();
                println!("{} => {:?}", input.trim(), qc);
                parser.process_operands();
            }
        }
    }
    #[test]
    fn test_zero_operands() {
        parse("ZERO", 0);
    }

    #[test]
    fn test_one_operand() {
        parse("ONE", 1);
    }

    #[test]
    fn test_two_operands() {
        parse("TWO", 2);
    }

    // drd = @{ ["D"] ~ ['0'..'7'] }
    // ard = @{ ["A"] ~ ['0'..'7'] }
    // ari = { ["("] ~ ard ~ [")"] }
    // api = { ["("] ~ ard ~ [")"] ~ ["+"] }
    // apd = { ["-"] ~["("] ~ ard ~ [")"] }
    // adi = { ["("] ~ displacement ~ [","] ~ ard ~ [")"] }
    // aix = { ["("] ~ displacement ~ [","] ~ ard ~ [","] ~ (drd | ard) ~ [")"] }
    // abs = { number ~ [".L"]? }
    // pcd = { ["("] ~ displacement ~ [","] ~ ["PC"] ~ [")"] }
    // pci = { ["("] ~ displacement ~ [","] ~ ["PC"] ~ [","] ~ (drd | ard) ~ [")"] }
    // imm = { ["#"] ~ number}

    fn random_size() -> &'static str {
        match self::rand::random::<u8>() % 10 {
            0 => ".L",
            1 => ".l",
            2 => ".W",
            3 => ".w",
            4 => ".B",
            5 => ".b",
            _ => "",
        }
    }
    fn random_num() -> String {
        let num = self::rand::random::<i16>();
        match self::rand::random::<u8>() % 10 {
            0 => format!("{}", num),
            1 => format!("@{:o}", num),
            2 => format!("@{:08o}", num),
            3 => format!("%{:b}", num),
            4 => format!("%{:016b}", num),
            5 => format!("${:x}", num),
            6 => format!("${:X}", num),
            7 => format!("${:08x}", num),
            8 => format!("${:08X}", num),
            _ => format!("${:x}", num),
        }
    }
    fn operand(id: u8, first: bool) -> String {
        let op = match id {
            1 => "?Dx".to_string(),
            2 => "?Ax".to_string(),
            3 => "?(Ax)".to_string(),
            4 => "?(Ax)+".to_string(),
            5 => "?-(Ax)".to_string(),
            6 => "?(z,Ax)".to_string(),
            7 => "?(z,Ax,Dy)".to_string(),
            8 => "?(z,Ax,Ay)".to_string(),
            9 => format!("?z{}", random_size()),
            10 => "?(z,PC)".to_string(),
            11 => "?(z,PC,Dy)".to_string(),
            12 => "?(z,PC,Ay)".to_string(),
            13 => format!("?#z{}", random_size()),
            _ => "?#z".to_string(),
        };
        op.replace("?", if first {" "} else {","})
        .replace("x", (self::rand::random::<u8>() % 8).to_string().as_str())
        .replace("y", (self::rand::random::<u8>() % 8).to_string().as_str())
        .replace("z", random_num().as_str())
    }

    fn parse(mnemonic: &str, ops: u8) {
        let mut mnemonic = mnemonic.to_string();
        mnemonic.push_str(random_size());
        let mnemonic = mnemonic.as_str();
        match ops {
            0 =>
                parse_ops(mnemonic, ops, "", ""),
            1 =>
                for o1 in 1..15 {
                    parse_ops(mnemonic, ops, operand(o1, true).as_str(), "");
                },
            _ =>
                for o1 in 1..15 {
                    for o2 in 1..15 {
                        parse_ops(mnemonic, ops, operand(o1, true).as_str(), operand(o2, false).as_str());
                    }
                },
        }
    }

    fn parse_ops(mnemonic: &str, ops: u8, op1: &str, op2: &str) {
        let with_space = format!(" {}{}{}", mnemonic, op1, op2);
        parse_with(with_space.as_str(), mnemonic, ops, op1, op2, false, false);
        let with_label = format!("label: {}{}{}", mnemonic, op1, op2);
        parse_with(with_label.as_str(), mnemonic, ops, op1, op2, true, false);
        let with_comment = format!(" {}{}{} ; or a comment", mnemonic, op1, op2);
        parse_with(with_comment.as_str(), mnemonic, ops, op1, op2, false, true);
        let with_both_comment_and_label = format!("label: {}{}{} ; and a comment", mnemonic, op1, op2);
        parse_with(with_both_comment_and_label.as_str(), mnemonic, ops, op1, op2, true, true);
    }

    fn parse_with(input: &str, mnemonic: &str, ops: u8, op1: &str, op2: &str, label: bool, comment: bool) {
        // println!("parse_with: {:?}", input);
        let mut parser = Rdp::new(StringInput::new(input));
        assert!(parser.statement());
        if !parser.end() {
            println!("input: {:?}", input);
            println!("queue: {:?}", parser.queue());
            println!("expected {:?}", parser.expected());
        }
        assert!(parser.end());
        let qc = parser.queue_with_captures();
        let mut i = 0;
        assert_eq!(Rule::statement, qc[i].0.rule);
        if label {
            i+=1;
            assert_eq!(Rule::label, qc[i].0.rule);
            i+=1;
        }
        while qc[i].0.rule != Rule::mnemonic {
            i+=1;
        }
        assert_eq!(Rule::mnemonic, qc[i].0.rule);
        assert_eq!(mnemonic, qc[i].1);
        i+=1;
        if ops > 0 {
            while qc[i].0.rule != Rule::operand {
                i+=1;
            }
            assert_eq!(Rule::operand, qc[i].0.rule);
            assert_eq!(op1[1..], qc[i].1);
            i+=1;
        }
        if ops > 1 {
            while qc[i].0.rule != Rule::operand {
                i+=1;
            }
            assert_eq!(Rule::operand, qc[i].0.rule);
            assert_eq!(op2[1..], qc[i].1);
        }
        if comment {
            while qc[i].0.rule != Rule::comment {
                i+=1;
            }
            assert_eq!(Rule::comment, qc[i].0.rule);
        }
    }
}