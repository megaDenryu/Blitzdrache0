//! 頂点・インデックス列をGPU境界のバイト列へ変換する。
//! ポインタ再解釈ではなく成分ごとの`to_le_bytes`で組み立て、構造体パディングの
//! 前提に依存しない。

use crate::vertex::頂点;

pub(super) fn 頂点をバイト列にする(頂点一覧: &[頂点]) -> Vec<u8> {
    let mut バイト列 = Vec::with_capacity(頂点一覧.len() * 24);
    for 頂点 in 頂点一覧 {
        for 成分 in 頂点.位置 {
            バイト列.extend_from_slice(&成分.to_le_bytes());
        }
        for 成分 in 頂点.色 {
            バイト列.extend_from_slice(&成分.to_le_bytes());
        }
    }
    バイト列
}

pub(super) fn インデックスをバイト列にする(インデックス一覧: &[u32]) -> Vec<u8> {
    let mut バイト列 = Vec::with_capacity(インデックス一覧.len() * 4);
    for &インデックス in インデックス一覧 {
        バイト列.extend_from_slice(&インデックス.to_le_bytes());
    }
    バイト列
}
