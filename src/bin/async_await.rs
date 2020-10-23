use futures::executor;

async fn async_add(left: i32, right: i32) -> i32 {
  left + right
}

async fn something_great_async_function() -> i32 {
  let ans = async_add(2, 3).await; //この時点で5という値を取り出せる
                                   // some process
  println!("{}", ans);
  ans
}

fn main() {
  executor::block_on(something_great_async_function())
}
