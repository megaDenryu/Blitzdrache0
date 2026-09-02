//! 剛体のGPUレイアウトの契約(判断22)。剛体の状態を4つのバッファ(運動状態・前状態・質量特性・実行状態)に分け、毎刻み書き換える量と原則不変の量を混ぜない。
//! 点自由度の32バイトの契約は拡張せず、姿勢自由度の別の密な配列を立てる。3成分の量は16バイト境界へ置く(構造化バッファの読み出しが16バイト単位で最も速い)。
//! バイト列化と、読み手の検証(長さ・姿勢の長さ・逆質量の非負と有限・符号の範囲を型付きエラーで拒む)をここが持ち、GPUの実行(`blitz_render`)はIssue #43が足す。
//! 1件のバイト数は`cargo xtask conform`の定数の組で検査の写しと結ぶ。
//!   剛体運動状態    重心位置 xyz + 詰め物1語・姿勢 xyzw・並進速度 xyz + 詰め物1語・角速度 xyz + 詰め物1語(64バイト。実体52バイト)
//!   剛体前状態      前の重心位置 xyz + 詰め物1語・前の姿勢 xyzw(32バイト。実体28バイト)
//!   剛体質量特性    逆質量・逆主慣性 xyz・主軸 xyzw(32バイト。詰め物なし)
//!   剛体実行状態    運動種別 u32・起きているか u32 + 詰め物2語(16バイト)
//! 参照: `_doc/設計/剛体の状態と接触.md`「判断22」。

mod error;
mod execution_state_bytes;
mod mass_properties_bytes;
mod motion_state_bytes;
mod previous_state_bytes;
#[cfg(test)]
mod rigid_tests;
mod word_reader;

pub use error::剛体レイアウトエラー;
pub use execution_state_bytes::{
    剛体実行状態の読み取り, 剛体実行状態をバイト列から読む, 剛体実行状態バイト列にする, 運動種別の符号
};
pub use mass_properties_bytes::{
    剛体質量特性の読み取り, 剛体質量特性をバイト列から読む, 剛体質量特性バイト列にする
};
pub use motion_state_bytes::{
    剛体運動状態の読み取り, 剛体運動状態をバイト列から読む, 剛体運動状態バイト列にする
};
pub use previous_state_bytes::{剛体前状態をバイト列から読む, 剛体前状態バイト列にする};

pub const 剛体運動状態1件のバイト数: usize = 64;
pub const 剛体前状態1件のバイト数: usize = 32;
pub const 剛体質量特性1件のバイト数: usize = 32;
pub const 剛体実行状態1件のバイト数: usize = 16;

/// 詰め物の1語。値に意味は無く、読み手は読まない。
const 詰め物: f32 = 0.0;

fn 単精度3つと詰め物を書く(バイト列: &mut Vec<u8>, 成分: [f32; 3]) {
    for 値 in 成分 {
        バイト列.extend_from_slice(&値.to_le_bytes());
    }
    バイト列.extend_from_slice(&詰め物.to_le_bytes());
}

fn 単精度4つを書く(バイト列: &mut Vec<u8>, 成分: [f32; 4]) {
    for 値 in 成分 {
        バイト列.extend_from_slice(&値.to_le_bytes());
    }
}
