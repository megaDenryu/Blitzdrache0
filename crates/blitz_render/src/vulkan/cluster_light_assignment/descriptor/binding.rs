//! 選別のコンピュートが読み書きする生成側のセットの束縛の並び。ヘッダ・局所光レコード列・クラスタ格子・
//! クラスタ光添字列の4本であり、前の2つは照明問い合わせのセットが画素段へ結ぶのと同じバッファを生成側へも結んだものである。
//!
//! 番号を並びの位置そのものにするのは、`shaders/cluster_light_assignment.slang`の宣言が0から連番だからである。
//! セットレイアウトの宣言もプールの内訳も割り当ても書き込みも、すべてこの1つの並びから配る。

use ash::vk;

use crate::vulkan::descriptor::{宣言した束縛の並び, 束縛番号};

const 計算段: vk::ShaderStageFlags = vk::ShaderStageFlags::COMPUTE;
const 記憶: vk::DescriptorType = vk::DescriptorType::STORAGE_BUFFER;

pub(super) const 束縛の宣言: 宣言した束縛の並び<4> = 宣言した束縛の並び::生成する([
    (束縛番号::生成する(0), vk::DescriptorType::UNIFORM_BUFFER, 計算段),
    (束縛番号::生成する(1), 記憶, 計算段),
    (束縛番号::生成する(2), 記憶, 計算段),
    (束縛番号::生成する(3), 記憶, 計算段),
]);
