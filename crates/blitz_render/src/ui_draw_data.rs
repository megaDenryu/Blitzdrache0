//! 開発用UI1フレームぶんの描画データ(判断33)。`フレーム描画入力::UI描画`が
//! `None`ならUIパス自体をグラフへ積まない(既定オフ時に既存スモークの厳密判定を無傷に保つ)。

use crate::ui_mesh::UIメッシュ;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UI描画データ {
    pub メッシュ一覧: Vec<UIメッシュ>,
}
