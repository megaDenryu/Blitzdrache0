//! 位置とラグランジュ乗数の読み戻し。ホスト可視の受け皿を2本確保し、転送パスで点と乗数を写して、送信の完了後に読む。
//! コピーをパス宣言にするのは、最後の反復の書き込みとコピーの間のバリアをグラフの導出に任せるためである。

use ash::vk;

use super::XPBD計測一式;
use crate::error::レンダラーエラー;
use crate::vulkan::allocator::{GPU資源の確保係, 専用メモリ付きバッファ};
use crate::vulkan::graph::{グラフ, バッファハンドル, バッファ用途, パス宣言, パス種別};
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) struct 読み戻しの受け皿 {
    位置: 専用メモリ付きバッファ,
    乗数: 専用メモリ付きバッファ,
    点の数: usize,
    拘束の数: usize,
}

impl 読み戻しの受け皿 {
    pub(super) fn 確保する(確保係: &GPU資源の確保係<'_>, 点の数: u32, 拘束の数: u32) -> Result<Self, レンダラーエラー> {
        let 点の数 = 数へ(点の数);
        let 拘束の数 = 数へ(拘束の数);
        let 用途 = vk::BufferUsageFlags::TRANSFER_DST;
        let 位置 = 確保係.読み戻し先のホスト可視バッファを確保する(バイト数へ(点の数 * 16), 用途)?;
        let 乗数 = match 確保係.読み戻し先のホスト可視バッファを確保する(バイト数へ(拘束の数 * 4), 用途) {
            Ok(乗数) => 乗数,
            Err(誤り) => {
                位置.破棄する(確保係.論理デバイス());
                return Err(誤り);
            }
        };
        Ok(Self {
            位置,
            乗数,
            点の数,
            拘束の数,
        })
    }

    /// 点と乗数をこの受け皿へ写す転送パスを積む。
    pub(super) fn コピーを積む<'a>(&'a self, グラフ: &mut グラフ<'a>, 点: バッファハンドル, 乗数: バッファハンドル) {
        let 点のバイト数 = バイト数へ(self.点の数 * 16);
        let 乗数のバイト数 = バイト数へ(self.拘束の数 * 4);
        let 位置の受け = self.位置.バッファのハンドル();
        let 乗数の受け = self.乗数.バッファのハンドル();
        グラフ.パスを積む(パス宣言::生成する(
            "XPBD読み戻し",
            Vec::new(),
            Vec::new(),
            vec![(点, バッファ用途::転送元), (乗数, バッファ用途::転送元)],
            Vec::new(),
            パス種別::転送,
            move |文脈| {
                let device = 文脈.積み先().論理デバイス();
                let command_buffer = 文脈.積み先().コマンドバッファ();
                // 安全性: command_bufferは記録中、転送元は用途の宣言からグラフがバリアを導き、受け皿は同じ長さで確保済みである。
                unsafe {
                    device.cmd_copy_buffer(
                        command_buffer,
                        文脈.宣言済みのバッファを参照する(点),
                        位置の受け,
                        &[vk::BufferCopy::default().size(点のバイト数)],
                    );
                    device.cmd_copy_buffer(
                        command_buffer,
                        文脈.宣言済みのバッファを参照する(乗数),
                        乗数の受け,
                        &[vk::BufferCopy::default().size(乗数のバイト数)],
                    );
                }
            },
        ));
    }

    /// 前提: コピーの送信がフェンス待機で完了済みである。
    pub(super) fn 読む(&self, device: &ash::Device) -> Result<(Vec<[f32; 4]>, Vec<f32>), レンダラーエラー> {
        let 位置のバイト列 = self.位置.ホスト可視のバイト列を写し取る(device, self.点の数 * 16)?;
        let 乗数のバイト列 = self.乗数.ホスト可視のバイト列を写し取る(device, self.拘束の数 * 4)?;
        let 位置 = 位置のバイト列
            .chunks_exact(16)
            .map(|塊| [単精度(塊, 0), 単精度(塊, 4), 単精度(塊, 8), 単精度(塊, 12)])
            .collect();
        let 乗数 = 乗数のバイト列.chunks_exact(4).map(|塊| 単精度(塊, 0)).collect();
        Ok((位置, 乗数))
    }

    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        self.位置.破棄する(device);
        self.乗数.破棄する(device);
    }
}

impl XPBD計測一式 {
    pub(super) fn 読み戻しの受け皿を確保する(
        &self,
        確保係: &GPU資源の確保係<'_>,
    ) -> Result<読み戻しの受け皿, レンダラーエラー> {
        読み戻しの受け皿::確保する(確保係, self.点の数, self.拘束の数)
    }
}

fn 単精度(塊: &[u8], 開始: usize) -> f32 {
    f32::from_le_bytes([塊[開始], 塊[開始 + 1], 塊[開始 + 2], 塊[開始 + 3]])
}

fn 数へ(値: u32) -> usize {
    usize::try_from(値).unwrap_or_else(|_| panic!("数がusizeに収まらない: {値}"))
}

fn バイト数へ(値: usize) -> u64 {
    u64::try_from(値).unwrap_or_else(|_| panic!("バイト数がu64に収まらない: {値}"))
}
