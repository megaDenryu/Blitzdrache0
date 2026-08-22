//! ソースアセットの綴りの2つの型の検査。受理する綴りから導かれるパスと、拒む綴りの両方を見る。
//!
//! 拒む側を`should_panic`で書くのは、`生成する`が`const fn`であり、破れを型付きの誤りでなく
//! コンパイルの失敗として出すためである。実行時に呼べば同じ判定が`panic`として現れる。

use std::path::{Path, PathBuf};

use super::{ソースアセットのファイル名, ソースアセットの相対パス};

#[test]
fn 相対パスの末尾からファイル名を取り出せる() {
    let 相対パス = ソースアセットの相対パス::生成する("samples/Fox/Fox.glb");
    assert_eq!(相対パス.末尾のファイル名(), ソースアセットのファイル名::生成する("Fox.glb"));
}

#[test]
fn 区切りを持たない相対パスの末尾のファイル名は綴りそのもの() {
    let 相対パス = ソースアセットの相対パス::生成する("Fox.glb");
    assert_eq!(相対パス.末尾のファイル名().綴りを見せる(), "Fox.glb");
}

#[test]
fn ルートの下のファイルを組める() {
    let 相対パス = ソースアセットの相対パス::生成する("samples/Fox/Fox.glb");
    assert_eq!(
        相対パス.ルートの下のファイル(Path::new("assets")),
        PathBuf::from("assets/samples/Fox/Fox.glb")
    );
}

#[test]
fn ルートの下の収める場所は末尾のファイル名を落とした場所になる() {
    let 相対パス = ソースアセットの相対パス::生成する("samples/Fox/Fox.glb");
    assert_eq!(相対パス.ルートの下の収める場所(Path::new("assets")), PathBuf::from("assets/samples/Fox"));
}

#[test]
fn 区切りを持たない相対パスを収める場所はルートそのもの() {
    let 相対パス = ソースアセットの相対パス::生成する("Fox.glb");
    assert_eq!(相対パス.ルートの下の収める場所(Path::new("assets")), PathBuf::from("assets"));
}

#[test]
fn ファイル名からディレクトリの直下のファイルを組める() {
    let ファイル名 = ソースアセットのファイル名::生成する("quad.bin");
    assert_eq!(
        ファイル名.ディレクトリの直下のファイル(Path::new("target/smoke")),
        PathBuf::from("target/smoke/quad.bin")
    );
}

#[test]
#[should_panic(expected = "逆斜線")]
fn 逆斜線を含む相対パスを拒む() {
    let _相対パス = ソースアセットの相対パス::生成する("props\\barrel.glb");
}

#[test]
#[should_panic(expected = "ルートの外へ出る")]
fn 上の場所へ出る成分を含む相対パスを拒む() {
    let _相対パス = ソースアセットの相対パス::生成する("props/../barrel.glb");
}

#[test]
#[should_panic(expected = "ルートからの相対でなく")]
fn 斜線で始まる相対パスを拒む() {
    let _相対パス = ソースアセットの相対パス::生成する("/props/barrel.glb");
}

#[test]
#[should_panic(expected = "ディレクトリを指している")]
fn 斜線で終わる相対パスを拒む() {
    let _相対パス = ソースアセットの相対パス::生成する("props/");
}

#[test]
#[should_panic(expected = "区切りを含み")]
fn 区切りを含むファイル名を拒む() {
    let _ファイル名 = ソースアセットのファイル名::生成する("props/barrel.glb");
}

#[test]
#[should_panic(expected = "コロンを含み")]
fn コロンを含む相対パスを拒む() {
    let _相対パス = ソースアセットの相対パス::生成する("C:escape/barrel.glb");
}
