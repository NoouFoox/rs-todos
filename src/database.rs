/// 模拟数据库模块
/// 用于操作文件
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Seek, Write};
// 定义一个结构体 Record 用于表示数据库中的记录
#[derive(Debug)]
pub struct Record {
    pub id: i32,
    pub content: String,
}
// 定义一个结构体 Database 用于表示数据库
// filename 为数据库文件的名称
// file 为数据库文件的句柄 句柄: 用于操作文件的对象
pub struct Database {
    pub filename: String,
    pub file: File,
}

// 实现 Database 结构体的方法
impl Database {
    // open 方法用于打开一个数据库文件
    pub fn open(filename: &str) -> Self {
        // OpenOptions::new() 创建一个新的 OpenOptions 对象
        // create(true) 如果文件不存在，则创建
        // read(true) 设置为读取模式
        // write(true) 设置为写入模式
        // open(filename) 打开一个文件
        // unwrap() 如果出现错误，则 panic
        // 返回一个 Database 结构体
        // Database 结构体包含了文件名和文件句柄
        let file = OpenOptions::new()
            .create(true) // 设置为写入模式
            .read(true)
            .write(true)
            .open(filename)
            .unwrap();
        Database {
            filename: filename.to_string(),
            file,
        }
    }
    // add_record 方法用于向数据库中添加一条记录
    pub fn add_record(&mut self, record: &Record) {
        // format! 用于格式化字符串 返回一个字符串
        let line = format!("{},{}\n", record.id, record.content);
        // use std::io::Write;
        // write! 用于将格式化的字符串写入文件 返回一个 Result 对象 
        writeln!(self.file, "{}", line).unwrap();
    }
    // read_record 方法用于读取数据库中的所有记录
    pub fn read_record(&mut self) -> Vec<Record> {
        // BufReader::new() 创建一个新的 BufReader 对象
        let file = BufReader::new(&self.file);
        // lines() 返回一个迭代器，其中包含文件的所有行
        // map_while() 用于将 Result 对象转换为 Option 对象
        // filter() 用于过滤空行
        // map() 用于将每一行转换为 Record 对象
        // collect() 用于将迭代器转换为一个 Vec 对象);
        file.lines()
            .map_while(Result::ok)
            .filter(|l| !l.is_empty())
            .map(|l| parse_record_line(&l))
            .collect()
    }
    // remove_record 方法用于删除数据库中的一条记录
    pub fn remove_record(&mut self, id: i32) {
        // BufReader::new() 创建一个新的 BufReader 对象
        let file = BufReader::new(&self.file);
        // enumerate 为迭代器添加索引
        let mut lines = file.lines().enumerate();
        // find() 用于查找满足条件的第一个元素
        // 如果找到则返回 Some(i, l) 元组
        // 如果没有找到则返回 None
        // parse_record_line() 用于将字符串转换为 Record 对象

        let line = lines.find(|(_, l)| {
            // as_ref 转换成字符串切片 讲一个值转换为一个引用
            let record = parse_record_line(l.as_ref().unwrap());
            record.id == id
        });
        match line {
            // 如果找到记录，则删除记录
            Some((i, _)) => {
                // 读取文件的内容
                let content = fs::read_to_string(&self.filename).unwrap();
                // 过滤掉要删除的记录
                // 将剩余的记录连接成一个字符串
                // 将字符串写入文件
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
// 用于将字符串转换为 Record 对象
pub fn parse_record_line(line: &str) -> Record {
    println!("line:{}", line);
    // split() 用于将字符串分割成一个字符串切片的迭代器
    let fields: Vec<&str> = line.split(",").collect();
    // 如果只有一个字段，则返回一个只有 content 字段的 Record 对象

    if fields.len() == 1 {
        return Record {
            id: 0,
            content: fields[0].to_string(),
        };
    }
    // 否则返回一个包含 id 和 content 字段的 Record 对象
    let content = fields[1..].join(",");
    Record {
        id: fields[0].parse().unwrap(),
        content,
    }
}
