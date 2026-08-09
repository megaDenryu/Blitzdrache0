//! 動く個体の変換バッファのフレームスロットぶんの確保と破棄の局面。呼ばれるのは束の読込と解除の1度だけであり、
//! 毎フレームの書き込みとは呼び出し頻度が違う。触れるのはこのバッファが所有するバッファとメモリだけである。

use ash::vk;

use super::content::個体変換内容;
use super::{bytes, 動く個体の変換バッファ};
use crate::error::レンダラーエラー;
use crate::vulkan::host_buffer;
use crate::vulkan::sync::進行中フレーム数;
use crate::vulkan::tracked_device::GPUデバイス;

impl 動く個体の変換バッファ {
    /// 注意: `動く個体添字一覧`の各添字が`内容一覧`の範囲内であることは`描画シーン素材`が束の読込より前に確かめている。
    /// 範囲外へ到達したらその検査が破れているためpanicで止める。
    pub(crate) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        内容一覧: &[個体変換内容],
        動く個体添字一覧: &[u32],
    ) -> Result<Self, レンダラーエラー> {
        let mut 初期列 = Vec::with_capacity(内容一覧.len() * bytes::バイト長);
        for 内容 in 内容一覧 {
            初期列.extend_from_slice(&bytes::バイト列にする(内容));
        }
        let 動く個体一覧 = 動く個体添字一覧.iter().map(|添字| (*添字, 読込時の内容を選ぶ(内容一覧, *添字))).collect();
        let (buffer一覧, memory一覧) = スロットぶん確保する(device, メモリプロパティ, &初期列)?;
        let 範囲 = u64::try_from(初期列.len()).unwrap_or_else(|_| panic!("動く個体の変換バッファのバイト長がu64に収まらない"));
        Ok(Self {
            buffer一覧,
            memory一覧,
            範囲,
            動く個体一覧,
        })
    }

    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        確保済みを解放する(device, &self.buffer一覧, &self.memory一覧, 進行中フレーム数);
    }
}

fn 読込時の内容を選ぶ(内容一覧: &[個体変換内容], 添字: u32) -> 個体変換内容 {
    let 位置 = usize::try_from(添字).unwrap_or_else(|_| panic!("動く個体の添字{添字}がusizeに収まらない"));
    match 内容一覧.get(位置) {
        Some(内容) => *内容,
        None => panic!("動く個体の添字{添字}が個体変換列({}件)の外を指している", 内容一覧.len()),
    }
}

fn スロットぶん確保する(
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    初期列: &[u8],
) -> Result<([vk::Buffer; 進行中フレーム数], [vk::DeviceMemory; 進行中フレーム数]), レンダラーエラー> {
    let mut buffer一覧 = [vk::Buffer::null(); 進行中フレーム数];
    let mut memory一覧 = [vk::DeviceMemory::null(); 進行中フレーム数];
    for 添字 in 0..進行中フレーム数 {
        match host_buffer::確保して書き込む(device, メモリプロパティ, 初期列, vk::BufferUsageFlags::STORAGE_BUFFER) {
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
    Ok((buffer一覧, memory一覧))
}

fn 確保済みを解放する(
    device: &GPUデバイス,
    buffer一覧: &[vk::Buffer; 進行中フレーム数],
    memory一覧: &[vk::DeviceMemory; 進行中フレーム数],
    件数: usize,
) {
    for 添字 in 0..件数 {
        // 安全性: bufferとmemoryは動く個体の変換バッファが所有し、呼び出し元がGPU使用完了を保証する。
        unsafe { device.destroy_buffer(buffer一覧[添字], None) };
        device.メモリを解放する(memory一覧[添字]);
    }
}
