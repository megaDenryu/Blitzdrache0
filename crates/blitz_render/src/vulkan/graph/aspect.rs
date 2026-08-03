//! 画像アスペクト(カラー/深度)。バリア発行時のImageSubresourceRange構築に使う。

use ash::vk;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum 画像アスペクト {
    カラー,
    深度,
}

impl 画像アスペクト {
    /// このアスペクトの画像全体(全縮小段、全レイヤー)を指す部分範囲。
    /// レイヤーを全部含めるのは、シャドウマップが距離区分ごとの層を持つ配列画像であり、
    /// グラフが配列全体を1資源として追跡するためである。
    ///
    /// 縮小段も全部含めるのは、鏡面畳込みの立方体画像が粗さの段ぶんの縮小列を持ち、その全段が
    /// 同じフレームの中で書かれてから読まれるためである。グラフは1つの画像に1つの状態しか持たないため、
    /// 段ごとに違う状態を追える形にはなっていない。段0だけを遷移させると残りの段が未定義のまま
    /// 使われるため、画像全体を1つの状態として扱うこの範囲が状態表と一致する唯一の指定である。
    /// 注意: 同じ画像の段Aを読みながら段Bへ書くパスは、この追跡では表せない。段をまたぐ参照を要る
    /// 生成は、別の画像を入力に取る形へ分ける(鏡面畳込みが遠方環境を別画像として読むのがその例である)。
    pub(crate) fn 部分範囲(&self) -> vk::ImageSubresourceRange {
        let aspect_mask = match self {
            Self::カラー => vk::ImageAspectFlags::COLOR,
            Self::深度 => vk::ImageAspectFlags::DEPTH,
        };
        vk::ImageSubresourceRange::default()
            .aspect_mask(aspect_mask)
            .base_mip_level(0)
            .level_count(vk::REMAINING_MIP_LEVELS)
            .base_array_layer(0)
            .layer_count(vk::REMAINING_ARRAY_LAYERS)
    }
}
