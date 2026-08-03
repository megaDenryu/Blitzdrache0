//! CLI既定値とフレーム時間報告フラグの解析を検証する。

use std::path::Path;

use super::{布モード, 引数を解析する, 描画対象の走査順, 粒子表示モード, 起動モード, 起動要求, 起動設定};

/// 描画実行の要求として解析できることを前提に、起動設定だけを取り出す。
fn 描画設定を解析する(引数一覧: &[String]) -> 起動設定 {
    match 引数を解析する(引数一覧) {
        Ok(起動要求::描画実行(設定)) => *設定,
        Ok(報告) => panic!("描画実行の要求になるはず(報告の種別{})", 報告.呼び名()),
        Err(誤り) => panic!("有効な引数は解析できるはず: {誤り}"),
    }
}

#[test]
fn 引数なしは既定値を保つ() {
    let 設定 = 描画設定を解析する(&[]);
    assert!(matches!(設定.モード, 起動モード::無期限実行));
    assert_eq!(設定.シェーダー監視パス, Path::new("shaders/scene.slang"));
    assert_eq!(設定.シーン名, "quad");
    assert_eq!(設定.アセットルート, Path::new("target/runtime_assets"));
    assert!(設定.描画対象の並べ方.件数.is_none());
    assert_eq!(設定.描画対象の並べ方.走査順, 描画対象の走査順::読込順);
    assert!(設定.ライティング有効);
    assert_eq!(設定.粒子表示, 粒子表示モード::なし);
    assert!(!設定.gpu時間報告);
    assert!(!設定.フレーム時間報告);
    assert!(!設定.gpuメモリ報告);
    assert!(!設定.開発ui初期有効);
    assert!(設定.フレームダンプ先.is_none());
    assert!(設定.ポスト処理有効);
    assert_eq!(設定.露出, 1.0);
    assert_eq!(設定.ブレンド, 0.0);
    assert_eq!(設定.布モード, 布モード::なし);
    assert!(!設定.実表示時間報告);
}

/// 実表示計測は測定対象を乱すため、`--report-frame-times`では有効にならないことを固定する。
#[test]
fn フレーム時間報告だけでは実表示計測を有効にしない() {
    let 設定 = 描画設定を解析する(&["--report-frame-times".to_string()]);
    assert!(設定.フレーム時間報告);
    assert!(!設定.実表示時間報告);
}

#[test]
fn 両方の報告を同時に指定できる() {
    let 引数一覧 = ["--report-frame-times".to_string(), "--report-display-timing".to_string()];
    let 設定 = 描画設定を解析する(&引数一覧);
    assert!(設定.フレーム時間報告);
    assert!(設定.実表示時間報告);
}

#[test]
fn フレーム時間報告と固定フレーム数を解析する() {
    let 引数一覧 = [
        "--benchmark-frames".to_string(),
        "600".to_string(),
        "--report-frame-times".to_string(),
        "--report-memory".to_string(),
    ];
    let 設定 = 描画設定を解析する(&引数一覧);
    assert!(matches!(設定.モード, 起動モード::ベンチ実行 { フレーム数: 600 }));
    assert!(設定.フレーム時間報告);
    assert!(設定.gpuメモリ報告);
}

#[test]
fn スモーク用フレーム数をベンチ実行と区別する() {
    let 設定 = 描画設定を解析する(&["--frames".to_string(), "60".to_string()]);
    assert!(matches!(設定.モード, 起動モード::スモーク実行 { フレーム数: 60 }));
}

#[test]
fn 描画対象数は1以上だけを受理する() {
    let 有効 = ["--object-count".to_string(), "100".to_string()];
    let 設定 = 描画設定を解析する(&有効);
    assert_eq!(設定.描画対象の並べ方.件数.map(super::描画対象数::usize値), Some(100));

    let 無効 = ["--object-count".to_string(), "0".to_string()];
    assert!(引数を解析する(&無効).is_err());
}

/// 走査順の入替は検収の指定であり、指定が無いときは読込順のままであることを固定する。
#[test]
fn 走査順の入替は指定したときだけ逆順になる() {
    let 設定 = 描画設定を解析する(&["--reverse-draw-order".to_string()]);
    assert_eq!(設定.描画対象の並べ方.走査順, 描画対象の走査順::逆順);
}

/// 逆順は同じ集合を逆に並べるだけであり、両端が入れ替わって全体が全単射であることを固定する。
#[test]
fn 逆順は束の中の位置を反転した添字へ写す() {
    let 写す = |位置| 描画対象の走査順::逆順.シーン内の添字へ写す(位置, 3);
    assert_eq!([写す(0), 写す(1), 写す(2)], [2, 1, 0]);
    assert_eq!(描画対象の走査順::読込順.シーン内の添字へ写す(1, 3), 1);
}
