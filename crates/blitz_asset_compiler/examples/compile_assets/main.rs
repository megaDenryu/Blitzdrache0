//! `cargo xtask compile-assets`から呼ばれる実行時アセット生成器。
//! 1回の実行が焼くのは1つのチャンク世界であり、出力ルートには1つのカタログと1つのチャンク目録が並ぶ。
//! この入口が担うのは起動引数の受け取りだけであり、焼く工程は`compilation`の実行時アセットのコンパイルが持つ。

mod arguments;
mod texture_policy_argument;

use blitz_asset_compiler::実行時アセットのコンパイル;

fn main() {
    if let Err(誤り) = 実行する() {
        eprintln!("[compile_assets] {誤り}");
        std::process::exit(1);
    }
}

fn 実行する() -> Result<(), String> {
    let 全引数一覧: Vec<String> = std::env::args().skip(1).collect();
    let 指定 = arguments::引数一覧から焼く世界の指定を読む(&全引数一覧)?;
    実行時アセットのコンパイル::始める(指定)
        .and_then(実行時アセットのコンパイル::世界を焼く)
        .map_err(|誤り| 誤り.to_string())
}
