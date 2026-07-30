//! 空段階の資源: 方式ごとの空パイプラインと、大気LUT腕だけが持つ標本ディスクリプタ。
//! フレーム構成に空段階があるときだけ`描画段階資源`が保持し、空パスの記録だけがこの一式を束縛する。
//!
//! 方式を列挙で持つのは、「大気LUT腕なのにLUTを引くディスクリプタが無い」という状態を型で作れなくするためである。
//! 腕の選択はパイプラインとディスクリプタの選択で完結し、フラグメントの中に方式の分岐を持たない
//! (参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「空の方式」)。
//! 生成の手順は`create`が担う。

mod create;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::shader_bundle::空シェーダー;
use crate::vulkan::atmosphere_lut::{大気LUT一式, 大気LUT標本ディスクリプタ};
use crate::vulkan::frame::{空描画の方式, 空描画入力};
use crate::vulkan::pipeline::空パイプライン;
use crate::vulkan::sync::フレームスロット添字;

pub(crate) enum 空段階資源 {
    /// Hosek-Wilkie解析近似の腕。読むのはフレームユニフォームだけであり、set0のシーンのレイアウトしか結ばない。
    Hosek解析近似 { パイプライン: 空パイプライン },
    /// 大気LUTの腕。set1でスカイビューLUTと透過率LUTと媒体のユニフォームを結ぶ。
    大気LUT {
        パイプライン: 空パイプライン,
        標本: 大気LUT標本ディスクリプタ,
    },
}

/// 空段階の資源を組み立てるのに要る材料。
pub(crate) struct 空段階の生成要求<'a> {
    pub(crate) カラー形式: vk::Format,
    pub(crate) 深度形式: vk::Format,
    /// set0に結ぶシーンのディスクリプタセットレイアウト。フレームユニフォームのbinding3をここから読む。
    pub(crate) シーンlayout: vk::DescriptorSetLayout,
    pub(crate) シェーダー: &'a 空シェーダー,
    /// 大気LUT腕がset1で引くLUTの持ち主。フレーム構成に空段階があるときは必ず作られている。
    pub(crate) 大気lut: &'a 大気LUT一式,
}

impl 空段階資源 {
    pub(crate) fn 生成する(device: &ash::Device, 要求: 空段階の生成要求<'_>) -> Result<Self, レンダラーエラー> {
        create::生成する(device, 要求)
    }

    /// 大気LUT腕であるか。空中遠近ボリュームを焼くかどうかがこの1つで決まる。
    pub(crate) fn 大気lut腕か(&self) -> bool {
        matches!(self, Self::大気LUT { .. })
    }

    /// 空パスが束縛する資源。`ディスクリプタセット`は走査順で最初の描画対象のもの(set0)である。
    pub(crate) fn 描画入力を作る(
        &self, ディスクリプタセット: vk::DescriptorSet, フレーム添字: フレームスロット添字
    ) -> 空描画入力 {
        let (パイプライン, 方式) = match self {
            Self::Hosek解析近似 { パイプライン } => (パイプライン, 空描画の方式::Hosek解析近似),
            Self::大気LUT { パイプライン, 標本 } => (
                パイプライン,
                空描画の方式::大気LUT {
                    標本セット: 標本.set(フレーム添字),
                },
            ),
        };
        空描画入力 {
            pipeline: パイプライン.handle,
            layout: パイプライン.layout,
            ディスクリプタセット,
            方式,
        }
    }

    /// 注意: パイプラインを標本ディスクリプタより先に破棄する(パイプラインレイアウトがディスクリプタセットレイアウトを参照するため)。
    /// 前提: レンダラー全体の破棄順は renderer/destroy.rs が持ち、この一式は`描画段階資源`の1段として呼ばれる(GPU待機済み)。
    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        match self {
            Self::Hosek解析近似 { パイプライン } => パイプライン.破棄する(device),
            Self::大気LUT { パイプライン, 標本 } => {
                パイプライン.破棄する(device);
                標本.破棄する(device);
            }
        }
    }
}
