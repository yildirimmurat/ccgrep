mod stream;

use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use regex::Regex;
use stream::{Stream, FileStream};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let args = &args[1..];

    let (regex, stream) = parse_args(&args);

    let reader: Box<dyn std::io::BufRead> = match stream {
        Stream::FileStream(file_stream) => {
            Box::new(BufReader::new(file_stream.file))
        }
        Stream::StdinStream(_stdin_stream) => {
            Box::new(std::io::stdin().lock())
        }
    };

    write_output(regex, reader).expect("Unexpected: Cannot write output");
}

fn parse_args(args: &[String]) -> (Option<Regex>, Stream) {
    let rg = args[0].clone();
    let regex = if !rg.is_empty() {
        match Regex::new(&rg) {
            Ok(r) => Some(r),
            Err(e) => {
                eprintln!("Invalid regex pattern: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        None
    };
    let filename = args[1].clone();

    let file_stream = FileStream::new(&filename);

    match file_stream {
        Ok(file_stream) => {
            let stream = Stream::FileStream(file_stream);

            (regex, stream)
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            std::process::exit(1);
        }
    }
}

fn write_output(regex: Option<Regex>, reader: Box<dyn std::io::BufRead>) -> io::Result<()> {
    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        lines.push(line);
    }

    for line in lines {
        if let Some(ref re) = regex {
            if re.is_match(&line) {
                // Try to write the result to stdout, and ignore BrokenPipe error
                if let Err(e) = writeln!(io::stdout(), "{}", line) {
                    return if e.kind() == ErrorKind::BrokenPipe {
                        Ok(()) // Gracefully exit if pipe is closed
                    } else {
                        Err(e) // Propagate other errors
                    }
                }
            }
        } else {
            // Try to write the result to stdout, and ignore BrokenPipe error
            if let Err(e) = writeln!(io::stdout(), "{}", line) {
                return if e.kind() == ErrorKind::BrokenPipe {
                    Ok(()) // Gracefully exit if pipe is closed
                } else {
                    Err(e) // Propagate other errors
                }
            }
        }
    }

    Ok(())
}
