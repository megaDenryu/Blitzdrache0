//! 空中遠近合成パスが束縛するディスクリプタ。binding0がシーンの深度、binding1が空中遠近ボリュームである。
//! 空パスの標本ディスクリプタと別に持つのは、束縛する資源が1枚も重ならないためである(合成はボリュームだけを参照し、
//! 空は透過率とスカイビューを参照する)。媒体のシェーダー定数を結ばないのは、合成が写像に惑星半径を要らず、
//! ボリュームの座標を画面の位置と距離だけから決めるためである。
//!
//! フレームスロットごとに1セットを持つのは、深度の束縛先を毎フレーム書き直すためである。深度画像は
//! ウィンドウ寸法の変更で作り直されるため、生成時に1度書いたビューは寸法変更で無効になる。1度書いて
//! 変更時だけ書き直す形を採らないのは、書き直しの契機を提示資源の再構築と結ぶ配線が要り、その配線の
//! 抜けが「破棄済みのビューを結んだセット」として現れるからである。書く量は画像1枚ぶんの記述子であり、
//! 条件で分けるより不変条件を1つに保つ方を採る(大気媒体のシェーダー定数と同じ判断)。
//! 生成の手順は`create`、番号と型の対応は`binding`が担う。
//! 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「空中遠近ボリュームの刻みと積分」

mod binding;
mod create;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::descriptor::{宣言から作ったセットレイアウト, 宣言から割り当てたセット};
use crate::vulkan::sync::{フレームスロット添字, 進行中フレーム数};

pub(crate) struct 空中遠近合成ディスクリプタ {
    layout: 宣言から作ったセットレイアウト<2>,
    pool: vk::DescriptorPool,
    深度サンプラー: vk::Sampler,
    ボリュームサンプラー: vk::Sampler,
    set一覧: [宣言から割り当てたセット<2>; 進行中フレーム数],
}

/// 生成時に結ぶ束縛先。深度はフレームごとに変わるためここに載せない。
pub(crate) struct 空中遠近合成の束縛先 {
    pub(crate) 空中遠近ビュー: vk::ImageView,
}

impl 空中遠近合成ディスクリプタ {
    pub(crate) fn 生成する(
        確保係: &GPU資源の確保係<'_>, 束縛先: &空中遠近合成の束縛先
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(確保係, 束縛先)
    }

    pub(crate) fn set(&self, フレーム添字: フレームスロット添字) -> vk::DescriptorSet {
        self.set一覧[フレーム添字.配列添字()].セットのハンドル()
    }

    /// パイプラインレイアウトの宣言へ渡す境界。
    pub(crate) fn レイアウトのハンドル(&self) -> vk::DescriptorSetLayout {
        self.layout.レイアウトのハンドル()
    }

    /// そのスロットのセットへ、このフレームの深度画像を結び直す。
    /// 前提: 呼び出し元はこのスロットのフェンス待機を済ませている(このセットを読むGPU作業が完了している)。
    pub(crate) fn 深度を結び直す(
        &self, device: &ash::Device, フレーム添字: フレームスロット添字, 深度ビュー: vk::ImageView
    ) {
        binding::深度を書き込む(device, &self.set一覧[フレーム添字.配列添字()], self.深度サンプラー, 深度ビュー);
    }

    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: poolの破棄がsetの解放を暗黙に行う。sampler・layout・poolはSelfが唯一の所有者であり、破棄時点でGPU側の使用が完了している。
        unsafe {
            device.destroy_descriptor_pool(self.pool, None);
            device.destroy_sampler(self.ボリュームサンプラー, None);
            device.destroy_sampler(self.深度サンプラー, None);
        }
        self.layout.破棄する(device);
    }
}
