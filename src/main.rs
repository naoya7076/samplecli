use clap::Clap;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

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
    formula_file: Option<String>,
}
fn main() {
    //https://docs.rs/clap/3.0.0-beta.2/clap/trait.Clap.html#method.parse
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        //unwrap メソッドは Ok なら中の値を返し、Err なら panic を起こす
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader, opts.verbose)
    } else {
        let stdin = stdin();
        // lockで入力をロックする -> StdinLock型のインスタンスを得られる
        let reader = stdin.lock();
        run(reader, opts.verbose)
    }
}

fn run(reader: BufReader<File>, _verbose: bool) {
    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line)
    }
}
