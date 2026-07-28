//! 植生インスタンスの検証用世界のソースアセット一式。1つの原型glTFと、それを指す1チャンクの目録ソースを書き出す。
//! 世界を1チャンクに留めるのは、この世界の役割が「インスタンス群が実際に描けること」の確認であり、
//! 既存の板の世界・地形世界の25チャンクと同じ広がりを持つ必要がないためである。
//! 参照: `_doc/設計/植生インスタンスと物量計測.md`「インスタンス群の表現」

mod geometry;
mod gltf_json;

use std::path::Path;

use blitz_engine::チャンク座標;

use crate::directory_source::{目録ソースを作る, 目録項目};

/// 原型の直方体の水平半辺と高さ。個体が画面上で塊として見え、かつ格子の刻みより十分小さい寸法にする。
const 原型の半辺: f32 = 0.8;
const 原型の高さ: f32 = 3.0;

const 共有バッファファイル名: &str = "archetype.bin";
const 原型文書ファイル名: &str = "archetype.gltf";
const 目録ソースファイル名: &str = "chunk_directory.txt";

/// この世界のチャンクは原点1つだけである。
const チャンクのアセット識別子: &str = "vegetation_chunk";

pub(crate) fn 書き出す(出力先ディレクトリ: &Path) -> Result<(), String> {
    let バッファ = geometry::バッファバイト列を作る();
    書き込む(&出力先ディレクトリ.join(共有バッファファイル名), &バッファ)?;
    書き込む(
        &出力先ディレクトリ.join(原型文書ファイル名),
        gltf_json::文書を作る(バッファ.len()).as_bytes(),
    )?;
    let 項目一覧 = vec![目録項目 {
        座標: チャンク座標::生成する(0, 0),
        アセット識別子: チャンクのアセット識別子.to_string(),
        ソース相対パス: 原型文書ファイル名.to_string(),
    }];
    書き込む(&出力先ディレクトリ.join(目録ソースファイル名), 目録ソースを作る(&項目一覧).as_bytes())
}

fn 書き込む(パス: &Path, バイト列: &[u8]) -> Result<(), String> {
    std::fs::write(パス, バイト列).map_err(|誤り| format!("{}: {誤り}", パス.display()))
}
