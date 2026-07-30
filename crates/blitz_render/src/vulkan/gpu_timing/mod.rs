//! パス別GPUタイムスタンプ計測(判断30)。グラフのパス境界にvkCmdWriteTimestamp2を
//! 発行し、フレームのフェンス待ち後に前回分を読んで移動平均(60フレーム窓)を保つ。
//! 参照: `_doc/設計/レンダーグラフ.md`「GPU計測」。

mod moving_average;
mod query_pool;
mod readback;

use std::collections::HashMap;

use ash::vk;

use moving_average::移動平均;

use crate::error::レンダラーエラー;
use crate::vulkan::sync::{フレームインフライト数, フレームスロット添字};

/// グラフが1フレームに持てるパス数の上限(クエリプール容量の元になる)。
/// M7のブルームピラミッド(判断41)で8→16、M9の布シミュレーション(判断54: 約8パス追加)で16→32、
/// OW5第5段のカスケードシャドウ(シャドウパスが1本から4本へ)で32→40、
/// OW5第7b段の大気のベイク済み画像生成(透過率・多重散乱の2本追加)で40→48へ拡張。
pub(crate) const パス数上限: u32 = 48;
const クエリ数上限: u32 = パス数上限 * 2;

pub(crate) struct パス別GPU計測 {
    プール一覧: [vk::QueryPool; フレームインフライト数],
    タイムスタンプ周期ns: f32,
    直近マッピング一覧: [Vec<(&'static str, u32)>; フレームインフライト数],
    移動平均表: HashMap<&'static str, 移動平均>,
}

impl パス別GPU計測 {
    /// `タイムスタンプ対応か`が`false`(timestamp_valid_bits == 0)の物理デバイスでは
    /// `None`を返す(判断30: 計測無効は型で表し、無言の0ミリ秒を返さない)。
    pub(crate) fn 生成する(
        device: &ash::Device,
        タイムスタンプ対応か: bool,
        タイムスタンプ周期ns: f32,
    ) -> Result<Option<Self>, レンダラーエラー> {
        if !タイムスタンプ対応か {
            return Ok(None);
        }
        let プール一覧 = query_pool::生成する(device)?;
        Ok(Some(Self {
            プール一覧,
            タイムスタンプ周期ns,
            直近マッピング一覧: std::array::from_fn(|_| Vec::new()),
            移動平均表: HashMap::new(),
        }))
    }

    pub(crate) fn クエリプール(&self, フレーム添字: フレームスロット添字) -> vk::QueryPool {
        self.プール一覧[フレーム添字.配列添字()]
    }

    /// このスロットの前回分(2周前のフレーム)のクエリ結果を読み、移動平均を更新する。
    /// 呼び出し元はこのスロットのフェンス待ち直後(=前回分のGPU完了が確定した直後)に
    /// 呼ぶこと。
    pub(crate) fn 読み取る(&mut self, device: &ash::Device, フレーム添字: フレームスロット添字) {
        readback::読み取る(
            device,
            self.プール一覧[フレーム添字.配列添字()],
            &self.直近マッピング一覧[フレーム添字.配列添字()],
            self.タイムスタンプ周期ns,
            &mut self.移動平均表,
        );
    }

    /// 今回このスロットへ書いたパス名→クエリ開始添字のマッピングを記録する
    /// (次にこのスロットが巡ってきたとき`読み取る`が使う)。
    pub(crate) fn 直近マッピングを記録する(
        &mut self, フレーム添字: フレームスロット添字, マッピング: Vec<(&'static str, u32)>
    ) {
        self.直近マッピング一覧[フレーム添字.配列添字()] = マッピング;
    }

    /// パス名ごとの移動平均ミリ秒を全件返す(登録順は不定)。
    pub(crate) fn 平均一覧を取得する(&self) -> Vec<(&'static str, f64)> {
        self.移動平均表.iter().map(|(&名前, 平均)| (名前, 平均.平均())).collect()
    }

    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: 各プールはSelfが唯一の所有者であり、破棄時点でGPU側の使用が
        // device_wait_idle済みであることを呼び出し元が保証する。
        for &pool in &self.プール一覧 {
            unsafe { device.destroy_query_pool(pool, None) };
        }
    }
}
