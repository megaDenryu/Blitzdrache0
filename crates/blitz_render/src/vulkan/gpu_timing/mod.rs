//! パス別GPUタイムスタンプ計測(判断30)。グラフのパス境界にvkCmdWriteTimestamp2を
//! 発行し、フレームのフェンス待ち後に前回分を読んで移動平均(60フレーム窓)を保つ。
//! 生成の局面は`create`、窓へ入る前の生の値の記録は`frame_samples`にある。
//! `合成区間一覧`の中身はパス名を所有する層が決めて生成時に渡す。
//! `フレーム別`は既定では採らない状態であり、計測の起動指定だけが記録を始めさせる。
//! 参照: `_doc/設計/レンダーグラフ.md`「GPU計測」。

mod composite_interval;
mod create;
mod frame_samples;
mod pass_time_window;
mod query_pool;
mod readback;

pub(crate) use composite_interval::合成区間の宣言;

use std::collections::HashMap;

use ash::vk;

use frame_samples::フレーム別の記録;
use pass_time_window::パス時間の窓;

use crate::gpu_pass_timing::{パス時間の分布, フレーム別の標本};
use crate::vulkan::sync::{フレームスロット添字, 進行中フレーム数};

/// グラフが1フレームに持てるパス数の上限(クエリプール容量の元になる)。
/// M7の光のにじみピラミッド(判断41)で8→16、M9の布シミュレーション(判断54: 約8パス追加)で16→32、
/// OW5第5段の多段シャドウ(シャドウパスが1本から4本へ)で32→40、
/// OW5第7b段の大気のベイク済み画像生成(透過率・多重散乱の2本追加)で40→48へ拡張、
/// 着手順5第5段階の点光源の影(影を持てる灯4件×6面で最大24本追加)で48→72へ拡張。
/// 超えたときは`graph::timestamp_write`がクエリの開始添字を求める時点でpanicする。
pub(crate) const パス数上限: u32 = 72;
const クエリ数上限: u32 = パス数上限 * 2;

pub(crate) struct パス別GPU計測 {
    プール一覧: [vk::QueryPool; 進行中フレーム数],
    タイムスタンプ周期ns: f32,
    直近マッピング一覧: [Vec<(&'static str, u32)>; 進行中フレーム数],
    合成区間一覧: Vec<合成区間の宣言>, // 複数のパスの1フレームぶんの和を1つの区間として持つための宣言
    窓表: HashMap<&'static str, パス時間の窓>,
    フレーム別: フレーム別の記録, // 窓へ入る前の生の値の記録
}

impl パス別GPU計測 {
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
            &self.合成区間一覧,
            &mut self.窓表,
            &mut self.フレーム別,
        );
    }

    /// 今回このスロットへ書いたパス名→クエリ開始添字のマッピングを記録する
    /// (次にこのスロットが巡ってきたとき`読み取る`が使う)。
    pub(crate) fn 直近マッピングを記録する(
        &mut self, フレーム添字: フレームスロット添字, マッピング: Vec<(&'static str, u32)>
    ) {
        self.直近マッピング一覧[フレーム添字.配列添字()] = マッピング;
    }

    /// 窓へ入る前の生の値の記録を始める。始めるまでは1要素も積まない。
    pub(crate) fn フレーム別の記録を始める(&mut self) {
        self.フレーム別.始める();
    }

    /// 記録したフレーム別の生標本。記録を始めていなければ空である。
    pub(crate) fn フレーム別の標本一覧(&self) -> &[フレーム別の標本] {
        self.フレーム別.標本一覧()
    }

    /// パス名ごとの直近の窓の分布を全件返す(登録順は不定)。
    pub(crate) fn 分布一覧を取得する(&self) -> Vec<(&'static str, パス時間の分布)> {
        self.窓表.iter().map(|(&名前, 窓)| (名前, 窓.分布())).collect()
    }

    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: 各プールはSelfが唯一の所有者であり、破棄時点でGPU側の使用が
        // device_wait_idle済みであることを呼び出し元が保証する。
        for &pool in &self.プール一覧 {
            unsafe { device.destroy_query_pool(pool, None) };
        }
    }
}
