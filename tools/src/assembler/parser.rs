use pest::prelude::*;
use super::super::{OpcodeInstance, Size};
use operand::Operand;
impl_rdp! {
    grammar! {
        statement = { something ~ asm_comment ~ eoi | asm_comment }
        something = _{ declaration | label? ~ (directive|instruction) | just_label }
        declaration = { symbol ~ (["="] | [i"equ"] | [i".equ"] ) ~ expression ~ asm_comment? }
        directive = { align | dc | dcb | ds | end_asm | even | odd | offset | org }
        just_label = @{ label ~ whitespaces? ~ asm_comment?  }
        // assembler directives
        align = { [i"align"] ~ expression }
        dc = { qual_dc ~ expressions }
        dcb = { qual_dcb ~ expression ~ (comma ~ expression)? }
        ds = { qual_ds ~ expression }
        qual_dc = @{ [i"dc"] ~ qualifier }
        qual_dcb = @{ [i"dcb"] ~ qualifier }
        qual_ds = @{ [i"ds"] ~ qualifier }
        end_asm = { [i"end"] ~ expression? }
        even = { [i"even"] }
        odd = { [i"odd"] }
        offset = { [i"offset"] ~ expression }
        org = { [i"org"] ~ expression }

        expressions = { expression ~ (comma ~ expression)* }
        expression = _{
            // precedence climbing, lowest to highest
            { (negate | complement)? ~ (["("] ~ expression ~ [")"] | symbol | number | quoted_string) }
            add = {  add_op  | sub_op }
            mul = {  mul_op | div_op | mod_op }
            ior = {  bitwise_ior_op }
            xor = {  bitwise_xor_op }
            and = {  bitwise_and_op }
            shift = {  shift_left_op | shift_right_op }
            // how to deal with unary ops?
            // compl = {  complement_op }
            // power          = {< pow } // < for right-associativity
        }
        negate = { ["-"] }
        complement = { ["~"] }
        add_op = { ["+"] }
        sub_op = { ["-"] }
        mul_op = { ["*"] }
        div_op = { ["/"] }
        mod_op = { ["%"] }
        shift_left_op  = { ["<<"] }
        shift_right_op = { [">>"] }
        bitwise_ior_op = { ["|"] }
        bitwise_xor_op = { ["^"] }
        bitwise_and_op = { ["&"] }
        complement_op  = { ["~"] }

        quoted_string = @{ ["\""] ~ (letter|digit| !["\""] ~ any )* ~ ["\""] | ["'"] ~ (letter|digit|!["'"] ~ any)* ~ ["'"] }
        // chr = {["!"]|["#"]|["$"]|["%"]|["&"]|["/"]|["("]|[")"]|["="]|["?"]|["*"]|[","]|["."]|[":"]|[";"]|["+"]|["-"]|["_"]|["<"]|[">"]|["["]|["]"]|["{"]|["}"]}
        instruction = _{ mnemonic ~ operands? }
        mnemonic = @{ name ~ qualifier? }
        qualifier = _{ longsize | wordsize | bytesize | short }
        longsize = { [i".L"] }
        wordsize = { [i".W"] }
        bytesize = { [i".B"] }
        short = { [i".S"] }
        asm_comment = @{ whitespaces? ~ ([";"] ~ any*)? }
        operands = { operand ~ (comma ~ operand)* }
        comma = {[","]}
        symbol = _{ name }
        operand = { drd | ard | api | apd | ari | pci | pcd | aix | adi | imm | abs  }

        // addressing modes
        drd = @{ [i"D"] ~ ['0'..'7'] ~ qualifier? ~ !letter}
        ard = @{ [i"A"] ~ ['0'..'7'] ~ qualifier? | [i"SP"] ~ qualifier? ~ !letter}
        ari = { ["("] ~ ard ~ [")"] }
        api = { ["("] ~ ard ~ [")"] ~ ["+"] }
        apd = { ["-"] ~["("] ~ ard ~ [")"] }
        adi = { ["("] ~ expression ~ [","] ~ ard ~ [")"] | expression ~ ["("] ~ ard ~ [")"] }
        aix = { ["("] ~ (expression ~ [","])? ~ ard ~ [","] ~ (drd | ard) ~ [")"] | expression? ~ ["("] ~ ard ~ [","] ~ (drd | ard) ~ [")"]}
        abs = @{ expression ~ qualifier? }
        pcd = { ["("] ~ (expression ~ [","])? ~ [i"PC"] ~ [")"] | expression? ~ ["("] ~ [i"PC"] ~ [")"]}
        pci = { ["("] ~ (expression ~ [","])? ~ [i"PC"] ~ [","] ~ (drd | ard) ~ [")"] | expression? ~ ["("] ~ [i"PC"] ~ [","] ~ (drd | ard) ~ [")"] }
        imm = @{ ["#"] ~ expression ~ qualifier? }

        number = { hex | bin | dec | oct}
        hex = @{ ["$"] ~ (['0'..'9'] | ['A'..'F'] | ['a'..'f'])+ }
        bin = @{ ["%"] ~ (['0'..'1'])+ }
        oct = @{ ["@"] ~ (['0'..'7'])+ }
        dec = @{ ['0'..'9']+ }

        label = @{ soi ~ name ~ [":"]? | whitespaces ~ name ~ [":"]}
        letter = _{ ['A'..'Z'] | ['a'..'z'] | ["_"] }
        digit = _{ ['0'..'9'] }
        name = @{ (letter | ["."]) ~ (letter | digit)* }
        whitespaces = @{ ([" "] | ["\t"])+ }
        whitespace = _{ [" "] | ["\t"] }
    }

    process! {
        process_instruction(&self) -> OpcodeInstance<'input> {
            (_: mnemonic, &mnemonic: name, _: bytesize, operands: process_operands()) => {
                OpcodeInstance {
                    mnemonic: mnemonic,
                    size: Size::Byte,
                    operands: operands,
                }
            },
            (_: mnemonic, &mnemonic: name, _: wordsize, operands: process_operands()) => {
                OpcodeInstance {
                    mnemonic: mnemonic,
                    size: Size::Word,
                    operands: operands,
                }
            },
            (_: mnemonic, &mnemonic: name, _: longsize, operands: process_operands()) => {
                OpcodeInstance {
                    mnemonic: mnemonic,
                    size: Size::Long,
                    operands: operands,
                }
            },
            (_: mnemonic, &mnemonic: name, operands: process_operands()) => {
                OpcodeInstance {
                    mnemonic: mnemonic,
                    size: Size::Word,
                    operands: operands,
                }
            },
        }
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
            (_: operand, _: adi, expression: process_expression(), &reg: ard) => {
                Operand::AddressRegisterIndirectWithDisplacement(reg[1..].parse().unwrap(), expression.eval().unwrap() as i16)
            },
            (_: operand, _: aix, expression: process_expression(), &reg: ard, &ireg: ard) => {
                Operand::AddressRegisterIndirectWithIndex(reg[1..].parse().unwrap(), 8u8+ireg[1..].parse::<u8>().unwrap(), expression.eval().unwrap() as i8)
            },
            (_: operand, _: aix, expression: process_expression(), &reg: ard, &ireg: drd) => {
                Operand::AddressRegisterIndirectWithIndex(reg[1..].parse().unwrap(), ireg[1..].parse().unwrap(), expression.eval().unwrap() as i8)
            },
            (_: operand, _: pcd, expression: process_expression()) => {
                Operand::PcWithDisplacement(expression.eval().unwrap() as i16)
            },
            (_: operand, _: pci, expression: process_expression(), &ireg: ard) => {
                Operand::PcWithIndex(8u8+ireg[1..].parse::<u8>().unwrap(), expression.eval().unwrap() as i8)
            },
            (_: operand, _: pci, expression: process_expression(), &ireg: drd) => {
                Operand::PcWithIndex(ireg[1..].parse().unwrap(), expression.eval().unwrap() as i8)
            },
            (_: operand, _: abs, expression: process_expression(), _: bytesize) => {
                Operand::AbsoluteWord(expression.eval().unwrap() as u16)
            },
            (_: operand, _: abs, expression: process_expression(), _: wordsize) => {
                Operand::AbsoluteWord(expression.eval().unwrap() as u16)
            },
            (_: operand, _: abs, expression: process_expression(), _: longsize) => {
                Operand::AbsoluteLong(expression.eval().unwrap() as u32)
            },
            (_: operand, _: abs, expression: process_expression()) => {
                Operand::AbsoluteWord(expression.eval().unwrap() as u16)
            },
            (_: operand, _: imm, expression: process_expression(), _: bytesize) => {
                Operand::Immediate(Size::Byte, expression.eval().unwrap() as u32)
            },
            (_: operand, _: imm, expression: process_expression(), _: wordsize) => {
                Operand::Immediate(Size::Word, expression.eval().unwrap() as u32)
            },
            (_: operand, _: imm, expression: process_expression(), _: longsize) => {
                Operand::Immediate(Size::Long, expression.eval().unwrap() as u32)
            },
            (_: operand, _: imm, expression: process_expression()) => {
                Operand::Immediate(Size::Unsized, expression.eval().unwrap() as u32)
            },
        }

        process_number(&self) -> i32 {
            (&dec: dec) => {
                dec.parse().unwrap()
            },
            (&hex: hex) => {
                i32::from_str_radix(&hex[1..], 16).unwrap()
            },
            (&oct: oct) => {
                i32::from_str_radix(&oct[1..], 8).unwrap()
            },
            (&bin: bin) => {
                i32::from_str_radix(&bin[1..], 2).unwrap()
            },
        }

        process_expression(&self) -> Expr {
            (_: number, num: process_number()) => {
                Expr::Num(num)
            },
            (&name: name) => {
                Expr::Sym(name.to_owned())
            },
            (_: complement, right: process_expression()) => {
                Expr::Cpl(Box::new(right))
            },
            (_: negate, right: process_expression()) => {
                Expr::Neg(Box::new(right))
            },
            (_: add, left: process_expression(), op, right: process_expression()) => {
                match op.rule {
                   Rule::add_op => Expr::Add(Box::new(left), Box::new(right)),
                   Rule::sub_op => Expr::Sub(Box::new(left), Box::new(right)),
                    _ => unreachable!()
                }
            },
            (_: mul, left: process_expression(), op, right: process_expression()) => {
                match op.rule {
                    Rule::mul_op => Expr::Mul(Box::new(left), Box::new(right)),
                    Rule::div_op => Expr::Div(Box::new(left), Box::new(right)),
                    Rule::mod_op => Expr::Mod(Box::new(left), Box::new(right)),
                    _ => unreachable!()
                }
            },
            (_: ior, left: process_expression(), op, right: process_expression()) => {
                match op.rule {
                    Rule::bitwise_ior_op => Expr::Ior(Box::new(left), Box::new(right)),
                    _ => unreachable!()
                }
            },
            (_: xor, left: process_expression(), op, right: process_expression()) => {
                match op.rule {
                    Rule::bitwise_xor_op => Expr::Xor(Box::new(left), Box::new(right)),
                    _ => unreachable!()
                }
            },
            (_: and, left: process_expression(), op, right: process_expression()) => {
                match op.rule {
                    Rule::bitwise_and_op => Expr::And(Box::new(left), Box::new(right)),
                    _ => unreachable!()
                }
            },
            (_: shift, left: process_expression(), op, right: process_expression()) => {
                match op.rule {
                    Rule::shift_left_op => Expr::Shl(Box::new(left), Box::new(right)),
                    Rule::shift_right_op => Expr::Shr(Box::new(left), Box::new(right)),
                    _ => unreachable!()
                }
            },
            () => {
                Expr::Num(0)
            }
        }
    }
}


