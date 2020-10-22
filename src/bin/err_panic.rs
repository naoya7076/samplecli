enum MyError {
  Io(std::io::Error),
  Num(std::num::ParseIntError),
}
fn get_int_from_file() -> Result<i32, MyError> {
  let path = "number.txt";
  // map_errはOKの場合はそのまま値を帰す、Errのときは処理を適用させる
  // ?は直前の結果のResultの方がOk(t)であればtを返してError(e)なら早期リターンして関数を終了する
  let num_str = std::fs::read_to_string(path).map_err(|e| MyError::Io(e))?;
  num_str
    .trim()
    .parse::<i32>()
    .map(|t| t * 2)
    .map_err(|e| MyError::Num(e))
}

fn main() {
  match get_int_from_file() {
    Ok(x) => println!("{}", x),
    Err(e) => match e {
      MyError::Io(cause) => println!("I/O Error: {}", cause),
      MyError::Num(cause) => println!("Parse Error: {}", cause),
    },
  }
}
