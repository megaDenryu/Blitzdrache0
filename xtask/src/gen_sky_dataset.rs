//! Hosek-Wilkie解析近似の係数データセットを、原著者の公開ヘッダから実行時に読む形式へ焼く工程。
//! 受け取るのは`ArHosekSkyModelData_RGB.h`のパス、書き出すのは`crates/blitz_engine/data/`のリトルエンディアンf32列である。
//!
//! 出典: Lukas Hosek and Alexander Wilkie, "An Analytic Model for Full Spectral Sky-Dome Radiance" (SIGGRAPH 2012)の
//! 公開実装 `HosekWilkie_SkylightModel_C_Source.1.4a.zip`
//! (https://cgg.mff.cuni.cz/projects/SkylightModelling/HosekWilkie_SkylightModel_C_Source.1.4a.zip、
//! 書庫のSHA-256は743e81a7fcbed06408490a303dcf1315083d7988a11fc69608e66f6e5417f9de)。3条項BSDライセンスで再配布できる。
//! 書庫を展開して得た`ArHosekSkyModelData_RGB.h`のパスを引数に渡す。

mod parse;
mod write;

use std::path::Path;
use std::process::ExitCode;

/// 焼く配列の名前と要素数。方向係数は アルベド2 × 濁度10 × 制御点6 × 係数9、放射輝度は アルベド2 × 濁度10 × 制御点6 である。
const 方向係数配列一覧: [(&str, usize); 3] = [("datasetRGB1", 1080), ("datasetRGB2", 1080), ("datasetRGB3", 1080)];
const 放射輝度配列一覧: [(&str, usize); 3] = [("datasetRGBRad1", 120), ("datasetRGBRad2", 120), ("datasetRGBRad3", 120)];
const 出力パス: &str = "crates/blitz_engine/data/hosek_wilkie_rgb_f32le.bin";

pub fn 実行する(引数一覧: &[String]) -> ExitCode {
    let Some(ヘッダパス) = 引数一覧.first() else {
        eprintln!("[xtask] gen-sky-dataset失敗: ArHosekSkyModelData_RGB.hのパスを引数に渡すこと");
        return ExitCode::FAILURE;
    };
    match 焼く(Path::new(ヘッダパス)) {
        Ok(要約) => {
            println!("[xtask] gen-sky-dataset成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] gen-sky-dataset失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 焼く(ヘッダパス: &Path) -> Result<String, String> {
    let 内容 = std::fs::read_to_string(ヘッダパス).map_err(|誤り| format!("{}を読めない: {誤り}", ヘッダパス.display()))?;
    let 注釈除去済み = parse::注釈を落とす(&内容);
    let mut 値一覧 = Vec::new();
    for (名前, 要素数) in 方向係数配列一覧.into_iter().chain(放射輝度配列一覧) {
        値一覧.extend(parse::配列を取り出す(&注釈除去済み, 名前, 要素数)?);
    }
    write::書き出す(Path::new(出力パス), &値一覧)?;
    Ok(format!("{}へ{}個のf32を書いた", 出力パス, 値一覧.len()))
}
