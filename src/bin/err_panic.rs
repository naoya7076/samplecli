use anyhow::{Context, Result};

fn get_int_from_file() -> Result<i32> {
  let path = "number.txt";
  // map_errはOKの場合はそのまま値を帰す、Errのときは処理を適用させる
  // ?は直前の結果のResultの方がOk(t)であればtを返してError(e)なら早期リターンして関数を終了する
  let num_str = std::fs::read_to_string(path)
    .with_context(|| format!("failed to read string from {}", path))?;
  num_str
    .trim()
    .parse::<i32>()
    .map(|t| t * 2)
    .context("failed from parse string")
}

fn main() {
  match get_int_from_file() {
    Ok(x) => println!("{}", x),
    Err(e) => println!("{:#?}", e),
  }
}
