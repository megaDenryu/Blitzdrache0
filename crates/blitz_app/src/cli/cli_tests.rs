//! CLI既定値とフレーム時間報告フラグの解析を検証する。

use std::path::Path;

use super::{布モード, 引数を解析する, 粒子表示モード, 起動モード};

#[test]
fn 引数なしは既定値を保つ() {
    let Ok(設定) = 引数を解析する(&[]) else {
        panic!("引数なしは解析できるはず");
    };
    assert!(matches!(設定.モード, 起動モード::無期限実行));
    assert_eq!(設定.シェーダー監視パス, Path::new("shaders/scene.slang"));
    assert_eq!(設定.シーン名, "quad");
    assert_eq!(設定.アセットルート, Path::new("target/runtime_assets"));
    assert!(設定.描画対象数.is_none());
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
}

#[test]
fn フレーム時間報告と固定フレーム数を解析する() {
    let 引数一覧 = [
        "--benchmark-frames".to_string(),
        "600".to_string(),
        "--report-frame-times".to_string(),
        "--report-memory".to_string(),
    ];
    let Ok(設定) = 引数を解析する(&引数一覧) else {
        panic!("有効な引数は解析できるはず");
    };
    assert!(matches!(設定.モード, 起動モード::ベンチ実行 { フレーム数: 600 }));
    assert!(設定.フレーム時間報告);
    assert!(設定.gpuメモリ報告);
}

#[test]
fn スモーク用フレーム数をベンチ実行と区別する() {
    let 引数一覧 = ["--frames".to_string(), "60".to_string()];
    let Ok(設定) = 引数を解析する(&引数一覧) else {
        panic!("有効な引数は解析できるはず");
    };
    assert!(matches!(設定.モード, 起動モード::スモーク実行 { フレーム数: 60 }));
}

#[test]
fn 描画対象数は1以上だけを受理する() {
    let 有効 = ["--object-count".to_string(), "100".to_string()];
    let Ok(設定) = 引数を解析する(&有効) else {
        panic!("100件は解析できるはず");
    };
    assert_eq!(設定.描画対象数.map(super::描画対象数::usize値), Some(100));

    let 無効 = ["--object-count".to_string(), "0".to_string()];
    assert!(引数を解析する(&無効).is_err());
}
