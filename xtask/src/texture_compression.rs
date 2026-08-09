//! ブロック圧縮の絵と誤差の統計をオーナーと機械が確かめるための入口。テクスチャ格納方針にブロック圧縮を渡す
//! 唯一の入口であり、ここで初めてBC1のテクスチャがGPUの上で標本される。
//!
//! 見るものは4つである。両方針の実行でvalidationの指摘が0件であること(ブロックの整合の主張の初の反証装置)、
//! 対照の素材の誤差の統計が制定した判定値の内側にあること、同じ方針の2度の焼き付けと2度の描画が同じ結果を出すこと、
//! そしてDamagedHelmetのベースカラーをBC1化した絵の目視材料が出ることである。絵の合否はオーナーが決める。
//!
//! 既存の入口の判定値を1つも動かさないのは、対照の素材が新しい検収世界だけに置かれ、既存の世界が
//! 既定の`全てRGBA8`のまま焼かれるためである。
//! 参照: `_doc/設計/テクスチャのブロック圧縮と縮小段生成.md`「段割りと各段の完了条件」の段4

mod contrast_run;
mod difference;
mod draw;
mod helmet_crop;
mod helmet_run;
mod judgment;
mod plate_area;
mod summary;
mod world_bake;

use std::path::PathBuf;
use std::process::ExitCode;

const 出力ディレクトリ: &str = "target/texture_compression";

pub fn 実行する() -> ExitCode {
    match 撮って判定する() {
        Ok(要約) => {
            println!("[xtask] texture-compression成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] texture-compression失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 撮って判定する() -> Result<String, String> {
    if !crate::gen_source_assets::生成する() {
        return Err("検証用ソースアセットの生成に失敗した".to_string());
    }
    let 出力先 = PathBuf::from(出力ディレクトリ);
    std::fs::create_dir_all(&出力先).map_err(|誤り| format!("出力先を作れなかった: {誤り}"))?;
    let mut 行一覧 = contrast_run::対照の板を検収する(&出力先)?;
    行一覧.extend(helmet_run::ヘルメットの目視材料を作る(&出力先)?);
    Ok(行一覧.join("、"))
}
