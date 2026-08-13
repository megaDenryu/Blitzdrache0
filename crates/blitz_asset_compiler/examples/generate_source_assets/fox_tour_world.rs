//! クソゲー1本目「キツネの場所巡り」が遊ばれる世界のソース一式の書き出しの入口。
//!
//! 世界そのものの決め事は`world_definition`が、ソース一式の置き場とファイル名の規則はライブラリの
//! `場所巡りの世界のソースディレクトリ`が、
//! 1回の書き出しが持ち回る状態と操作は`source_writing`が、種から高さを決める関数は`height`が、
//! 目印の柱の形と文書は`marker_geometry`と`marker_gltf_json`が持つ。

mod height;
mod lattice_noise;
mod marker_geometry;
mod marker_gltf_json;
mod source_writing;
mod world_definition;

use blitz_asset_compiler::{ソースルート, マップ生成の乱数の種, 世界の広がり, 焼き直しの勘定};

use source_writing::場所巡りの世界のソース書き出し;

pub(crate) fn 書き出す(
    ソースルート: &ソースルート, 種: マップ生成の乱数の種, 広がり: 世界の広がり
) -> Result<焼き直しの勘定, String> {
    let 書き出し = 場所巡りの世界のソース書き出し::始める(ソースルート, 種, 広がり)?;
    let 置き場の綴り = 書き出し.表示の綴り().to_string();
    let 勘定 = 書き出し.ソース一式を書き出して台帳を確定する()?;
    println!(
        "[generate_source_assets] {置き場の綴り}へ種{}から生成完了、{}",
        種.値(),
        勘定.報告の行を作る()
    );
    Ok(勘定)
}
