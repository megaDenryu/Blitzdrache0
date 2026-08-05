//! SPIR-Vのバイト列と32ビット語の並びの相互変換、および命令の先頭位置の列挙。
//! 受け取るのはバイト列または語の並び、返すのは語の並び・バイト列・命令の先頭位置の並びである。
//!
//! 語の詰め方(先頭5語がヘッダ、以降は語数と命令コードを詰めた1語で始まる可変長の命令)を1箇所へ閉じるのは、
//! 装飾を読む工程と装飾を挿し込む工程が同じ詰め方をそれぞれ持たないようにするためである。

use super::error::位置の不変装飾エラー;

const 魔法数: u32 = 0x0723_0203;
const ヘッダ語数: usize = 5;

pub(super) fn 語へ分解する(spirv: &[u8]) -> Result<Vec<u32>, 位置の不変装飾エラー> {
    if !spirv.len().is_multiple_of(4) || spirv.len() < ヘッダ語数 * 4 {
        return Err(位置の不変装飾エラー::語の並びとして読めない(spirv.len()));
    }
    let 語一覧: Vec<u32> = spirv.chunks_exact(4).map(|塊| u32::from_le_bytes([塊[0], 塊[1], 塊[2], 塊[3]])).collect();
    if 語一覧[0] != 魔法数 {
        return Err(位置の不変装飾エラー::魔法数が合わない(語一覧[0]));
    }
    Ok(語一覧)
}

pub(super) fn バイト列へ戻す(語一覧: &[u32]) -> Vec<u8> {
    語一覧.iter().flat_map(|語| 語.to_le_bytes()).collect()
}

/// 1つの命令が占める語の範囲。
#[derive(Clone, Copy)]
pub(super) struct 命令の範囲 {
    pub(super) 命令コード: u16,
    pub(super) 開始: usize,
    pub(super) 終了: usize,
}

impl 命令の範囲 {
    /// この命令の被演算子の語。先頭の語数と命令コードを詰めた1語を除いた並びである。
    pub(super) fn 被演算子(self, 語一覧: &[u32]) -> &[u32] {
        &語一覧[self.開始 + 1..self.終了]
    }
}

/// ヘッダの後ろに並ぶ命令を先頭から列挙する。語数0の語や末尾を越える語数はそこで打ち切る(壊れた並びを読み進めないため)。
pub(super) fn 命令一覧を数える(語一覧: &[u32]) -> Vec<命令の範囲> {
    let mut 一覧 = Vec::new();
    let mut 位置 = ヘッダ語数;
    while 位置 < 語一覧.len() {
        let 語数 = usize::try_from(語一覧[位置] >> 16).unwrap_or(0);
        let 命令コード = u16::try_from(語一覧[位置] & 0xFFFF).unwrap_or(0);
        if 語数 == 0 || 位置 + 語数 > 語一覧.len() {
            break;
        }
        一覧.push(命令の範囲 {
            命令コード,
            開始: 位置,
            終了: 位置 + 語数,
        });
        位置 += 語数;
    }
    一覧
}
