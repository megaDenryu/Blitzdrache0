//! 地形の詳細段の語彙。段そのものと、束ごとにどの段を描くかの選択の2つを持つ。
//! 段の選択が段の定義と離れると、番号の意味が2箇所で決まるため同じ木の下へ置く。
//! 参照: `_doc/設計/地形とカメラ相対描画.md`「LOD」

mod level;
mod selection;

pub use level::地形詳細段;
pub use selection::地形詳細段選択;
pub(crate) use selection::段を参照する;
