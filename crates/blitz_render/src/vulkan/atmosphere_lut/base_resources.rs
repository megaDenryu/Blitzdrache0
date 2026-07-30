//! 大気LUTのうち、GPUメモリを占める資源(4枚のLUT画像と媒体のユニフォーム)の所有者。
//! 触れるのはこの5つだけであり、ディスクリプタもパイプラインも持たない。生成の途中で失敗したら
//! それまでに確保したメモリをその場で解放する。4枚の画像を順に確保する巻き戻しは`images`が持つ。

mod images;

use ash::vk;

use super::{image, medium_uniform, 大気LUTの形};
use crate::atmosphere::{大気LUT解像度, 大気散乱媒体};
use crate::error::レンダラーエラー;
use crate::vulkan::sync::{フレームインフライト数, フレームスロット添字};
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) struct 大気LUT基盤資源 {
    pub(super) 透過率: image::大気LUT画像,
    pub(super) 多重散乱: image::大気LUT画像,
    pub(super) スカイビュー: image::大気LUT画像,
    pub(super) 空中遠近: image::大気LUT画像,
    媒体ユニフォーム: medium_uniform::媒体ユニフォーム一式,
}

impl 大気LUT基盤資源 {
    pub(super) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        解像度: 大気LUT解像度,
    ) -> Result<Self, レンダラーエラー> {
        let 多重散乱の一辺 = 解像度.多重散乱の一辺();
        let 形一覧 = [
            平面(解像度.透過率の幅(), 解像度.透過率の高さ()),
            平面(多重散乱の一辺, 多重散乱の一辺),
            平面(解像度.スカイビューの幅(), 解像度.スカイビューの高さ()),
            大気LUTの形::立体 {
                一辺: 解像度.空中遠近の一辺(),
            },
        ];
        let [透過率, 多重散乱, スカイビュー, 空中遠近] = images::順に作る(device, メモリプロパティ, 形一覧)?;
        match medium_uniform::媒体ユニフォーム一式::生成する(device, メモリプロパティ) {
            Ok(媒体ユニフォーム) => Ok(Self {
                透過率,
                多重散乱,
                スカイビュー,
                空中遠近,
                媒体ユニフォーム,
            }),
            Err(誤り) => {
                空中遠近.破棄する(device);
                スカイビュー.破棄する(device);
                多重散乱.破棄する(device);
                透過率.破棄する(device);
                Err(誤り)
            }
        }
    }

    /// ディスクリプタが結ぶユニフォームのバッファをスロット順に並べたもの。
    pub(super) fn ユニフォーム一覧(&self) -> [vk::Buffer; フレームインフライト数] {
        let mut 一覧 = [vk::Buffer::null(); フレームインフライト数];
        for 添字 in フレームスロット添字::全スロット() {
            一覧[添字.配列添字()] = self.媒体ユニフォーム.buffer(添字);
        }
        一覧
    }

    /// 前提: 呼び出し元はこのスロットのフェンス待機を済ませている(`draw_execute/prepare.rs`)。
    pub(super) fn 媒体を書き込む(
        &self,
        device: &ash::Device,
        フレーム添字: フレームスロット添字,
        媒体: &大気散乱媒体,
    ) -> Result<(), レンダラーエラー> {
        self.媒体ユニフォーム.書き込む(device, フレーム添字, 媒体)
    }

    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        self.媒体ユニフォーム.破棄する(device);
        self.空中遠近.破棄する(device);
        self.スカイビュー.破棄する(device);
        self.多重散乱.破棄する(device);
        self.透過率.破棄する(device);
    }
}

fn 平面(幅: u32, 高さ: u32) -> 大気LUTの形 {
    大気LUTの形::平面 { 幅, 高さ }
}
