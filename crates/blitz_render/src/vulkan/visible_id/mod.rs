//! そのフレームに描く個体の添字をパス別・LOD段別の区間へ並べたストレージバッファ。シーンとシャドウの両方の頂点シェーダーが
//! これを参照する。フレームインフライトごとに1本を持ち、束の読込時に容量ぶんを一度だけ確保する。
//! 可視判定の結果や段の選択が変わってもGPU確保は起こらず、毎フレーム行うのは書き込みだけである
//! (参照: `_doc/設計/植生インスタンスと物量計測.md`「描画発行」)。
//! 1個体はシーンの区間と帯ごとの区間へ最大でパス数ぶん現れるため、書き込む列の長さは個体数と一致せず、
//! フレームごとに容量までのあいだで変わる。
//! 注意: 確保時に書く恒等の列は一度も書き換えられていないことが前提である。ある描画対象が可視個体選択を持つフレームと
//! 持たないフレームを行き来すると、選択を持たないフレームで前のフレームの並べ替えた列を読んでしまう。
//! インスタンス群は可視判定と段選択の入切によらず毎フレーム選択を受け取り、通常メッシュと地形は一度も受け取らないため、
//! この前提が保たれている。
//! 前提: 個体数は1以上である(描画対象素材の個体変換列が非空であることによる)。
//! 確保と破棄はこのファイル、容量と初期の中身は`capacity`、ディスクリプタが結ぶ参照は`reference`、毎フレームの書き込みは`write`にある。

mod capacity;
mod reference;
mod write;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::host_buffer;
use crate::vulkan::sync::{フレームインフライト数, フレームスロット添字};
use crate::vulkan::tracked_device::GPUデバイス;

pub(crate) use reference::可視ID列参照;

/// 可視ID1件のバイト長。シェーダーの`StructuredBuffer<uint>`の1要素と一致する。
pub(crate) const 可視IDバイト長: usize = 4;

pub(crate) struct 可視ID列バッファ {
    buffer一覧: [vk::Buffer; フレームインフライト数],
    memory一覧: [vk::DeviceMemory; フレームインフライト数],
    範囲: vk::DeviceSize,
    /// 書き込める可視IDの件数。個体数とパス数の積であり、書き込みの側はこれを超えないことだけを確かめる。
    容量件数: usize,
}

impl 可視ID列バッファ {
    pub(crate) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        個体数: u32,
    ) -> Result<Self, レンダラーエラー> {
        let 初期列 = capacity::初期バイト列を作る(個体数);
        let mut buffer一覧 = [vk::Buffer::null(); フレームインフライト数];
        let mut memory一覧 = [vk::DeviceMemory::null(); フレームインフライト数];
        for 添字 in 0..フレームインフライト数 {
            match host_buffer::確保して書き込む(device, メモリプロパティ, &初期列, vk::BufferUsageFlags::STORAGE_BUFFER) {
                Ok((buffer, memory)) => {
                    buffer一覧[添字] = buffer;
                    memory一覧[添字] = memory;
                }
                Err(誤り) => {
                    確保済みを解放する(device, &buffer一覧, &memory一覧, 添字);
                    return Err(誤り);
                }
            }
        }
        let 範囲 = u64::try_from(初期列.len()).unwrap_or_else(|_| panic!("可視ID列のバイト長がu64に収まらない"));
        Ok(Self {
            buffer一覧,
            memory一覧,
            範囲,
            容量件数: capacity::容量件数を求める(個体数),
        })
    }

    pub(crate) fn 参照(&self) -> 可視ID列参照 {
        可視ID列参照::生成する(self.buffer一覧, self.範囲)
    }

    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        確保済みを解放する(device, &self.buffer一覧, &self.memory一覧, フレームインフライト数);
    }

    fn memory(&self, フレーム添字: フレームスロット添字) -> vk::DeviceMemory {
        self.memory一覧[フレーム添字.配列添字()]
    }
}

fn 確保済みを解放する(
    device: &GPUデバイス,
    buffer一覧: &[vk::Buffer; フレームインフライト数],
    memory一覧: &[vk::DeviceMemory; フレームインフライト数],
    件数: usize,
) {
    for 添字 in 0..件数 {
        // 安全性: bufferとmemoryはSelfが所有し、呼び出し元がGPU使用完了を保証する。
        unsafe { device.destroy_buffer(buffer一覧[添字], None) };
        device.メモリを解放する(memory一覧[添字]);
    }
}
