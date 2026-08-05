//! SPIR-Vのバイト列を命令の並びへ読み解く工程。受け取るのはバイト列、返すのは命令の並びと、そこから名前と装飾を引く操作である。
//!
//! 読み解きを問い合わせと分けるのは、語の詰め方(先頭5語がヘッダ、以降は語数と命令コードを詰めた1語で始まる可変長の命令)が
//! 何を問い合わせるかと無関係だからである。

/// SPIR-Vの語の並びに現れる命令1つ。
pub(super) struct 命令 {
    pub(super) 命令コード: u16,
    pub(super) 語一覧: Vec<u32>,
}

const 命令_OP_NAME: u16 = 5;
const 命令_OP_DECORATE: u16 = 71;
const 魔法数: u32 = 0x0723_0203;

pub(super) fn 命令へ読み解く(spirv: &[u8]) -> Result<Vec<命令>, String> {
    let 語一覧 = 語へ分解する(spirv)?;
    let mut 命令一覧 = Vec::new();
    let mut 位置 = 5usize;
    while 位置 < 語一覧.len() {
        let 語数 = usize::try_from(語一覧[位置] >> 16).unwrap_or(0);
        let 命令コード = u16::try_from(語一覧[位置] & 0xFFFF).unwrap_or(0);
        if 語数 == 0 || 位置 + 語数 > 語一覧.len() {
            break;
        }
        命令一覧.push(命令 {
            命令コード,
            語一覧: 語一覧[位置 + 1..位置 + 語数].to_vec(),
        });
        位置 += 語数;
    }
    Ok(命令一覧)
}

/// OpNameの語は[対象id, 文字列...]の順に並ぶ。文字列はナル終端のUTF-8を語へ詰めたものである。
pub(super) fn 名前からidを引く(命令一覧: &[命令], 名前: &str) -> Result<u32, String> {
    for 命令 in 命令一覧 {
        if 命令.命令コード != 命令_OP_NAME || 命令.語一覧.is_empty() {
            continue;
        }
        if 文字列へ戻す(&命令.語一覧[1..]) == 名前 {
            return Ok(命令.語一覧[0]);
        }
    }
    Err(format!("SPIR-Vに{名前}という名前の宣言が無い"))
}

/// SPIR-Vの装飾BuiltInの値と組み込みPositionの値。
const 装飾_BUILT_IN: u32 = 11;
const 組み込み_POSITION: u32 = 0;

/// 位置の組み込み出力として装飾された変数のid。頂点段のSPIR-Vには必ず1つある。
pub(super) fn 位置の組み込み出力のidを引く(命令一覧: &[命令]) -> Result<u32, String> {
    命令一覧
        .iter()
        .filter(|命令| 命令.命令コード == 命令_OP_DECORATE && 命令.語一覧.len() >= 3)
        .find(|命令| 命令.語一覧[1] == 装飾_BUILT_IN && 命令.語一覧[2] == 組み込み_POSITION)
        .map(|命令| 命令.語一覧[0])
        .ok_or_else(|| "SPIR-Vに位置の組み込み出力の宣言が無い".to_string())
}

/// OpDecorateの語は[対象id, 装飾, 追加の値...]の順に並ぶ。
pub(super) fn 装飾の付いたidを集める(命令一覧: &[命令], 装飾: u32) -> Vec<u32> {
    命令一覧
        .iter()
        .filter(|命令| 命令.命令コード == 命令_OP_DECORATE && 命令.語一覧.len() >= 2 && 命令.語一覧[1] == 装飾)
        .map(|命令| 命令.語一覧[0])
        .collect()
}

fn 語へ分解する(spirv: &[u8]) -> Result<Vec<u32>, String> {
    if !spirv.len().is_multiple_of(4) || spirv.len() < 20 {
        return Err(format!("SPIR-Vのバイト長が語の並びとして読めない: {}", spirv.len()));
    }
    let 語一覧: Vec<u32> = spirv.chunks_exact(4).map(|塊| u32::from_le_bytes([塊[0], 塊[1], 塊[2], 塊[3]])).collect();
    if 語一覧[0] != 魔法数 {
        return Err(format!("SPIR-Vの魔法数が合わない: {:#x}", 語一覧[0]));
    }
    Ok(語一覧)
}

fn 文字列へ戻す(語一覧: &[u32]) -> String {
    let バイト列: Vec<u8> = 語一覧.iter().flat_map(|語| 語.to_le_bytes()).take_while(|バイト| *バイト != 0).collect();
    String::from_utf8_lossy(&バイト列).into_owned()
}
