//! 世界配置に属するCLI引数の三軸対応を検査する。

use super::引数を解析する;

#[test]
fn 大域オフセットの三軸を別々に解析する() {
    let 引数一覧 = ["--global-offset".to_string(), "1".to_string(), "2".to_string(), "3".to_string()];
    let Ok(設定) = 引数を解析する(&引数一覧) else {
        panic!("有限な三成分は解析できるはず");
    };
    let 位置 = 設定.平行移動.大域オフセット;
    assert_eq!([位置.x().値(), 位置.y().値(), 位置.z().値()], [1.0, 2.0, 3.0]);
}
