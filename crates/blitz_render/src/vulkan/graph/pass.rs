//! パス宣言: 読み/書きするリソースと種別、実際に発行するコマンドを記録するクロージャ。
//! 参照: `_doc/設計/レンダーグラフ.md`「パス」。

use super::clear_spec::クリア指定;
use super::context::記録文脈;
use super::handle::{バッファハンドル, 画像ハンドル};
use super::usage::{バッファ用途, 画像用途};

/// グラフィックス/コンピュート/転送のパス種別。begin/end renderingの要否と
/// アタッチメント構成をここから実行器が導く（クロージャはバインド+ドローだけを書く）。
pub(crate) enum パス種別 {
    グラフィックス { カラー: 画像ハンドル, 深度: Option<画像ハンドル>, クリア指定: クリア指定 },
    コンピュート,
    転送,
}

pub(crate) struct パス宣言<'a> {
    pub(crate) 名前: &'static str,
    pub(crate) 読み画像: Vec<(画像ハンドル, 画像用途)>,
    pub(crate) 書き画像: Vec<(画像ハンドル, 画像用途)>,
    pub(crate) 読みバッファ: Vec<(バッファハンドル, バッファ用途)>,
    pub(crate) 書きバッファ: Vec<(バッファハンドル, バッファ用途)>,
    pub(crate) 種別: パス種別,
    pub(crate) 記録: Box<dyn FnOnce(&記録文脈) + 'a>,
}

impl<'a> パス宣言<'a> {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn 生成する(
        名前: &'static str,
        読み画像: Vec<(画像ハンドル, 画像用途)>,
        書き画像: Vec<(画像ハンドル, 画像用途)>,
        読みバッファ: Vec<(バッファハンドル, バッファ用途)>,
        書きバッファ: Vec<(バッファハンドル, バッファ用途)>,
        種別: パス種別,
        記録: impl FnOnce(&記録文脈) + 'a,
    ) -> Self {
        Self { 名前, 読み画像, 書き画像, 読みバッファ, 書きバッファ, 種別, 記録: Box::new(記録) }
    }

    /// このパスが宣言している画像ハンドル(読み+書き)の一覧。記録文脈の解決許可リストに使う。
    pub(crate) fn 宣言済み画像一覧(&self) -> Vec<画像ハンドル> {
        self.読み画像
            .iter()
            .chain(self.書き画像.iter())
            .map(|&(ハンドル, _)| ハンドル)
            .collect()
    }

    /// このパスが宣言しているバッファハンドル(読み+書き)の一覧。記録文脈の解決許可リストに使う。
    pub(crate) fn 宣言済みバッファ一覧(&self) -> Vec<バッファハンドル> {
        self.読みバッファ
            .iter()
            .chain(self.書きバッファ.iter())
            .map(|&(ハンドル, _)| ハンドル)
            .collect()
    }
}
