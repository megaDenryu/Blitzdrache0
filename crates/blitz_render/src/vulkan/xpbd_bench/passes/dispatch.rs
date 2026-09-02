//! 1つの計測コンピュートパスをレンダーグラフへ登録する。彩色の方式だけがプッシュ定数(色の区間)を伴う。
//! 班のスレッド数は`shaders/xpbd_step.slang`等の`numthreads`と同じ値であり、`cargo xtask conform`が突き合わせる。

use ash::vk;

use crate::vulkan::graph::{グラフ, バッファハンドル, バッファ用途, パス宣言, パス種別};

const 班のスレッド数: u32 = 64;

/// 1回のディスパッチの束縛。パイプラインごとに変わるのはパイプラインとスレッド数だけである。
#[derive(Clone, Copy)]
pub(super) struct 発行の束縛 {
    pub(super) layout: vk::PipelineLayout,
    pub(super) セット: vk::DescriptorSet,
}

/// 1本のパスの発行の指定。読み書きの宣言はバリアの導出だけが読む。
pub(super) struct 発行の指定 {
    pub(super) 名前: &'static str,
    pub(super) 読み: Vec<(バッファハンドル, バッファ用途)>,
    pub(super) 書き: Vec<(バッファハンドル, バッファ用途)>,
    pub(super) pipeline: vk::Pipeline,
    pub(super) スレッド数: u32,
    pub(super) 色の区間: Option<[u32; 2]>, // 彩色の方式だけがプッシュ定数で運ぶ(開始, 本数)
}

pub(super) fn 積む<'a>(グラフ: &mut グラフ<'a>, 束縛: 発行の束縛, 指定: 発行の指定) {
    let セット一覧 = [束縛.セット];
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
                device.cmd_bind_descriptor_sets(command_buffer, vk::PipelineBindPoint::COMPUTE, 束縛.layout, 0, &セット一覧, &[]);
                if let Some([開始, 本数]) = 色の区間 {
                    let mut バイト列 = [0u8; 8];
                    バイト列[..4].copy_from_slice(&開始.to_le_bytes());
                    バイト列[4..].copy_from_slice(&本数.to_le_bytes());
                    device.cmd_push_constants(command_buffer, 束縛.layout, vk::ShaderStageFlags::COMPUTE, 0, &バイト列);
                }
                device.cmd_dispatch(command_buffer, スレッド数.div_ceil(班のスレッド数), 1, 1);
            }
        },
    ));
}
