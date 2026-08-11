//! シェーダーの差し替えが成立したときにアプリが1回だけ出す行。
//! 検収はこの行を数えて「差し替えが1度も起きなかった実行」と区別するため、綴りが片側だけ動くと
//! 差し替え0回の実行を差し替え済みとして通すことになる。

use super::綴りの契約;

pub(super) const 綴り一覧: [綴りの契約; 1] = [綴りの契約 {
    綴り: "[hot-reload] 契約別のシェーダー束でシーンのパイプラインを差し替えた",
    現れるファイル一覧: &["crates/blitz_app/src/app/hot_reload_apply.rs", "xtask/src/shader_reload_draw/judgment.rs"],
}];
