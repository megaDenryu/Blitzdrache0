//! 世界の高さ場アセットを焼いて実行時カタログへ登録する工程。受け取るのは出力ルートとチャンクごとの高さ格子のソース一覧、
//! 返すのは登録の成否である。呼ばれるのは1つの世界につき高々1回であり、高さ場を宣言しない世界では1度も呼ばない。
//!
//! 安定IDを綴りで固定するのは、焼く側のこの工程と読む側のblitz_engineが同じ1つのアセットを指すためである
//! (綴りは`crates/blitz_engine/src/height_field/stable_id.rs`が持ち、ここはそれを参照する)。
//! 高さ場はチャンク目録に載らない。チャンク目録が1つの座標へ1つのアセットIDしか持てず、高さ場が矩形全体を1枚で覆うためである。

use std::path::{Path, PathBuf};

use blitz_asset_compiler::高さ場アセットをコンパイルする;
use blitz_engine::height_field::世界の高さ場の安定IDの綴り;
use blitz_engine::{アセットID, カタログ, チャンク座標};

pub(super) fn 高さ場を焼いて登録する(
    出力ルート: &Path,
    チャンクごとのソース一覧: &[(チャンク座標, PathBuf)],
    実行時カタログ: &mut カタログ,
) -> Result<(), String> {
    let id = アセットID::生成する(世界の高さ場の安定IDの綴り).map_err(|誤り| 誤り.to_string())?;
    let 結果 = 高さ場アセットをコンパイルする(チャンクごとのソース一覧).map_err(|誤り| format!("{id}: {誤り}"))?;
    let ファイル名 = format!("{世界の高さ場の安定IDの綴り}.blitzasset");
    let 出力パス = 出力ルート.join(&ファイル名);
    std::fs::write(&出力パス, &結果.実行時バイト列).map_err(|誤り| format!("{}を書き出せない: {誤り}", 出力パス.display()))?;
    実行時カタログ.詳細を登録する(id, PathBuf::from(ファイル名), 結果.ソース依存一覧, 結果.メタデータ);
    println!("[compile_assets] {}: {}バイト", 出力パス.display(), 結果.実行時バイト列.len());
    Ok(())
}
