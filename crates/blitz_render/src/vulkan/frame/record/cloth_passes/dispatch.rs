//! 1つの布コンピュートパスをレンダーグラフへ登録する。拘束の工程だけがプッシュ定数(色の区間)を伴う。
//! 班のスレッド数は布のシェーダーの`numthreads`と同じ値であり、`cargo xtask conform`が`cloth_constraint.slang`の写しと突き合わせる。

use ash::vk;

use crate::cloth_material::布の彩色の区間;
use crate::vulkan::frame::布描画入力;
use crate::vulkan::graph::{グラフ, バッファハンドル, バッファ用途, パス宣言, パス種別};

const 班のスレッド数: u32 = 64;

/// 1本のパスの発行の指定。読み書きの宣言はバリアの導出だけが読む。
pub(super) struct 発行の指定 {
    pub(super) 名前: &'static str,
    pub(super) 読み: Vec<(バッファハンドル, バッファ用途)>,
    pub(super) 書き: Vec<(バッファハンドル, バッファ用途)>,
    pub(super) pipeline: vk::Pipeline,
    pub(super) スレッド数: u32,
    pub(super) 色の区間: Option<布の彩色の区間>, // 拘束の工程だけがプッシュ定数で運ぶ
}

pub(super) fn 積む<'a>(グラフ: &mut グラフ<'a>, 入力: &'a 布描画入力, 指定: 発行の指定) {
    let セット一覧 = [入力.ディスクリプタセット];
    let layout = 入力.layout;
    let pipeline = 指定.pipeline;
    let スレッド数 = 指定.スレッド数;
    let 色の区間 = 指定.色の区間;
    グラフ.パスを積む(パス宣言::生成する(
        指定.名前,
        Vec::new(),
        Vec::new(),
        指定.読み,
        指定.書き,
        パス種別::コンピュート,
        move |文脈| {
            let device = 文脈.積み先().論理デバイス();
            let command_buffer = 文脈.積み先().コマンドバッファ();
            // 安全性: command_bufferは記録中で、pipeline・レイアウト・ディスクリプタセットは生成済み。
            // プッシュ定数の8バイトはパイプラインレイアウトが宣言した範囲そのものである。
            unsafe {
                device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::COMPUTE, pipeline);
                device.cmd_bind_descriptor_sets(command_buffer, vk::PipelineBindPoint::COMPUTE, layout, 0, &セット一覧, &[]);
                if let Some(区間) = 色の区間 {
                    let mut バイト列 = [0u8; 8];
                    バイト列[..4].copy_from_slice(&区間.開始.to_le_bytes());
                    バイト列[4..].copy_from_slice(&区間.本数.to_le_bytes());
                    device.cmd_push_constants(command_buffer, layout, vk::ShaderStageFlags::COMPUTE, 0, &バイト列);
                }
                device.cmd_dispatch(command_buffer, スレッド数.div_ceil(班のスレッド数), 1, 1);
            }
        },
    ));
}
