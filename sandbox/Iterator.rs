fn main() {
  let it = Iter {
    current: 0,
    max: 10,
  };
  for num in it {
    println!("{}", num);
  }
}

struct Iter {
  // usizeは主に配列やベクタの要素にアクセスしたりサイズを表現したりするときにつかわれる
  current: usize,
  max: usize,
}

/**
 * Iteratorトレイトを適用するための2つの方法
 * 1. Iteratorが出力する型を決定し、type Itemに紐付ける
 * 2. 次の要素を返すnext()メソッドを実装すること
 **/
impl Iterator for Iter {
  type Item = usize; // 出力する型の紐付け

  // next() メソッドの実装
  fn next(&mut self) -> Option<usize> {
    self.current += 1;
    if self.current - 1 < self.max {
      Some(self.current - 1)
    } else {
      None
    }
  }
}
