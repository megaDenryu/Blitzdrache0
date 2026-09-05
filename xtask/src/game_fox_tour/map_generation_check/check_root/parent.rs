//! 生成検収が使い捨てるルートの親ディレクトリ。通常の場所巡りと大規模世界を互いに隔離する。

use std::path::PathBuf;

use crate::verify::{検証の出力の置き場名, 検証の出力ルート};
use blitz_asset_compiler::遠景地形の焼き方;

const 場所巡りの検収用ディレクトリ: 検証の出力の置き場名 = 検証の出力の置き場名::生成する("fox_tour_generation_check");
const 大規模世界の検収用ディレクトリ: 検証の出力の置き場名 = 検証の出力の置き場名::生成する("large_world_generation_check");

#[derive(Clone, Copy)]
pub(in crate::game_fox_tour::map_generation_check) enum 生成検収の親ディレクトリ {
    場所巡り,
    大規模世界,
}

impl 生成検収の親ディレクトリ {
    /// この親の下で焼く世界が遠景地形を持つか。大規模世界だけが持つ。
    ///
    /// 世界の種別で決めないのは、場所巡りの世界が3×3のクソゲーの世界としても8km×10kmの大規模世界としても
    /// 焼かれるためである。3×3の側にも遠景が入ると、遠景の有無で大規模世界を見分ける実行時の判別が誤爆する。
    pub(in crate::game_fox_tour::map_generation_check) fn 遠景地形の焼き方(self) -> 遠景地形の焼き方 {
        match self {
            Self::場所巡り => 遠景地形の焼き方::焼かない,
            Self::大規模世界 => 遠景地形の焼き方::焼く,
        }
    }

    pub(super) fn ディレクトリ(self) -> PathBuf {
        検証の出力ルート::既定().名前が指す置き場(match self {
            Self::場所巡り => 場所巡りの検収用ディレクトリ,
            Self::大規模世界 => 大規模世界の検収用ディレクトリ,
        })
    }
}
