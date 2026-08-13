//! 生成検収が使い捨てるルートの親。通常の場所巡りと大規模世界を互いに隔離する。

use std::path::Path;

const 場所巡りの検収用ディレクトリ: &str = "target/fox_tour_generation_check";
const 大規模世界の検収用ディレクトリ: &str = "target/large_world_generation_check";

#[derive(Clone, Copy)]
pub(in crate::game_fox_tour::map_generation_check) enum 検収用の親 {
    場所巡り,
    大規模世界,
}

impl 検収用の親 {
    pub(super) fn ディレクトリ(self) -> &'static Path {
        match self {
            Self::場所巡り => Path::new(場所巡りの検収用ディレクトリ),
            Self::大規模世界 => Path::new(大規模世界の検収用ディレクトリ),
        }
    }
}
