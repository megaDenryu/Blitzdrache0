//! フレームダンプの引数の排他を検査する。1回の起動で読み戻せる画像は1枚であり、2つを同時に求めた起動は成功させない。
//!
//! 順序を変えた2つを別のテストにするのは、片方だけでは「先に来たほうが勝つ」実装も「後から来たほうが勝つ」実装も通ってしまうためである。
//! どちらの実装も、利用者が求めた画像とは別の画像だけを書いて成功として終える。

use super::{引数を解析する, 起動要求, 起動設定};

fn 描画設定を解析する(引数一覧: &[String]) -> 起動設定 {
    match 引数を解析する(引数一覧) {
        Ok(起動要求::描画実行(設定)) => *設定,
        Ok(報告) => panic!("描画実行の要求になるはず(報告の種別{})", 報告.呼び名()),
        Err(誤り) => panic!("有効な引数は解析できるはず: {誤り}"),
    }
}

/// 片方だけの指定が受理されることを同じテストで確かめ、失敗が組み合わせによるものだと分かる形にする。
#[test]
fn 提示画像を先に指定してから圧縮前hdrを重ねると失敗する() {
    let 単独 = ["--dump-frame".to_string(), "target/a".to_string()];
    assert!(描画設定を解析する(&単独).フレームダンプ先.書き出すか());

    let 正順 = [
        "--dump-frame".to_string(),
        "target/a".to_string(),
        "--dump-hdr-frame".to_string(),
        "target/b".to_string(),
    ];
    assert!(引数を解析する(&正順).is_err());
}

#[test]
fn 圧縮前hdrを先に指定してから提示画像を重ねると失敗する() {
    let 単独 = ["--dump-hdr-frame".to_string(), "target/b".to_string()];
    assert!(描画設定を解析する(&単独).フレームダンプ先.書き出すか());

    let 逆順 = [
        "--dump-hdr-frame".to_string(),
        "target/b".to_string(),
        "--dump-frame".to_string(),
        "target/a".to_string(),
    ];
    assert!(引数を解析する(&逆順).is_err());
}
