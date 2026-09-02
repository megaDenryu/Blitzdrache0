//! 計測の刻みごとのGPU時間。提示のフレームと同じパス別GPU計測(`vulkan::gpu_timing`)を、刻み1本を1フレームとして立てる。
//! タイムスタンプ非対応の機材では計器を作らず、読み戻しの標本が空になることで「測れなかった」が表れる。
//! クエリプールは刻みごとの積み始めにリセットする(提示側の`frame::session`と同じ規律)。

use ash::vk;

use super::pass_names;
use crate::error::レンダラーエラー;
use crate::gpu_pass_timing::{パス時間の分布, フレーム別の標本};
use crate::vulkan::command_sink::GPU命令の積み先;
use crate::vulkan::gpu_timing::{パス別GPU計測, パス数上限};
use crate::vulkan::headless::ウィンドウなし実行GPU環境;
use crate::vulkan::sync::フレームスロット添字;
use crate::xpbd_solver_bench_probe::XPBD並列方式;

pub(super) struct 刻みごとのGPU計測 {
    計測: Option<パス別GPU計測>,
}

impl 刻みごとのGPU計測 {
    pub(super) fn 生成する(環境: &ウィンドウなし実行GPU環境, 方式: XPBD並列方式) -> Result<Self, レンダラーエラー> {
        let (対応か, 周期ns) = 環境.タイムスタンプ計測条件を調べる();
        let mut 計測 = パス別GPU計測::生成する(環境.device(), 対応か, 周期ns, vec![pass_names::合成区間を宣言する(方式)])?;
        if let Some(計測) = 計測.as_mut() {
            計測.フレーム別の記録を始める();
        }
        Ok(Self { 計測 })
    }

    /// この刻みのコマンドバッファへクエリプールのリセットを積み、グラフの実行器へ渡すプールを返す。
    pub(super) fn 積み始める(&self, 積み先: GPU命令の積み先<'_>) -> Option<vk::QueryPool> {
        let プール = self.計測.as_ref()?.クエリプール(フレームスロット添字::先頭());
        // 安全性: command_bufferは積み込み開始済みで、この刻みで書くクエリより前にリセットする。
        unsafe {
            積み先
                .論理デバイス()
                .cmd_reset_query_pool(積み先.コマンドバッファ(), プール, 0, パス数上限 * 2)
        };
        Some(プール)
    }

    /// 送信の完了後に呼び、この刻みのクエリを読んで標本にする。
    pub(super) fn 刻みを読み取る(&mut self, device: &ash::Device, マッピング: Vec<(&'static str, u32)>) {
        if let Some(計測) = self.計測.as_mut() {
            計測.直近マッピングを記録する(フレームスロット添字::先頭(), マッピング);
            計測.読み取る(device, フレームスロット添字::先頭());
        }
    }

    pub(super) fn 刻み別の標本一覧(&self) -> Vec<フレーム別の標本> {
        self.計測.as_ref().map(|計測| 計測.フレーム別の標本一覧().to_vec()).unwrap_or_default()
    }

    pub(super) fn 分布一覧(&self) -> Vec<(&'static str, パス時間の分布)> {
        self.計測.as_ref().map(パス別GPU計測::分布一覧を取得する).unwrap_or_default()
    }

    pub(super) fn 破棄する(&self, device: &ash::Device) {
        if let Some(計測) = &self.計測 {
            計測.破棄する(device);
        }
    }
}
