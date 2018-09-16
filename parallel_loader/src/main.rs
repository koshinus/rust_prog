use std::{env, mem, thread};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Result};

//[dependencies]
//reqwest = "0.8.0"
extern crate reqwest;

fn loading_fn(line : &str) -> Result<()>
{
    //let unwrapped = &line.unwrap();
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
    /*
    for line in BufReader::new(fin).lines()
    {
        let unwrapped  = &line.unwrap();
        let thr = thread::spawn(move || loading_fn(unwrapped));
        /*
        let unwrapped = &line.unwrap();
        let mut options = OpenOptions::new();

        // Create an output file with a name equal to the last domain
        let fout = options
            .write(true)
            .create(true)
            .open(&unwrapped[unwrapped.rfind('/').unwrap() + 1..])?;
        let mut writer = BufWriter::new(&fout);

        let mut b = reqwest::get(unwrapped).unwrap();
        println!("{} -> {}", unwrapped, mem::size_of_val(&b));

        std::io::copy(&mut b, &mut writer)?;
        */
    }
    */
    Ok(())
}
/*
fn parallel_print( arg : i32 )
{
    println!("{}", arg)
}

fn main() -> Result<()>
{
    let mut children = vec![];
    for i in 1..10
    {
        children.push(thread::spawn(move|| parallel_print(i)));
    }
    for child in children
    {
        let _ = child.join();
    }
    Ok(())
}
*/
/*
use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{ BufRead, BufReader, Result, BufWriter, Write, Read };

//[dependencies]
//reqwest = "0.8.0"
extern crate reqwest;

fn main() -> Result<()>
{
    // All URL's are stored in file passed as a command line argument
    let args : Vec<String> = env::args().collect();
    let fin = File::open(&args[1])?;

    let buf : &mut Vec<u8> = & mut Vec::new();
    let mut readed : usize;
    for line in BufReader::new(fin).lines()
        {
            let unwrapped = &line.unwrap();
            let mut options = OpenOptions::new();

            // Create an output file with a name equal to the last domain
            let fout = options.write(true).create(true).open(&unwrapped[unwrapped.rfind('/').unwrap() + 1..])?;
            let mut writer = BufWriter::new(&fout);

            readed = reqwest::get(unwrapped).unwrap().read_to_end(buf).unwrap();
            println!("{}", readed); // Just to know how many read
            writer.write(buf)?;

            // Close the file and clear "buf" to ensure that
            // the subsequent reading is correct
            writer.flush()?;
            buf.clear();
        }
    Ok(())
}
*/
/*
use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{ BufRead, BufReader, Result, BufWriter, Write, Read };

extern crate reqwest;

/*
https://habr.com/post/270371/
https://proglib.io/p/burned-out-programming/
https://proglib.io/p/project-list/
https://hermanradtke.com/2015/09/21/get-data-from-a-url-rust.html
*/

fn main() -> Result<()>
{
    let args : Vec<String> = env::args().collect();
    let fin = File::open(&args[1])?;
    //let buf : &mut [u8; 4*1024] = &mut [0; 4*1024];
    let buf : &mut Vec<u8> = & mut Vec::new();
    let mut readed : usize;
    for line in BufReader::new(fin).lines()
    {
        let unwrapped = &line.unwrap();
        let mut options = OpenOptions::new();
        let fout = options.write(true).create(true).open(&unwrapped[unwrapped.rfind('/').unwrap() + 1..])?;
        let mut writer = BufWriter::new(&fout);

        readed = reqwest::get(unwrapped).unwrap().read_to_end(buf).unwrap();
        println!("{}", readed);
        writer.write(buf)?;

        /*
        loop
        {
            readed = reqwest::get(unwrapped).unwrap().read(buf).unwrap();
            println!("{}", readed);
            writer.write(buf)?;

            if readed == 0
            {
                break;
            }

        }
        */
        writer.flush()?;
        buf.clear();
        /*
        for i in 0..buf.len()
        {
            buf[i] = 0;
        }
        */
    }
    Ok(())
}
*/