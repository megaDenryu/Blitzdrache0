//! 原点移動不変性の検査: 同じ局所シーンを、世界全体へ大きな大域平行移動を加えた条件でも描き、読み戻し画像がバイト一致することを確かめる。
//! GPUへ届く値はどちらもカメラ相対フレームで同一になるため、ポスト処理を含めたままバイト一致を要求できる。
//! 一致だけを見る検査は、指定を受け取って何もしない実装でも通ってしまう。カメラだけを微小に動かした条件を負の対照に置き、毎回それが不一致になることで検査の空回りを排除する。
//! 参照: `_doc/設計/地形とカメラ相対描画.md`「DoDの検査設計」

mod centroid;
mod compare;
mod run;

use std::path::Path;
use std::process::ExitCode;

use run::検査条件;

/// 検査対象のシーン。方向光の影と点光源を持つため、対象・カメラ・光源・影基準の4つすべてが同じ原点で相対化されていないと一致しない。
const シーン名: &str = "shadow_scene";
const フレーム数: &str = "60";
/// km級の平行移動。設計のDoDが求める大座標での判定であり、f32へ直接持ち込めば桁が失われる大きさである。
/// 3成分を別の値にするのは、軸を取り違えた実装が同じ値どうしの置換で見逃されないようにするためである。符号も揃えない。
const 大移動メートル: [f64; 3] = [10_000_000.0, 20_000_000.0, -30_000_000.0];
/// 負の対照でカメラだけをずらす量。シャドウ検証シーンのカメラ距離は6メートルであり、この量なら画素は確実に動く。
const カメラずれメートル: f64 = 0.25;
const 出力ディレクトリ: &str = "target/origin_invariance";

fn 静止条件(大域オフセット: [f64; 3]) -> 検査条件 {
    検査条件 {
        大域オフセット,
        カメラずれメートル: 0.0,
    }
}

pub fn 実行する() -> ExitCode {
    if !crate::gen_source_assets::生成する() || !crate::compile_assets::既定を生成する() {
        return ExitCode::FAILURE;
    }
    let 出力先 = Path::new(出力ディレクトリ);
    if let Err(誤り) = std::fs::create_dir_all(出力先) {
        eprintln!("[xtask] 出力ディレクトリの作成に失敗した: {誤り}");
        return ExitCode::FAILURE;
    }

    let 近傍 = match run::読み戻しを取る(出力先, "a_near", &静止条件([0.0, 0.0, 0.0])) {
        Some(画像) => 画像,
        None => return ExitCode::FAILURE,
    };
    let 遠方 = match run::読み戻しを取る(出力先, "b_far", &静止条件(大移動メートル)) {
        Some(画像) => 画像,
        None => return ExitCode::FAILURE,
    };
    let 近傍カメラずらし = match run::読み戻しを取る(出力先, "a_near_nudged", &カメラずらし条件([0.0, 0.0, 0.0])) {
        Some(画像) => 画像,
        None => return ExitCode::FAILURE,
    };
    let 遠方カメラずらし = match run::読み戻しを取る(出力先, "b_far_nudged", &カメラずらし条件(大移動メートル)) {
        Some(画像) => 画像,
        None => return ExitCode::FAILURE,
    };

    let 判定一覧 = [
        compare::一致を求める("AとBがバイト一致する", &近傍, &遠方),
        compare::一致を求める("A'とB'がバイト一致する", &近傍カメラずらし, &遠方カメラずらし),
        centroid::移動を比較する(&近傍, &近傍カメラずらし, &遠方, &遠方カメラずらし),
    ];
    compare::結果を表示する(&判定一覧)
}

fn カメラずらし条件(大域オフセット: [f64; 3]) -> 検査条件 {
    検査条件 {
        大域オフセット,
        カメラずれメートル,
    }
}
