use regex::Regex;
use serde_json::Value;
use std::fs::File;
use std::io::{self, Read};

fn flatten_list(nested_list: Vec<Value>) -> Vec<i64> {
  let mut list = Vec::new();
  let mut result = Vec::new();

  for item in nested_list.into_iter().rev() {
    list.push(item);
  }

  while let Some(value) = list.pop() {
    match value {
      Value::Number(n) => {
        if let Some(i) = n.as_i64() {
          result.push(i);
        }
      }
      Value::Array(arr) => {
        for item in arr.into_iter().rev() {
          list.push(item);
        }
      }
      _ => {}
    }
  }
  result
}

fn flatten_list_by_regex(input: &str) -> Vec<i64> {
  // 数値とカンマだけ残し、ベクタに変換する
  let re = Regex::new(r"[^0-9,]").unwrap();
  let result = re.replace_all(input, "");
  result.split(',').filter_map(|s| s.parse().ok()).collect()
}

fn parse_nested_list(input: &str) -> Vec<Value> {
  serde_json::from_str(input).expect("Invalid JSON string")
}

fn read_input_from_file(file_path: &str) -> io::Result<Vec<String>> {
  let mut file = File::open(file_path)?;
  let mut content = String::new();
  file.read_to_string(&mut content)?;
  Ok(
    content
      .lines()
      .map(|line| line.trim().to_string())
      .collect(),
  )
}

pub fn main() {
  let file_path = "./test/mock/flatten1.txt";
  match read_input_from_file(file_path) {
    Ok(lines) => {
      let start_time = std::time::Instant::now();
      for (i, line) in lines.iter().enumerate() {
        if !line.is_empty() {
          let nested_list = parse_nested_list(line);
          let flattened = flatten_list(nested_list);
          println!("Flatten test case {}: {:?}", i + 1, flattened);
        }
      }
      println!("Flatten Elapsed time: {:?}", start_time.elapsed());

      let start_time = std::time::Instant::now();
      for (i, line) in lines.iter().enumerate() {
        if !line.is_empty() {
          let numbers = flatten_list_by_regex(line);
          println!("Regex test case {}: {:?}", i + 1, numbers);
        }
      }
      println!("Regex Elapsed time: {:?}", start_time.elapsed());
    }
    Err(e) => {
      eprintln!("Error reading file: {}", e);
    }
  }
}
