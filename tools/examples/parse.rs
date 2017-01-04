extern crate r68k_tools;
extern crate pest;

use r68k_tools::assembler::parser::Rdp;
use std::io::{BufReader, BufRead};
use std::fs::File;
use pest::{Parser, StringInput};
use std::env;

fn main() {
    let file = if let Some(asmfile) = env::args().nth(1) {
        File::open(asmfile.as_str()).expect("could not open file")
    } else {
        panic!("Provide the path to an assembler file as first argument, to have it parsed");
    };
    let reader = BufReader::new(&file);
    let mut correct = 0;
    let mut fail = 0;
    let mut unended = 0;
    let mut lines = 0;
    for (num, line) in reader.lines().enumerate() {
        let input = match line {
            Ok(text) => text,
            Err(ex) => {
                println!("errd:{:04}: {}", num+1, ex);
                continue;
            }
        };
        let mut parser = Rdp::new(StringInput::new(&input));
        if parser.statement() {
            if parser.end() {
                correct += 1;
            } else {
                unended += 1;
                let qc = parser.queue_with_captures();
                println!("!end:{:04}: {:80} : {:?} (expected {:?})", num+1, input, qc, parser.expected());
            };
        } else {
            fail += 1;
            println!("fail:{:04}: {}", num+1, input);
        };
        lines = num;
    }
    println!("= END = total lines {:04}: correct {}, failed {}, incomplete {}", lines+1, correct, fail, unended);
}
