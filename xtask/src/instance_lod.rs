//! OW4第3段の検収入口。個体別LODが実際に効いていることを、計数・画素・段の振動・GPU確保・ディスク読込の5つで確かめる。
//! 使う世界は段を2つ持つ原型の`vegetation_lod`であり、15体をカメラからの等距離の弧5本へ3体ずつ並べてある。
//! 判定の中身は`judgment`、画素の分類は`pixel_check`にある。
//! 参照: `_doc/設計/植生インスタンスと物量計測.md`「段階導入」

mod judgment;
mod pixel_check;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use crate::acceptance::{描画フレーム数, 描画検収の実行環境, 検収の実行名, 検収シーン名};
use crate::vegetation_run;

const 出力ディレクトリ: &str = "target/instance_lod";
const シェーダーコピー先: &str = "target/instance_lod_shaders";
/// 段を2つ持つ原型を等距離の弧へ並べた世界。
const 検収シーン: 検収シーン名 = 検収シーン名::生成する("vegetation_lod");
const フレーム数: 描画フレーム数 = 描画フレーム数::生成する(12);

/// 段の選択を入れた実行と止めた実行で共通の引数。ライティングを止めるのは、この検収が段ごとのジオメトリの違いを
/// 背景色との差で数えるためであり、陰影が入ると近景の領域が遠景の段の影で動きうるからである。
const 共通引数: [&str; 4] = ["--unlit", "--no-post", "--report-draw-issue", "--report-memory"];

/// ヒステリシス帯(基準距離50メートルに対して±5メートル)の内側でカメラを往復させる刻み。
/// 48.5メートルの弧が50メートルをまたぎながら55メートルを超えないため、段は入れ替わらない。
const 帯の内側の刻み: &str = "3";

/// 帯を超える刻み。同じ弧が55メートルを超えるため段が粗い側へ入れ替わり、帯の内側の判定が空振りでないことを示す。
const 帯の外側の刻み: &str = "12";

pub fn 実行する() -> ExitCode {
    match 検収する() {
        Ok(要約) => {
            println!("[xtask] instance-lod成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] instance-lod失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 検収する() -> Result<String, String> {
    if !crate::gen_source_assets::生成する() || !crate::compile_assets::植生世界を既定で生成する() {
        return Err("検証用アセットの生成に失敗した".to_string());
    }
    let 実行環境 = vegetation_run::植生世界の実行環境を作る(PathBuf::from(出力ディレクトリ))?;
    let シェーダー入口 = crate::shader_copy::一時コピーを作る(Path::new(シェーダーコピー先))?;

    let 段あり = 描く(&実行環境, "on", &シェーダー入口, &[])?;
    let 段なし = 描く(&実行環境, "off", &シェーダー入口, &["--no-instance-lod"])?;
    let 帯の内側 = 描く(&実行環境, "band_inside", &シェーダー入口, &["--lod-probe-step", 帯の内側の刻み])?;
    let 帯の外側 = 描く(&実行環境, "band_outside", &シェーダー入口, &["--lod-probe-step", 帯の外側の刻み])?;

    let 段別 = judgment::段が同時に立つことを検査する(&段あり)?;
    let 画素 = judgment::段の違いが絵に出ることを検査する(&段あり, &段なし)?;
    judgment::段が振動しないことを検査する(&帯の内側, &帯の外側)?;
    judgment::確保と読込が動かないことを検査する(&[&段あり, &段なし, &帯の内側, &帯の外側])?;
    Ok(format!(
        "段別個体数{段別:?}、遠景の非背景画素は段ありで{}・段なしで{}、近景はバイト一致、帯の内側の段切替0回・外側{}回、GPU確保数{}で不変、起動後のシーン読込0件",
        画素.遠景の段あり, 画素.遠景の段なし, 帯の外側.計数.段切替回数, 段あり.計数.現在確保数
    ))
}

fn 描く(
    実行環境: &描画検収の実行環境, 名前: &str, シェーダー入口: &Path, 追加引数: &[&str]
) -> Result<judgment::実行, String> {
    let mut 引数: Vec<&str> = 共通引数.to_vec();
    引数.extend_from_slice(追加引数);
    let 結果 = 実行環境.描いて読み戻す(
        検収の実行名::生成する(名前)?,
        &vegetation_run::植生世界の起動指定を組み立てる(検収シーン, フレーム数, シェーダー入口, &引数),
    )?;
    結果.報告().画面へ流す();
    let 計数 = crate::report_parse::取り出す(結果.報告())?;
    Ok(judgment::実行 {
        名前: 名前.to_string(),
        画像: 結果.画像を取り出す(),
        計数,
    })
}
