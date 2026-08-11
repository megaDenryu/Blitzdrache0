//! 条件が使う実行時アセットの世界。担当するのは、世界の選択を生成時の世界名と置き場の名前へ写すことである。
//!
//! 頂点量の軸だけが診断世界を選ぶ。診断世界は代表世界と同じ地面・同じ配置・同じ密度を持ち、
//! 同居植生の原型のトポロジー量(頂点数とインデックス数)だけが違う。外形・配置・可視集合が固定されるため、
//! 条件間で動く量は投入インデックス数を代理変数として読める。
//! 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「シャドウ性能の是正(フェーズ2性能課題、2026-08-03着手)」

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::shadow_probe) enum 計測世界 {
    代表の地形世界,
    頂点診断の粗い世界,
    頂点診断の細かい世界,
}

impl 計測世界 {
    pub(in crate::shadow_probe) fn 世界名(self) -> crate::compile_assets::世界名 {
        match self {
            Self::代表の地形世界 => crate::compile_assets::世界名::地形の世界,
            Self::頂点診断の粗い世界 => crate::compile_assets::世界名::頂点診断の粗い世界,
            Self::頂点診断の細かい世界 => crate::compile_assets::世界名::頂点診断の細かい世界,
        }
    }

    /// 実行時アセットの置き場の名前。代表世界の綴りを変えないのは、既に焼いてある置き場をそのまま使い回すためである。
    pub(in crate::shadow_probe) fn 置き場の名前(self) -> &'static str {
        match self {
            Self::代表の地形世界 => "assets",
            Self::頂点診断の粗い世界 => "assets_vertex_diag_coarse",
            Self::頂点診断の細かい世界 => "assets_vertex_diag_fine",
        }
    }
}
