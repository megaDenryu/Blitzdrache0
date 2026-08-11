//! 読み戻しの4つの受けバッファの所有者。担うのは、4つをそろって作りそろって破棄することと、
//! 記録へ渡すハンドルの組と読み取った結果を返すことである。反射率積分表だけを2成分で作るのは、
//! その画像だけが画素形式に2成分の半精度を採るためである。

use super::resources::受けバッファの口;
use crate::error::レンダラーエラー;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::atmosphere_lut::readback_buffer::ベイク済み画像の読み戻しバッファ;
use crate::vulkan::tracked_device::GPUデバイス;

/// 4つのテクセル数。
pub(super) struct 受けバッファのテクセル数 {
    pub(super) 遠方環境: usize,
    pub(super) 拡散照度: usize,
    pub(super) 鏡面畳込み: usize,
    pub(super) 反射率積分表: usize,
}

/// 不変条件: 4つがそろって存在するか1つも存在しないかのどちらかである。
pub(super) struct 受けバッファ四点 {
    遠方環境: ベイク済み画像の読み戻しバッファ,
    拡散照度: ベイク済み画像の読み戻しバッファ,
    鏡面畳込み: ベイク済み画像の読み戻しバッファ,
    反射率積分表: ベイク済み画像の読み戻しバッファ,
}

impl 受けバッファ四点 {
    pub(super) fn 生成する(
        確保係: &GPU資源の確保係<'_>, テクセル数: &受けバッファのテクセル数
    ) -> Result<Self, レンダラーエラー> {
        let device = 確保係.論理デバイス();
        let mut 作った = Vec::with_capacity(3);
        for 数 in [テクセル数.遠方環境, テクセル数.拡散照度, テクセル数.鏡面畳込み] {
            match ベイク済み画像の読み戻しバッファ::生成する(確保係, 数) {
                Ok(バッファ) => 作った.push(バッファ),
                Err(誤り) => return Err(作った途中を片付けて返す(device, &作った, 誤り)),
            }
        }
        let 表 = ベイク済み画像の読み戻しバッファ::二成分で生成する(確保係, テクセル数.反射率積分表);
        let 反射率積分表 = match 表 {
            Ok(バッファ) => バッファ,
            Err(誤り) => return Err(作った途中を片付けて返す(device, &作った, 誤り)),
        };
        let 鏡面畳込み = 作った.pop().unwrap_or_else(|| panic!("受けバッファの組み立てが3本を作れていない"));
        let 拡散照度 = 作った.pop().unwrap_or_else(|| panic!("受けバッファの組み立てが3本を作れていない"));
        let 遠方環境 = 作った.pop().unwrap_or_else(|| panic!("受けバッファの組み立てが3本を作れていない"));
        Ok(Self {
            遠方環境,
            拡散照度,
            鏡面畳込み,
            反射率積分表,
        })
    }

    pub(super) fn 口(&self) -> 受けバッファの口 {
        受けバッファの口 {
            遠方環境: self.遠方環境.handle(),
            拡散照度: self.拡散照度.handle(),
            鏡面畳込み: self.鏡面畳込み.handle(),
            反射率積分表: self.反射率積分表.handle(),
        }
    }

    pub(super) fn 読み取る(&self, device: &ash::Device) -> Result<読み取った四点, レンダラーエラー> {
        Ok(読み取った四点 {
            遠方環境: self.遠方環境.読み取る(device)?,
            拡散照度: self.拡散照度.読み取る(device)?,
            鏡面畳込み: self.鏡面畳込み.読み取る(device)?,
            反射率積分表: self.反射率積分表.二成分で読み取る(device)?,
        })
    }

    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        self.反射率積分表.破棄する(device);
        self.鏡面畳込み.破棄する(device);
        self.拡散照度.破棄する(device);
        self.遠方環境.破棄する(device);
    }
}

pub(super) struct 読み取った四点 {
    pub(super) 遠方環境: Vec<[f32; 4]>,
    pub(super) 拡散照度: Vec<[f32; 4]>,
    pub(super) 鏡面畳込み: Vec<[f32; 4]>,
    pub(super) 反射率積分表: Vec<[f32; 2]>,
}

fn 作った途中を片付けて返す(
    device: &GPUデバイス,
    一覧: &[ベイク済み画像の読み戻しバッファ],
    誤り: レンダラーエラー,
) -> レンダラーエラー {
    for バッファ in 一覧 {
        バッファ.破棄する(device);
    }
    誤り
}
