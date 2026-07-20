//! blitz_appのビルドスクリプト。Slangシェーダーをビルド時にSPIR-Vへコンパイルする。
//! 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`「判断6」。

mod build_support;

fn main() {
    if let Err(誤り) = build_support::シェーダーをビルドする() {
        panic!("{誤り}");
    }
}
