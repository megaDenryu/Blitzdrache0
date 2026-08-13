//! 場所巡りの世界が乱数の種を1行の十進表記で保存し、型付きで読み戻す契約の検査。

use super::{場所巡りの世界のソースディレクトリ, 種を書き出すファイル名};
use crate::{アセット配置エラー, ソースルート, マップ生成の乱数の種};

fn 一時のソースルート(名前: &str) -> (std::path::PathBuf, ソースルート) {
    let パス = std::env::temp_dir().join(format!("blitz_fox_tour_seed_{}_{}", std::process::id(), 名前));
    let _ = std::fs::remove_dir_all(&パス);
    (パス.clone(), ソースルート::生成する(パス))
}

fn 種を往復する(名前: &str, 種の値: u32) {
    let (パス, ルート) = 一時のソースルート(名前);
    let Ok(世界) = 場所巡りの世界のソースディレクトリ::ソースルートの下に作る(&ルート) else {
        panic!("試験用の世界の置き場を作れなかった");
    };
    let 種 = マップ生成の乱数の種::生成する(種の値);
    assert!(世界.生成に使った乱数の種を書き出す(種).is_ok());
    let Ok(復元した種) = 世界.生成に使った乱数の種を読む() else {
        panic!("試験用の乱数の種を復元できなかった");
    };
    assert_eq!(復元した種, 種);
    let Ok(綴り) = std::fs::read_to_string(パス.join("fox_tour_world").join(種を書き出すファイル名)) else {
        panic!("試験用の乱数の種を読めなかった");
    };
    assert_eq!(綴り, format!("{種の値}\n"));
    assert!(std::fs::remove_dir_all(パス).is_ok());
}

#[test]
fn 乱数の種0の綴りを往復する() {
    種を往復する("zero", 0);
}

#[test]
fn 乱数の種の最大値の綴りを往復する() {
    種を往復する("maximum", u32::MAX);
}

#[test]
fn 十進の非負整数でない乱数の種を拒む() {
    let (パス, ルート) = 一時のソースルート("invalid");
    let Ok(世界) = 場所巡りの世界のソースディレクトリ::ソースルートの下に作る(&ルート) else {
        panic!("試験用の世界の置き場を作れなかった");
    };
    assert!(世界.0.直下へ書き込む(種を書き出すファイル名, b"not-a-seed\n").is_ok());
    assert!(matches!(
        世界.生成に使った乱数の種を読む(),
        Err(アセット配置エラー::乱数の種が不正 { .. })
    ));
    assert!(std::fs::remove_dir_all(パス).is_ok());
}
