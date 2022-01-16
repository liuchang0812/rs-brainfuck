use core::panic;
use std::{env, fs::File, io::Read};

#[derive(Debug)]
#[derive(Clone)]
enum Token {
    IncPointer,
    DecPointer,
    Inc,
    Dec,
    Write,
    Read,
    LoopBegin,
    LoopEnd,
}

#[derive(Debug)]
#[derive(Clone)]
enum Instruction{
    IncPointer,
    DecPointer,
    Inc,
    Dec,
    Write,
    Read,
    Loop(Vec<Instruction>),
}

fn lex(source:String) -> Vec<Token> {
    let mut tokens = Vec::new();

    for ch in source.chars() {
        let token = match ch {
            '>' => Some(Token::IncPointer),
            '<' => Some(Token::DecPointer),
            '+' => Some(Token::Inc),
            '-' => Some(Token::Dec),
            '.' => Some(Token::Write),
            ',' => Some(Token::Read),
            '[' => Some(Token::LoopBegin),
            ']' => Some(Token::LoopEnd),
            _ => None
        };
        match token {
            Some(op) => tokens.push(op),
            None => ()
        }
    }
    tokens
}

fn parse(tokens: Vec<Token>) -> Vec<Instruction> {
    let mut program: Vec<Instruction> = Vec::new();
    let mut loop_stack = 0;
    let mut loop_start = 0;

    for (i, token) in tokens.iter().enumerate() {
        if loop_stack == 0 {
            // no loop begin token
            let op = match token {
                Token::IncPointer => Some(Instruction::IncPointer),
                Token::DecPointer => Some(Instruction::DecPointer),
                Token::Inc => Some(Instruction::Inc),
                Token::Dec => Some(Instruction::Dec),
                Token::Write => Some(Instruction::Write),
                Token::Read => Some(Instruction::Read),

                Token::LoopBegin => {
                    loop_start = i;
                    loop_stack += 1;
                    None
                },
                Token::LoopEnd => panic!("loop ending at {} has no beginning", i)
            };

            match op {
                Some(instr) => program.push(instr),
                None => ()
            }
        } else {
            match token {
                Token::LoopBegin => loop_stack += 1,
                Token::LoopEnd => {
                    loop_stack -= 1;
                    if loop_stack == 0 {
                        program.push(Instruction::Loop(parse(tokens[loop_start+1..i].to_vec())));
                    }
                },
                _ => (),
            }
        }
    }
    if loop_stack != 0 {
        panic!("loop that starts at #{} has no matching ending!", loop_start);
    }
    program
}
fn run(instructions: &Vec<Instruction>, tape: &mut Vec<u8>, data_pointer: &mut usize) {
    for instr in instructions{
        match instr {
            Instruction::IncPointer => *data_pointer += 1,
            Instruction::DecPointer => *data_pointer -= 1,

            Instruction::Inc => tape[*data_pointer] +=1 ,
            Instruction::Dec => tape[*data_pointer] -=1 ,

            &Instruction::Read => {
                let mut input: [u8; 1] = [0; 1];
                std::io::stdin().read_exact(&mut input).expect("read one char failed");
                tape[*data_pointer] = input[0];
            },

            Instruction::Write => {
                print!("{}", tape[*data_pointer] as char);
            },
            
            Instruction::Loop(nest_instructions) => {
                while tape[*data_pointer] != 0 {
                    run(&nest_instructions, tape, data_pointer);
                }
            }

        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(filename) = args.get(1) {
        println!("running {}", filename);

        let mut file = File::open(filename).expect("file not found");
        let mut source = String::new();
        file.read_to_string(&mut source).expect("read file to string");

        // lex
        let tokens = lex(source);
        println!("{:?}", tokens);

        // parse
        let program = parse(tokens);
        println!("{:?}", program);

        // run
        let mut tape: Vec<u8> = vec![0; 1024];
        let mut data_pointer = 512;
        run(&program, &mut tape, &mut data_pointer);
    } else {
        let cmd = &args[0];
        println!("usage: {} <file.bf>", cmd);
        std::process::exit(-1);
    }
}
