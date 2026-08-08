//! 1回の積み込みに要る材料の束。担うのは、積み込みへ渡す値をまとめて名前で受け取ることだけである。
//! 受けバッファの所有と読み取りは`readback_set`が持つ。
//!
//! 引数の列を1つに束ねるのは、画像・ビュー・寸法・範囲・入力の組が4組あり、平らに並べると
//! 取り違えが型で防げなくなるためである。

use ash::vk;

use crate::vulkan::derived_environment::派生表現の生成入力;

/// グラフへ登録する画像1枚ぶんの値。
pub(super) struct 登録する画像 {
    pub(super) 画像: vk::Image,
    pub(super) ビュー: vk::ImageView,
    pub(super) 寸法: vk::Extent2D,
}

pub(super) struct 積む材料<'a> {
    pub(super) 遠方環境: 登録する画像,
    pub(super) 拡散照度: 登録する画像,
    pub(super) 鏡面畳込み: 登録する画像,
    pub(super) 反射率積分表: 登録する画像,
    pub(super) 遠方環境の範囲: vk::Extent3D,
    pub(super) 拡散照度の範囲: vk::Extent3D,
    pub(super) 鏡面畳込みの段ごとの範囲: Vec<vk::Extent3D>,
    pub(super) 反射率積分表の範囲: vk::Extent3D,
    pub(super) 層数: u32,
    pub(super) 書き込み元: vk::Buffer,
    pub(super) 拡散照度入力: &'a 派生表現の生成入力,
    pub(super) 鏡面畳込み入力一覧: &'a [派生表現の生成入力],
    pub(super) 反射率積分表入力: &'a 派生表現の生成入力,
    pub(super) 受け: 受けバッファの口,
}

/// 記録が書き込み先に取るバッファのハンドルだけの組。所有は`受けバッファ四点`が持つ。
#[derive(Clone, Copy)]
pub(super) struct 受けバッファの口 {
    pub(super) 遠方環境: vk::Buffer,
    pub(super) 拡散照度: vk::Buffer,
    pub(super) 鏡面畳込み: vk::Buffer,
    pub(super) 反射率積分表: vk::Buffer,
}
