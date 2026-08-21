//! コマンド一覧の唯一の正本。`cargo xtask`の引数なし一覧表示とメニューの一覧表示は、
//! ともにこのモジュールが返す一覧だけを見る。分類ごとにファイルを分けているのは、各ファイルが
//! 担当する分類に名前が付き、コマンドを1件足すときに触るのがその分類のファイルだけで済むためである
//! (参照: CLAUDE.md「切り出しの根拠義務」の3号、触れるフィールドが限定された操作の分離)。

mod asset;
mod benchmark;
mod core;
mod editor;
mod entry;
mod material_check;
mod measurement;
mod play;
mod render_check;
mod sky_environment;

pub(crate) use entry::コマンド項目;

/// 全コマンドを分類の並び順で返す。分類の中の並び順は各分類ファイルの定義順のままである。
pub(crate) fn 全件() -> Vec<コマンド項目> {
    [
        core::一覧,
        asset::一覧,
        benchmark::一覧,
        measurement::一覧,
        play::一覧,
        render_check::一覧,
        material_check::一覧,
        sky_environment::一覧,
        editor::一覧,
    ]
    .into_iter()
    .flatten()
    .copied()
    .collect()
}

#[cfg(test)]
mod tests;
