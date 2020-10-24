use anyhow::{bail, ensure, Context, Result};
use clap::Clap;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::PathBuf;

struct RpnCalculator(bool);

impl RpnCalculator {
    // newを使うことで方から関数を呼ぶ形式でで意義される
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> Result<i32> {
        // あとでpopするからrev()している
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        // どうしてここでポインタを渡しているのか -> 借用
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> Result<i32> {
        let mut stack = Vec::new();
        let mut pos = 0;

        while let Some(token) = tokens.pop() {
            pos += 1;
            // 数値に変換できるかどうかを試す
            if let Ok(x) = token.parse::<i32>() {
                stack.push(x)
            } else {
                // expectはunwrapとほぼ同じだが、パニック時に指定したメッセージを出力できる
                let y = stack.pop().context(format!("invalid syntax at {}", pos))?;
                let x = stack.pop().context(format!("invalid syntax at {}", pos))?;
                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => bail!("invalid token at {}", pos),
                };
                stack.push(res)
            }

            // `-v`オプションが指定されている場合は、この時点でトークンとスタックの状態を出力
            // 無名フィールドはself.0で取得できる。0は何番目のフィールドかを表す
            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }

        ensure!(stack.len() == 1, "invalid syntax");

        Ok(stack[0])
    }
}

/**
 * [#derive]アトリビュートを用いることで型に対して特定のトレイトの標準的な実装を提供する機能があります。
 * より複雑なことを行わせたい場合には、同名のトレイトを手動で実装することも可能です。
 * deriveマクロは#[derive(MACRO_NAME)]という形式でstructやenumへの修飾を可能にするマクロ
 * DOC:https://docs.rs/clap/3.0.0-beta.2/clap/index.html#using-derive-macros
 **/
#[derive(Clap, Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "Shimmy",
    about = "Super awesome sample RPN calculator"
)]
struct Opts {
    #[clap(short, long)]
    verbose: bool,

    #[clap(name = "FILE")]
    formula_file: Option<PathBuf>,
}
fn main() -> Result<()> {
    //https://docs.rs/clap/3.0.0-beta.2/clap/trait.Clap.html#method.parse
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        //unwrap メソッドは Ok なら中の値を返し、Err なら panic を起こす
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        run(reader, opts.verbose)
    } else {
        let stdin = stdin();
        // lockで入力をロックする -> StdinLock型のインスタンスを得られる
        let reader = stdin.lock();
        run(reader, opts.verbose)
    }
}

fn run<R: BufRead>(reader: R, verbose: bool) -> Result<()> {
    let calc = RpnCalculator::new(verbose);

    for line in reader.lines() {
        let line = line?;
        match calc.eval(&line) {
            Ok(answer) => println!("{}", answer),
            Err(e) => eprintln!("{:#?}", e),
        }
    }
    Ok(())
}

/**
 * アトリビュート
 * cfg(...)はコンパイル時の設定によって有効無効を切り替えられる
 * modはモジュールの作成
 **/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let calc = RpnCalculator::new(false);
        assert_eq!(calc.eval("5").unwrap(), 5);
        assert_eq!(calc.eval("50").unwrap(), 50);
        assert_eq!(calc.eval("-50").unwrap(), -50);

        assert_eq!(calc.eval("2 3 +").unwrap(), 5);
        assert_eq!(calc.eval("2 3 *").unwrap(), 6);
        assert_eq!(calc.eval("2 3 -").unwrap(), -1);
        assert_eq!(calc.eval("2 3 /").unwrap(), 0);
        assert_eq!(calc.eval("2 3 %").unwrap(), 2);
    }

    #[test]
    fn test_ng() {
        let calc = RpnCalculator::new(false);
        assert!(calc.eval("").is_err());
        assert!(calc.eval("1 1 1 +").is_err());
        assert!(calc.eval("+ 1 1").is_err());
    }
}
