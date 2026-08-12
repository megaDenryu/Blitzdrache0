//! 光のにじみ用ディスクリプタ: 単一読み(binding0のみ)と二読み(binding0+1)の2つの束縛の宣言と、
//! その宣言から作る2つのレイアウトである。段数に応じたプールとセット群の割り当ては`allocate`にある。
//! 分けるのは、レイアウトがウィンドウ寸法に依存せず1度だけ作られるのに対し、セット群がピラミッドの作り直しごとに
//! 取り直されるためであり、呼ばれる時点と回数が違う。

mod allocate;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::descriptor::{
    宣言から作ったセットレイアウト, 宣言から割り当てたセット, 宣言した束縛の並び, 束縛番号
};

const 標本器つき: vk::DescriptorType = vk::DescriptorType::COMBINED_IMAGE_SAMPLER;
const 画素段: vk::ShaderStageFlags = vk::ShaderStageFlags::FRAGMENT;

/// 読み元1枚のセットの宣言。前処理と縮小が使う。
pub(super) const 単一読みの宣言: 宣言した束縛の並び<1> = 宣言した束縛の並び::生成する([(束縛番号::生成する(0), 標本器つき, 画素段)]);

/// 読み元2枚のセットの宣言。拡大が1段小さい結果と同じ段の縮小結果を混ぜるために使う。
pub(super) const 二読みの宣言: 宣言した束縛の並び<2> =
    宣言した束縛の並び::生成する([(束縛番号::生成する(0), 標本器つき, 画素段), (束縛番号::生成する(1), 標本器つき, 画素段)]);

pub(in crate::vulkan::bloom) use allocate::生成する;

pub(crate) struct 光のにじみセット群 {
    pool: vk::DescriptorPool,
    pub(super) 前処理set: 宣言から割り当てたセット<1>,
    pub(super) 縮小set一覧: Vec<宣言から割り当てたセット<1>>,
    pub(super) 拡大set一覧: Vec<宣言から割り当てたセット<2>>,
}

impl 光のにじみセット群 {
    pub(super) fn 束ねる(
        pool: vk::DescriptorPool,
        前処理set: 宣言から割り当てたセット<1>,
        縮小set一覧: Vec<宣言から割り当てたセット<1>>,
        拡大set一覧: Vec<宣言から割り当てたセット<2>>,
    ) -> Self {
        Self {
            pool,
            前処理set,
            縮小set一覧,
            拡大set一覧,
        }
    }

    /// フレームの記録が束縛するセットのハンドル。前処理・縮小の並び・拡大の並びの順で返る。
    pub(crate) fn 束縛するセットのハンドル(&self) -> (vk::DescriptorSet, Vec<vk::DescriptorSet>, Vec<vk::DescriptorSet>) {
        (
            self.前処理set.セットのハンドル(),
            self.縮小set一覧.iter().map(宣言から割り当てたセット::セットのハンドル).collect(),
            self.拡大set一覧.iter().map(宣言から割り当てたセット::セットのハンドル).collect(),
        )
    }

    /// 前提: 呼び出し元はGPU側の使用完了を保証する。プールの破棄がセットの解放を暗黙に行う。
    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: プールはSelfが唯一の所有者である。
        unsafe { device.destroy_descriptor_pool(self.pool, None) };
    }
}

/// 単一読み(前処理・縮小用)と二読み(拡大用)のレイアウトを作る。失敗時は前者を片付ける。
pub(super) fn レイアウト2種を作る(
    device: &ash::Device,
) -> Result<(宣言から作ったセットレイアウト<1>, 宣言から作ったセットレイアウト<2>), レンダラーエラー> {
    let 単一読み = 単一読みの宣言.セットレイアウトを確保する(device)?;
    match 二読みの宣言.セットレイアウトを確保する(device) {
        Ok(二読み) => Ok((単一読み, 二読み)),
        Err(誤り) => {
            単一読み.破棄する(device);
            Err(誤り)
        }
    }
}
