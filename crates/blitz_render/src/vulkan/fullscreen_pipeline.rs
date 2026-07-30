//! 全画面三角形ポストプロセス用パイプラインの共通組み立て(トーンマップ・ブルームが共用):
//! 頂点入力なし・深度なし・ブレンドなし・動的ビューポート/シザー。
//! フラグメントエントリ名とプッシュ定数バイト数(0なら範囲なし)だけがパスごとに異なる。

mod finish;
mod fixed_function;

/// 生成結果からハンドルを取り出す工程。ブレンドや深度の要否が違う全画面パス(空中遠近合成)も
/// 失敗時のlayoutの後始末は同じであるため、固定機能を共有しないパスへもこの1つを貸す。
pub(crate) use finish::取り出す as パイプラインを取り出す;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;
use crate::vulkan::shader_module;

/// 前提: 全ポストプロセスシェーダーの頂点エントリはこの名前で統一する(tonemap.slang/bloom.slang)。
const 頂点エントリ名: &std::ffi::CStr = c"vertexMain";

pub(crate) fn 組み立てる(
    device: &ash::Device,
    カラー形式: vk::Format,
    ディスクリプタlayout: vk::DescriptorSetLayout,
    シェーダー: &シェーダー一式,
    フラグメントエントリ名: &std::ffi::CStr,
    プッシュ定数バイト数: u32,
) -> Result<(vk::Pipeline, vk::PipelineLayout), レンダラーエラー> {
    let 頂点モジュール = shader_module::生成する(device, シェーダー.頂点コード())?;
    let フラグメントモジュール = match shader_module::生成する(device, シェーダー.フラグメントコード()) {
        Ok(モジュール) => モジュール,
        Err(誤り) => {
            // 安全性: 頂点モジュールはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_shader_module(頂点モジュール, None) };
            return Err(誤り);
        }
    };

    let 結果 = fixed_function::組み立てる(
        device,
        カラー形式,
        ディスクリプタlayout,
        頂点モジュール,
        頂点エントリ名,
        フラグメントモジュール,
        フラグメントエントリ名,
        プッシュ定数バイト数,
    );

    // 安全性: モジュールはパイプライン生成呼び出しの間だけ必要で、生成後は破棄してよい。
    unsafe {
        device.destroy_shader_module(頂点モジュール, None);
        device.destroy_shader_module(フラグメントモジュール, None);
    }
    結果
}
