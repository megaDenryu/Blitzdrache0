//! エディター関連コマンドの一覧。担当するのは、ブラウザ版ゲーム開発用エディターの
//! 起動と、Rust側の型契約をTypeScriptへ書き出す2件である。
//! 参照: `_doc/計画/ゲーム開発用エディター計画.md`

use super::entry::コマンド項目;

pub(super) const 一覧: &[コマンド項目] = &[
    コマンド項目 {
        日本語名: "エディターサーバーの起動",
        説明文: "  editor           編集サーバー(editor_server)とeditor_webの開発サーバーを併せて起動する(editor_web/node_modules未導入時はサーバーのみ起動する)",
    },
    コマンド項目 {
        日本語名: "型契約の書き出し",
        説明文: "  contract-export  editor_serverのRust側の型からeditor_web/src/生成配下のTypeScript契約ファイルを生成し直す",
    },
];
