//! 段4bで廃止した材質スロットごとのディスクリプタセットの経路が書き戻されていないことの検査。
//! 受け取るのは無し(対象の語の表がここにある)、返すのは廃止した語を持つファイルの違反一覧である。
//!
//! 廃止したのは、材質スロットごとにテクスチャ3枚をcombined image samplerで結ぶセットと、そのセットを描画発行ごとに
//! 選び直す経路である。この形はコンパイルの通る形で書き戻せてしまい、書き戻しても単一材質の絵は同じに見えるため、
//! 名前の不在を機械的に守る(参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「段階導入」の段4bの検収ゲート(vi))。
//!
//! 走査対象をcratesとshadersに限るのは、この検査自身のソースが対象の語を持つためである。

use std::path::{Path, PathBuf};

use super::error::規約検査の破れ;
use super::violation::違反;
use crate::file_scan;

const 走査対象ディレクトリ一覧: [&str; 2] = ["crates", "shaders"];
const 走査対象拡張子一覧: [&str; 2] = ["rs", "slang"];

/// 書き戻しを禁じる語。旧セットが結んでいた資源の型名、スロットごとの選択が返していた参照の型名、
/// 描画対象ごとの材質レコードの内容の型名、GPU側の1枚ずつの標本器の宣言である。
const 廃止語一覧: [&str; 7] = [
    "マテリアルテクスチャ一式",
    "スロット別材質資源",
    "スロット材質資源",
    "束内材質参照",
    "材質セット参照",
    "材質レコード内容",
    "Sampler2D baseColorTexture",
];

pub fn 全ファイルを検査する() -> Result<Vec<違反>, 規約検査の破れ> {
    let ファイル一覧 = file_scan::対象ファイル一覧を集める(&走査対象ディレクトリ一覧, &走査対象拡張子一覧)?;
    let mut 違反一覧 = Vec::new();
    for パス in &ファイル一覧 {
        let 内容 = std::fs::read_to_string(パス).map_err(|誤り| 規約検査の破れ::ファイルを読めなかった(パス, 誤り))?;
        違反一覧.extend(ファイル1つを検査する(パス, &内容));
    }
    Ok(違反一覧)
}

fn ファイル1つを検査する(パス: &Path, 内容: &str) -> Vec<違反> {
    let mut 違反一覧 = Vec::new();
    for (行番号, 行) in 内容.lines().enumerate() {
        for 語 in 廃止語一覧 {
            if 行.contains(語) {
                違反一覧.push(違反::行単位(
                    PathBuf::from(パス),
                    行番号 + 1,
                    format!("段4bで廃止した材質スロットごとのディスクリプタセットの語({語})を書き戻している"),
                ));
            }
        }
    }
    違反一覧
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 廃止したスロット別資源の型名を書き戻した行を違反にする() {
        let 違反一覧 = ファイル1つを検査する(Path::new("crates/blitz_render/src/新規.rs"), "struct スロット別材質資源 {\n");
        assert_eq!(違反一覧.len(), 1);
    }

    #[test]
    fn 廃止した1枚ずつの標本器の宣言を書き戻した行を違反にする() {
        let 違反一覧 = ファイル1つを検査する(Path::new("shaders/scene.slang"), "[[vk::binding(1, 2)]] Sampler2D baseColorTexture;\n");
        assert_eq!(違反一覧.len(), 1);
    }

    #[test]
    fn 現行の名前は違反にしない() {
        let 違反一覧 = ファイル1つを検査する(Path::new("crates/blitz_render/src/新規.rs"), "struct スロット別材質ID {\n");
        assert!(違反一覧.is_empty());
    }
}
