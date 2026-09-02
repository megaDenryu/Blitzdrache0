//! 布データの生成(判断52): 粒子グリッド+距離拘束+曲げ拘束+描画用インデックス。

use super::data::布データ;
use super::error::布生成エラー;
use super::grid_bending_constraints::曲げ拘束一覧を作る;
use super::grid_constraints::距離拘束一覧を作る;
use super::grid_indices::{インデックス一覧を作る, 上端行を作る};
use super::grid_particles::粒子一覧を作る;
use super::index_convert::usizeへ;
use super::spec::布仕様;

/// 布仕様から布データ一式を組み立てる(判断52)。
pub fn 布を生成する(仕様: &布仕様) -> Result<布データ, 布生成エラー> {
    let 一辺粒子数 = usizeへ(仕様.一辺粒子数());
    Ok(布データ {
        粒子一覧: 粒子一覧を作る(仕様, 一辺粒子数),
        距離拘束一覧: 距離拘束一覧を作る(仕様, 一辺粒子数),
        曲げ拘束一覧: 曲げ拘束一覧を作る(一辺粒子数),
        描画用インデックス一覧: インデックス一覧を作る(一辺粒子数),
        上端行の粒子添字一覧: 上端行を作る(一辺粒子数),
    })
}
