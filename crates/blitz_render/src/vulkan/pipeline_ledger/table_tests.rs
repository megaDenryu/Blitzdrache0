//! パイプライン記載表の状態遷移の検査と、その検査が使うGPUの実物を持たない供給元。
//! 実体をu32の通し番号にすることで、生成が何回起きたか・同じキーが同じ実体を返すか・破棄がちょうど1回かを数えられる。
//! キーそのものの性質(決定性・OrdとEqの整合・非混同)は`key_tests`、表の構築と破棄は`build_tests`が見る。

#![allow(clippy::unwrap_used)]

mod build_tests;
mod key_tests;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::material_variant::材質変種キー;

use super::key::パイプラインキー;
use super::lighting_contract::照明問い合わせ契約;
use super::render_target::描画先の一意化;
use super::supplier::パイプライン供給元;

/// 検査用の供給元。何回目の生成かを実体の値にし、破棄された実体を記録する。
struct 数える供給元 {
    生成回数: u32,
    /// 何番目の生成で失敗させるか。`None`なら失敗しない。
    失敗させる回: Option<u32>,
    破棄した実体: Vec<u32>,
}

impl 数える供給元 {
    fn 常に成功する() -> Self {
        Self {
            生成回数: 0,
            失敗させる回: None,
            破棄した実体: Vec::new(),
        }
    }

    fn 指定回で失敗する(回: u32) -> Self {
        Self {
            生成回数: 0,
            失敗させる回: Some(回),
            破棄した実体: Vec::new(),
        }
    }
}

impl パイプライン供給元 for 数える供給元 {
    type 実体 = u32;

    fn 生成する(&mut self, _キー: パイプラインキー) -> Result<u32, レンダラーエラー> {
        self.生成回数 += 1;
        if self.失敗させる回 == Some(self.生成回数) {
            return Err(レンダラーエラー::粒子入力不一致);
        }
        Ok(self.生成回数)
    }

    fn 破棄する(&mut self, 実体: u32) {
        self.破棄した実体.push(実体);
    }
}

fn シーンの描画先() -> 描画先の一意化 {
    描画先の一意化::色と深度へ描く(vk::Format::R8G8B8A8_UNORM, vk::Format::D32_SFLOAT, vk::SampleCountFlags::TYPE_1)
}

fn 別の描画先() -> 描画先の一意化 {
    描画先の一意化::色と深度へ描く(vk::Format::B8G8R8A8_SRGB, vk::Format::D32_SFLOAT, vk::SampleCountFlags::TYPE_1)
}

fn シーンのキー(描画先: 描画先の一意化) -> パイプラインキー {
    パイプラインキー::シーン {
        描画先,
        材質変種: 材質変種キー::標準金属粗さPBRの不透明両面,
        照明問い合わせ: 照明問い合わせ契約::直接光と多段影,
    }
}

fn シャドウのキー() -> パイプラインキー {
    パイプラインキー::シャドウ {
        描画先: 描画先の一意化::深度だけへ描く(vk::Format::D32_SFLOAT, vk::SampleCountFlags::TYPE_1),
    }
}
