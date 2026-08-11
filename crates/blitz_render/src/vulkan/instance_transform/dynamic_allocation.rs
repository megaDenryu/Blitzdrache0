//! 動く個体の変換バッファのフレームスロットぶんの確保と破棄の局面。呼ばれるのは束の読込と解除の1度だけであり、
//! 毎フレームの書き込みとは呼び出し頻度が違う。触れるのはこのバッファが所有するバッファとメモリだけである。

use ash::vk;

use super::content::個体変換内容;
use super::{bytes, 動く個体の変換バッファ};
use crate::error::レンダラーエラー;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::tracked_device::GPUデバイス;

impl 動く個体の変換バッファ {
    /// 注意: `動く個体添字一覧`の各添字が`内容一覧`の範囲内であることは`描画シーン素材`が束の読込より前に確かめている。
    /// 範囲外へ到達したらその検査が破れているためpanicで止める。
    pub(crate) fn 生成する(
        確保係: &GPU資源の確保係<'_>,
        内容一覧: &[個体変換内容],
        動く個体添字一覧: &[u32],
    ) -> Result<Self, レンダラーエラー> {
        let mut 初期列 = Vec::with_capacity(内容一覧.len() * bytes::バイト長);
        for 内容 in 内容一覧 {
            初期列.extend_from_slice(&bytes::バイト列にする(内容));
        }
        let 動く個体一覧 = 動く個体添字一覧.iter().map(|添字| (*添字, 読込時の内容を選ぶ(内容一覧, *添字))).collect();
        let スロットごとのバッファ = 確保係
            .フレームスロットごとのホスト可視バッファを確保して書き込む(&初期列, vk::BufferUsageFlags::STORAGE_BUFFER)?;
        let 範囲 = u64::try_from(初期列.len()).unwrap_or_else(|_| panic!("動く個体の変換バッファのバイト長がu64に収まらない"));
        Ok(Self {
            スロットごとのバッファ,
            範囲,
            動く個体一覧,
        })
    }

    /// 前提: 呼び出し元がGPU側の使用完了を保証する。
    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        self.スロットごとのバッファ.破棄する(device);
    }
}

fn 読込時の内容を選ぶ(内容一覧: &[個体変換内容], 添字: u32) -> 個体変換内容 {
    let 位置 = usize::try_from(添字).unwrap_or_else(|_| panic!("動く個体の添字{添字}がusizeに収まらない"));
    match 内容一覧.get(位置) {
        Some(内容) => *内容,
        None => panic!("動く個体の添字{添字}が個体変換列({}件)の外を指している", 内容一覧.len()),
    }
}
