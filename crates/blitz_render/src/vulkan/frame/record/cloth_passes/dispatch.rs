//! 1つの布コンピュートパスをレンダーグラフへ登録する。拘束の工程だけがプッシュ定数(拘束の区間: 開始・本数・乗数の開始)を伴う。
//! 班のスレッド数は布のシェーダーの`numthreads`と同じ値であり、`cargo xtask conform`が`cloth_constraint.slang`の写しと突き合わせる。

use ash::vk;

use crate::cloth_material::布の彩色の区間;
use crate::vulkan::frame::布描画入力;
use crate::vulkan::graph::{グラフ, バッファハンドル, バッファ用途, パス宣言, パス種別};

const 班のスレッド数: u32 = 64;

/// 拘束の工程がプッシュ定数で運ぶ3語。開始と本数はその拘束の並びの中の区間、乗数の開始は乗数バッファの中でその拘束族が始まる位置である。
/// 距離拘束と目標拘束のシェーダーは先頭の2語だけを読む(目標拘束は開始を乗数の開始として受ける)。曲げ拘束は3語目で乗数の開始を受ける。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct 拘束の区間の指定 {
    pub(super) 開始: u32,
    pub(super) 本数: u32,
    pub(super) 乗数の開始: u32,
}

impl 拘束の区間の指定 {
    /// 乗数の開始を持たない区間(距離拘束・零化・目標拘束・目標の確定)。
    pub(super) fn 色の区間から生成する(区間: 布の彩色の区間) -> Self {
        Self {
            開始: 区間.開始,
            本数: 区間.本数,
            乗数の開始: 0,
        }
    }
}

/// 1本のパスの発行の指定。読み書きの宣言はバリアの導出だけが読む。
pub(super) struct 発行の指定 {
    pub(super) 名前: &'static str,
    pub(super) 読み: Vec<(バッファハンドル, バッファ用途)>,
    pub(super) 書き: Vec<(バッファハンドル, バッファ用途)>,
    pub(super) pipeline: vk::Pipeline,
    pub(super) スレッド数: u32,
    pub(super) 色の区間: Option<拘束の区間の指定>, // 拘束の工程だけがプッシュ定数で運ぶ
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
            // プッシュ定数の12バイトはパイプラインレイアウトが宣言した範囲そのものである。
            unsafe {
                device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::COMPUTE, pipeline);
                device.cmd_bind_descriptor_sets(command_buffer, vk::PipelineBindPoint::COMPUTE, layout, 0, &セット一覧, &[]);
                if let Some(区間) = 色の区間 {
                    let mut バイト列 = [0u8; 12];
                    バイト列[..4].copy_from_slice(&区間.開始.to_le_bytes());
                    バイト列[4..8].copy_from_slice(&区間.本数.to_le_bytes());
                    バイト列[8..].copy_from_slice(&区間.乗数の開始.to_le_bytes());
                    device.cmd_push_constants(command_buffer, layout, vk::ShaderStageFlags::COMPUTE, 0, &バイト列);
                }
                device.cmd_dispatch(command_buffer, スレッド数.div_ceil(班のスレッド数), 1, 1);
            }
        },
    ));
}