#[derive(Debug, PartialEq)]
pub enum Expr {
    Num(i32),
    Sym(String),
    Neg(Box<Expr>),
    Cpl(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    Ior(Box<Expr>, Box<Expr>),
    Xor(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Shl(Box<Expr>, Box<Expr>),
    Shr(Box<Expr>, Box<Expr>),
}
impl Expr {
    fn eval(&self) -> Option<i32> {
        match *self {
            Expr::Num(n) => Some(n),
            Expr::Sym(_) => None,
            Expr::Neg(ref right) => right.eval().map(|lv| -lv),
            Expr::Cpl(ref right) => right.eval().map(|lv| !lv),
            Expr::Add(ref left, ref right) => left.eval().and_then(|lv| right.eval().and_then(|rv| Some(lv + rv))),
            Expr::Sub(ref left, ref right) => left.eval().and_then(|lv| right.eval().and_then(|rv| Some(lv - rv))),
            Expr::Mul(ref left, ref right) => left.eval().and_then(|lv| right.eval().and_then(|rv| Some(lv * rv))),
            Expr::Div(ref left, ref right) => left.eval().and_then(|lv| right.eval().and_then(|rv| Some(lv / rv))),
            Expr::Mod(ref left, ref right) => left.eval().and_then(|lv| right.eval().and_then(|rv| Some(lv % rv))),
            Expr::Ior(ref left, ref right) => left.eval().and_then(|lv| right.eval().and_then(|rv| Some(lv | rv))),
            Expr::Xor(ref left, ref right) => left.eval().and_then(|lv| right.eval().and_then(|rv| Some(lv ^ rv))),
            Expr::And(ref left, ref right) => left.eval().and_then(|lv| right.eval().and_then(|rv| Some(lv & rv))),
            Expr::Shl(ref left, ref right) => left.eval().and_then(|lv| right.eval().and_then(|rv| Some(lv << rv))),
            Expr::Shr(ref left, ref right) => left.eval().and_then(|lv| right.eval().and_then(|rv| Some(lv >> rv))),
        }
    }
    fn resolve(&self, name: &str, value: i32) -> Expr {
        match *self {
            Expr::Neg(ref right) => {
                let res = Expr::Neg(Box::new(right.resolve(name, value)));
                if let Some(num) = res.eval() {
                    Expr::Num(num)
                } else {
                    res
                }
            },
            Expr::Cpl(ref right) => {
                let res = Expr::Cpl(Box::new(right.resolve(name, value)));
                if let Some(num) = res.eval() {
                    Expr::Num(num)
                } else {
                    res
                }
            },
            Expr::Add(ref left, ref right) => {
                let res = Expr::Add(Box::new(left.resolve(name, value)), Box::new(right.resolve(name, value)));
                if let Some(num) = res.eval() {
                    Expr::Num(num)
                } else {
                    res
                }
            },
            Expr::Sub(ref left, ref right) => {
                let res = Expr::Sub(Box::new(left.resolve(name, value)), Box::new(right.resolve(name, value)));
                if let Some(num) = res.eval() {
                    Expr::Num(num)
                } else {
                    res
                }
            },
            Expr::Mul(ref left, ref right) => {
                let res = Expr::Mul(Box::new(left.resolve(name, value)), Box::new(right.resolve(name, value)));
                if let Some(num) = res.eval() {
                    Expr::Num(num)
                } else {
                    res
                }
            },
            Expr::Div(ref left, ref right) => {
                let res = Expr::Div(Box::new(left.resolve(name, value)), Box::new(right.resolve(name, value)));
                if let Some(num) = res.eval() {
                    Expr::Num(num)
                } else {
                    res
                }
            },
            Expr::Mod(ref left, ref right) => {
                let res = Expr::Mod(Box::new(left.resolve(name, value)), Box::new(right.resolve(name, value)));
                if let Some(num) = res.eval() {
                    Expr::Num(num)
                } else {
                    res
                }
            },
            Expr::And(ref left, ref right) => {
                let res = Expr::And(Box::new(left.resolve(name, value)), Box::new(right.resolve(name, value)));
                if let Some(num) = res.eval() {
                    Expr::Num(num)
                } else {
                    res
                }
            },
            Expr::Ior(ref left, ref right) => {
                let res = Expr::Ior(Box::new(left.resolve(name, value)), Box::new(right.resolve(name, value)));
                if let Some(num) = res.eval() {
                    Expr::Num(num)
                } else {
                    res
                }
            },
            Expr::Xor(ref left, ref right) => {
                let res = Expr::Xor(Box::new(left.resolve(name, value)), Box::new(right.resolve(name, value)));
                if let Some(num) = res.eval() {
                    Expr::Num(num)
                } else {
                    res
                }
            },
            Expr::Shl(ref left, ref right) => {
                let res = Expr::Shl(Box::new(left.resolve(name, value)), Box::new(right.resolve(name, value)));
                if let Some(num) = res.eval() {
                    Expr::Num(num)
                } else {
                    res
                }
            },
            Expr::Shr(ref left, ref right) => {
                let res = Expr::Shr(Box::new(left.resolve(name, value)), Box::new(right.resolve(name, value)));
                if let Some(num) = res.eval() {
                    Expr::Num(num)
                } else {
                    res
                }
            },
            Expr::Sym(ref symbol) if symbol == name => Expr::Num(value),
            Expr::Sym(ref symbol) => Expr::Sym(symbol.clone()),
            Expr::Num(n) => Expr::Num(n),
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
        process_operand("%10(A7)", &Operand::AddressRegisterIndirectWithDisplacement(7, 2));
        process_operand("(%10,A7)", &Operand::AddressRegisterIndirectWithDisplacement(7, 2));
    }
    #[test]
    fn test_aix_operand() {
        process_operand("( 10,A0,D0)", &Operand::AddressRegisterIndirectWithIndex(0, 0, 10));
        process_operand("(A0,A1)", &Operand::AddressRegisterIndirectWithIndex(0, 9, 0));
        process_operand("($10,A0,A1)", &Operand::AddressRegisterIndirectWithIndex(0, 9, 16));
        process_operand("$10(A0,A1)", &Operand::AddressRegisterIndirectWithIndex(0, 9, 16));
        process_operand("(%10,A7,D7)", &Operand::AddressRegisterIndirectWithIndex(7, 7, 2));
        process_operand("(@10,A7,A6)", &Operand::AddressRegisterIndirectWithIndex(7, 14, 8));
    }
    #[test]
    fn test_abs_operand() {
        process_operand("100", &Operand::AbsoluteWord(100));
        process_operand("$100.B", &Operand::AbsoluteWord(256));
        process_operand("@100.W", &Operand::AbsoluteWord(64));
        process_operand("%100.L", &Operand::AbsoluteLong(4));
        process_operand("-100", &Operand::AbsoluteWord(-100 as i16 as u16));
        process_operand("-$100.B", &Operand::AbsoluteWord(-256 as i16 as u16));
        process_operand("-@100.W", &Operand::AbsoluteWord(-64 as i16 as u16));
        process_operand("-%100.L", &Operand::AbsoluteLong(-4 as i32 as u32));
    }
    #[test]
    fn test_pcd_operand() {
        process_operand("(-10,PC)", &Operand::PcWithDisplacement(-10));
        process_operand("-10(PC)", &Operand::PcWithDisplacement(-10));
        process_operand("(PC)", &Operand::PcWithDisplacement(0));
    }
    #[test]
    fn test_pci_operand() {
        process_operand("(10,PC,D0)", &Operand::PcWithIndex(0, 10));
        process_operand("10(PC,D0)", &Operand::PcWithIndex(0, 10));
        process_operand("(PC,D0)", &Operand::PcWithIndex(0, 0));
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
    fn test_different_operands() {
        process_operands("%111.B,(A7)", &vec![Operand::AbsoluteWord(7), Operand::AddressRegisterIndirect(7)]);
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
        let with_comment = format!(" {}{}{};or a comment", mnemonic, op1, op2);
        parse_with(with_comment.as_str(), mnemonic, ops, op1, op2, false, true);
        let with_both_comment_and_label = format!("label: {}{}{} ;and a comment", mnemonic, op1, op2);
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
            while qc[i].0.rule != Rule::asm_comment {
                i+=1;
            }
            assert_eq!(Rule::asm_comment, qc[i].0.rule);
        }
    }

    #[test]
    fn test_instruction() {
        process_instruction("ADD #%111,(A7)", &OpcodeInstance {
            mnemonic: "ADD",
            size: Size::Word,
            operands: vec![Operand::Immediate(Size::Unsized, 7), Operand::AddressRegisterIndirect(7)]
        });
        process_instruction("MUL.L\t-( A0 ), ( 8 , PC )", &OpcodeInstance {
            mnemonic: "MUL",
            size: Size::Long,
            operands: vec![Operand::AddressRegisterIndirectWithPredecrement(0), Operand::PcWithDisplacement(8)]
        });
        process_instruction("WARPSPEED.B D0,D1,D2,D3,D4", &OpcodeInstance {
            mnemonic: "WARPSPEED",
            size: Size::Byte,
            operands: (0..5).map(|i|Operand::DataRegisterDirect(i)).collect::<Vec<Operand>>()
        });
    }

    fn process_instruction(input: &str, expected: &OpcodeInstance) {
        let mut parser = Rdp::new(StringInput::new(input));
        if !parser.instruction() || !parser.end() {
            let qc = parser.queue_with_captures();
            panic!("{} => {:?}", input, qc);
        }
        let qc = parser.queue_with_captures();
        println!("{} => {:?}", input.trim(), qc);
        assert_eq!(*expected, parser.process_instruction());
    }

    use std::io::BufReader;
    use std::io::BufRead;
    use std::fs::File;

    #[test]
    fn just_a_comment() {
        process_statement(";just a comment");
        process_statement("; just a comment");
        process_statement(" ; just a comment ");
    }

    #[test]
    fn just_whitespace() {
        process_statement("");
        process_statement(" \t ");
    }
    #[test]
    fn just_strange() {
        process_statement("             subq    #1,d0   ");
    }
    #[test]
    fn just_a_label_possibly_with_comment() {
        process_statement("lab_101:");
        process_statement("lab_102");
        process_statement(" lab_103:");
        process_statement(" lab_104: ");
        process_statement("lab_105 ");
        process_statement(".lab_101:");
        process_statement(".lab_102");
        process_statement(" .lab_103:");
        process_statement(" .lab_104: ");
        process_statement(".lab_105 ");
        process_statement("LAB_101: \t ; just a comment ");
        process_statement("LAB_102 \t ; just a comment ");
        process_statement(" LAB_103: \t ; just a comment ");
        process_statement(" LAB_104:  \t ; just a comment ");
        process_statement("LAB_105  \t ; just a comment ");
        process_statement(".LAB_101: \t ; just a comment ");
        process_statement(".LAB_102 \t ; just a comment ");
        process_statement(" .LAB_103: \t ; just a comment ");
        process_statement(" .LAB_104:  \t ; just a comment ");
        process_statement(".LAB_105  \t ; just a comment ");
    }

    #[test]
    fn process_whitespaces() {
        let input = " \t ";
        let mut parser = Rdp::new(StringInput::new(input));
        if !parser.whitespaces() || !parser.end() {
            let qc = parser.queue_with_captures();
            panic!("{} => {:?}", input, qc);
        }
    }

    fn process_statement(input: &str) {
        let mut parser = Rdp::new(StringInput::new(input));
        if !parser.statement() || !parser.end() {
            let qc = parser.queue_with_captures();
            panic!("{} => {:?}", input, qc);
        }
    }

    use super::Expr;

    #[test]
    fn symbolic_expr_evaluates_to_none() {
        // loop * (5 + 4)
        let res = Expr::Mul(
            Box::new(Expr::Sym("loop".to_owned())),
            Box::new(Expr::Add(
                Box::new(Expr::Num(5)),
                Box::new(Expr::Num(4)))));
        let evaluated = res.eval();
        println!("{:?} = {:?}", res, evaluated);
        assert_eq!(None, evaluated);
    }
    #[test]
    fn nonsymbolic_expr_evaluates_to_some() {
        // 11 * (5 + 4)
        let res = Expr::Mul(
            Box::new(Expr::Num(11)),
            Box::new(Expr::Add(
                Box::new(Expr::Num(5)),
                Box::new(Expr::Num(4)))));
        let evaluated = res.eval();
        println!("{:?} = {:?}", res, evaluated);
        assert_eq!(Some(99), evaluated);
    }
    #[test]
    fn symbolic_expr_can_be_resolve_with_other_symbol_and_remains_symbolic() {
        // loop * (5 + 4)
        let res = Expr::Mul(
            Box::new(Expr::Sym("loop".to_owned())),
            Box::new(Expr::Add(
                Box::new(Expr::Num(5)),
                Box::new(Expr::Num(4)))));
        let resolved = res.resolve("other", 42);
        let evaluated = resolved.eval();
        println!("{:?} => {:?} = {:?}", res, resolved, evaluated);
        let expected = Expr::Mul(
            Box::new(Expr::Sym("loop".to_owned())),
            Box::new(Expr::Num(9)));
        assert_eq!(expected, resolved);
        assert_eq!(None, evaluated);
    }
    #[test]
    fn symbolic_expr_can_be_resolved_and_becomes_nonsymbolic() {
        // (5 + loop) * 11
        let res = Expr::Mul(
            Box::new(Expr::Add(
                Box::new(Expr::Num(5)),
                Box::new(Expr::Sym("loop".to_owned())))),
            Box::new(Expr::Num(11)));
        let resolved = res.resolve("loop", 4);
        let evaluated = resolved.eval();
        println!("{:?} => {:?} = {:?}", res, resolved, evaluated);
        assert_eq!(Some(99), evaluated);
        assert_eq!(Expr::Num(99), resolved);
    }
    #[test]
    fn complement_symbol() {
        let input = "1 + ~length";
        let expected = Expr::Add(
            Box::new(Expr::Num(1)),
            Box::new(Expr::Cpl(
                Box::new(Expr::Sym("length".to_owned())))));
        process_expression(input, expected);
    }
    #[test]
    fn complement_symbol_first() {
        let input = "~length";
        let expected = Expr::Cpl(
                Box::new(Expr::Sym("length".to_owned())));
        process_expression(input, expected);
    }
    #[test]
    fn complement_number() {
        let input = "1 + ~42";
        let expected = Expr::Add(
            Box::new(Expr::Num(1)),
            Box::new(Expr::Cpl(
                Box::new(Expr::Num(42)))));
        process_expression(input, expected);
    }

    #[test]
    fn negate_symbol() {
        let input = "1 + -length";
        let expected = Expr::Add(
            Box::new(Expr::Num(1)),
            Box::new(Expr::Neg(
                Box::new(Expr::Sym("length".to_owned())))));
        process_expression(input, expected);
    }

    #[test]
    fn complement_expression() {
        let input = "~(1 + 2)";
        let expected = Expr::Cpl(
            Box::new(Expr::Add(
                Box::new(Expr::Num(1)),
                Box::new(Expr::Num(2)))));
        process_expression(input, expected);
    }

    #[test]
    fn negate_expression() {
        let input = "-(1 + 2)";
        let expected = Expr::Neg(
            Box::new(Expr::Add(
                Box::new(Expr::Num(1)),
                Box::new(Expr::Num(2)))));
        process_expression(input, expected);
    }

    #[test]
    fn compound_expressions() {
        let input = "40>>2 & (-11 + length<<2)/2";
        let expected = Expr::Div(
            Box::new(Expr::And(
                Box::new(Expr::Shr(
                    Box::new(Expr::Num(40)),
                    Box::new(Expr::Num(2)))),
                Box::new(Expr::Add(
                    Box::new(Expr::Neg(
                        Box::new(Expr::Num(11)))),
                    Box::new(Expr::Shl(
                        Box::new(Expr::Sym("length".to_owned())),
                        Box::new(Expr::Num(2)))))))),
            Box::new(Expr::Num(2)));
        process_expression(input, expected);
    }

    fn process_expression(input: &str, expected: Expr) {
        let mut parser = Rdp::new(StringInput::new(input));
        assert!(parser.expression());
        if !parser.end() {
            println!("input: {:?}", input);
            println!("queue: {:?}", parser.queue());
            println!("expected {:?}", parser.expected());
        }
        assert!(parser.end());
        let result = parser.process_expression();
        let qc = parser.queue_with_captures();
        println!("qc: {:?}", qc);
        assert_eq!(expected, result);
        println!("{} => {:?}", input, result);
    }

    #[test]
    fn expression_results_seem_correct() {
        calculate("1+2", 3);
        calculate("-10", -10);
        calculate("-1+2", 1);
        calculate("1+-2", -1);
        calculate("1+2*3", 7);
        calculate("1-2", -1);
        calculate("1-2*3", -5);
        calculate("2*3", 6);
        calculate("2*3-1", 5);
        calculate("6/3", 2);
        calculate("6/3+1", 3);
        calculate("6/(3+1)", 1);
        calculate("6%4", 2);
        calculate("6%4*8/2", 8);
        calculate("6%%100", 2); // 6 mod binary 4
        calculate("%111&%101", 0b101);
        calculate("%110|%011", 0b111);
        calculate("%110^%011", 0b101);
        calculate("%110<<1", 0b1100);
        calculate("2*$c+%110<<1", 36);
        calculate("%110>>1", 0b11);
        calculate("~%1101", !13);
        calculate("-%1101", -13);
    }

    fn calculate(input: &str, expected: i32) {
        let mut parser = Rdp::new(StringInput::new(input));
        assert!(parser.expression());
        if !parser.end() {
            println!("input: {:?}", input);
            println!("queue: {:?}", parser.queue());
            println!("expected {:?}", parser.expected());
        }
        let result = parser.process_expression();
        match result.eval() {
            Some(actual) => if expected != actual {
              panic!("{} => {} but expected {}", input, actual, expected);
            },
            None => panic!("{} => None but expected {}", input, expected),
        }
    }

    #[test]
    fn declaration_parsing() {
        process_declaration("answer  equ 42 * life & universe", Rule::name);
        process_declaration("answer  .equ 42 * life & universe", Rule::name);
        process_declaration("answer = 42 * life & universe", Rule::name);
    }
    fn process_declaration(input: &str, expected: Rule) {
        let mut parser = Rdp::new(StringInput::new(input));
        assert!(parser.statement());
        if !parser.end() {
            println!("input: {:?}", input);
            println!("queue: {:?}", parser.queue());
            println!("expected {:?}", parser.expected());
        }
        let qc = parser.queue_with_captures();
        println!("qc: {:?}", qc);
        assert_eq!(Rule::statement, qc[0].0.rule);
        assert_eq!(Rule::declaration, qc[1].0.rule);
        assert_eq!(expected, qc[2].0.rule);
    }
    #[test]
    fn directive_parsing() {
        // directive = { align | dc | dcb | ds | end_asm | even | odd | offset | org }
        process_directive(" align 4", Rule::align);
        process_directive(" dc.b $A,$B,$C,'STUFF'", Rule::dc);
        process_directive(" dcb.w $1000", Rule::dcb);
        process_directive(" dcb.w $1000,$FFFF", Rule::dcb);
        process_directive(" ds.l $1000", Rule::ds);
        process_directive(" end", Rule::end_asm);
        process_directive(" end start", Rule::end_asm);
        process_directive(" even", Rule::even);
        process_directive(" odd", Rule::odd);
        process_directive(" offset 0", Rule::offset);
        process_directive(" org $2000", Rule::org);
    }
    fn process_directive(input: &str, expected: Rule) {
        let mut parser = Rdp::new(StringInput::new(input));
        assert!(parser.statement());
        if !parser.end() {
            println!("input: {:?}", input);
            println!("queue: {:?}", parser.queue());
            println!("expected {:?}", parser.expected());
        }
        let qc = parser.queue_with_captures();
        println!("qc: {:?}", qc);
        assert_eq!(Rule::statement, qc[0].0.rule);
        assert_eq!(Rule::directive, qc[1].0.rule);
        assert_eq!(expected, qc[2].0.rule);
    }
}