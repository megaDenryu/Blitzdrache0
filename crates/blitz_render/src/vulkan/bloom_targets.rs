//! ブルームピラミッドの中間画像(判断41)。縮小チェーンは1/2解像度から1/2ずつ最大5段
//! (最小辺16px未満で打ち切り)、拡大チェーンは最終段を除く各段に1枚。
//! 形式・用途はHDR中間画像と同一のため`HDRターゲット`を再利用する。
//! スワップチェーン再構築と連動して作り直す。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::hdr_target::HDRターゲット;
use crate::vulkan::tracked_device::GPUデバイス;

pub(crate) const 段数上限: usize = 5;
const 最小辺PX: u32 = 16;

pub(crate) struct ブルームピラミッド {
    /// 縮小チェーン。[0]が1/2解像度で、以降1/2ずつ小さくなる。長さは1以上・段数上限以下。
    pub(crate) 縮小一覧: Vec<HDRターゲット>,
    /// 拡大チェーン。[i]は縮小一覧[i]と同解像度。長さは縮小一覧の長さ-1(1段のみなら空)。
    pub(crate) 拡大一覧: Vec<HDRターゲット>,
    /// 各段の寸法(縮小一覧と同じ添字)。
    pub(crate) 寸法一覧: Vec<vk::Extent2D>,
}

impl ブルームピラミッド {
    pub(crate) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        フル解像度: vk::Extent2D,
    ) -> Result<Self, レンダラーエラー> {
        let 寸法一覧 = 段の寸法列(フル解像度);
        let mut 一式 = Self {
            縮小一覧: Vec::new(),
            拡大一覧: Vec::new(),
            寸法一覧,
        };
        for 添字 in 0..一式.寸法一覧.len() {
            let 寸法 = 一式.寸法一覧[添字];
            match HDRターゲット::生成する(device, メモリプロパティ, 寸法) {
                Ok(画像) => 一式.縮小一覧.push(画像),
                Err(誤り) => {
                    一式.破棄する(device);
                    return Err(誤り);
                }
            }
            if 添字 + 1 < 一式.寸法一覧.len() {
                match HDRターゲット::生成する(device, メモリプロパティ, 寸法) {
                    Ok(画像) => 一式.拡大一覧.push(画像),
                    Err(誤り) => {
                        一式.破棄する(device);
                        return Err(誤り);
                    }
                }
            }
        }
        Ok(一式)
    }

    /// トーンマップが読む最終結果のビュー。2段以上なら拡大チェーンの最上段、1段のみなら縮小結果。
    pub(crate) fn 最終ビュー(&self) -> vk::ImageView {
        self.拡大一覧.first().unwrap_or_else(|| &self.縮小一覧[0]).画像ビュー
    }

    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        for 画像 in self.縮小一覧.iter().chain(self.拡大一覧.iter()) {
            画像.破棄する(device);
        }
    }
}

/// 1/2解像度から1/2ずつ、最小辺が16px未満になる手前まで(最大5段)の寸法列を返す。長さは常に1以上。
fn 段の寸法列(フル解像度: vk::Extent2D) -> Vec<vk::Extent2D> {
    let mut 一覧 = Vec::new();
    let mut 幅 = (フル解像度.width / 2).max(1);
    let mut 高さ = (フル解像度.height / 2).max(1);
    loop {
        一覧.push(vk::Extent2D { width: 幅, height: 高さ });
        if 一覧.len() >= 段数上限 || 幅 / 2 < 最小辺PX || 高さ / 2 < 最小辺PX {
            return 一覧;
        }
        幅 /= 2;
        高さ /= 2;
    }
}
