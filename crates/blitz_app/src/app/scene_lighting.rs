//! シーンごとの初期ライト方針の引き当て。担当するのは、シーン名からどの世界の方針を選ぶかと、
//! 局所光の件数の上書きを読まない世界へその指定が来たときに失敗させることだけである。
//! 方針そのものは世界ごとのモジュールが持ち、多段影の設定は`cascade_policy`が全世界ぶんを持つ。
//! 既定は`blitz_engine`の既定ライティングであり、名前で選ばれなかった世界はすべてそれになる。

mod cascade_policy;
mod indirect_probe;
mod night_lights;
mod stone_hut_interior;
mod vegetation_verification;

use blitz_math::大域ワールド位置;
use blitz_render::ライティング入力;

use crate::cli::局所光の件数の起動指定;
use crate::error::起動エラー;

/// 群が両方の視錐台の外にある検収シーン(`cargo xtask cloth-empty`)。ライト方針は既定のままであり、
/// 多段の最大影距離だけを世界に合わせるため、`cascade_policy`がこの名前を読む。
pub(super) const 両視錐台外の群シーン: &str = "instance_all_culled";

pub(super) fn シーン初期ライティングを作る(
    シーン名: &str,
    有効: bool,
    大域ずらし量: 大域ワールド位置,
    シャドウ計測: &crate::cli::シャドウ計測起動設定,
    局所光の件数: 局所光の件数の起動指定,
) -> Result<ライティング入力, 起動エラー> {
    let 多段設定 = cascade_policy::シーンの多段設定を作る(シーン名, シャドウ計測);
    if シーン名 == night_lights::シーン識別子 {
        return Ok(night_lights::方針を作る(有効, 大域ずらし量, 局所光の件数).多段設定を差し替える(多段設定));
    }
    if シーン名 == stone_hut_interior::シーン識別子 {
        return Ok(stone_hut_interior::方針を作る(有効, 大域ずらし量, 局所光の件数).多段設定を差し替える(多段設定));
    }
    局所光の件数の上書きを読まない世界であることを確かめる(シーン名, 局所光の件数)?;
    if シーン名 == indirect_probe::シーン識別子 {
        return Ok(indirect_probe::方針を作る(有効, 大域ずらし量).多段設定を差し替える(多段設定));
    }
    if シーン名.starts_with(vegetation_verification::接頭辞) {
        return Ok(vegetation_verification::方針を作る(シーン名, 有効, 大域ずらし量).多段設定を差し替える(多段設定));
    }
    Ok(blitz_engine::既定ライティングを作る(有効, 大域ずらし量).多段設定を差し替える(多段設定))
}

/// 局所光の件数の上書きを読む世界は夜と屋内の2つの検収世界だけである。読まない世界へ指定が来たら、
/// 起動の時点で失敗させる。黙って捨てると、件数を振ったつもりの計測が全部同じ条件の繰り返しになる。
fn 局所光の件数の上書きを読まない世界であることを確かめる(
    シーン名: &str,
    局所光の件数: 局所光の件数の起動指定,
) -> Result<(), 起動エラー> {
    match 局所光の件数 {
        局所光の件数の起動指定::世界の宣言に従う => Ok(()),
        局所光の件数の起動指定::件数を上書きする(_) => {
            Err(起動エラー::局所光の件数の上書きを読まない世界(format!(
                "{シーン名}(読むのは{}と{}だけである)",
                night_lights::シーン識別子,
                stone_hut_interior::シーン識別子
            )))
        }
    }
}
