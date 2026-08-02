//! 世代の画像を常駐させる工程。受け取るのは供給元・容量・材質一覧で、返すのはテクスチャ台帳と正準フォールバックの解決である。
//! 積んだ画像は呼び出し元が持つ列へ順に足すため、途中で失敗しても呼び出し元がその世代のために作った画像だけを退役させられる。
//!
//! 不変条件: 画像を列へ足す順番と台帳が発番するスロットの番号は常に一致する。ここが唯一その2つを同時に進める場所である。

use crate::error::レンダラーエラー;
use crate::vulkan::material_table::fallback_usage::正準フォールバック用途;
use crate::vulkan::material_table::pack_input::梱包対象材質;
use crate::vulkan::material_table::packer::fallback_slots::正準フォールバック解決;
use crate::vulkan::material_table::residency_count::世代の常駐枚数;
use crate::vulkan::material_table::supplier::常駐テクスチャ供給元;
use crate::vulkan::material_table::texture_registry::{スロットの引き当て, テクスチャ台帳};
use crate::vulkan::material_table::texture_role::材質テクスチャ役割;
use crate::vulkan::material_table::texture_slot::テクスチャスロット;

pub(super) struct 常駐の結果 {
    pub(super) 台帳: テクスチャ台帳,
    pub(super) フォールバック: 正準フォールバック解決,
}

pub(super) fn 積む<供給元: 常駐テクスチャ供給元>(
    供給元: &mut 供給元,
    画像集合: &mut Vec<供給元::常駐画像>,
    常駐枚数: 世代の常駐枚数,
    材質一覧: &[梱包対象材質<'_>],
) -> Result<常駐の結果, レンダラーエラー> {
    let mut 台帳 = テクスチャ台帳::新規();
    let フォールバック = フォールバックを常駐させる(供給元, 画像集合, &mut 台帳, 常駐枚数)?;
    for 材質 in 材質一覧 {
        for 役割 in 材質テクスチャ役割::全役割 {
            let Some(指定) = 材質.役割の指定(役割) else {
                continue;
            };
            if let スロットの引き当て::常駐させる必要がある(スロット) = 台帳.引き当てる(指定, 役割, 常駐枚数)? {
                積んで並びを確かめる(供給元, 画像集合, スロット, 指定.素材())?;
            }
        }
    }
    Ok(常駐の結果 {
        台帳, フォールバック
    })
}

/// フォールバックを先に常駐させるのは、材質が1件も無い世代でも表が実在する画像だけを持つ状態にするためである。
fn フォールバックを常駐させる<供給元: 常駐テクスチャ供給元>(
    供給元: &mut 供給元,
    画像集合: &mut Vec<供給元::常駐画像>,
    台帳: &mut テクスチャ台帳,
    常駐枚数: 世代の常駐枚数,
) -> Result<正準フォールバック解決, レンダラーエラー> {
    let mut 用途別スロット = Vec::with_capacity(正準フォールバック用途::全用途.len());
    for 用途 in 正準フォールバック用途::全用途 {
        let スロット = 台帳.台帳外のスロットを発番する(常駐枚数)?;
        積んで並びを確かめる(供給元, 画像集合, スロット, &用途.素材を作る())?;
        用途別スロット.push(スロット);
    }
    let 用途別スロット: [テクスチャスロット; 3] = 用途別スロット
        .try_into()
        .unwrap_or_else(|_| panic!("正準フォールバックの用途数とスロット数が食い違った"));
    Ok(正準フォールバック解決::生成する(用途別スロット))
}

fn 積んで並びを確かめる<供給元: 常駐テクスチャ供給元>(
    供給元: &mut 供給元,
    画像集合: &mut Vec<供給元::常駐画像>,
    スロット: テクスチャスロット,
    素材: &crate::texture_material::テクスチャ素材,
) -> Result<(), レンダラーエラー> {
    assert_eq!(
        画像集合.len(),
        スロット.配列添字(),
        "テクスチャスロットの発番順と画像集合の並びが食い違った"
    );
    let 画像 = 供給元.常駐させる(素材)?;
    画像集合.push(画像);
    Ok(())
}
