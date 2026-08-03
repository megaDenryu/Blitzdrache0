//! 生成側に焼かせる検収が与える2枚ぶんのホスト可視バッファ。触れるのは遠方環境の立方体画像と反射率積分表の
//! 転送元だけであり、拡散照度と鏡面畳込みには触れない(その2枚は生成側が焼く)。
//!
//! 反射率積分表を一緒に持つのは、その表が遠方環境に依らないためである。焼いた表を使うと、期待値の側が
//! 表の標本の補間まで再現しなければならず、確かめたい連なりに関係のない誤差が混ざる。
//!
//! 生成は注入の入口が呼ばれた1回だけであり、以降のフレームは同じバッファを転送元に取る。

use ash::vk;

use super::bytes;
use super::upload_buffer::注入元バッファ;
use crate::atmosphere::遠方環境の解像度;
use crate::distant_environment::遠方環境の焼かせる解析入力;
use crate::error::レンダラーエラー;
use crate::vulkan::derived_environment::派生表現一式;
use crate::vulkan::tracked_device::GPUデバイス;

/// 転送が使う元バッファ。1フレームの積み上げが名前で受け取る。
#[derive(Clone, Copy)]
pub(crate) struct 焼かせる注入 {
    pub(crate) 遠方環境の元: vk::Buffer,
    pub(crate) 反射率積分表の元: vk::Buffer,
    pub(crate) 反射率積分表の範囲: vk::Extent3D,
}

pub(crate) struct 焼かせる注入資源 {
    遠方環境: 注入元バッファ,
    反射率積分表: 注入元バッファ,
}

impl 焼かせる注入資源 {
    pub(crate) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        派生表現: &派生表現一式,
        解像度: 遠方環境の解像度,
        入力: &遠方環境の焼かせる解析入力,
    ) -> Result<Self, レンダラーエラー> {
        let 遠方環境 = 注入元バッファ::生成する(device, メモリプロパティ, &bytes::遠方環境のバイト列(入力, 解像度))?;
        let 表のバイト列 = bytes::反射率積分表のバイト列(入力.反射率積分表(), 派生表現.反射率積分表の解像度());
        match 注入元バッファ::生成する(device, メモリプロパティ, &表のバイト列) {
            Ok(反射率積分表) => Ok(Self {
                遠方環境, 反射率積分表
            }),
            Err(誤り) => {
                遠方環境.破棄する(device);
                Err(誤り)
            }
        }
    }

    pub(crate) fn 転送の材料を作る(&self, 派生表現: &派生表現一式) -> 焼かせる注入 {
        焼かせる注入 {
            遠方環境の元: self.遠方環境.handle,
            反射率積分表の元: self.反射率積分表.handle,
            反射率積分表の範囲: 派生表現.反射率積分表の画像().範囲(),
        }
    }

    /// 前提: レンダラー全体の破棄順は renderer/destroy.rs が持ち、この資源は遠方環境の照明資源の1段として呼ばれる(GPU待機済み)。
    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        self.反射率積分表.破棄する(device);
        self.遠方環境.破棄する(device);
    }
}
