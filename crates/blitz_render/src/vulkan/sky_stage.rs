//! 空段階の資源: 方式ごとの空パイプラインと、大気LUT腕だけが持つ標本ディスクリプタと空中遠近合成の一式。
//! フレーム構成に空段階があるときだけ`描画段階資源`が保持し、空パスと合成パスの記録だけがこの一式を束縛する。
//!
//! 方式を列挙で持つのは、「大気LUT腕なのにLUTを引くディスクリプタが無い」という状態を型で作れなくするためである。
//! 腕の選択はパイプラインとディスクリプタの選択で完結し、フラグメントの中に方式の分岐を持たない
//! (参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「空の方式」)。
//! 生成の手順は`create`、毎フレームの束縛先の組み立ては`draw_input`、合成の対は`aerial_composite`が担う。

mod aerial_composite;
mod create;
mod draw_input;

use ash::vk;

pub(crate) use aerial_composite::空中遠近合成資源;

use crate::error::レンダラーエラー;
use crate::shader_bundle::空シェーダー;
use crate::vulkan::atmosphere_lut::{大気LUT一式, 大気LUT標本ディスクリプタ};
use crate::vulkan::pipeline::空パイプライン;

pub(crate) enum 空段階資源 {
    /// Hosek-Wilkie解析近似の腕。読むのはフレームユニフォームだけであり、set0のシーンのレイアウトしか結ばない。
    Hosek解析近似 { パイプライン: 空パイプライン },
    /// 大気LUTの腕。set1でスカイビューLUTと透過率LUTと媒体のユニフォームを結ぶ。
    大気LUT {
        パイプライン: 空パイプライン,
        標本: 大気LUT標本ディスクリプタ,
        /// 空中遠近の全画面合成。`None`は起動指定で合成を切った状態である。合成が無いフレームは
        /// ボリュームを1枚も焼かないため、この`Option`が「焼くかどうか」も決める。
        合成: Option<空中遠近合成資源>,
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

    /// 空中遠近ボリュームを引く合成パスを持つか。ボリュームを焼くかどうかがこの1つで決まる。
    /// 誰も引かないボリュームを焼く時間はそのまま無駄になるため、合成が無い構成では1枚も焼かない。
    pub(crate) fn 空中遠近合成があるか(&self) -> bool {
        matches!(self, Self::大気LUT { 合成: Some(_), .. })
    }

    /// 合成の一式。合成を持たない構成では`None`を返す。
    pub(crate) fn 合成(&self) -> Option<&空中遠近合成資源> {
        match self {
            Self::Hosek解析近似 { .. } | Self::大気LUT { 合成: None, .. } => None,
            Self::大気LUT { 合成: Some(合成), .. } => Some(合成),
        }
    }

    /// 注意: 合成を空パイプラインより先に破棄する(合成のディスクリプタが大気LUTの画像ビューを結んでいるため、
    /// 大気LUTの破棄より前に片付ける必要がある)。空パイプラインを標本ディスクリプタより先に破棄するのは、
    /// パイプラインレイアウトがディスクリプタセットレイアウトを参照するためである。
    /// 前提: レンダラー全体の破棄順は renderer/destroy.rs が持ち、この一式は`描画段階資源`の1段として呼ばれる(GPU待機済み)。
    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        match self {
            Self::Hosek解析近似 { パイプライン } => パイプライン.破棄する(device),
            Self::大気LUT {
                パイプライン, 標本, 合成
            } => {
                if let Some(合成) = 合成 {
                    合成.破棄する(device);
                }
                パイプライン.破棄する(device);
                標本.破棄する(device);
            }
        }
    }
}
