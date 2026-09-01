//! 移動とカメラの計器: ゲーム配線が毎描画作る開発用の観測値。開発用UIはこれを描くだけであり、この型は開発用UIの型を名指ししない。
//! この便が持つのはカメラの分だけであり、移動の分(移動状態・水平の速さ・接地面の角・問い合わせ件数・完全性で動かなかった刻みの数)は
//! 第2段階でこの型へ欄を足す。
//! 参照: `_doc/設計/キャラクターの移動とカメラ.md`「判断13」

use blitz_game::遮蔽の判定;
use blitz_math::メートル;

use super::camera_system::カメラの系統;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct 移動とカメラの計器 {
    pub(crate) カメラ: カメラの計器,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct カメラの計器 {
    pub(crate) 系統: カメラの系統,
    pub(crate) 直前の描画: 直前の描画のカメラ,
    pub(crate) 完全性で保った描画の数: u32, // 掃引の答えが評価できない領域を含み、前の表示距離を保った描画の累計
}

/// 直前の描画で決めたカメラの値。まだ1度も決めていない描画をそのまま枝で持つ。
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum 直前の描画のカメラ {
    まだ描いていない,
    描いた {
        理想距離: メートル,
        表示距離: メートル,
        判定: 遮蔽の判定,
    },
}
