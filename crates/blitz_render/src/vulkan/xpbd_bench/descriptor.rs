//! 計測用ディスクリプタ: b0=定数UBO、b1〜b8=ストレージ8本の統一レイアウトと、1枚だけのセット。
//! 束縛番号の正本はこのファイルの定数であり、`shaders/xpbd_step.slang`冒頭の表と`cargo xtask conform`が突き合わせる。
//! セットを1枚にするのは、刻みごとに送信して完了を待つため進行中のフレームが常に1つだからである。

use ash::vk;

use super::buffers::XPBD計測バッファ;
use crate::error::レンダラーエラー;
use crate::vulkan::descriptor::{宣言から作ったセットレイアウト, 宣言から割り当てたセット, 宣言した束縛の並び, 束縛番号, 結ぶ現物};

pub(crate) const 定数の束縛番号: 束縛番号 = 束縛番号::生成する(0);
pub(crate) const 点の束縛番号: 束縛番号 = 束縛番号::生成する(1);
pub(crate) const 前の位置の束縛番号: 束縛番号 = 束縛番号::生成する(2);
pub(crate) const 拘束の引数の束縛番号: 束縛番号 = 束縛番号::生成する(3);
pub(crate) const ラグランジュ乗数の束縛番号: 束縛番号 = 束縛番号::生成する(4);
pub(crate) const 補正の累積の束縛番号: 束縛番号 = 束縛番号::生成する(5);
pub(crate) const 補正の候補の束縛番号: 束縛番号 = 束縛番号::生成する(6);
pub(crate) const 隣接の区間の束縛番号: 束縛番号 = 束縛番号::生成する(7);
pub(crate) const 隣接の項目の束縛番号: 束縛番号 = 束縛番号::生成する(8);

const 束縛の本数: usize = 9;
const 計算段: vk::ShaderStageFlags = vk::ShaderStageFlags::COMPUTE;
const 記憶: vk::DescriptorType = vk::DescriptorType::STORAGE_BUFFER;

const 束縛の宣言: 宣言した束縛の並び<束縛の本数> = 宣言した束縛の並び::生成する([
    (定数の束縛番号, vk::DescriptorType::UNIFORM_BUFFER, 計算段),
    (点の束縛番号, 記憶, 計算段),
    (前の位置の束縛番号, 記憶, 計算段),
    (拘束の引数の束縛番号, 記憶, 計算段),
    (ラグランジュ乗数の束縛番号, 記憶, 計算段),
    (補正の累積の束縛番号, 記憶, 計算段),
    (補正の候補の束縛番号, 記憶, 計算段),
    (隣接の区間の束縛番号, 記憶, 計算段),
    (隣接の項目の束縛番号, 記憶, 計算段),
]);

pub(super) struct XPBD計測ディスクリプタ {
    レイアウト: 宣言から作ったセットレイアウト<束縛の本数>,
    プール: vk::DescriptorPool,
    セット: 宣言から割り当てたセット<束縛の本数>,
}

impl XPBD計測ディスクリプタ {
    pub(super) fn 生成する(device: &ash::Device, バッファ: &XPBD計測バッファ) -> Result<Self, レンダラーエラー> {
        let レイアウト = 束縛の宣言.セットレイアウトを確保する(device)?;
        let 内訳 = 束縛の宣言.プールの内訳(1);
        let プールの生成情報 = vk::DescriptorPoolCreateInfo::default().max_sets(1).pool_sizes(&内訳);
        // 安全性: deviceは生成済みで有効。失敗時はレイアウトを片付ける。
        let プール = match unsafe { device.create_descriptor_pool(&プールの生成情報, None) } {
            Ok(プール) => プール,
            Err(誤り) => {
                レイアウト.破棄する(device);
                return Err(誤り.into());
            }
        };
        let セット = match レイアウト.プールからセットを割り当てる(device, プール, 1).map(|mut 一覧| 一覧.pop()) {
            Ok(Some(セット)) => セット,
            Ok(None) => panic!("1枚要求したセットの割り当てが0枚で成功した(Vulkan実装の契約違反)"),
            Err(誤り) => {
                // 安全性: プールはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_descriptor_pool(プール, None) };
                レイアウト.破棄する(device);
                return Err(誤り);
            }
        };
        セット.書き込み先(device).並びの位置ごとに結ぶ([
            結ぶ現物::バッファ全体(バッファ.定数.バッファのハンドル()),
            結ぶ現物::バッファ全体(バッファ.点.バッファのハンドル()),
            結ぶ現物::バッファ全体(バッファ.前の位置.バッファのハンドル()),
            結ぶ現物::バッファ全体(バッファ.拘束の引数.バッファのハンドル()),
            結ぶ現物::バッファ全体(バッファ.ラグランジュ乗数.バッファのハンドル()),
            結ぶ現物::バッファ全体(バッファ.補正の累積.バッファのハンドル()),
            結ぶ現物::バッファ全体(バッファ.補正の候補.バッファのハンドル()),
            結ぶ現物::バッファ全体(バッファ.隣接の区間.バッファのハンドル()),
            結ぶ現物::バッファ全体(バッファ.隣接の項目.バッファのハンドル()),
        ]);
        Ok(Self { レイアウト, プール, セット })
    }

    pub(super) fn レイアウトのハンドル(&self) -> vk::DescriptorSetLayout {
        self.レイアウト.レイアウトのハンドル()
    }

    pub(super) fn セットのハンドル(&self) -> vk::DescriptorSet {
        self.セット.セットのハンドル()
    }

    pub(super) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: プールはこの構造体が唯一の所有者であり、その破棄がセットの解放を暗黙に行う。
        unsafe { device.destroy_descriptor_pool(self.プール, None) };
        self.レイアウト.破棄する(device);
    }
}
