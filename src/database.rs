use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Seek, Write};
pub struct Record {
    pub id: i32,
    pub content: String,
}
pub struct Database {
    pub filename: String,
    pub file: File,
}
impl Database {
    pub fn open(filename: &str) -> Self {
        let file = OpenOptions::new()
            .create(true) // 如果文件不存在，则创建
            .read(true) // 设置为写入模式
            .write(true)
            .open(filename)
            .unwrap();
        Database {
            filename: filename.to_string(),
            file,
        }
    }
    pub fn add_record(&mut self, record: &Record) {
        let line = format!("{},{}\n", record.id, record.content);
        // use std::io::Write;
        writeln!(self.file, "{}", line).unwrap();
        println!("📒Item added :{}", record.content)
    }
    pub fn read_record(&mut self) -> Vec<Record> {
        let file = BufReader::new(&self.file);
        file.lines()
            .map_while(Result::ok)
            .filter(|l| !l.is_empty())
            .map(|l| parse_record_line(&l))
            .collect()
    }
    pub fn remove_record(&mut self, id: i32) {
        let file = BufReader::new(&self.file);
        // enumerate 为迭代器添加索引
        let mut lines = file.lines().enumerate();
        let line = lines.find(|(_, l)| {
            // as_ref 转换成字符串切片 讲一个值转换为一个引用
            let record = parse_record_line(l.as_ref().unwrap());
            record.id == id
        });
        match line {
            Some((i, _)) => {
                let content = fs::read_to_string(&self.filename).unwrap();
                let new_content = content
                    .lines()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_, l)| l)
                    .collect::<Vec<_>>()
                    .join("\n");
                  // 文件指针移动到文件的开始位置 
                self.file.seek(std::io::SeekFrom::Start(0)).unwrap();
                self.file.write_all(new_content.as_bytes()).unwrap();
                // 设置文件的新长度
                self.file.set_len(new_content.len() as u64).unwrap();
                println!(" ❌ Item removed!\n");
            }
            None => println!("No record found with id:{}", id),
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
