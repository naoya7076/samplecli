fn main() {
  let byte_array = [b'h', b'e', b'l', b'l', b'o'];
  print(Box::new(byte_array));
  let byte_array = [b'w', b'o', b'r', b'l', b'd', b'!'];
  print(Box::new(byte_array));
}

/**
 * Boxはコンパイル時にサイズ不定の値でも
 * 実行時にサイズがわかればヒープ領域に値を確保することができる
 **/
fn print(s: Box<[u8]>) {
  println!("{:?}", s);
}
