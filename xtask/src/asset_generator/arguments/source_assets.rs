//! ソース生成だけが使う、種・出力先・世界の広がりの引数列を組み立てる。

use blitz_asset_compiler::{ソースルート, マップ生成の乱数の種, 世界の広がり};

use super::spelling::{南北チャンク数の綴り, 東西チャンク数の綴り, 種の綴り};
use super::word_list::語の並び;
use crate::asset_generator::error::生成器エラー;

const 種の選択肢の綴り: &str = "--game-map-seed";
const ソースルートの選択肢の綴り: &str = "--source-root";
const 東西チャンク数の選択肢の綴り: &str = "--world-east-chunks";
const 南北チャンク数の選択肢の綴り: &str = "--world-south-chunks";

pub(super) fn ソース生成の引数を足す(
    並び: &mut 語の並び,
    種: Option<マップ生成の乱数の種>,
    ソースルート: Option<&ソースルート>,
    広がり: Option<世界の広がり>,
) -> Result<(), 生成器エラー> {
    並び.値を足す(Some(種の選択肢の綴り), "場所巡りの世界の種", 種.map(種の綴り))?;
    if let Some(ルート) = ソースルート {
        並び.語を足す(ソースルートの選択肢の綴り);
        並び.パスを足す(ルート.プロセスの引数へ渡すディレクトリ());
    }
    並び.値を足す(
        Some(東西チャンク数の選択肢の綴り),
        "世界の東西チャンク数",
        広がり.map(東西チャンク数の綴り),
    )?;
    並び.値を足す(
        Some(南北チャンク数の選択肢の綴り),
        "世界の南北チャンク数",
        広がり.map(南北チャンク数の綴り),
    )
}
