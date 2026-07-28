//! 原点移動不変性の検査: 同じ局所シーンを、世界全体へ大きな大域平行移動を加えた条件でも描き、読み戻し画像がバイト一致することを確かめる。
//! GPUへ届く値はどちらもカメラ相対フレームで同一になるため、ポスト処理を含めたままバイト一致を要求できる。
//! 参照: `_doc/設計/地形とカメラ相対描画.md`「DoDの検査設計」

mod compare;
mod run;

use std::path::Path;
use std::process::ExitCode;

/// 検査対象のシーン。方向光の影と点光源を持つため、対象・カメラ・光源・影基準の4つすべてが同じ原点で相対化されていないと一致しない。
const シーン名: &str = "shadow_scene";
const フレーム数: &str = "60";
/// km級の平行移動。設計のDoDが求める大座標での判定であり、f32へ直接持ち込めば桁が失われる大きさである。
const 大移動メートル: &str = "10000000";
const 出力ディレクトリ: &str = "target/origin_invariance";

pub fn 実行する() -> ExitCode {
    if !crate::gen_source_assets::生成する() || !crate::compile_assets::既定を生成する() {
        return ExitCode::FAILURE;
    }
    let 出力先 = Path::new(出力ディレクトリ);
    if let Err(誤り) = std::fs::create_dir_all(出力先) {
        eprintln!("[xtask] 出力ディレクトリの作成に失敗した: {誤り}");
        return ExitCode::FAILURE;
    }

    let 近傍 = match run::読み戻しを取る(出力先, "near", "0") {
        Some(画像) => 画像,
        None => return ExitCode::FAILURE,
    };
    let 遠方一回目 = match run::読み戻しを取る(出力先, "far_first", 大移動メートル) {
        Some(画像) => 画像,
        None => return ExitCode::FAILURE,
    };
    let 遠方二回目 = match run::読み戻しを取る(出力先, "far_second", 大移動メートル) {
        Some(画像) => 画像,
        None => return ExitCode::FAILURE,
    };

    let 判定一覧 = [
        compare::判定する("同じ指定の2回実行が一致する", &遠方一回目, &遠方二回目),
        compare::判定する("オフセット0と大移動が一致する", &近傍, &遠方一回目),
    ];
    compare::結果を表示する(&判定一覧)
}
