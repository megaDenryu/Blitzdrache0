//! 逃がし記号の復号の検査。走査の側でなく、原文の中身から実行時の中身を作る規則だけを見る。

use super::escape::逃がしを復号する;

#[test]
fn 記号の逃がしを実行時の文字へ写す() {
    let 復号 = 逃がしを復号する(r#"改行\nタブ\t逆斜線\\引用符\""#);
    assert_eq!(復号, "改行\nタブ\t逆斜線\u{5c}引用符\u{22}");
}

#[test]
fn 十六進の逃がしを実行時の文字へ写す() {
    assert_eq!(逃がしを復号する(r"a\x2eb"), "a.b");
}

#[test]
fn 符号位置の逃がしを実行時の文字へ写す() {
    assert_eq!(逃がしを復号する(r"a\u{2e}b"), "a.b");
}

#[test]
fn 桁の下線を読み飛ばして符号位置へ写す() {
    assert_eq!(逃がしを復号する(r"a\u{2_e}b"), "a.b");
    assert_eq!(逃がしを復号する(r"a\u{00_00_2e}b"), "a.b");
}

#[test]
fn 閉じの波括弧が無い符号位置は原文のまま残す() {
    assert_eq!(逃がしを復号する(r"a\u{2eb"), r"a\u{2eb");
    assert_eq!(逃がしを復号する(r"a\u2e"), r"a\u2e");
}

#[test]
fn 逃がしで書いた綴りと素の綴りが同じ中身になる() {
    assert_eq!(逃がしを復号する(r"shaders/scene\x2eslang"), "shaders/scene.slang");
}

#[test]
fn 復号できない逃がしは原文のまま残す() {
    assert_eq!(逃がしを復号する(r"a\q b"), r"a\q b");
    assert_eq!(逃がしを復号する(r"a\xzzb"), r"a\xzzb");
}

#[test]
fn 行末の逃がしは改行と続く字下げを消す() {
    let 原文 = concat!("a", r"\", "\n    b");
    assert_eq!(逃がしを復号する(原文), "ab");
}
