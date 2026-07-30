//! 画像用途 → (stage, access, layout) の写像を1関数に集約する。
//! 参照: `_doc/設計/レンダーグラフ.md`「用途(usage)は1関数に集約」。

use ash::vk;

use super::画像用途;
use crate::vulkan::graph::state::画像状態;

/// 画像用途から、その用途で画像を使うときのVulkan同期状態を導く。
pub(crate) fn 状態へ写像する(用途: 画像用途) -> 画像状態 {
    match 用途 {
        // 注意: 読みも含める。カラーアタッチメントをLOADするパスはレンダリング開始そのものが読みであり、
        // 固定機能ブレンドを有効にしたパスは宛先のカラーを読む。読みを落とすと、直前のパスが同じ画像へ書いた内容が
        // 合成側の読みへ可視化されず、依存が書き込みから書き込みへの向きだけになる(LOADするのは空・粒子・UI・
        // 空中遠近合成の4パスで、うち空中遠近合成とUIはブレンドで宛先も読む)。
        // LOADするカラー用途を別の腕に分けないのは、載せる情報が`パス種別`の`クリア指定`と同じであり、
        // 2箇所が食い違うと絵にしか現れない同期の欠陥になるためである。深度出力が同じ理由で読みを持つ。
        画像用途::カラー出力 => 画像状態::生成する(
            vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT,
            vk::AccessFlags2::COLOR_ATTACHMENT_WRITE | vk::AccessFlags2::COLOR_ATTACHMENT_READ,
            vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        ),
        // 注意: 読みも含める。深度アタッチメントを束ねるパスは深度テストで既存の値を比べ、LOADするパスは
        // レンダリング開始そのものが読みである。読みを落とすと、直前に別の用途(空中遠近合成の深度シェーダー読み)から
        // 遷移してきた場合にLOADの読みがバリアの範囲外になり、validationがREAD_AFTER_WRITEを報告する。
        画像用途::深度出力 => 画像状態::生成する(
            深度書き込み段(),
            vk::AccessFlags2::DEPTH_STENCIL_ATTACHMENT_WRITE | vk::AccessFlags2::DEPTH_STENCIL_ATTACHMENT_READ,
            vk::ImageLayout::DEPTH_ATTACHMENT_OPTIMAL,
        ),
        画像用途::シェーダー読みフラグメント => 画像状態::生成する(
            vk::PipelineStageFlags2::FRAGMENT_SHADER,
            vk::AccessFlags2::SHADER_SAMPLED_READ,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        ),
        画像用途::深度シェーダー読み => 画像状態::生成する(
            vk::PipelineStageFlags2::FRAGMENT_SHADER,
            vk::AccessFlags2::SHADER_SAMPLED_READ,
            vk::ImageLayout::DEPTH_READ_ONLY_OPTIMAL,
        ),
        画像用途::コンピュート書き => 画像状態::生成する(
            vk::PipelineStageFlags2::COMPUTE_SHADER,
            vk::AccessFlags2::SHADER_STORAGE_WRITE,
            vk::ImageLayout::GENERAL,
        ),
        画像用途::コンピュート読み => 画像状態::生成する(
            vk::PipelineStageFlags2::COMPUTE_SHADER,
            vk::AccessFlags2::SHADER_SAMPLED_READ,
            vk::ImageLayout::GENERAL,
        ),
        画像用途::大気のベイク済み画像のフラグメント参照 => 画像状態::生成する(
            vk::PipelineStageFlags2::FRAGMENT_SHADER,
            vk::AccessFlags2::SHADER_SAMPLED_READ,
            vk::ImageLayout::GENERAL,
        ),
        画像用途::転送元 => 画像状態::生成する(
            vk::PipelineStageFlags2::COPY,
            vk::AccessFlags2::TRANSFER_READ,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
        ),
        画像用途::転送先 => 画像状態::生成する(
            vk::PipelineStageFlags2::COPY,
            vk::AccessFlags2::TRANSFER_WRITE,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        ),
        画像用途::提示 => 画像状態::生成する(
            vk::PipelineStageFlags2::BOTTOM_OF_PIPE,
            vk::AccessFlags2::empty(),
            vk::ImageLayout::PRESENT_SRC_KHR,
        ),
    }
}

/// 深度アタッチメントの読み書きが起こりうる段（early/late fragment tests）。
/// 深度出力の書き込み段と、深度の初期状態(前フレーム書き込み直後)の双方で使うため公開する。
pub(crate) fn 深度書き込み段() -> vk::PipelineStageFlags2 {
    vk::PipelineStageFlags2::EARLY_FRAGMENT_TESTS | vk::PipelineStageFlags2::LATE_FRAGMENT_TESTS
}

/// アクセスマスクに書き込みビットが含まれるか。バリア省略判定に使う
/// （読み→読みはレイアウト同一なら省略。参照: レンダーグラフ.md「同期導出の規則」）。
pub(crate) fn 書き込みを含むか(access: vk::AccessFlags2) -> bool {
    access.contains(vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
        || access.contains(vk::AccessFlags2::DEPTH_STENCIL_ATTACHMENT_WRITE)
        || access.contains(vk::AccessFlags2::TRANSFER_WRITE)
        || access.contains(vk::AccessFlags2::SHADER_WRITE)
        || access.contains(vk::AccessFlags2::SHADER_STORAGE_WRITE)
        || access.contains(vk::AccessFlags2::HOST_WRITE)
        || access.contains(vk::AccessFlags2::MEMORY_WRITE)
}
