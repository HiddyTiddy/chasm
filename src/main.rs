use chasm::parser::translator::{link, translate, LinkError, ParseError};
use clap::{Arg, Command, ValueHint};
use std::{
    collections::HashSet,
    fs::{self, File},
    io::Write,
    path::Path,
};

const HEADER_SIZE: usize = 0x0400usize;
fn write_instructions(path: &Path, instructions: Vec<u16>, debug_print: bool) {
    let mut buf: Vec<u8> = Vec::with_capacity(instructions.len() << 1);

    // header
    for (i, instr) in instructions.iter().enumerate() {
        if debug_print {
            println!("[{:4x}]  x{instr:04x}", i * 2 + HEADER_SIZE);
        }

        buf.push((instr & 0xff) as u8);
        buf.push(((instr >> 8) & 0xff) as u8);
    }

    let mut file = File::create(path).expect("create failed");
    file.write_all(&buf).expect("write failed");
}

macro_rules! exit_failure {
    () => {
        std::process::exit(-1)
    };
    ($msg:expr) => {
        eprintln!("{}", $msg);
        std::process::exit(-1)
    };
}

fn verify_includes<'a>(includes: Vec<&'a str>, main_name: &'a str) -> Vec<&'a str> {
    let mut seen = HashSet::new();
    let mut incl = vec![];
    for file in includes {
        if file == main_name {
            eprintln!("cannot include {main_name:?}, already main program");
            exit_failure!();
        }
        if !seen.contains(&file) {
            seen.insert(file);
            incl.push(file);
        }
    }
    incl
}

fn main() {
    let app = Command::new("chasm")
        .about("lc3 assembler")
        .author("hyde")
        .version("0.1.0")
        .arg(
            Arg::new("outfile")
                .value_name("FILENAME")
                .short('o')
                .takes_value(true)
                .value_hint(ValueHint::FilePath)
                .help("Write output to <filename>"),
        )
        .arg(
            Arg::new("input")
                .value_name("INPUT")
                .required(true)
                .takes_value(true)
                .value_hint(ValueHint::FilePath),
        )
        .arg(
            Arg::new("print-debug")
                .takes_value(false)
                .required(false)
                .long("print-debug"),
        )
        .arg(
            Arg::new("includes")
                .takes_value(true)
                .required(false)
                .multiple_values(true)
                .value_hint(ValueHint::FilePath)
                .short('I')
                .long("include")
                .value_name("FILENAMES"),
        );
    let matches = app.get_matches();
    let outfile = if let Some(outfile) = matches.value_of("outfile") {
        Path::new(outfile)
    } else {
        Path::new("a.out")
    };

    let infile = matches.value_of("input").unwrap();

    let includes = if let Some(includes) = matches.values_of("includes") {
        let includes: Vec<_> = includes.collect();
        verify_includes(includes, infile)
    } else {
        vec![]
    };

    // read main assembly file
    let text = fs::read_to_string(infile).expect("read failed");

    // parse main assembly file
    let mut translation = match translate(&text) {
        Ok(translation) => translation,
        Err(err) => {
            match err {
                ParseError::StatementSyntaxError(instruction, line_number) => {
                    eprintln!("{infile}:{line_number} syntax error while parsing {instruction:?}");
                }
                ParseError::LabelSyntaxError(label, line_number) => {
                    eprintln!("{infile}:{line_number} syntax error while parsing label {label:?}");
                }
                ParseError::UnexpectedToken(token, line_number) => {
                    eprintln!("{infile}:{line_number} unexpected token {token}");
                }
                ParseError::InvalidLocation(line_number) => {
                    eprintln!("{infile}:{line_number} cannot set location")
                }
            }
            exit_failure!();
        }
    };

    // parse included files
    for included in includes {
        let source_text = match fs::read_to_string(included) {
            Ok(source_text) => source_text,
            Err(_) => {
                eprintln!("couldnt read {included:?}");
                exit_failure!();
            }
        };

        let lib_translation = match translate(&source_text) {
            Ok(translation) => translation,
            Err(err) => {
                match err {
                    ParseError::StatementSyntaxError(instruction, line_number) => {
                        eprintln!(
                            "{infile}:{line_number} syntax error while parsing {instruction:?} in ${included:?}"
                        );
                    }
                    ParseError::LabelSyntaxError(label, line_number) => {
                        eprintln!(
                            "{infile}:{line_number} syntax error while parsing label {label:?} in ${included:?}"
                        );
                    }
                    ParseError::UnexpectedToken(token, line_number) => {
                        eprintln!(
                            "{infile}:{line_number} unexpected token {token} in ${included:?}"
                        );
                    }
                    ParseError::InvalidLocation(line_number) => {
                        eprintln!("{infile}:{line_number} cannot set location")
                    }
                }
                exit_failure!();
            }
        };

        translation.extend(lib_translation);
    }

    // link files (aka resolve addresses of labels)
    match link(translation) {
        Ok(instructions) => {
            write_instructions(outfile, instructions, matches.is_present("print-debug"));
        }
        Err(err) => {
            match err {
                LinkError::LabelNotResolvedError(label, line_number) => {
                    eprintln!(
                        "{infile}:{line_number} could not resolve label {label:?} during link"
                    )
                }
            }
            exit_failure!();
        }
    }
}
