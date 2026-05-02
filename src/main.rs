use rs_grep::search;
use std::env;
use std::error;
use std::fs;

struct Config {
    query: String,
    contents: String,
}

fn parse_args(mut argv: env::Args) -> Result<Config, &'static str> {
    argv.next();
    let query = match argv.next() {
        Some(arg) => arg,
        None => return Err("Usage: grep-rs <query> <content_file>"),
    };
    
    let contents = match argv.next() {
        Some(arg) => arg,
        None => return Err("Usage: grep-rs <query> <content_file>"),
    };

    Ok(Config { query, contents })
}

fn run(conf: Config) -> Result<(), Box<dyn error::Error>> {
    let dump = fs::read_to_string(conf.contents)?;
    let hits = search(&conf.query, &dump);
    for hit in hits {
        println!("{hit}");
    }
    Ok(())
}

fn main() {
    let conf = parse_args(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    if let Err(err) = run(conf) {
        eprintln!("grep-rs error: {err}");
        std::process::exit(1);
    }
}
