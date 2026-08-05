//! 局所可視度の2枚の画像をグラフへ登録するときの初期状態。
//!
//! 焼いた画像の初期状態と別に持つのは、この2枚が記憶画像として読まれる点が違うためである。焼いた画像はサンプラーで
//! しか読まれず、両側ぼかしは生の可視度を記憶画像として読む。段とアクセスは「前のフレームでこの画像へ最後に起きたこと」の
//! 和であり、その和がこちらだけSHADER_STORAGE_READを含む。
//!
//! 注意: レイアウトはGENERALである。2枚はフレームをまたいで中身を保つ。拡散間接方式が環境のみの世界では
//! パスが1本も積まれず、確保時に埋めた遮蔽なしの符号値がそのまま残り続ける。UNDEFINEDにするとその値を捨てられる。

use ash::vk;

use crate::vulkan::graph::state::画像状態;

/// 2枚に共通の、前フレーム直後を想定した状態。
pub(crate) fn 局所可視度の画像の前フレーム直後状態() -> 画像状態 {
    画像状態::生成する(
        vk::PipelineStageFlags2::COMPUTE_SHADER | vk::PipelineStageFlags2::FRAGMENT_SHADER,
        vk::AccessFlags2::SHADER_STORAGE_WRITE | vk::AccessFlags2::SHADER_STORAGE_READ | vk::AccessFlags2::SHADER_SAMPLED_READ,
        vk::ImageLayout::GENERAL,
    )
}
