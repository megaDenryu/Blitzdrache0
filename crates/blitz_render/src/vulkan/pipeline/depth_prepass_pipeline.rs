//! 深度プリパスのパイプライン(深度のみ・画素段なし)の生成。シェーダーモジュールの生成から固定機能の組み立て、
//! モジュールの破棄までの1回ぶんの寿命をここが持つ。固定機能の中身は`assemble`にある。
//!
//! 受け取る`シェーダー一式`が色パスと同じものであることが、この段の要点である。頂点段のSPIR-Vを色パスと共有し、
//! 深度専用の頂点段を焼かないことが、両者が同じ位置を出すことの最も強い保証である
//! (参照: `_doc/設計/放射輝度問い合わせ階層.md`「IIaの実装設計」)。画素段のSPIR-Vはこの段では1度も読まない。

mod assemble;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;
use crate::vulkan::allocator::GPU資源の確保係;

pub(super) fn 生成する(
    確保係: &GPU資源の確保係<'_>,
    深度形式: vk::Format,
    標本数: vk::SampleCountFlags,
    layout: vk::PipelineLayout,
    シェーダー: &シェーダー一式,
) -> Result<vk::Pipeline, レンダラーエラー> {
    let device = 確保係.論理デバイス();
    let 頂点モジュール = 確保係.シェーダーモジュールを生成する(シェーダー.頂点コード())?;
    let 結果 = assemble::組み立てる(device, 深度形式, 標本数, layout, 頂点モジュール);
    // 安全性: モジュールはパイプライン生成呼び出しの間だけ必要で、生成後は破棄してよい。
    unsafe { device.destroy_shader_module(頂点モジュール, None) };
    結果
}
