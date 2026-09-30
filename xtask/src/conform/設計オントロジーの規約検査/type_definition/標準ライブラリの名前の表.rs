//! 標準ライブラリの名前の表。標準ライブラリのクレートの名前と、Rust 2024 の標準のプレリュードが修飾も取り込みも無しに見せるトレイトの名前を持ち、
//! あるモジュールが標準ライブラリのクレートの中か、ある名前がプレリュードのトレイトの名前かを答える。依存を持たない純粋な関数だけを置く。
//! 修飾の無い名前の出どころを求める工程(`標準ライブラリの名前.rs`)と、修飾したパスの定義の探索(`修飾したパスの定義の探索.rs`)の両方がこの表を読む。

use super::super::module_path::モジュールパス;

/// 標準ライブラリのクレートの名前。
pub(super) const 標準ライブラリのクレート一覧: [&str; 3] = ["std", "core", "alloc"];

/// Rust 2024 の標準のプレリュードが、修飾も取り込みも無しに名前を見せるトレイト。
const プレリュードのトレイト一覧: [&str; 31] = [
    "Copy",
    "Send",
    "Sized",
    "Sync",
    "Unpin",
    "Drop",
    "Fn",
    "FnMut",
    "FnOnce",
    "AsMut",
    "AsRef",
    "From",
    "Into",
    "TryFrom",
    "TryInto",
    "Clone",
    "Default",
    "Eq",
    "Ord",
    "PartialEq",
    "PartialOrd",
    "ToOwned",
    "ToString",
    "Iterator",
    "IntoIterator",
    "DoubleEndedIterator",
    "ExactSizeIterator",
    "Extend",
    "FromIterator",
    "Future",
    "IntoFuture",
];

// そのモジュールが標準ライブラリのクレートの中か。
pub(super) fn 標準ライブラリのモジュールか(モジュール: &モジュールパス) -> bool {
    標準ライブラリのクレート一覧.contains(&モジュール.クレート().表記())
}

// その名前が、標準のプレリュードが見せるトレイトの名前か。
pub(super) fn プレリュードのトレイトの名前か(名前: &str) -> bool {
    プレリュードのトレイト一覧.contains(&名前)
}
