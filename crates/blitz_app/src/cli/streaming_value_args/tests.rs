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
fn 零の容量を拒む() {
    let 引数 = 語("0");
    assert!(読込引数を反映する(チャンク読込設定::既定値(), &mut 引数.iter(), "--streaming-request-capacity").is_err());
}

#[test]
fn 不正な固定経路値を拒んでも設定を変えない() {
    let mut 経路 = 固定経路起動設定::既定値();
    let 変更前 = 経路;
    assert!(固定経路引数を反映する(&mut 経路, &mut 語("NaN").iter(), "--streaming-route-start-east-meters").is_err());
    assert_eq!(経路, 変更前);
    assert!(固定経路引数を反映する(&mut 経路, &mut 語("0").iter(), "--streaming-route-meters-per-frame").is_err());
    assert_eq!(経路, 変更前);
}

#[test]
fn 起動引数の全配線が二次元経路と読込設定へ届く() {
    let 引数: Vec<String> = [
        "--streaming-route-start-east-meters",
        "-3000",
        "--streaming-route-start-south-meters",
        "-4000",
        "--streaming-route-end-east-meters",
        "3000",
        "--streaming-route-end-south-meters",
        "4000",
        "--streaming-route-meters-per-frame",
        "20",
        "--streaming-loader-workers",
        "3",
        "--streaming-request-capacity",
        "96",
        "--streaming-completion-capacity",
        "12",
    ]
    .map(str::to_string)
    .to_vec();
    let Ok(crate::cli::起動要求::描画実行(設定)) = crate::cli::引数を解析する(&引数) else {
        panic!("固定経路と読込設定の起動引数を解析できなかった");
    };
    let 経路 = 設定.ストリーミング.固定経路;
    assert_eq!(
        (経路.始点東メートル, 経路.始点南メートル, 経路.終点東メートル, 経路.終点南メートル),
        (-3000.0, -4000.0, 3000.0, 4000.0)
    );
    assert_eq!(
        設定.ストリーミング.読込,
        チャンク読込設定::生成する(3, 96, 12).ok().unwrap_or(チャンク読込設定::既定値())
    );
}
