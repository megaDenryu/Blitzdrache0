//! 材質変種キーの語彙と、材質の3軸から正規化済みのキーを導出する中央の1工程。担当するのは、
//! 「どの材質がどのシェーダーと固定機能で描かれるか」を決める唯一の場所を持つことである。
//!
//! キーが正規化済みなのは、原材料の軸をそのままPSOの変種にすると、同じシェーダーと同じ固定機能のパイプラインを
//! 何本も重複生成するためである。現行の画素段は材質特徴ビットを読まず、テクスチャを持たない役割も正準フォールバックの
//! スロットを標本するため、テクスチャの有無8通りはどれも同じパイプラインで描ける。
//! 3軸の型は`shading_model`・`feature`(材質資源表の材質特徴集合)・`surface_state`、3つを束ねた入力は`three_axes`、
//! 描ける組合せの閉じた一覧は`capability_table`、導出の結果は`variant_key`にある。
//! 参照: `_doc/設計/マルチマテリアルと材質境界.md`「材質の3軸」

mod capability_table;
mod error;
mod shading_model;
mod surface_state;
mod three_axes;
mod variant_key;
#[cfg(test)]
mod variant_tests;

pub(crate) use error::材質能力エラー;
pub(crate) use shading_model::シェーディングモデル種別;
pub(crate) use surface_state::表面描画状態;
pub(crate) use three_axes::材質の3軸;
pub(crate) use variant_key::材質変種キー;
