//! 基礎要件を満たした物理デバイス1件が、選ぶ規則へ差し出す材料。機材名・discreteかどうか・
//! ディスクリプタ索引の機能・テクスチャのブロック圧縮への対応と、呼び出し元がハンドルを引き当てるための添字を持つ。
//!
//! Vulkanのハンドルを持たないのは、選ぶ規則を物理デバイス無しで検査できるようにするためである。
//! 添字は候補を作った走査が振る番号であり、同じ走査で積んだハンドル一覧の位置を指す。
//! `機材名`を持つのは、不足を機材別に報告するためである。

use crate::error::ディスクリプタ索引機能項目;
use crate::vulkan::descriptor_indexing::ディスクリプタ索引機能;

pub(crate) struct 選定候補 {
    添字: usize,
    機材名: String, // `VkPhysicalDeviceProperties::deviceName`
    discreteか: bool,
    索引機能: ディスクリプタ索引機能,
    テクスチャのブロック圧縮に対応するか: bool, // `VkPhysicalDeviceFeatures::textureCompressionBC`
    立方体の配列画像に対応するか: bool,         // `VkPhysicalDeviceFeatures::imageCubeArray`
}

impl 選定候補 {
    pub(crate) fn 生成する(
        添字: usize,
        機材名: String,
        discreteか: bool,
        索引機能: ディスクリプタ索引機能,
        テクスチャのブロック圧縮に対応するか: bool,
        立方体の配列画像に対応するか: bool,
    ) -> Self {
        Self {
            添字,
            機材名,
            discreteか,
            索引機能,
            テクスチャのブロック圧縮に対応するか,
            立方体の配列画像に対応するか,
        }
    }

    pub(crate) fn 添字(&self) -> usize {
        self.添字
    }

    pub(crate) fn 機材名(&self) -> &str {
        &self.機材名
    }

    pub(crate) fn discreteか(&self) -> bool {
        self.discreteか
    }

    pub(crate) fn 索引の最低要件を満たすか(&self) -> bool {
        self.索引機能.最低要件を満たすか()
    }

    pub(crate) fn 索引の不足一覧(&self) -> Vec<ディスクリプタ索引機能項目> {
        self.索引機能.不足一覧()
    }

    pub(crate) fn テクスチャのブロック圧縮に対応するか(&self) -> bool {
        self.テクスチャのブロック圧縮に対応するか
    }

    pub(crate) fn 立方体の配列画像に対応するか(&self) -> bool {
        self.立方体の配列画像に対応するか
    }
}
