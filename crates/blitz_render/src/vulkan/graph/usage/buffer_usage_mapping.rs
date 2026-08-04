//! バッファ用途 → (stage, access) の写像を1関数に集約する。画像用途の
//! `image_usage_mapping`と対になるが、バッファにはレイアウトが無いため戻り値は
//! `バッファ状態`(2つ組)になる。参照: `_doc/設計/レンダーグラフ.md`「用途(usage)」。

use ash::vk;

use super::バッファ用途;
use crate::vulkan::graph::buffer_state::バッファ状態;

/// バッファ用途から、その用途でバッファを使うときのVulkan同期状態を導く。
pub(crate) fn 状態へ写像する(用途: バッファ用途) -> バッファ状態 {
    match 用途 {
        バッファ用途::頂点読み => {
            バッファ状態::生成する(vk::PipelineStageFlags2::VERTEX_INPUT, vk::AccessFlags2::VERTEX_ATTRIBUTE_READ)
        }
        バッファ用途::インデックス読み => {
            バッファ状態::生成する(vk::PipelineStageFlags2::INDEX_INPUT, vk::AccessFlags2::INDEX_READ)
        }
        バッファ用途::シェーダー定数読み => バッファ状態::生成する(
            vk::PipelineStageFlags2::VERTEX_SHADER | vk::PipelineStageFlags2::FRAGMENT_SHADER,
            vk::AccessFlags2::UNIFORM_READ,
        ),
        バッファ用途::コンピュート読み => {
            バッファ状態::生成する(vk::PipelineStageFlags2::COMPUTE_SHADER, vk::AccessFlags2::SHADER_STORAGE_READ)
        }
        バッファ用途::コンピュート書き => {
            バッファ状態::生成する(vk::PipelineStageFlags2::COMPUTE_SHADER, vk::AccessFlags2::SHADER_STORAGE_WRITE)
        }
        バッファ用途::コンピュート読み書き => バッファ状態::生成する(
            vk::PipelineStageFlags2::COMPUTE_SHADER,
            vk::AccessFlags2::SHADER_STORAGE_READ | vk::AccessFlags2::SHADER_STORAGE_WRITE,
        ),
        バッファ用途::画素段シェーダー読み => {
            バッファ状態::生成する(vk::PipelineStageFlags2::FRAGMENT_SHADER, vk::AccessFlags2::SHADER_STORAGE_READ)
        }
        バッファ用途::頂点段シェーダー読み => {
            バッファ状態::生成する(vk::PipelineStageFlags2::VERTEX_SHADER, vk::AccessFlags2::SHADER_STORAGE_READ)
        }
        バッファ用途::転送元 => バッファ状態::生成する(vk::PipelineStageFlags2::COPY, vk::AccessFlags2::TRANSFER_READ),
        バッファ用途::転送先 => バッファ状態::生成する(vk::PipelineStageFlags2::COPY, vk::AccessFlags2::TRANSFER_WRITE),
    }
}

/// アクセスマスクに書き込みビットが含まれるか。バリア省略判定に使う
/// （読み→読みはバッファにレイアウトが無いため常に省略可能。参照:
/// レンダーグラフ.md「同期導出の規則」）。
pub(crate) fn 書き込みを含むか(access: vk::AccessFlags2) -> bool {
    access.contains(vk::AccessFlags2::SHADER_STORAGE_WRITE)
        || access.contains(vk::AccessFlags2::TRANSFER_WRITE)
        || access.contains(vk::AccessFlags2::SHADER_WRITE)
        || access.contains(vk::AccessFlags2::HOST_WRITE)
        || access.contains(vk::AccessFlags2::MEMORY_WRITE)
}
