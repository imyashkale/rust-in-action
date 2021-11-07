use clap::{App, Arg};
use regex::Regex;

// files
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    read_from_file()
}

pub fn read_from_file(){
    let f = File::open("/home/imyashkale/rustprojects/rust-in-action/src/storeroom/sample.txt").unwrap();
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    loop {
        let len = reader.read_line(&mut line).unwrap();
        if len == 0 {
            break;
        }
        println!("{} ({} bytes long)",line,len);
        line.truncate(0);
    }
}

// grep
pub fn grep() {
    let arg = Arg::with_name("pattern")
        .help("The pattern to search for")
        .takes_value(true)
        .required(true);
    let args = App::new("grep-lite")
        .version("0.1")
        .about("Searched for patterns")
        .arg(arg)
        .get_matches();
    let pattern = args.value_of("pattern").unwrap();
    println!("Pattern : {}", pattern);
    let re = Regex::new(pattern).unwrap();
    let quote = "\
        Every face, every shop, bedroom window, public-house, and
        dark square is a picture feverishly turned--in search of what?
        It is the same with books.
        What do we seek through millions of pages?";
    for line in quote.lines() {
        let contains_substring = re.find(line);
        match contains_substring {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}


pub fn intro_vec() {
    let ctx_lines = 2;
    let needle = "oo";
    let haystack = "\
    Every face, every shop,
    bedroom window, public-house, and
    dark square is a picture
    feverishly turned--in search of what?
    It is the same with books.
    What do we seek
    through millions of pages?";
    let mut tags: Vec<usize> = vec![];
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];

    for (index, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            tags.push(index);

            let v = Vec::with_capacity(2 * ctx_lines + 1);
            ctx.push(v)
        }
    }
    if tags.is_empty() {
        return;
    }

    for (index, line) in haystack.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;

            if (index >= lower_bound) && (index <= upper_bound) {
                let line_as_string = String::from(line);
                let local_ctx = (index, line_as_string);
                ctx[j].push(local_ctx)
            }
        }
    }
    for local_ctx in ctx.iter() {
        for &(index, ref line) in local_ctx.iter() {
            let line_num = index + 1;
            println!("{}: {}", line_num, line)
        }
    }
}

pub fn slice() {
    //slices can grow
    // array have fixed length
}

pub fn practice_array() {
    let one = [1, 2, 3];
    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];

    let arrays = [one, two, blank1, blank2];

    for item in &arrays {
        print!("{:?}:", item);
        for n in item.iter() {
            print!("\t{} + 10 = {}", n, n + 10);
        }
        let mut sum = 0;
        for l in 0..item.len() {
            sum += item[l]
        }
        println!("\t({:?} = {})", item, sum)
    }
}
