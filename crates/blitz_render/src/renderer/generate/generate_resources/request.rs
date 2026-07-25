//! スワップチェーン後の資源生成に必要な借用値をまとめる。

use crate::cloth_material::布素材;
use crate::frame_composition::フレーム構成;
use crate::particle_material::粒子素材;
use crate::render_scene_material::描画シーン素材;
use crate::shader_bundle::シェーダー束;
use crate::skin_mesh::スキンメッシュ素材;
use crate::vulkan;
use crate::vulkan::gpu_environment::GPU環境;

pub(in crate::renderer::generate) struct 生成要求<'a> {
    pub(in crate::renderer::generate) 環境: &'a GPU環境,
    pub(in crate::renderer::generate) swapchain: &'a vulkan::swapchain::スワップチェーン,
    pub(in crate::renderer::generate) シェーダー: &'a シェーダー束,
    pub(in crate::renderer::generate) 描画シーン: &'a 描画シーン素材,
    pub(in crate::renderer::generate) スキン: Option<&'a スキンメッシュ素材>,
    pub(in crate::renderer::generate) 布: Option<&'a 布素材>,
    pub(in crate::renderer::generate) 粒子素材: Option<&'a 粒子素材>,
    pub(in crate::renderer::generate) フレーム構成: フレーム構成,
    pub(in crate::renderer::generate) タイムスタンプ対応か: bool,
    pub(in crate::renderer::generate) タイムスタンプ周期ns: f32,
}
