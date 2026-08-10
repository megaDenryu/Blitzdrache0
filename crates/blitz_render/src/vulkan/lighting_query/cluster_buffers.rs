//! 1つのフレームスロットが持つクラスタ格子の2本のバッファ。触れるのはセルごとの区間表と、その区間が指す
//! 局所光の添字列だけであり、誰が書いて誰が読むかを知らない。
//!
//! ホスト可視でなくデバイスローカルで確保するのは、この2本を書くのが選別のコンピュートだけであり、CPUが
//! 1バイトも書かないためである。書き込みの口を持たないことが、CPUから書ける余地を無くす。
//!
//! フレームスロットごとに持つのは、同じフレームの中で書いてから読む資源だからである。1本を全スロットで
//! 共有すると、次のフレームの選別が前のフレームの画素段が読んでいる最中の内容を上書きしうる。

use ash::vk;

use crate::clustered_lighting::{クラスタ光添字列の要素数, クラスタ格子の分割数};
use crate::error::レンダラーエラー;
use crate::vulkan::device_buffer;
use crate::vulkan::tracked_device::GPUデバイス;

/// セル1つぶんの区間表のバイト数。`shaders/lighting_query_records.slang`の`ClusterCell`(開始位置と件数の2つ)と一致させる。
const セル1つのバイト数: usize = 8;
/// クラスタ光添字列の1要素のバイト数。局所光レコード列への添字1つである。
const 添字1つのバイト数: usize = 4;

pub(super) struct クラスタ格子の資源 {
    pub(super) 格子: vk::Buffer,
    格子のメモリ: vk::DeviceMemory,
    pub(super) 光添字列: vk::Buffer,
    光添字列のメモリ: vk::DeviceMemory,
}

/// 仮置きの分割数が決める格子の2本のバイト数。ヘッダへ書く分割数と同じ1箇所から読む。
fn 格子と光添字列のバイト数() -> (u64, u64) {
    let 分割数 = クラスタ格子の分割数::仮置きの分割数を返す();
    let セルの総数 = usize::try_from(分割数.セルの総数()).unwrap_or_else(|_| panic!("セルの総数がusizeに収まらない"));
    let 格子 = u64::try_from(セルの総数 * セル1つのバイト数).unwrap_or_else(|_| panic!("クラスタ格子のバイト数がu64に収まらない"));
    let 光添字列 =
        u64::try_from(クラスタ光添字列の要素数(分割数) * 添字1つのバイト数).unwrap_or_else(|_| panic!("クラスタ光添字列のバイト数がu64に収まらない"));
    (格子, 光添字列)
}

impl クラスタ格子の資源 {
    /// 2本を順に確保する。途中で失敗したら、そこまでに確保したぶんをその場で破棄する。
    pub(super) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    ) -> Result<Self, レンダラーエラー> {
        let (格子のバイト数, 光添字列のバイト数) = 格子と光添字列のバイト数();
        let 用途 = vk::BufferUsageFlags::STORAGE_BUFFER;
        let (格子, 格子のメモリ) = device_buffer::確保する(device, メモリプロパティ, 格子のバイト数, 用途)?;
        match device_buffer::確保する(device, メモリプロパティ, 光添字列のバイト数, 用途) {
            Ok((光添字列, 光添字列のメモリ)) => Ok(Self {
                格子,
                格子のメモリ,
                光添字列,
                光添字列のメモリ,
            }),
            Err(誤り) => {
                // 安全性: 格子はこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_buffer(格子, None) };
                device.メモリを解放する(格子のメモリ);
                Err(誤り)
            }
        }
    }

    /// 注意: 確保の逆順に破棄する。
    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        // 安全性: 2本のバッファとメモリはSelfが唯一の所有者であり、破棄時点でGPU側の使用完了を呼び出し元が保証する。
        unsafe { device.destroy_buffer(self.光添字列, None) };
        device.メモリを解放する(self.光添字列のメモリ);
        // 安全性: 同上。
        unsafe { device.destroy_buffer(self.格子, None) };
        device.メモリを解放する(self.格子のメモリ);
    }
}
