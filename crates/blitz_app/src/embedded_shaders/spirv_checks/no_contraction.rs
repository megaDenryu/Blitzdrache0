//! 自動露出の単精度の四則へ、最終SPIR-Vで融合しない装飾(NoContraction)が付いていることの検査。
//!
//! 原文の`precise`修飾では足りないことが分かっているためこの検査を置く。slangc 2025.11の生成物には
//! `precise`が装飾を1つも残さず(`-fp-mode precise`でも同じ)、積和融合を許すかどうかがドライバー任せになる。
//! 融合が起きると境界のすぐ近くの画素が共有した境界の反対側のビンへ落ち、CPU正本とGPUのビンが1つずれる。
//! 装飾は`shaders/unfused_arithmetic.slang`のSPIR-Vインライン記述が付ける。
//!
//! 「全部の単精度の四則に付いていること」を条件にするのは、どの演算がCPU正本との一致契約に含まれるかを
//! 検査側がもう1つの台帳として持たないためである。自動露出の2つのコンピュートの浮動小数の演算はすべて正本の写しである。

use super::spirv_module::{命令へ読み解く, 装飾の付いたidを集める};

const 集計SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/auto_exposure_histogram.spv"));
const 導出と適応SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/auto_exposure_resolve.spv"));

/// 融合しうる単精度の演算の命令コード。SPIR-VのOpFAdd・OpFSub・OpFMulである。
const 融合しうる命令コード一覧: [u16; 3] = [129, 131, 133];
/// SPIR-Vの装飾NoContractionの値。
const 装飾_NO_CONTRACTION: u32 = 42;

#[test]
fn 集計の単精度の四則はすべて融合しない装飾が付く() {
    装飾が全演算へ付くことを確かめる(集計SPIRV, "集計");
}

#[test]
fn 導出と適応の単精度の四則はすべて融合しない装飾が付く() {
    装飾が全演算へ付くことを確かめる(導出と適応SPIRV, "導出と適応");
}

fn 装飾が全演算へ付くことを確かめる(spirv: &[u8], エントリ名: &str) {
    let 命令一覧 = match 命令へ読み解く(spirv) {
        Ok(一覧) => 一覧,
        Err(誤り) => panic!("{エントリ名}のSPIR-Vを読めない: {誤り}"),
    };
    let 装飾済み = 装飾の付いたidを集める(&命令一覧, 装飾_NO_CONTRACTION);
    // OpFAdd等の語は[結果の型, 結果のid, 被演算子...]の順に並ぶ。
    let 演算のid一覧: Vec<u32> = 命令一覧
        .iter()
        .filter(|命令| 融合しうる命令コード一覧.contains(&命令.命令コード) && 命令.語一覧.len() >= 2)
        .map(|命令| 命令.語一覧[1])
        .collect();
    assert!(!演算のid一覧.is_empty(), "{エントリ名}に単精度の四則が1つも無い(写しが消えている)");
    let 付いていない: Vec<u32> = 演算のid一覧.iter().copied().filter(|id| !装飾済み.contains(id)).collect();
    assert!(
        付いていない.is_empty(),
        "{エントリ名}の単精度の四則{}件のうち{}件に融合しない装飾が付いていない(id{:?})",
        演算のid一覧.len(),
        付いていない.len(),
        付いていない
    );
}
