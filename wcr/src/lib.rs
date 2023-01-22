use std::{error::Error}; 
use std::fs::File;
use std::io::{BufReader, BufRead, Read};
use clap::{Args, Parser, Subcommand, ValueEnum};

type WcrResult<T> = Result<T, Box<dyn Error>>;


struct FileCounts {
    lines: usize,
    words: usize,
    chars: usize,
    bytes: usize,
}

impl FileCounts {
    fn new(lines: usize, words: usize, chars: usize, bytes: usize) -> Self {
        FileCounts { lines, words, chars, bytes, }
    }
}

impl Default for FileCounts {
    fn default() -> Self {
        FileCounts { lines: 0, words: 0, chars: 0, bytes: 0 }
    }
}

/// WCRConfig的参数字段中我们没有使用`Option`， 
/// 是因为wc解析参数是预设了参数的优先级的
/// 用`Option`不如用bool运算好处理。
#[derive(Debug, Parser)]
#[command(name = "WCR")]
#[command(args_conflicts_with_subcommands = true)]
#[command(about = "Rust version of wc", long_about = "A new wc written by rust, justs for learning. 
    wcr -- lines, words, bytes, counter written by rust. always in the following order: newline->word->character->byte->max‐
       imum line length.")]
#[command(author = "Egg Shaw")]
pub struct WCRConfig {
    files: Vec<String>,
    #[arg(short = 'l', long, default_value="true")]
    lines: bool,
    #[arg(short = 'w', long, default_value="true")]
    words: bool, 
    #[arg(short = 'b', long, default_value="true")]
    bytes: bool,
    #[arg(short = 'c', long, default_value="true")]
    chars: bool,
    #[arg(short = 'e', long, default_value="false")]
    notempty: bool, 
}


pub fn get_args() -> WcrResult<WCRConfig> {
    let config = WCRConfig::parse();
    Ok(config)
}

pub fn run(config: WCRConfig) -> WcrResult<()>{

    // mode_default: 1111
    // let mut model: u8 = 0b1111;  1111 << 2... 
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_chars = 0;
    let mut total_bytes = 0;
    for (fidx, fname) in config.files.iter().enumerate() {
        match open(fname) {
           Err(e) => { eprintln!("[ERROR!] failed to open '{}'：{}", fname, e); },
           Ok(f) => {
            let res = counter(f, &config).unwrap_or_default();
            println!("\t{fname}");
            total_lines += res.lines;
            total_words += res.words;
            total_chars += res.chars;
            total_bytes += res.bytes;
            // let mut counts: Vec<usize> = Vec::new();
            // if config.lines { counts.push()}
           } 
        }
    }
    println!("{total_lines}  {total_words}  {total_chars}  {total_chars} total");
    Ok(())
}


pub fn open(path: &str) -> WcrResult<Box<dyn BufRead>> {
    match path {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(path)?))),
    }
}


fn counter(mut file: impl BufRead, config: &WCRConfig) -> WcrResult<FileCounts>{
    let (cntline, cntword, cntbyte, cntchar) = (config.lines, config.words, config.bytes, config.chars);
    let mut lines = 0;
    let mut words = 0;
    let mut bytes = 0;
    let mut chars = 0;
    let mut line_buffer = String::new();
    let ignore_empty_line = config.notempty;
    // let mut buffer = 
    // let mut context: [u8; ] = [];
    loop {
    if let Ok(line_bytes) = file.read_line(&mut line_buffer) {
        if line_bytes == 0 { break; }
        lines += 1;
        bytes += line_bytes;
        words += line_buffer.split_whitespace().count();
        chars += line_buffer.chars().count();
        line_buffer.clear();
    }
}

    if cntline { 
        print!("{lines}  ");
    }
    if cntword { 
        print!("{words}  ");
    }
    if cntchar { 
        print!("{chars}  ");
    }
    if cntbyte { 
        print!("{bytes}  ");
    }

    Ok(FileCounts::new(lines, words, chars, bytes))
}

