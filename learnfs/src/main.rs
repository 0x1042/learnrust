use std::{
    fs,
    fs::File,
    io::{BufRead, BufReader, ErrorKind, Read, Seek, SeekFrom, Write},
    path::Path,
};

use anyhow::anyhow;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// create file path
    #[clap(short, long, default_value = "/tmp/learn.txt")]
    path: String,

    /// read file path
    #[clap(short, long, default_value = "/var/log/weekly.out")]
    read_path: String,

    /// write file path
    #[clap(short, long, default_value = "/tmp/learn1024.txt")]
    write_path: String,

    /// del file path
    #[clap(short, long, default_value = "/tmp/learn.txt")]
    del_file_path: String,

    /// exist file path
    #[clap(short, long, default_value = "/tmp/learn.txt")]
    exist_path: String,
}

// create file
fn create_file(file_name: &str) -> anyhow::Result<File> {
    let file = File::create(file_name)?;
    Ok(file)
}

// read file by line
fn read_file(file_name: &str) -> anyhow::Result<Vec<String>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    let mut res = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(line) => {
                println!("read line: {}", &line);
                res.push(line);
            }
            Err(e) => {
                println!("read error: {}", e)
            }
        }
    }

    Ok(res)
}

fn read_to_string(file_name: &str) -> anyhow::Result<String> {
    let mut file = File::open(file_name)?;
    let mut resp = String::new();
    file.read_to_string(&mut resp)?;
    Ok(resp)
}

fn write_file(file_name: &str, text: &str) -> anyhow::Result<u64> {
    let mut file = File::create(file_name)?;
    let _ = file.write(text.as_bytes())?;
    let size = file.seek(SeekFrom::End(0))?;
    Ok(size)
}

fn rename_file(old_name: &str, new_name: &str) -> anyhow::Result<()> {
    std::fs::rename(old_name, new_name)?;
    Ok(())
}

fn copy_file(f1: &str, f2: &str) -> anyhow::Result<bool> {
    fs::copy(f1, f2)?;
    Ok(true)
}

fn get_file_size(file_name: &str) -> anyhow::Result<u64> {
    let mut file = File::open(file_name)?;
    let meta = file.metadata()?;
    let size1 = meta.len();
    // or use seek
    let size2 = file.seek(SeekFrom::End(0));

    println!("size1:{},size2:{:?}", size1, size2);
    Ok(size1)
}

// write file
fn delete_file(file_name: &str) -> anyhow::Result<()> {
    let is_exist = Path::new(file_name).is_file();
    if !is_exist {
        return Err(anyhow!(std::io::Error::new(
            ErrorKind::NotFound,
            "file path is not exist"
        )));
    }
    fs::remove_file(file_name)?;
    Ok(())
}

#[derive(Debug)]
enum RespType {
    FILE,
    DIR,
}

fn is_exist(path_str: &str) -> anyhow::Result<RespType> {
    let path = Path::new(path_str);

    if path.is_dir() {
        return Ok(RespType::DIR);
    }

    if path.is_file() {
        return Ok(RespType::FILE);
    }

    Err(anyhow!(std::io::Error::new(
        ErrorKind::NotFound,
        format!("file path `{}` is not exist", path_str)
    )))
}

fn main() {
    let _args = Args::parse();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_file() {
        match create_file("/tmp/learn.txt") {
            Ok(f) => {
                println!("{:?}", f.metadata())
            }
            Err(e) => {
                println!("create with error: {:?}", e);
            }
        };
    }

    #[test]
    fn test_read_to_string() {
        match read_to_string("/tmp/learn.txt") {
            Ok(str) => {
                println!("read success:{}", str);
            }
            Err(err) => {
                println!("read `/tmp/learn.txt` fail:{}", err)
            }
        };
    }

    #[test]
    fn test_write_file() {
        let pnb = std::env::current_exe().unwrap();

        let exe = pnb.as_path().to_str().unwrap();

        println!("exe:{:?}", &exe);

        match write_file("/tmp/learn1024.txt", exe) {
            Ok(str) => {
                println!("read success:{}", str);
            }
            Err(err) => {
                println!("read `/tmp/learn1024.txt` fail:{}", err)
            }
        };
    }

    #[test]
    fn test_del_file() {
        match delete_file("/tmp/learn2.txt") {
            Ok(_) => {
                println!("delete /tmp/learn2.txt success ");
            }
            Err(e) => {
                println!("delete /tmp/learn2.txt with error:{:?}", e);
            }
        };
    }

    #[test]
    fn test_get_file_size() {
        match get_file_size("/var/log/weekly.out") {
            Ok(tt) => {
                println!("size {:?}", tt);
            }
            Err(e) => {
                println!("get_file_size error {:?}", e);
            }
        };
    }

    #[test]
    fn test_is_exist() {
        match is_exist("/tmp/learn.txt") {
            Ok(tt) => {
                println!("file type {:?}", tt);
            }
            Err(e) => {
                println!("error {:?}", e);
            }
        };
    }
}
