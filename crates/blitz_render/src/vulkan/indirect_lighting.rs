//! 遠方環境から間接照明を答えるためのGPU資源一式。遠方環境の立方体画像と、そこから導く3つの派生表現
//! (拡散照度・鏡面畳込み・反射率積分表)をまとめて持つ。
//!
//! 2つの一式を1つの型にするのは、どちらか片方だけが在る状態が意味を持たないためである。派生表現は遠方環境の
//! 配列ビューを束縛して焼くため遠方環境なしには焼けず、遠方環境だけを焼いても標準PBRが読む表現は揃わない。
//! 別々の`Option`で持つと、その成立しない組を型の上で作れてしまう。
//!
//! 参照するスカイビューのベイク済み画像と媒体のシェーダー定数は大気のベイク済み画像一式が所有し、この型は
//! ビューを借りるだけである。
//! 参照: `_doc/設計/放射輝度問い合わせ階層.md`「世界の間接照明方針と契約の2枝(3-Ic)」

mod create;
mod draw_input;
mod injection;
mod inputs;

use ash::vk;

use crate::distant_environment::遠方環境の解析入力エラー;
use crate::distant_environment::{遠方環境のシェーダー一式, 遠方環境の解析入力};
use crate::error::レンダラーエラー;
use crate::indirect_lighting::{焼き始めの記録, 間接照明エラー};
use crate::vulkan::derived_environment::派生表現一式;
use crate::vulkan::descriptor::lighting_set::distant_environment::遠方環境の束縛先;
use crate::vulkan::distant_environment::{遠方環境が借りる束縛先, 遠方環境一式};
use crate::vulkan::tracked_device::GPUデバイス;

pub(crate) use injection::解析入力の注入;
pub(crate) use inputs::{登録する画像, 間接照明の描画入力, 間接照明の焼く組};

pub(crate) struct 遠方環境の照明資源 {
    遠方環境: 遠方環境一式,
    派生表現: 派生表現一式,
    焼き始め: 焼き始めの記録,
    /// 検収が解析入力を注入した実行だけが`Some`。本番のフレーム経路はこの入口を1度も呼ばない。
    /// 注入がある実行は焼き上げの計画が「何も焼かない」に固定され、3つの画像が毎フレーム転送で埋まる。
    注入: Option<injection::解析入力の注入資源>,
}

impl 遠方環境の照明資源 {
    pub(crate) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        借りる束縛先: 遠方環境が借りる束縛先<'_>,
        シェーダー: &遠方環境のシェーダー一式,
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(device, メモリプロパティ, 借りる束縛先, シェーダー)
    }

    /// 標準PBRが読む3つの画像のビュー。焼き直しても束縛先が変わらないため、照明問い合わせのセットは
    /// これを生成時に一度だけ結ぶ。
    pub(crate) fn 照明問い合わせへの束縛先(&self) -> 遠方環境の束縛先 {
        遠方環境の束縛先 {
            拡散照度: self.派生表現.拡散照度画像().立方体ビュー,
            鏡面畳込み: self.派生表現.鏡面畳込み画像().立方体ビュー,
            反射率積分表: self.派生表現.反射率積分表の画像().ビュー,
        }
    }

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
        self.注入 = Some(injection::解析入力の注入資源::生成する(
            device,
            メモリプロパティ,
            &self.派生表現,
            入力,
        )?);
        Ok(())
    }

    /// 前提: レンダラー全体の破棄順は renderer/destroy.rs が持ち、この一式は`描画段階資源`の1段として呼ばれる(GPU待機済み)。
    /// 派生表現を先に片付けるのは、派生表現のディスクリプタが遠方環境の配列ビューを結んでいるためである。
    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        if let Some(注入) = &self.注入 {
            注入.破棄する(device);
        }
        self.派生表現.破棄する(device);
        self.遠方環境.破棄する(device);
    }
}
