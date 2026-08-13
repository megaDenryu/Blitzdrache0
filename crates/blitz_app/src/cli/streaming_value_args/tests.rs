use super::*;

fn 語(値: &str) -> Vec<String> {
    vec![値.to_string()]
}

#[test]
fn 読込設定の三値を別々に変えられる() {
    let mut 設定 = チャンク読込設定::既定値();
    for (名前, 値) in [
        ("--streaming-loader-workers", "3"),
        ("--streaming-request-capacity", "96"),
        ("--streaming-completion-capacity", "12"),
    ] {
        let 引数 = 語(値);
        let Ok(次) = 読込引数を反映する(設定, &mut 引数.iter(), 名前) else {
            panic!("有効な読込設定を反映できなかった");
        };
        設定 = 次;
    }
    assert_eq!(設定, チャンク読込設定::生成する(3, 96, 12).ok().unwrap_or(チャンク読込設定::既定値()));
}

#[test]
fn 固定経路の端点と速さを変えられる() {
    let mut 設定 = 固定経路起動設定::既定値();
    for (名前, 値) in [
        ("--streaming-route-start-east-meters", "-3000"),
        ("--streaming-route-start-south-meters", "-4000"),
        ("--streaming-route-end-east-meters", "3000"),
        ("--streaming-route-end-south-meters", "4000"),
        ("--streaming-route-meters-per-frame", "20"),
    ] {
        let 引数 = 語(値);
        assert!(固定経路引数を反映する(&mut 設定, &mut 引数.iter(), 名前).is_ok());
    }
    assert_eq!(
        (
            設定.始点東メートル,
            設定.始点南メートル,
            設定.終点東メートル,
            設定.終点南メートル,
            設定.一フレーム移動量メートル
        ),
        (-3000.0, -4000.0, 3000.0, 4000.0, 20.0)
    );
}

#[test]
fn 零の容量と速さを拒む() {
    let 引数 = 語("0");
    assert!(読込引数を反映する(チャンク読込設定::既定値(), &mut 引数.iter(), "--streaming-request-capacity").is_err());
    let mut 経路 = 固定経路起動設定::既定値();
    assert!(固定経路引数を反映する(&mut 経路, &mut 語("0").iter(), "--streaming-route-meters-per-frame").is_err());
}
