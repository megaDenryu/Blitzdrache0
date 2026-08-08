//! 提示側の資源の生成後に走る資源生成に必要な借用値をまとめる。

use crate::auto_exposure::自動露出の設定;
use crate::cascade::影の一辺解像度;
use crate::cloth_material::布素材;
use crate::frame_composition::フレーム構成;
use crate::indirect_lighting::照明問い合わせ契約;
use crate::local_visibility::局所可視性の描画設定;
use crate::particle_material::粒子素材;
use crate::render_scene_material::描画シーン素材;
use crate::renderer::presentation::提示;
use crate::shader_bundle::シェーダー束;
use crate::skin_mesh::スキンメッシュ素材;
use crate::temporal_reconstruction::時間再構成の描画設定;
use crate::vulkan::gpu_environment::GPU環境;

pub(in crate::renderer::generate) struct 生成要求<'a> {
    pub(in crate::renderer::generate) 環境: &'a GPU環境,
    pub(in crate::renderer::generate) 提示: &'a 提示,
    pub(in crate::renderer::generate) シェーダー: &'a シェーダー束,
    pub(in crate::renderer::generate) 描画シーン: &'a 描画シーン素材,
    pub(in crate::renderer::generate) スキン: Option<&'a スキンメッシュ素材>,
    pub(in crate::renderer::generate) 布: Option<&'a 布素材>,
    pub(in crate::renderer::generate) 粒子素材: Option<&'a 粒子素材>,
    pub(in crate::renderer::generate) フレーム構成: フレーム構成,
    pub(in crate::renderer::generate) 照明問い合わせ契約: 照明問い合わせ契約,
    /// 世界が宣言した露出の決め方と、導出に要る定数。ポスト処理を組む構成でだけ資源になる。
    pub(in crate::renderer::generate) 自動露出の設定: 自動露出の設定,
    /// 世界が宣言した拡散間接方式と、遮蔽の標本化とぼかしが読む数値の設定。方式に関わらず資源は作る。
    pub(in crate::renderer::generate) 局所可視性の描画設定: 局所可視性の描画設定,
    /// 世界が宣言した時間再構成方式と、パスが押し込む2つの数。方式に関わらず4枚とパスの実体を作る。
    pub(in crate::renderer::generate) 時間再構成の描画設定: 時間再構成の描画設定,
    /// シャドウマップ資源の一辺。多段の構築が使う一辺と同じ値を呼び出し元が渡す責務を負う。
    pub(in crate::renderer::generate) 影の一辺: 影の一辺解像度,
    pub(in crate::renderer::generate) タイムスタンプ対応か: bool,
    pub(in crate::renderer::generate) タイムスタンプ周期ns: f32,
}
