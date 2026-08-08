//! 局所可視性補正の資源一式。担当するのは、世界の宣言(`crate::local_visibility::局所可視性の描画設定`)をGPUの資源へ落とし、毎フレームのパスが要るハンドルと定数を1つの入力へまとめることである。
//!
//! 拡散間接方式が環境のみの世界でも資源を作るのは、作るかどうかを方式で分けると照明問い合わせのセット(set3)の形が世界ごとに変わり、束縛の一致条件が1つ増えるためである。環境のみの世界では遮蔽なしの符号値だけを持つ画像が結ばれ、掛け算が拡散間接照度を1ビットも変えない。パスを積むかどうかだけが方式で分かれる。
//! 参照: `_doc/設計/放射輝度問い合わせ階層.md`「IIaの実装設計」

mod bind;
mod descriptor;
mod fill;
mod images;
mod input;
mod pipelines;
mod resize;
mod setting;

pub(crate) use input::局所可視性描画入力;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::local_visibility::{局所可視性のシェーダー一式, 局所可視性の描画設定};
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::転送実行環境;

pub(crate) struct 局所可視性一式 {
    画像組: images::局所可視度の画像組,
    ディスクリプタ: descriptor::局所可視性のディスクリプタ,
    パイプライン: pipelines::局所可視性のパイプライン一式,
    描画設定: 局所可視性の描画設定,
}

impl 局所可視性一式 {
    pub(crate) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        転送環境: &転送実行環境,
        シェーダー: &局所可視性のシェーダー一式,
        描画設定: 局所可視性の描画設定,
        画面: 画面の入力,
    ) -> Result<Self, レンダラーエラー> {
        let 画像組 = images::局所可視度の画像組::生成する(device, メモリプロパティ, 画面.寸法)?;
        let 一式 = 組み上げる(device, シェーダー, 描画設定, 画像組)?;
        if let Err(誤り) = fill::一定の符号値で埋める(転送環境, &一式.画像組, 描画設定.埋める符号値()) {
            一式.破棄する(device);
            return Err(誤り);
        }
        一式.資源を束縛する(device, 画面.深度ビュー);
        Ok(一式)
    }

    /// 遮蔽の標本化と両側ぼかしをそのフレームへ積むか。判断は描画設定が持つ。
    pub(crate) fn パスを積むか(&self) -> bool {
        self.描画設定.パスを積むか()
    }

    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        self.パイプライン.破棄する(device);
        self.ディスクリプタ.破棄する(device);
        self.画像組.破棄する(device);
    }
}

/// 生成のときと寸法へ追従したときで別々になりうる値をまとめた入力。画面寸法と深度のビューは必ず同じ世代のものである。
#[derive(Clone, Copy)]
pub(crate) struct 画面の入力 {
    pub(crate) 寸法: vk::Extent2D,
    pub(crate) 深度ビュー: vk::ImageView,
}

/// 確保済みの画像組へディスクリプタとパイプラインを足す。失敗したら受け取った画像組まで片付ける。
fn 組み上げる(
    device: &GPUデバイス,
    シェーダー: &局所可視性のシェーダー一式,
    描画設定: 局所可視性の描画設定,
    画像組: images::局所可視度の画像組,
) -> Result<局所可視性一式, レンダラーエラー> {
    let ディスクリプタ = match descriptor::局所可視性のディスクリプタ::生成する(device) {
        Ok(ディスクリプタ) => ディスクリプタ,
        Err(誤り) => {
            画像組.破棄する(device);
            return Err(誤り);
        }
    };
    match pipelines::局所可視性のパイプライン一式::生成する(device, &ディスクリプタ, シェーダー) {
        Ok(パイプライン) => Ok(局所可視性一式 {
            画像組,
            ディスクリプタ,
            パイプライン,
            描画設定,
        }),
        Err(誤り) => {
            ディスクリプタ.破棄する(device);
            画像組.破棄する(device);
            Err(誤り)
        }
    }
}
