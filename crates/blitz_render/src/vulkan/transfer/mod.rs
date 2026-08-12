//! セットアップ時専用の転送実行環境: 一時コマンドバッファをグラフィックスキューへ
//! submitしてfence待ちで完了させるヘルパー(判断2)。頂点/インデックス/テクスチャの
//! ステージング転送、および `シーンを差し替える` の再アップロードが共通で使う。
//!
//! 論理デバイスを操作のたびに引数で受け取らず複製して保持するのは、転送を頼む側にデバイスを運ばせないためである。
//! 依存を保持しないと、転送を頼む側から見える署名にまでデバイスが漏れる。

mod pool;
mod session;
mod staging;

use ash::vk;

pub(crate) use session::転送コマンドを積む一時コマンドバッファ;
pub(crate) use staging::ステージング経由の転送係;

use crate::error::レンダラーエラー;
use crate::vulkan::unsent_command_buffers::未送信のコマンドバッファ数;

pub(crate) struct 転送実行環境 {
    device: ash::Device,
    queue: vk::Queue,
    command_pool: vk::CommandPool,
    未送信のコマンドバッファ数: 未送信のコマンドバッファ数,
}

impl 転送実行環境 {
    pub(crate) fn 生成する(device: &ash::Device, queue: vk::Queue, キューファミリ添字: u32) -> Result<Self, レンダラーエラー> {
        let command_pool = pool::生成する(device, キューファミリ添字)?;
        Ok(Self {
            device: device.clone(),
            queue,
            command_pool,
            未送信のコマンドバッファ数: 未送信のコマンドバッファ数::零から数え始める(),
        })
    }

    /// 一時コマンドバッファを1本確保して積み込みを開始する。
    /// 返る値は`送信して完了を待つ`で必ず閉じる。閉じないまま捨てると`破棄する`が止める。
    pub(crate) fn 転送コマンドを積み始める(
        &self,
    ) -> Result<session::転送コマンドを積む一時コマンドバッファ<'_>, レンダラーエラー> {
        session::転送コマンドを積む一時コマンドバッファ::積み始める(self)
    }

    /// 前提: 呼び出し元はGPUの全作業の完了を待っており、この環境から取った一時コマンドバッファをすべて送信済みである。
    pub(crate) fn 破棄する(&self) {
        self.未送信のコマンドバッファ数.未送信が1本も残っていないことを確かめる();
        // 安全性: command_poolはSelfが唯一の所有者であり、破棄時点でGPU側の使用が
        // device_wait_idle済みであることを呼び出し元が保証する。
        unsafe { self.device.destroy_command_pool(self.command_pool, None) };
    }
}
