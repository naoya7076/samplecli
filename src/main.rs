use clap::Clap;
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
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);

        for line in reader.lines() {
            let line = line.unwrap();
            println!("{}", line)
        }
    } else {
        //ファイルを指定しなかった場合
        println!("No file is specified")
    }
}
