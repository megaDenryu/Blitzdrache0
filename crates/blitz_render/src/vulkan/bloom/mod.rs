//! 光のにじみピラミッドのパイプライン・ディスクリプタ一式(判断41)。
//! パイプライン3本(前処理・縮小・拡大)とサンプラー・レイアウトはウィンドウ寸法に依存せず永続し、
//! ディスクリプタ(プールとセット)は段数が解像度依存のためピラミッドの作り直しと連動して作り直す。
//! `セット群`はその作り直しのたびに丸ごと入れ替わるため、作り直しの途中で失敗したときに破棄済みのプールが残らないよう、
//! 有無を`Option`で表して破棄済みの状態を`None`で持つ。
//! 生成手順は`create`、ディスクリプタは`descriptor`、ビュー束縛は`rebind`にある。

mod create;
mod descriptor;
mod rebind;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::bloom_targets::光のにじみピラミッド;
use crate::vulkan::descriptor::宣言から作ったセットレイアウト;
use crate::vulkan::fullscreen_pipeline::全画面パスのパイプライン;

pub(crate) struct 光のにじみ一式 {
    pub(crate) 前処理: 全画面パスのパイプライン,
    pub(crate) 縮小: 全画面パスのパイプライン,
    pub(crate) 拡大: 全画面パスのパイプライン,
    sampler: vk::Sampler,
    単一読みlayout: 宣言から作ったセットレイアウト<1>,
    二読みlayout: 宣言から作ったセットレイアウト<2>,
    セット群: Option<descriptor::光のにじみセット群>, // ピラミッドを作り直すたびに丸ごと入れ替える
}

impl 光のにじみ一式 {
    pub(crate) fn 生成する(
        確保係: &GPU資源の確保係<'_>,
        前処理シェーダー: &シェーダー一式,
        縮小シェーダー: &シェーダー一式,
        拡大シェーダー: &シェーダー一式,
        hdrビュー: vk::ImageView,
        ピラミッド: &光のにじみピラミッド,
    ) -> Result<Self, レンダラーエラー> {
        let device = 確保係.論理デバイス();
        let mut 一式 = create::パイプライン部を生成する(確保係, 前処理シェーダー, 縮小シェーダー, 拡大シェーダー)?;
        if let Err(誤り) = 一式.ディスクリプタを作り直す(device, hdrビュー, ピラミッド) {
            一式.破棄する(device);
            return Err(誤り);
        }
        Ok(一式)
    }

    /// ピラミッドを作り直した後(生成直後・リサイズ後)に呼び、プールとセットを段数に合わせて
    /// 作り直してビューを束縛する。
    /// 前提: 呼び出し時点でGPUがこれらのディスクリプタセットを使用していないこと(device_wait_idle後)。
    pub(crate) fn ディスクリプタを作り直す(
        &mut self,
        device: &ash::Device,
        hdrビュー: vk::ImageView,
        ピラミッド: &光のにじみピラミッド,
    ) -> Result<(), レンダラーエラー> {
        // 注意: 旧いセット群を先に外してから破棄する。以降の生成が失敗して抜けたとき、破棄済みのプールが残っていると`破棄する`が二重破棄する。
        if let Some(旧い) = self.セット群.take() {
            旧い.破棄する(device);
        }
        self.セット群 = Some(descriptor::生成する(
            device,
            &self.単一読みlayout,
            &self.二読みlayout,
            ピラミッド.縮小一覧.len(),
        )?);
        self.ビューを書く(device, hdrビュー, ピラミッド);
        Ok(())
    }

    /// フレームの記録とビューの束縛へ渡す境界。
    /// 前提: 生成の直後にセット群が入っており、`None`は呼び出し規律の破れである。
    pub(crate) fn 確保済みのセット群(&self) -> &descriptor::光のにじみセット群 {
        self.セット群
            .as_ref()
            .unwrap_or_else(|| panic!("光のにじみのセット群が未確保のまま参照された"))
    }

    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: 各ハンドルはSelfが唯一の所有者であり、破棄時点でGPU側の使用がdevice_wait_idle済みであることを呼び出し元が保証する。
        for パイプライン in [&self.前処理, &self.縮小, &self.拡大] {
            パイプライン.破棄する(device);
        }
        if let Some(セット群) = &self.セット群 {
            セット群.破棄する(device);
        }
        // 安全性: samplerはSelfが唯一の所有者である。
        unsafe { device.destroy_sampler(self.sampler, None) };
        self.二読みlayout.破棄する(device);
        self.単一読みlayout.破棄する(device);
    }
}
