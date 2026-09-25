//! シェーダーの差し替えが成立したときにアプリが1回だけ出す行。
//! 検収はこの行を数えて「差し替えが1度も起きなかった実行」と区別するため、文言が片側だけ動くと
//! 差し替え0回の実行を差し替え済みとして通すことになる。

use super::文言の契約;

pub(super) const 文言一覧: [文言の契約; 1] = [文言の契約 {
    文言: "[hot-reload] 契約別のシェーダー束でシーンのパイプラインを差し替えた",
    現れるファイル一覧: &["crates/blitz_app/src/app/resource_wiring/hot_reload_check.rs", "xtask/src/shader_reload_draw/judgment.rs"],
}];
