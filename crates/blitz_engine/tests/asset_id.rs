//! アセットIDの検証: 空文字拒否と正常系。

use blitz_engine::アセットID;

#[test]
fn 空文字は拒否される() {
    let 結果 = アセットID::生成する("");
    assert!(結果.is_err());
}

#[test]
fn 空でない名前は生成できる() {
    let 結果 = アセットID::生成する("quad");
    match 結果 {
        Ok(id) => assert_eq!(id.文字列を返す(), "quad"),
        Err(誤り) => panic!("空でない名前は生成に成功するはず: {誤り}"),
    }
}
