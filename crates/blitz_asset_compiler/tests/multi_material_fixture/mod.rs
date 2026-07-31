//! 複数材質の試験材料となるGLBを組み立てて書き出す工程。返すのは書き出したパスである。
//! Blenderの書き出しに頼らないのは、三角形の並び順が実行ごとに変わり試験材料として固定できないためである。
//! 生成物は実行環境の一時ディレクトリへ置く。作業ディレクトリの下へ書くと、`cargo test`の作業ディレクトリがクレート直下であるためリポジトリの中に生成物が残る。

mod binary;
mod document_json;

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

const JSONチャンク種別: u32 = 0x4E4F_534A;
const BINチャンク種別: u32 = 0x004E_4942;

/// 書き出し先を呼び出しごとに分けるための通し番号。
static 通し番号: AtomicU64 = AtomicU64::new(0);

/// 注意: 書き出し先のファイル名へ通し番号を混ぜる。同じ試験材料を要求する試験が同じ試験バイナリの中で並行して走るため、
/// 固定のファイル名にすると、片方が上書きしている最中にもう片方が読み、glTFの解析が途中のバイト列で失敗する。
pub fn glbを書き出す(名前: &str) -> PathBuf {
    let バイト列 = glbバイト列を組み立てる(&document_json::文書jsonを作る(), &binary::二三角形のバイナリ());
    let ディレクトリ = std::env::temp_dir().join("blitzdrache0_multi_material_fixture");
    match std::fs::create_dir_all(&ディレクトリ) {
        Ok(()) => {}
        Err(誤り) => panic!("試験材料のディレクトリを作れなかった: {誤り}"),
    }
    let 番号 = 通し番号.fetch_add(1, Ordering::Relaxed);
    let パス = ディレクトリ.join(format!("{名前}_{}_{番号}.glb", std::process::id()));
    match std::fs::write(&パス, バイト列) {
        Ok(()) => パス,
        Err(誤り) => panic!("試験材料のGLBを書き出せなかった: {誤り}"),
    }
}

fn glbバイト列を組み立てる(文書json: &str, バイナリ: &[u8]) -> Vec<u8> {
    let json部 = 境界へ揃える(文書json.as_bytes(), b' ');
    let bin部 = 境界へ揃える(バイナリ, 0);
    let mut バイト列 = Vec::new();
    バイト列.extend_from_slice(b"glTF");
    バイト列.extend_from_slice(&2u32.to_le_bytes());
    バイト列.extend_from_slice(&長さをu32へ(28 + json部.len() + bin部.len()).to_le_bytes());
    チャンクを足す(&mut バイト列, JSONチャンク種別, &json部);
    チャンクを足す(&mut バイト列, BINチャンク種別, &bin部);
    バイト列
}

fn チャンクを足す(出力: &mut Vec<u8>, 種別: u32, 中身: &[u8]) {
    出力.extend_from_slice(&長さをu32へ(中身.len()).to_le_bytes());
    出力.extend_from_slice(&種別.to_le_bytes());
    出力.extend_from_slice(中身);
}

fn 長さをu32へ(長さ: usize) -> u32 {
    match u32::try_from(長さ) {
        Ok(値) => 値,
        Err(誤り) => panic!("試験材料の長さがu32を超えた: {誤り}"),
    }
}

fn 境界へ揃える(中身: &[u8], 詰め物: u8) -> Vec<u8> {
    let mut 結果 = 中身.to_vec();
    while !結果.len().is_multiple_of(4) {
        結果.push(詰め物);
    }
    結果
}
