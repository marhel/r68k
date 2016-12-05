use pest::prelude::*;

impl_rdp! {
    grammar! {
        instruction = { label? ~ mnemonic ~ operands? ~ comment? }
        mnemonic = @{ letter+ ~ qualifier? }
        qualifier = { longsize | wordsize | bytesize }
        longsize = { [".L"] | [".l"] }
        wordsize = { [".W"] | [".w"] }
        bytesize = { [".B"] | [".b"] }
        comment = { [";"] ~ any* ~ eoi }
        operands = _{ operand ~ ([","] ~ operand)* }
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
        imm = { ["#"] ~ number}

        displacement = _{ number }
        number = { hex | bin | dec | oct}
        hex = @{ ["$"] ~ ["-"]? ~ (['0'..'9'] | ['A'..'F'] | ['a'..'f'])+ }
        bin = @{ ["%"] ~ (['0'..'1'])+ }
        oct = @{ ["@"] ~ (['0'..'7'])+ }
        dec = @{ ["-"]? ~ ['0'..'9']+ }

        label = @{ name ~ [":"] }
        letter = _{ ['A'..'Z'] | ['a'..'z'] }
        digit = _{ ['0'..'9'] }
        name = @{ letter ~ (letter | digit)* }

        whitespace = _{ [" "] | ["\t"] }
    }
}

#[cfg(test)]
mod tests {
    use super::{Rdp, Rule};
    use pest::prelude::*;
    extern crate rand;

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
            9 => "?z".to_string(),
            10 => "?(z,PC)".to_string(),
            11 => "?(z,PC,Dy)".to_string(),
            12 => "?(z,PC,Ay)".to_string(),
            _ => "?#z".to_string(),
        };
        op.replace("?", if first {" "} else {","})
        .replace("x", (self::rand::random::<u8>() % 8).to_string().as_str())
        .replace("y", (self::rand::random::<u8>() % 8).to_string().as_str())
        .replace("z", random_num().as_str())
    }
    fn parse(mnemonic: &str, ops: u8) {
        match ops {
            0 =>
                parse_ops(mnemonic, ops, "", ""),
            1 =>
                for o1 in 1..14 {
                    parse_ops(mnemonic, ops, operand(o1, true).as_str(), "");
                },
            _ =>
                for o1 in 1..14 {
                    for o2 in 1..14 {
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
        assert!(parser.instruction());
        if !parser.end() {
            println!("input: {:?}", input);
            println!("queue: {:?}", parser.queue());
            println!("expected {:?}", parser.expected());
        }
        assert!(parser.end());
        let qc = parser.queue_with_captures();
        let mut i = 0;
        assert_eq!(Rule::instruction, qc[i].0.rule);
        if label {
            i+=1;
            assert_eq!(Rule::label, qc[i].0.rule);
            i+=1;
        }
        i+=1;
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