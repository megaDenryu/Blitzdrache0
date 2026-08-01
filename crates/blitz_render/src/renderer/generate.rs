//! レンダラーの生成手順。インスタンス〜スワップチェーンは`core_setup`、残りの資源は`generate_resources`へ委ねる。

mod assemble;
mod core_setup;
mod frame_resources;
mod generate_resources;
#[cfg(test)]
mod generate_tests;
use super::レンダラー;
use crate::cloth_material::布素材;
use crate::error::{フレーム入力不一致エラー, レンダラーエラー};
use crate::extent::ウィンドウ寸法;
use crate::frame_composition::{フレーム構成, フレーム段階};
use crate::particle_material::粒子素材;
use crate::present_display::実表示計測要求;
use crate::render_scene_material::描画シーン素材;
use crate::shader_bundle::シェーダー束;
use crate::skin_mesh::スキンメッシュ素材;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

impl レンダラー {
    /// Vulkanインスタンスからスワップチェーン・各バッファ・各パイプライン・開発用UI(判断33)・ポストプロセス(判断38)・スキニング(判断44)・布(判断54)までを構築する。
    /// 前提: `表示ハンドル`と`ウィンドウハンドル`の指すウィンドウは、戻り値のレンダラーより長生きすること(呼び出し元のフィールド宣言順で担保する)。
    #[allow(clippy::too_many_arguments)]
    pub fn 生成する(
        表示ハンドル: RawDisplayHandle,
        ウィンドウハンドル: RawWindowHandle,
        寸法: ウィンドウ寸法,
        シェーダー: シェーダー束,
        描画シーン: 描画シーン素材,
        スキン: Option<スキンメッシュ素材>,
        布: Option<布素材>,
        粒子: Option<粒子素材>,
        フレーム構成: フレーム構成,
        実表示計測要求: 実表示計測要求,
    ) -> Result<Self, レンダラーエラー> {
        構成と素材を検査する(&フレーム構成, スキン.is_some(), 布.is_some(), 粒子.is_some())?;
        let コア = core_setup::組み立てる(表示ハンドル, ウィンドウハンドル, 寸法, 実表示計測要求)?;
        let 資源 = generate_resources::組み立てる(generate_resources::生成要求 {
            環境: &コア.環境,
            提示: &コア.提示,
            シェーダー: &シェーダー,
            描画シーン: &描画シーン,
            スキン: スキン.as_ref(),
            布: 布.as_ref(),
            粒子素材: 粒子.as_ref(),
            フレーム構成,
            タイムスタンプ対応か: コア.タイムスタンプ対応か,
            タイムスタンプ周期ns: コア.タイムスタンプ周期ns,
        })?;
        Ok(assemble::レンダラーを組み立てる(コア, 資源, フレーム構成))
    }
}

fn 構成と素材を検査する(
    構成: &フレーム構成, スキンあり: bool, 布あり: bool, 粒子あり: bool
) -> Result<(), レンダラーエラー> {
    let 対応 = [
        (スキンあり, フレーム段階::スキニング),
        (布あり, フレーム段階::布シミュレーション),
        (粒子あり, フレーム段階::粒子),
    ];
    for (素材あり, 段階) in 対応 {
        if 素材あり && !構成.含む(段階) {
            return Err(フレーム入力不一致エラー::フレーム構成素材不一致(段階).into());
        }
    }
    Ok(())
}
