//! 局所可視性補正の検収とフレームダンプの排他を検査する。検収は深度画像を合成深度で丸ごと書き換えるため、
//! 同じフレームで撮った絵は本番のジオメトリを映さない。両方を求めた起動は成功させない。
//!
//! 順序を変えた2つを別のテストにするのは、片方だけでは走査の途中で落とす実装も通ってしまうためである。
//! その実装では先に来たほうだけが検査され、入れ替えた起動が黙って通る。
//!
//! ダンプの3つの引数を並べるのは、排他がベース名の指定そのものでなく「読み戻す画像を求めたこと」に対して
//! 立つことを固定するためである。提示画像だけを検査すると、圧縮前のHDRと最終深度が漏れる。

use super::{引数を解析する, 起動要求, 起動設定};

fn 描画設定を解析する(引数一覧: &[String]) -> 起動設定 {
    match 引数を解析する(引数一覧) {
        Ok(起動要求::描画実行(設定)) => *設定,
        Ok(報告) => panic!("描画実行の要求になるはず(報告の種別{})", 報告.呼び名()),
        Err(誤り) => panic!("有効な引数は解析できるはず: {誤り}"),
    }
}

fn 検収だけの引数() -> [String; 2] {
    ["--local-visibility-shape".to_string(), "concave".to_string()]
}

/// 片方だけの指定が受理されることを同じテストで確かめ、失敗が組み合わせによるものだと分かる形にする。
#[test]
fn 検収を先に指定してからフレームダンプを重ねると失敗する() {
    assert!(描画設定を解析する(&検収だけの引数()).読み戻し検収.局所可視性の検収の形.is_some());

    for ダンプ引数 in ["--dump-frame", "--dump-hdr-frame", "--dump-depth-frame"] {
        let 正順 = [
            "--local-visibility-shape".to_string(),
            "concave".to_string(),
            ダンプ引数.to_string(),
            "target/a".to_string(),
        ];
        assert!(引数を解析する(&正順).is_err(), "{ダンプ引数}を後に重ねた起動が通った");
    }
}

#[test]
fn フレームダンプを先に指定してから検収を重ねると失敗する() {
    let ダンプだけ = ["--dump-frame".to_string(), "target/a".to_string()];
    assert!(描画設定を解析する(&ダンプだけ).フレームダンプ先.書き出すか());

    for ダンプ引数 in ["--dump-frame", "--dump-hdr-frame", "--dump-depth-frame"] {
        let 逆順 = [
            ダンプ引数.to_string(),
            "target/a".to_string(),
            "--local-visibility-shape".to_string(),
            "concave".to_string(),
        ];
        assert!(引数を解析する(&逆順).is_err(), "{ダンプ引数}を先に置いた起動が通った");
    }
}

/// 形の語が不正なら、組み合わせを見る前に引数そのものの誤りとして落ちる。
#[test]
fn 知らない形の語は受け付けない() {
    let 引数 = ["--local-visibility-shape".to_string(), "torus".to_string()];
    assert!(引数を解析する(&引数).is_err());
}
