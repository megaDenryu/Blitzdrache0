//! パス別GPU計測の生成の局面。呼ばれるのはレンダラー生成時の1回だけであり、以降のフレームは参照と追加しかしない。
//!
//! 生成を分けるのは、タイムスタンプ非対応のデバイスで計測そのものを作らない判断がここにしか無いためである。
//! 非対応を無言の0ミリ秒で表すと、測れなかったことと0だったことが値から区別できなくなる。

use std::collections::HashMap;

use super::composite_interval::合成区間の宣言;
use super::frame_samples::フレーム別の記録;
use super::query_pool;
use super::パス別GPU計測;
use crate::error::レンダラーエラー;

impl パス別GPU計測 {
    /// `タイムスタンプ対応か`が`false`(timestamp_valid_bits == 0)の物理デバイスでは
    /// `None`を返す(判断30: 計測無効は型で表し、無言の0ミリ秒を返さない)。
    pub(crate) fn 生成する(
        device: &ash::Device,
        タイムスタンプ対応か: bool,
        タイムスタンプ周期ns: f32,
        合成区間一覧: Vec<合成区間の宣言>,
    ) -> Result<Option<Self>, レンダラーエラー> {
        if !タイムスタンプ対応か {
            return Ok(None);
        }
        let プール一覧 = query_pool::生成する(device)?;
        Ok(Some(Self {
            プール一覧,
            タイムスタンプ周期ns,
            直近マッピング一覧: std::array::from_fn(|_| Vec::new()),
            合成区間一覧,
            窓表: HashMap::new(),
            フレーム別: フレーム別の記録::新規(),
        }))
    }
}
