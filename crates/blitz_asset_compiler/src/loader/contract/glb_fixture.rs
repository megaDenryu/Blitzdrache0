//! 検査の試験材料となるGLBをコードから組み立てる工程。受け取るのはJSON本体と三角形1枚ぶんのバイナリ、返すのは書き出したパスである。
//! Blenderの書き出しに依存しないのは、三角形の並び順が実行ごとに変わり試験材料として固定できないためである(参照: `_doc/設計/Blenderアセット運用.md`)。
//! 生成物は実行環境の一時ディレクトリへ置く。作業ディレクトリの下へ書くと、`cargo test`の作業ディレクトリがクレート直下であるためリポジトリの中に生成物が残る。

use std::path::PathBuf;

const JSONチャンク種別: u32 = 0x4E4F_534A;
const BINチャンク種別: u32 = 0x004E_4942;

/// 位置3頂点・法線3頂点・UV3頂点・インデックス3個を、この順に並べたバイナリ。JSON側のバッファビューの並びと長さがこの順序に一致する。
pub(super) fn 三角形のバイナリ() -> Vec<u8> {
    let mut バイト列 = Vec::new();
    for 値 in [0.0f32, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0] {
        バイト列.extend_from_slice(&値.to_le_bytes());
    }
    for 値 in [0.0f32, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0] {
        バイト列.extend_from_slice(&値.to_le_bytes());
    }
    for 値 in [0.0f32, 0.0, 1.0, 0.0, 0.0, 1.0] {
        バイト列.extend_from_slice(&値.to_le_bytes());
    }
    for 値 in [0u16, 1, 2] {
        バイト列.extend_from_slice(&値.to_le_bytes());
    }
    バイト列
}

pub(super) fn glbを書き出す(名前: &str, 文書json: &str, バイナリ: &[u8]) -> Result<PathBuf, String> {
    let バイト列 = glbバイト列を組み立てる(文書json, バイナリ)?;
    let ディレクトリ = std::env::temp_dir().join("blitzdrache0_contract_fixture");
    std::fs::create_dir_all(&ディレクトリ).map_err(|誤り| 誤り.to_string())?;
    let パス = ディレクトリ.join(format!("{名前}.glb"));
    std::fs::write(&パス, バイト列).map_err(|誤り| 誤り.to_string())?;
    Ok(パス)
}

fn glbバイト列を組み立てる(文書json: &str, バイナリ: &[u8]) -> Result<Vec<u8>, String> {
    let json部 = 境界へ揃える(文書json.as_bytes(), b' ');
    let bin部 = 境界へ揃える(バイナリ, 0);
    let 全長 = 長さをu32へ(28 + json部.len() + bin部.len())?;

    let mut バイト列 = Vec::new();
    バイト列.extend_from_slice(b"glTF");
    バイト列.extend_from_slice(&2u32.to_le_bytes());
    バイト列.extend_from_slice(&全長.to_le_bytes());
    チャンクを足す(&mut バイト列, JSONチャンク種別, &json部)?;
    チャンクを足す(&mut バイト列, BINチャンク種別, &bin部)?;
    Ok(バイト列)
}

fn チャンクを足す(出力: &mut Vec<u8>, 種別: u32, 中身: &[u8]) -> Result<(), String> {
    出力.extend_from_slice(&長さをu32へ(中身.len())?.to_le_bytes());
    出力.extend_from_slice(&種別.to_le_bytes());
    出力.extend_from_slice(中身);
    Ok(())
}

fn 長さをu32へ(長さ: usize) -> Result<u32, String> {
    u32::try_from(長さ).map_err(|誤り| format!("試験材料の長さがu32を超えた: {誤り}"))
}

/// GLBの各チャンクは4バイト境界へ揃える必要がある。JSONは空白、バイナリは0で詰める。
fn 境界へ揃える(中身: &[u8], 詰め物: u8) -> Vec<u8> {
    let mut 結果 = 中身.to_vec();
    while !結果.len().is_multiple_of(4) {
        結果.push(詰め物);
    }
    結果
}
