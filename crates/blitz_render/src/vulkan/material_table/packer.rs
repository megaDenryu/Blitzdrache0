//! 検証済み材質から、材質レコードとテクスチャスロットの解決を同時に作る1工程。担当するのは、
//! 係数・特徴ビット・スロットの3つが必ず同じ材質から同時に決まることと、その整合を作った直後に検証することである。
//!
//! 注意: 「テクスチャ無し」の変換をここ以外で書いてはならない。用途ごとの正準フォールバックのスロットを知るのはこの工程だけであり、
//! 外へ予約添字の約束を公開しない。テクスチャIDの解決も台帳へ委ね、この工程は生の添字を組み立てない。
//! 参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「材質レコードとテクスチャ台帳」

pub(in crate::vulkan::material_table) mod fallback_slots;
mod validate;

use std::collections::HashMap;

use crate::error::材質資源表エラー;

use crate::vulkan::material_variant::{材質の3軸, 表面描画状態};

use super::feature_set::材質特徴集合;
use super::generation_record::世代内材質レコード;
use super::generation_resolution::世代内材質解決;
use super::material_id::大域材質ID;
use super::pack_input::梱包対象材質;
use super::record_index::材質レコード添字;
use super::texture_registry::テクスチャ台帳;
use super::texture_role::材質テクスチャ役割;
use super::texture_slot::テクスチャスロット;
use fallback_slots::正準フォールバック解決;

pub(in crate::vulkan::material_table) struct 梱包結果 {
    pub(in crate::vulkan::material_table) レコード列: Vec<世代内材質レコード>,
    pub(in crate::vulkan::material_table) 材質別解決: HashMap<大域材質ID, 世代内材質解決>,
}

pub(in crate::vulkan::material_table) fn 梱包する(
    材質一覧: &[梱包対象材質<'_>],
    台帳: &テクスチャ台帳,
    フォールバック: &正準フォールバック解決,
) -> Result<梱包結果, 材質資源表エラー> {
    let mut レコード列 = Vec::with_capacity(材質一覧.len());
    let mut 材質別解決 = HashMap::with_capacity(材質一覧.len());
    for 材質 in 材質一覧 {
        let 添字番号 = u32::try_from(レコード列.len()).unwrap_or_else(|_| panic!("材質レコードの件数がu32に収まらない"));
        let (レコード, 変種キー) = 一件を梱包する(材質, 台帳, フォールバック)?;
        validate::検証する(材質.材質id(), &レコード, フォールバック)?;
        let 解決 = 世代内材質解決::生成する(材質レコード添字::生成する(添字番号), 変種キー);
        if 材質別解決.insert(材質.材質id(), 解決).is_some() {
            return Err(材質資源表エラー::材質IDの重複 {
                材質id: 材質.材質id().値()
            });
        }
        レコード列.push(レコード);
    }
    Ok(梱包結果 {
        レコード列, 材質別解決
    })
}

/// テクスチャの有無から特徴ビットとスロットを同時に決め、同じ特徴集合から材質変種キーも同時に導く。
/// 無しなら実在するフォールバックのスロットを置き、有りなら台帳が解決したスロットを置く。
/// 台帳に無いIDは世代の構築が常駐させ損ねた場合だけであり、材質の入力の誤りではない。
/// 表面描画状態が常に不透明片面なのは、現行の入力境界がそれ以外を実行時形式へ運ばないためである(参照: `material_variant`)。
fn 一件を梱包する(
    材質: &梱包対象材質<'_>,
    台帳: &テクスチャ台帳,
    フォールバック: &正準フォールバック解決,
) -> Result<(世代内材質レコード, crate::vulkan::material_variant::材質変種キー), 材質資源表エラー> {
    let mut 有無 = [false; 3];
    let mut 役割別スロット: [テクスチャスロット; 3] =
        std::array::from_fn(|添字| フォールバック.用途で引く(材質テクスチャ役割::全役割[添字].正準フォールバック用途()));
    for 役割 in 材質テクスチャ役割::全役割 {
        let Some(指定) = 材質.役割の指定(役割) else {
            continue;
        };
        let Some(スロット) = 台帳.解決する(指定.テクスチャid()) else {
            return Err(材質資源表エラー::特徴ビットとスロットの不整合 {
                材質id: 材質.材質id().値()
            });
        };
        有無[役割.配列添字()] = true;
        役割別スロット[役割.配列添字()] = スロット;
    }
    let 三軸 = 材質の3軸::生成する(
        材質.シェーディングモデル(),
        材質特徴集合::役割ごとの有無から導出する(有無),
        表面描画状態::不透明片面(),
    );
    let 変種キー = 三軸.変種キーへ正規化する()?;
    let レコード = 世代内材質レコード::組み立てる(
        材質.ベースカラー係数(),
        材質.金属度係数(),
        材質.粗さ係数(),
        三軸.特徴集合(),
        役割別スロット,
    );
    Ok((レコード, 変種キー))
}
