use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Seek, Write};

use crate::utils::{check_db_file, get_db_file_path};
#[derive(Debug)]
pub struct Record {
    pub id: i32,
    pub content: String,
}
pub struct Database {
    pub file: File,
}

impl Database {
    pub fn open() -> Self {
        check_db_file().unwrap();
        let db_file = get_db_file_path();
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(db_file)
            .unwrap();
        Database { file }
    }
    pub fn add_record(&mut self, record: &Record) -> Result<(), io::Error> {
        let line = format!("{},{}\n", record.id, record.content);
        writeln!(self.file, "{}", line)
    }
    pub fn read_records(&mut self) -> Vec<Record> {
        let file = BufReader::new(&self.file);
        file.lines()
            .map_while(Result::ok)
            .filter(|l| !l.is_empty())
            .map(|l| parse_record_line(&l))
            .collect()
    }
    pub fn remove_record(&mut self, id: i32) -> Result<(), io::Error> {
        let file = BufReader::new(&self.file);
        let mut lines = file.lines().enumerate();
        let line = lines.find(|(_, l)| {
            // as_ref 转换成字符串切片 讲一个值转换为一个引用
            let record = parse_record_line(l.as_ref().unwrap());
            record.id == id
        });
        match line {
            Some((i, _)) => {
                let db_file = get_db_file_path();
                let content = fs::read_to_string(db_file).unwrap();
                let new_content = content
                    .lines()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_, l)| l)
                    .collect::<Vec<_>>()
                    .join("\n");
                self.file.seek(std::io::SeekFrom::Start(0)).unwrap();
                self.file.write_all(new_content.as_bytes()).unwrap();
                self.file.set_len(new_content.len() as u64).unwrap();
                Ok(())
            }
            None => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("No record found with id:{}", id),
            )),
        }
    }
}
pub fn parse_record_line(line: &str) -> Record {
    let fields: Vec<&str> = line.split(",").collect();
    if fields.len() == 1 {
        return Record {
            id: 0,
            content: fields[0].to_string(),
        };
    }
    let content = fields[1..].join(",");
    Record {
        id: fields[0].parse().unwrap(),
        content,
    }
}
