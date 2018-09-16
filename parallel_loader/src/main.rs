use std::{env, mem, thread};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Result};

//[dependencies]
//reqwest = "0.8.0"
extern crate reqwest;

fn loading_fn(line : &str) -> Result<()>
{
    let mut options = OpenOptions::new();

    // Create an output file with a name equal to the last domain
    let fout = options
        .write(true)
        .create(true)
        .open(&line[line.rfind('/').unwrap() + 1..])?;
    let mut writer = BufWriter::new(&fout);

    let mut b = reqwest::get(line).unwrap();
    println!("{} -> {}", line, mem::size_of_val(&b));

    std::io::copy(&mut b, &mut writer)?;
    Ok(())
}

fn main() -> Result<()>
{
    // All URL's are stored in file passed as a command line argument
    let args: Vec<String> = env::args().collect();
    let fin = File::open(&args[1])?;

    let mut threads_pool = vec![];
    for line in BufReader::new(fin).lines()
    {
        threads_pool.push(thread::spawn(move || loading_fn(&line.unwrap())))
    }

    for thread in threads_pool
    {
        let _ = thread.join();
    }
    Ok(())
}
