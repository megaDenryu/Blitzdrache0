//! 布のライティングが方向光の強度と環境光係数に追従することの検収入口。
//!
//! 布の描画シェーダーは以前、方向光の色だけを読んで強度(directionalLightColor.w)を掛けず、環境光も0.15の固定値だった。
//! そのため布だけが夜も方向光で照らされ続け、夜の環境光へも追従しなかった。この検収はその欠陥が戻れば落ちる。
//! 見るのは布が覆う画素の平均輝度であり、夜が環境光だけの水準に収まることと、昼がそれより明確に明るいことの2つを課す。
//! 布の領域は同じ時刻の布ありと布なしの差で切り出す。布の色は時刻ごとの光で変わるため、色で切り出すと時刻を跨げない。
//! 描く世界は`cargo xtask cloth-empty`と同じ「群が両方の視錐台の外にある世界」である。この世界の検収計画は自己操作も
//! 画素判定も持たないため、時刻を変えて絵を変えても既存の判定と衝突しない。`--sky`で空を持たない世界の方針を上書きし、
//! 方向光・環境光・露出を時刻から導く経路へ載せる。判定の定数と中身は`judgment`にある。

mod judgment;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use crate::acceptance::{検収の1回の実行, 読み戻しの置き場};

const 出力ディレクトリ: &str = "target/cloth_night";
const シェーダーコピー先: &str = "target/cloth_night_shaders";
/// 群がどちらの視錐台にも入らない世界。布だけが画素に出るため、布の領域を切り出しやすい。
const シーン: &str = "instance_all_culled";
/// 布が垂れて画素に出る最小の歩進。自己衝突の揺れを溜めないため短く取る。
const フレーム数: &str = "12";
/// 一日内秒。`cargo xtask sky-time`の代表時刻と同じ値を使い、同じ場面を別の観点で語れるようにする。
const 夜の一日内秒: &str = "75600";
const 昼の一日内秒: &str = "43200";

pub fn 実行する() -> ExitCode {
    match 検収する() {
        Ok(要約) => {
            println!("[xtask] cloth-night成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] cloth-night失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 検収する() -> Result<String, String> {
    if !crate::gen_source_assets::生成する() || !crate::compile_assets::植生世界を既定で生成する() {
        return Err("検証用アセットの生成に失敗した".to_string());
    }
    let 出力先 = 読み戻しの置き場::作って受け取る(PathBuf::from(出力ディレクトリ))?;
    let 入口 = crate::shader_copy::一時コピーを作る(Path::new(シェーダーコピー先))?;

    let 夜 = 布領域を測る(&出力先, &入口, "night", 夜の一日内秒)?;
    let 昼 = 布領域を測る(&出力先, &入口, "day", 昼の一日内秒)?;

    let 明るさ = judgment::明るさの時刻への追従を判定する(&夜, &昼)?;
    let 夜png = 出力先.中の書き出し先("night_cloth").目視用の絵へ変換する()?;
    Ok(format!("{明るさ}、絵は{}", 夜png.display()))
}

fn 布領域を測る(出力先: &読み戻しの置き場, 入口: &Path, 名前: &str, 一日内秒: &str) -> Result<judgment::布領域, String> {
    let 布あり = 描く(出力先, &format!("{名前}_cloth"), 入口, 一日内秒, &["--cloth"])?;
    let 布なし = 描く(出力先, &format!("{名前}_no_cloth"), 入口, 一日内秒, &[])?;
    judgment::布領域を測る(布あり.画像(), 布なし.画像())
}

fn 描く(
    出力先: &読み戻しの置き場, 名前: &str, 入口: &Path, 一日内秒: &str, 追加: &[&str]
) -> Result<検収の1回の実行, String> {
    let mut 引数: Vec<&str> = vec!["--sky", "--time-of-day", 一日内秒];
    引数.extend_from_slice(追加);
    crate::vegetation_run::描画する(&出力先.中の書き出し先(名前), シーン, 入口, フレーム数, &引数)
}
