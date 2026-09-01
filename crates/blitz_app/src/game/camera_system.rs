//! カメラの系統: プレイヤー用とデバッグ用の2枝。鍵1つで交互に切り替える。
//! 参照: `_doc/設計/キャラクターの移動とカメラ.md`「判断9」

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum カメラの系統 {
    プレイヤー用, // キツネを注視し、遮蔽の回避と復帰を持つ。通常の遊びが使う
    デバッグ用,   // 現行のオービットカメラそのものであり、めり込みを許す。作業と検収の確認が使う
}

impl カメラの系統 {
    pub(super) fn 切り替えた系統(self) -> Self {
        match self {
            Self::プレイヤー用 => Self::デバッグ用,
            Self::デバッグ用 => Self::プレイヤー用,
        }
    }

    pub(crate) fn 表示名(self) -> &'static str {
        match self {
            Self::プレイヤー用 => "プレイヤー用(遮蔽を避ける)",
            Self::デバッグ用 => "デバッグ用(めり込みを許す)",
        }
    }
}
