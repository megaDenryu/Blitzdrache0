//! 検収が与える中身を受け付ける局面。呼ばれるのはレンダラー生成の直後の1回だけであり、以降のフレームは
//! 出来上がった注入の資源を参照するだけである。触れる状態は`注入`の1つに限る。
//!
//! 2つの入口を分けるのは、置き換える対象が違うためである。派生表現へ直に与える入口は生成側を止めて消費式だけを
//! 見せ、遠方環境へ与える入口は生成側を働かせて生成から消費までの連なりを見せる。
//! 2度目の注入を拒むのは、1度目のバッファを破棄せずに置き換えると以降の転送元がどちらの中身かを型から決められないためである。

use ash::vk;

use super::{injection, 遠方環境の照明資源};
use crate::atmosphere::遠方環境の解像度;
use crate::distant_environment::{遠方環境の焼かせる解析入力, 遠方環境の解析入力, 遠方環境の解析入力エラー};
use crate::error::レンダラーエラー;
use crate::indirect_lighting::間接照明エラー;
use crate::vulkan::tracked_device::GPUデバイス;

impl 遠方環境の照明資源 {
    /// 検収専用の入口。3つの派生表現の中身を解析入力で置き換え、以降のフレームで焼き上げを止める。
    /// 呼ばれるのはレンダラー生成の直後の1回だけであり、2度目の注入は前のバッファを取り違えないよう拒む。
    pub(crate) fn 解析入力を注入する(
        &mut self,
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        入力: &遠方環境の解析入力,
    ) -> Result<(), レンダラーエラー> {
        if self.注入.is_some() {
            return Err(間接照明エラー::from(遠方環境の解析入力エラー::二重注入).into());
        }
        let 資源 = injection::解析入力の注入資源::生成する(device, メモリプロパティ, &self.派生表現, 入力)?;
        self.注入 = Some(injection::検収の注入::派生表現へ直に与える(資源));
        Ok(())
    }

    /// 検収専用の入口。遠方環境の立方体画像と反射率積分表を与えた値で置き換え、拡散照度と鏡面畳込みは生成側に焼かせる。
    pub(crate) fn 焼かせる解析入力を注入する(
        &mut self,
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        入力: &遠方環境の焼かせる解析入力,
    ) -> Result<(), レンダラーエラー> {
        if self.注入.is_some() {
            return Err(間接照明エラー::from(遠方環境の解析入力エラー::二重注入).into());
        }
        let 解像度 = 遠方環境の解像度::既定値();
        let 資源 = injection::焼かせる注入資源::生成する(device, メモリプロパティ, &self.派生表現, 解像度, 入力)?;
        self.注入 = Some(injection::検収の注入::遠方環境へ与えて焼かせる(資源));
        Ok(())
    }
}
