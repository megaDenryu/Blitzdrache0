//! GPU継ぎ目検査の対象2チャンクだけへ指定段を適用し、他の常駐チャンクは本番選択器の結果を保つ工程。

use blitz_engine::チャンク座標;
use blitz_render::地形詳細段;

use crate::cli::LOD継ぎ目検査設定;

pub(super) fn 検査段を適用する(
    座標: チャンク座標, 通常段: 地形詳細段, 検査: Option<LOD継ぎ目検査設定>
) -> 地形詳細段 {
    let Some(検査) = 検査 else {
        return 通常段;
    };
    if 座標 == 検査.一方座標 {
        検査.一方段
    } else if 座標 == 検査.他方座標 {
        検査.他方段
    } else {
        通常段
    }
}
