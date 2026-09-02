//! SHA-256の既知の入力との一致。空の入力・"abc"・2塊にまたがる長い入力の3つで、詰め物と長さの符号化と圧縮を固定する。

use super::sha256::十六進の指紋を計算する;

#[test]
fn 空の入力の指紋は既知の値である() {
    assert_eq!(
        十六進の指紋を計算する(b""),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}

#[test]
fn abcの指紋は既知の値である() {
    assert_eq!(
        十六進の指紋を計算する(b"abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

#[test]
fn 二塊にまたがる入力の指紋は既知の値である() {
    assert_eq!(
        十六進の指紋を計算する(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"),
        "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
    );
}
