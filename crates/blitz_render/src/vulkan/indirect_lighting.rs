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
mod generation_interval;
mod inject;
mod injection;
mod injection_material;
mod inputs;

use crate::distant_environment::遠方環境のシェーダー一式;
use crate::error::レンダラーエラー;
use crate::indirect_lighting::{焼き上げ本数の見込み, 焼き始めの記録};
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::derived_environment::派生表現一式;
use crate::vulkan::descriptor::lighting_set::distant_environment::遠方環境の束縛先;
use crate::vulkan::distant_environment::{遠方環境が借りる束縛先, 遠方環境一式};
use crate::vulkan::tracked_device::GPUデバイス;

pub(crate) use generation_interval::合成区間を宣言する;
pub(crate) use injection::{焼かせる注入, 解析入力の注入};
pub(crate) use inputs::{
    描画入力の材料, 登録する画像, 遠方環境用スカイビューの焼き上げ, 間接照明の描画入力, 間接照明の焼く組
};

pub(crate) struct 遠方環境の照明資源 {
    遠方環境: 遠方環境一式,
    派生表現: 派生表現一式,
    焼き始め: 焼き始めの記録,
    /// 検収が中身を注入した実行だけが`Some`。本番のフレーム経路は注入の入口を1度も呼ばない。
    /// どちらの枝を注入したかでそのフレームの焼き上げの計画が変わる(`inject`と`draw_input`)。
    注入: Option<injection::検収の注入>,
}

impl 遠方環境の照明資源 {
    pub(crate) fn 生成する(
        確保係: &GPU資源の確保係<'_>,
        借りる束縛先: 遠方環境が借りる束縛先<'_>,
        シェーダー: &遠方環境のシェーダー一式,
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(確保係, 借りる束縛先, シェーダー)
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

    /// この資源が1フレームに積む本数の見込み。焼き上げの計画と同じ粗さ段数から導くため、検収が読む期待値と
    /// レンダーグラフが積む本数が同じ1つの導出から出る。
    pub(crate) fn 焼き上げ本数の見込み(&self) -> 焼き上げ本数の見込み {
        焼き上げ本数の見込み::導く(self.派生表現.鏡面畳込みの解像度().粗さ段数())
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
