//! 派生表現の1本の生成パスが束縛する資源と、計算の発行の前へ押し込む定数のバイト並び。
//!
//! バイト並びをここが単独で持つのは、これがCPUとシェーダーの即時定数の構造体の間の唯一の契約だからである。
//! 押し込む値が粗さ1つだけであっても、詰める側と読む側を1対1に保つ場所は1箇所にする。
//!
//! 粗さをシェーダー定数でなく即時定数で渡すのは、段ごとに違う値であり、計算の発行のたびに書き換わるためである。

use ash::vk;

/// そのパスが押し込む即時定数。持たないパスと持つパスを型で分け、バイト数0の範囲を宣言しない。
#[derive(Debug, Clone, Copy, PartialEq)]
pub(in crate::vulkan) enum 派生表現の即時定数 {
    無し,
    粗さ(f32),
}

/// 押し込むバイト数。`shaders/specular_prefilter.slang`の`SpecularPrefilterCondition`と一致させる。
pub(in crate::vulkan) const 粗さの即時定数のバイト数: u32 = 4;

impl 派生表現の即時定数 {
    /// 計算の発行前に書くバイト列。押し込むものが無い枝は空の列を返し、押し込みそのものを起こさない。
    pub(in crate::vulkan) fn バイト列(self) -> Vec<u8> {
        match self {
            Self::無し => Vec::new(),
            Self::粗さ(値) => 値.to_le_bytes().to_vec(),
        }
    }
}

/// 1本のコンピュートパスが束縛する資源と計算の発行の大きさ。
pub(in crate::vulkan) struct 派生表現の生成入力 {
    pub(in crate::vulkan) pipeline: vk::Pipeline,
    pub(in crate::vulkan) layout: vk::PipelineLayout,
    pub(in crate::vulkan) ディスクリプタセット: vk::DescriptorSet,
    pub(in crate::vulkan) 計算の班数: [u32; 3],
    pub(in crate::vulkan) 即時定数: 派生表現の即時定数,
}

/// 1計算の班が受け持つテクセルの一辺。遠方環境の生成と同じ8である。
const 計算の班の一辺: u32 = 8;

/// 立方体の1つの段を覆う計算の班数。奥行きは1計算の班につき1層である。
pub(in crate::vulkan) fn 立方体の計算の班数を求める(一辺: u32, 層数: u32) -> [u32; 3] {
    [一辺.div_ceil(計算の班の一辺), 一辺.div_ceil(計算の班の一辺), 層数]
}

/// 2次元の表を覆う計算の班数。層を持たないため奥行きは1である。
pub(in crate::vulkan) fn 表の計算の班数を求める(横: u32, 縦: u32) -> [u32; 3] {
    [横.div_ceil(計算の班の一辺), 縦.div_ceil(計算の班の一辺), 1]
}

#[cfg(test)]
mod byte_layout_tests;
