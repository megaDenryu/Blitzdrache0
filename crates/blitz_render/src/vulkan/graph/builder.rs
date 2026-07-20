//! グラフ構築: 外部リソース登録・パスの追加・最終用途の宣言を集めるだけの器。
//! 実際の記録は`executor::実行する`が`分解する`で取り出した内容を使って行う。

use std::collections::HashMap;

use ash::vk;

use super::aspect::画像アスペクト;
use super::handle::画像ハンドル;
use super::pass::パス宣言;
use super::registry::画像レジストリ;
use super::state::画像状態;
use super::usage::画像用途;

pub(crate) struct グラフ<'a> {
    寸法: vk::Extent2D,
    レジストリ: 画像レジストリ,
    初期状態表: HashMap<画像ハンドル, 画像状態>,
    最終用途表: Vec<(画像ハンドル, 画像用途)>,
    パス列: Vec<パス宣言<'a>>,
}

impl<'a> グラフ<'a> {
    pub(crate) fn 新規(寸法: vk::Extent2D) -> Self {
        Self {
            寸法,
            レジストリ: 画像レジストリ::新規(),
            初期状態表: HashMap::new(),
            最終用途表: Vec::new(),
            パス列: Vec::new(),
        }
    }

    /// 外部持ち込み画像をグラフへ登録する。`初期状態`はこの画像が現在すでにある
    /// Vulkan同期状態（フレーム先頭のスワップチェーン画像ならUNDEFINED起点等）。
    pub(crate) fn 画像を登録する(
        &mut self,
        画像: vk::Image,
        画像ビュー: vk::ImageView,
        アスペクト: 画像アスペクト,
        初期状態: 画像状態,
    ) -> 画像ハンドル {
        let ハンドル = self.レジストリ.登録する(画像, 画像ビュー, アスペクト);
        self.初期状態表.insert(ハンドル, 初期状態);
        ハンドル
    }

    pub(crate) fn パスを積む(&mut self, パス: パス宣言<'a>) {
        self.パス列.push(パス);
    }

    /// グラフ終端でこのハンドルが到達すべき用途を宣言する（実行モデルの範囲での
    /// 「パスではない最終遷移」。参照: `_doc/設計/レンダーグラフ.md`「実行モデル」）。
    pub(crate) fn 最終用途を宣言する(&mut self, ハンドル: 画像ハンドル, 用途: 画像用途) {
        self.最終用途表.push((ハンドル, 用途));
    }

    #[allow(clippy::type_complexity)]
    pub(crate) fn 分解する(
        self,
    ) -> (
        vk::Extent2D,
        画像レジストリ,
        HashMap<画像ハンドル, 画像状態>,
        Vec<(画像ハンドル, 画像用途)>,
        Vec<パス宣言<'a>>,
    ) {
        (self.寸法, self.レジストリ, self.初期状態表, self.最終用途表, self.パス列)
    }
}
