//! 最終SPIR-Vの中で、名前の付いた配列変数への参照にNonUniformの装飾が付いているかを数える問い合わせ。
//! 受け取るのはSPIR-Vのバイト列と変数名、返すのは「その変数への参照が何件あり、そのうち装飾が付いたものが何件か」である。
//!
//! 原文の文字列でなく最終SPIR-Vを見るのは、`NonUniformResourceIndex`が上流で落ちても原文からは分からないためである
//! (参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「段階導入」の段4bの検収ゲート(i))。
//! 前提: 仕様は装飾を添字そのものでなく、その添字で作ったポインタ(アクセス連鎖の結果)へ付けると定める。

use super::spirv_module::{名前からidを引く, 命令へ読み解く, 装飾の付いたidを集める};

pub(super) struct 非一様装飾の集計 {
    /// その変数を基点とするアクセス連鎖の件数。
    pub(super) 参照件数: usize,
    /// そのうち、アクセス連鎖の結果にNonUniformの装飾が付いていた件数。
    pub(super) 非一様装飾付きの件数: usize,
}

const 命令_OP_ACCESS_CHAIN: u16 = 65;
const 装飾_NON_UNIFORM: u32 = 5300;

pub(super) fn 集計する(spirv: &[u8], 変数名: &str) -> Result<非一様装飾の集計, String> {
    let 命令一覧 = 命令へ読み解く(spirv)?;
    let 対象id = 名前からidを引く(&命令一覧, 変数名)?;
    let 非一様id一覧 = 装飾の付いたidを集める(&命令一覧, 装飾_NON_UNIFORM);
    let mut 集計 = 非一様装飾の集計 {
        参照件数: 0,
        非一様装飾付きの件数: 0,
    };
    for 命令 in &命令一覧 {
        // OpAccessChainの語は[結果の型, 結果id, 基点id, 添字...]の順に並ぶ。
        if 命令.命令コード != 命令_OP_ACCESS_CHAIN || 命令.語一覧.len() < 4 || 命令.語一覧[2] != 対象id {
            continue;
        }
        集計.参照件数 += 1;
        if 非一様id一覧.contains(&命令.語一覧[1]) {
            集計.非一様装飾付きの件数 += 1;
        }
    }
    Ok(集計)
}
