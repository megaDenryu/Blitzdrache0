use std::path::Path;

use super::*;

#[test]
fn 既定計画は長軸八キロ経路と現行読込設定を渡す() {
    let 引数 = 起動引数を作る(&大規模世界の計測指定::default(), Path::new("shader.slang"));
    let 語 = 引数.join(" ");
    assert!(語.contains("--streaming-preload-radius 8"));
    assert!(語.contains("--streaming-loader-workers 1"));
    assert!(語.contains("--streaming-request-capacity 64"));
    assert!(語.contains("--streaming-completion-capacity 4"));
    assert!(語.contains("--streaming-route-start-south-meters -4000"));
    assert!(語.contains("--streaming-route-end-south-meters 4000"));
    assert!(語.contains(&format!("--scene {}", crate::fox_tour_launch::シーン名.綴り())));
    assert_eq!(引数.iter().filter(|語| 語.as_str() == "--asset-root").count(), 1);
    let 追加 = ストリーミング計測の起動引数を作る(&大規模世界の計測指定::default(), Path::new("shader.slang"));
    assert!(追加.iter().any(|語| 語 == "--streaming"));
    assert!(追加.iter().any(|語| 語 == "--streaming-route"));
}

#[test]
fn 容量とフレーム数を引数へ写す() {
    let 指定 = 大規模世界の計測指定 {
        フレーム数: 1200,
        先読み半径: 6,
        ram上限: 1234,
        vram上限: 5678,
        ..大規模世界の計測指定::default()
    };
    let 語 = 起動引数を作る(&指定, Path::new("shader.slang")).join(" ");
    assert!(語.contains("--benchmark-frames 1200"));
    assert!(語.contains("--streaming-preload-radius 6"));
    assert!(語.contains("--streaming-ram-limit 1234"));
    assert!(語.contains("--streaming-vram-limit 5678"));
}
