//! 読み戻しの受け皿: 粒子と乗数を写し取るホスト可視のバッファ2本と、その中身を粒子の位置と乗数の値へ読む口。
//! 転送パスの積み方は`readback`が持ち、ここは確保・バイト列の解釈・破棄だけを担当する。
//! 乗数のバッファは距離拘束の後ろに目標拘束が続く1本であり、読む口が距離拘束の数で2つの並びへ分ける。

use ash::vk;

use crate::cloth_material::布の読み戻し;
use crate::error::レンダラーエラー;
use crate::vulkan::allocator::{GPU資源の確保係, 専用メモリ付きバッファ};
use crate::vulkan::tracked_device::GPUデバイス;

/// 粒子1件の32バイト(位置と逆質量のfloat4 + 速度のfloat4)。位置は先頭の12バイトである。
const 粒子1件のバイト数: usize = 32;
const 乗数1件のバイト数: usize = 4;

/// 転送パスが1本のバッファを写すのに要る、受け側のハンドルとバイト数。
#[derive(Clone, Copy)]
pub(super) struct コピーの指定 {
    pub(super) 受け: vk::Buffer,
    pub(super) バイト数: u64,
}

pub(super) struct 読み戻しの受け皿 {
    粒子: 専用メモリ付きバッファ,
    乗数: 専用メモリ付きバッファ,
    粒子のバイト数: usize,
    乗数のバイト数: usize,
    距離拘束の数: usize,
}

impl 読み戻しの受け皿 {
    pub(super) fn 確保する(
        確保係: &GPU資源の確保係<'_>,
        粒子数: u32,
        拘束の数: u32,
        目標拘束の数: u32,
    ) -> Result<Self, レンダラーエラー> {
        let 粒子のバイト数 = 件数をusizeへ変換する(粒子数) * 粒子1件のバイト数;
        let 距離拘束の数 = 件数をusizeへ変換する(拘束の数);
        let 乗数のバイト数 = (距離拘束の数 + 件数をusizeへ変換する(目標拘束の数)) * 乗数1件のバイト数;
        let 用途 = vk::BufferUsageFlags::TRANSFER_DST;
        let 粒子 = 確保係.読み戻し先のホスト可視バッファを確保する(バイト数をu64へ変換する(粒子のバイト数), 用途)?;
        let 乗数 = match 確保係.読み戻し先のホスト可視バッファを確保する(バイト数をu64へ変換する(乗数のバイト数), 用途)
        {
            Ok(乗数) => 乗数,
            Err(誤り) => {
                粒子.破棄する(確保係.論理デバイス());
                return Err(誤り);
            }
        };
        Ok(Self {
            粒子,
            乗数,
            粒子のバイト数,
            乗数のバイト数,
            距離拘束の数,
        })
    }

    pub(super) fn 粒子のコピーの指定(&self) -> コピーの指定 {
        コピーの指定 {
            受け: self.粒子.バッファのハンドル(),
            バイト数: バイト数をu64へ変換する(self.粒子のバイト数),
        }
    }

    pub(super) fn 乗数のコピーの指定(&self) -> コピーの指定 {
        コピーの指定 {
            受け: self.乗数.バッファのハンドル(),
            バイト数: バイト数をu64へ変換する(self.乗数のバイト数),
        }
    }

    /// 前提: コピーの送信がフェンス待機で完了済みである。
    pub(super) fn 読む(&self, device: &ash::Device) -> Result<布の読み戻し, レンダラーエラー> {
        let 粒子のバイト列 = self.粒子.ホスト可視のバイト列を写し取る(device, self.粒子のバイト数)?;
        let 乗数のバイト列 = self.乗数.ホスト可視のバイト列を写し取る(device, self.乗数のバイト数)?;
        let mut 乗数一覧: Vec<f32> = 乗数のバイト列.chunks_exact(乗数1件のバイト数).map(|塊| 単精度(塊, 0)).collect();
        let 目標拘束のラグランジュ乗数一覧 = 乗数一覧.split_off(self.距離拘束の数);
        Ok(布の読み戻し {
            位置一覧: 粒子のバイト列
                .chunks_exact(粒子1件のバイト数)
                .map(|塊| [単精度(塊, 0), 単精度(塊, 4), 単精度(塊, 8)])
                .collect(),
            ラグランジュ乗数一覧: 乗数一覧,
            目標拘束のラグランジュ乗数一覧,
        })
    }

    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        self.粒子.破棄する(device);
        self.乗数.破棄する(device);
    }
}

fn 単精度(塊: &[u8], 開始: usize) -> f32 {
    f32::from_le_bytes([塊[開始], 塊[開始 + 1], 塊[開始 + 2], 塊[開始 + 3]])
}

fn 件数をusizeへ変換する(値: u32) -> usize {
    usize::try_from(値).unwrap_or_else(|_| panic!("数がusizeに収まらない: {値}"))
}

fn バイト数をu64へ変換する(値: usize) -> u64 {
    u64::try_from(値).unwrap_or_else(|_| panic!("バイト数がu64に収まらない: {値}"))
}
