//! OW4補遺6eの検収入口。通常の描画対象が1件も可視でないフレームで、布のシーン描画とシャドウ記録が成立することを確かめる。
//! 群がカメラ視錐台にもどの距離区分のライト視錐台にも入らないシーンを、布ありと布なしで1回ずつ描き、シーンと全距離区分の発行数が0であることと、
//! 布ありの実行だけが画素を描くことを見る。布なしの実行は、描くものが1つも無いシャドウパスが消去だけを行う経路の検査でもある。
//! 判定の中身は`judgment`にある。

mod judgment;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use crate::acceptance::{検収の1回の実行, 読み戻しの置き場};
use crate::vegetation_run;

const 出力ディレクトリ: &str = "target/cloth_empty";
const シェーダーコピー先: &str = "target/cloth_empty_shaders";
/// 群がどちらの視錐台にも入らないシーン。中身は`vegetation_4`と同じ4個体の群であり、名前だけが違う。
/// 名前が`vegetation`で始まらないため、初期カメラは既定の「原点を+Z側3メートルから見る」姿勢になり、
/// 影の範囲も既定の「世界原点を中心とした半幅4メートル」になる(`crates/blitz_app/src/app/scene_camera.rs`・
/// `crates/blitz_app/src/app/scene_lighting.rs`)。群はチャンク中央(50, 0, 50)へ敷かれるため、カメラの背後にあり、
/// 影の範囲からも外れる。布は既定カメラの正面へ垂れるため、群が1体も描かれないフレームでも布だけが画素に出る。
const シーン: &str = "instance_all_culled";
const フレーム数: &str = "12";
const 布ありの引数: [&str; 4] = ["--no-post", "--report-draw-issue", "--report-memory", "--cloth"];
const 布なしの引数: [&str; 3] = ["--no-post", "--report-draw-issue", "--report-memory"];

pub fn 実行する() -> ExitCode {
    match 検収する() {
        Ok(要約) => {
            println!("[xtask] cloth-empty成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] cloth-empty失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 検収する() -> Result<String, String> {
    if !crate::gen_source_assets::生成する() || !crate::compile_assets::植生世界を既定で生成する() {
        return Err("検証用アセットの生成に失敗した".to_string());
    }
    let 出力先 = 読み戻しの置き場::作って受け取る(PathBuf::from(出力ディレクトリ))?;
    let シェーダー入口 = crate::shader_copy::一時コピーを作る(Path::new(シェーダーコピー先))?;

    let 布あり = 描く(&出力先, "cloth", &シェーダー入口, &布ありの引数)?;
    let 布なし = 描く(&出力先, "no_cloth", &シェーダー入口, &布なしの引数)?;
    let 布あり計数 = crate::report_parse::取り出す(布あり.報告().本文())?;
    let 布なし計数 = crate::report_parse::取り出す(布なし.報告().本文())?;
    judgment::入力が空であることを検査する("布あり", &布あり計数)?;
    judgment::入力が空であることを検査する("布なし", &布なし計数)?;
    judgment::布の段階切替を検査する("布あり", &布あり計数, true)?;
    judgment::布の段階切替を検査する("布なし", &布なし計数, false)?;
    let 布の画素数 = judgment::布だけが画素を描くことを検査する(布あり.画像(), 布なし.画像())?;
    Ok(format!(
        "{シーン}はシーンと全距離区分のどれも候補{}体・可視0体・発行0件で、布ありの実行だけが{布の画素数}画素を描いた(布ありはどのパスもパイプライン切替1回・描画1回、布なしはどちらも0回。validationは両実行とも0件)",
        布あり計数.シーン.候補数
    ))
}

fn 描く(
    出力先: &読み戻しの置き場, 名前: &str, シェーダー入口: &Path, 追加引数: &[&str]
) -> Result<検収の1回の実行, String> {
    vegetation_run::描画する(&出力先.中の書き出し先(名前), シーン, シェーダー入口, フレーム数, 追加引数)
}
