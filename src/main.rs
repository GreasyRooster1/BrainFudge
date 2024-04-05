use std::fs::File;
use std::io::{Read, stdin, stdout, Write};
use std::ops::Deref;
use std::path::Path;
use clap::{arg, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filename: String,

    #[arg(short, long)]
    debug: Option<String>,
}

fn main() {
    let args = Args::parse();
    let file_contents = read_file(args.filename);
    let debug = parse_bool(args.debug);

    let max_memory:usize = 2^16;
    let mut buffer = vec![0u8; max_memory];
    let mut pointer:usize = 0;

    let mut i =0;
    let mut bracket_index = vec![];
    while i<file_contents.len() {
        let c = file_contents.as_bytes()[i] as char;
        match c {
            '>'=>pointer+=1,
            '<'=>pointer-=1,
            '+'=>{
                buffer[pointer]=buffer[pointer].wrapping_add(1);
            },
            '-'=>{
                buffer[pointer]=buffer[pointer].wrapping_sub(1);
            },
            '.'=>{
                print!("{}",buffer[pointer] as char);
                stdout().flush().unwrap();
            }
            ','=>{
                let mut line=String::new();
                stdin().read_line(&mut line).unwrap();
                buffer[pointer] = line.chars().next().unwrap() as u8;

            }
            '['=>{

                if buffer[pointer]!=0 {
                    bracket_index.append(&mut vec![i]);
                }
            }
            ']'=>{
                if buffer[pointer]!=0 {
                    i+=1;
                    continue;
                }
                match bracket_index.pop() {
                    None => {
                        println!("Found unmatched brackets!");
                        std::process::exit(0);
                    }
                    Some(index) => {
                        i=index;
                    }
                }
            }
            _ => {}
        }
        i+=1;
    }
}

fn parse_bool(arg:Option<String>)->bool{
    match arg.as_deref() {
        None => false,
        Some("true") =>true,
        Some("1") =>true,
        Some("yes") =>true,
        Some("on") =>true,
        Some(_) => false,
    }
}

fn read_file(p:String) ->String{
    let path = Path::new(&p);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => {
            println!("couldn't open {}: {}", display, why);
            std::process::exit(0);
        }
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => {
            println!("couldn't read {}: {}", display, why);
            std::process::exit(0);
        }
        Ok(_) => s,
    }
}