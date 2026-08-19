//! `editor_serverへ渡す引数を組み立てる`の単体試験。実際にプロセスを起動する側は
//! 副作用を持つため試験しない(手動確認に留める)。

use super::*;

#[test]
fn 追加引数が無ければ空になる() {
    assert!(editor_serverへ渡す引数を組み立てる(&[]).is_empty());
}

#[test]
fn 追加引数はdashdashで区切って渡す() {
    let 追加引数 = vec!["--project".to_string(), "/tmp/game".to_string()];
    assert_eq!(
        editor_serverへ渡す引数を組み立てる(&追加引数),
        vec!["--".to_string(), "--project".to_string(), "/tmp/game".to_string()]
    );
}
