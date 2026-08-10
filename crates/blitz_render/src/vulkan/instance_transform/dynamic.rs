//! 動く個体を持つ描画対象の個体レコードを、フレームスロットごとのホスト可視バッファで持つ置き場。
//! 確保時に全個体の読込時の内容を各スロットへ書き、以後は動く個体1件ぶんの112バイトだけを毎フレーム書き換える。
//!
//! フレームスロットごとに別のバッファを持つのは、可視ID列と同じ理由である。1本を毎フレーム書き換えると、
//! 前のフレームの描画をGPUが読んでいる最中に上書きすることになる。
//! ホスト可視メモリに置くのは、書き換えるのが動く個体の数ぶんの112バイトだけであり、静的な個体変換のように
//! 大個体数の頂点シェーダーがホスト側のメモリを読み続ける形にならないためである(参照: `mod.rs`のメモリ配置の理由)。
//! 前提: 動く個体の数は束の中のごく一部である。全個体が動く形が要るようになったら、確保の仕方から設計し直す。
//! 確保と破棄の局面は`dynamic_allocation`にある。

use ash::vk;
use blitz_math::{ワールド, 位置};

use super::bytes;
use super::content::個体変換内容;
use super::reference::個体レコード参照;
use crate::error::{フレーム入力不一致エラー, レンダラーエラー};
use crate::frame_input::読込時の向きから天頂軸まわりに回す角;
use crate::vulkan::sync::{フレームスロット添字, 進行中フレーム数};

pub(crate) struct 動く個体の変換バッファ {
    pub(super) buffer一覧: [vk::Buffer; 進行中フレーム数],
    pub(super) memory一覧: [vk::DeviceMemory; 進行中フレーム数],
    pub(super) 範囲: vk::DeviceSize,
    /// 動く個体の添字と、その個体の読込時の内容。差し替えるのは平行移動と天頂軸まわりの向きだけであり、
    /// 寸法と読込時の姿勢はここが持ち続けて毎フレームの書き込みの土台になる。
    pub(super) 動く個体一覧: Vec<(u32, 個体変換内容)>,
}

impl 動く個体の変換バッファ {
    pub(crate) fn 参照(&self) -> 個体レコード参照 {
        個体レコード参照::スロット別バッファから生成する(self.buffer一覧, self.範囲)
    }

    /// そのフレームの動く個体1体の位置と向きを書き込む。宣言されていない個体を指す指定は無言で飛ばさず型付きエラーで拒む。
    pub(crate) fn 動く個体の位置と向きを書き込む(
        &self,
        device: &ash::Device,
        フレーム添字: フレームスロット添字,
        個体添字: u32,
        読込時の向きから回す角: 読込時の向きから天頂軸まわりに回す角,
        基準原点からの相対位置: 位置<ワールド>,
    ) -> Result<(), レンダラーエラー> {
        let Some((_, 読込時の内容)) = self.動く個体一覧.iter().find(|(添字, _)| *添字 == 個体添字) else {
            return Err(フレーム入力不一致エラー::動く個体として宣言されていない個体への位置指定 { 個体添字 }.into());
        };
        let 書き込む内容 = 読込時の内容
            .天頂軸まわりに回して平行移動成分を置き換えた内容を返す(読込時の向きから回す角, 基準原点からの相対位置);
        let レコード = bytes::バイト列にする(&書き込む内容);
        self.個体レコードを写す(device, self.memory一覧[フレーム添字.配列添字()], 個体添字, &レコード)
    }

    /// 個体レコード1件をその個体の位置へ写す。列の先頭からの位置は個体添字とレコード長の積で決まる。
    fn 個体レコードを写す(
        &self,
        device: &ash::Device,
        memory: vk::DeviceMemory,
        個体添字: u32,
        レコード: &[u8; bytes::バイト長],
    ) -> Result<(), レンダラーエラー> {
        let 位置 = usize::try_from(個体添字)
            .ok()
            .and_then(|添字| 添字.checked_mul(bytes::バイト長))
            .unwrap_or_else(|| panic!("動く個体の書き込み位置がusizeを超えた: 個体添字{個体添字}"));
        let 全長 = usize::try_from(self.範囲).unwrap_or_else(|_| panic!("動く個体の変換バッファのバイト長がusizeに収まらない"));
        // 安全性: memoryはHOST_VISIBLE|HOST_COHERENTで確保済みであり、確保時に書いた全個体ぶんの長さが`範囲`である。
        let ポインタ = unsafe { device.map_memory(memory, 0, vk::WHOLE_SIZE, vk::MemoryMapFlags::empty())? };
        // 安全性: ポインタはmap_memoryが返した確保容量全体の先頭であり、`全長`はその容量と等しい。
        let 書込先 = unsafe { std::slice::from_raw_parts_mut(ポインタ.cast::<u8>(), 全長) };
        書込先[位置..位置 + bytes::バイト長].copy_from_slice(レコード);
        // 安全性: memoryはこの直前にmap_memory済みの同一ハンドル。
        unsafe { device.unmap_memory(memory) };
        Ok(())
    }
}
