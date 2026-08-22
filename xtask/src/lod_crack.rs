//! 地形LODのGPU継ぎ目検査。白い地形を番兵背景色へ描き、内側の継ぎ目に背景画素が露出しないことを全方向・段差・細粗入替で測る。

mod cases;
mod image_check;
mod run;

use std::path::PathBuf;
use std::process::ExitCode;

use crate::acceptance::{描画検収の実行環境, 検収の実行名};

const 出力ディレクトリ: &str = "target/lod_crack";

pub fn 地形段差の継ぎ目を確認する() -> ExitCode {
    if !crate::gen_source_assets::生成する() || !crate::compile_assets::地形世界を既定で生成する() {
        return ExitCode::FAILURE;
    }
    let 実行環境 = match run::実行環境を作る(PathBuf::from(出力ディレクトリ)) {
        Ok(実行環境) => 実行環境,
        Err(誤り) => {
            eprintln!("[xtask] LOD継ぎ目検査の出力先を作れなかった: {誤り}");
            return ExitCode::FAILURE;
        }
    };
    let 条件一覧 = cases::全条件();
    let 総条件数 = 条件一覧.len();
    let mut 失敗数 = 0;
    let mut 総roi画素数 = 0_u64;
    for 条件 in 条件一覧 {
        let Some(画像) = 描いて報せる(&実行環境, &条件) else {
            失敗数 += 1;
            continue;
        };
        match image_check::継ぎ目を検査する(&画像, 条件.継ぎ目方向) {
            Ok(結果) => {
                総roi画素数 += 結果.roi画素数;
                println!("[xtask] LOD継ぎ目合格 {}: ROI {}画素、番兵背景0画素", 条件.名前, 結果.roi画素数);
            }
            Err(理由) => {
                eprintln!("[xtask] LOD継ぎ目不合格 {}: {理由}", 条件.名前);
                失敗数 += 1;
            }
        }
    }
    if 失敗数 == 0 {
        println!("[xtask] lod-crack成功: {総条件数}組合せ、ROI合計{総roi画素数}画素、番兵背景0画素");
        ExitCode::SUCCESS
    } else {
        eprintln!("[xtask] lod-crack失敗: {失敗数}組合せ");
        ExitCode::FAILURE
    }
}

/// 1組を描いて絵を返す。描けなかったことを条件の名前つきで報せてから無しへ畳む。呼び出し元は無しを
/// 「この組は判定しない」と読むため、読めなかった理由を捨てると判定を飛ばしたのか合格したのかが出力から分からなくなる。
fn 描いて報せる(実行環境: &描画検収の実行環境, 条件: &cases::検査条件) -> Option<crate::acceptance::読み戻し画像> {
    let 実行名 = 検収の実行名::生成する(&条件.名前).map_err(|誤り| eprintln!("[xtask] {誤り}")).ok()?;
    match 実行環境.描いて読み戻す(実行名, &run::起動指定を組み立てる(条件)) {
        Ok(実行) => Some(実行.画像を取り出す()),
        Err(誤り) => {
            eprintln!("[xtask] {}の描画か読み戻しに失敗した: {誤り}", 条件.名前);
            None
        }
    }
}
