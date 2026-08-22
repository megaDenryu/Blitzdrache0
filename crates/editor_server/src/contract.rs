//! ts-rsで、editor_serverの素データ型からTypeScriptの型契約の本文を組み立てる。
//! この関数の結果を、書き出す側(`bin/contract_export.rs`)と検査する側
//! (鮮度検査テスト)の両方が呼ぶ。ここでは文字列を作るだけで、ファイルへは触れない。
//! `TS::decl()`は単体のimport文を作らないため、互いに参照し合う型は同じ1本の
//! ファイルへまとめる(手本: GameScriptingTheoryの同名モジュール)。生存確認応答だけは
//! 何にも依存しないため、段1どおり別ファイルのまま残す。

use ts_rs::{Config, TS};

const 手書き禁止の注記: &str = "// このファイルはts-rsによる生成物である。手で編集しない。\n\
// 生成し直す手順（リポジトリルートで実行する）:\n\
//   cargo xtask contract-export\n";

pub fn 契約ファイルの本文を組み立てる() -> String {
    let 設定 = Config::new();
    let 型宣言一覧: [String; 1] = [<crate::生存確認応答 as TS>::decl(&設定)];
    本文を組み立てる(&型宣言一覧)
}

pub fn 編集資源契約の本文を組み立てる() -> String {
    let 設定 = Config::new();
    let 型宣言一覧: [String; 51] = [
        <crate::プロジェクト情報応答 as TS>::decl(&設定),
        <crate::建物定義の用途 as TS>::decl(&設定),
        <crate::ベイ構造 as TS>::decl(&設定),
        <crate::建物の入口方向 as TS>::decl(&設定),
        <crate::建物の外接箱 as TS>::decl(&設定),
        <crate::建物外形定義 as TS>::decl(&設定),
        <crate::建物外形カタログ as TS>::decl(&設定),
        <crate::壁の種類 as TS>::decl(&設定),
        <crate::壁の外面の飾り as TS>::decl(&設定),
        <crate::はめ口の値 as TS>::decl(&設定),
        <crate::升目の座標 as TS>::decl(&設定),
        <crate::升目の床 as TS>::decl(&設定),
        <crate::升目の屋根 as TS>::decl(&設定),
        <crate::升目の宣言 as TS>::decl(&設定),
        <crate::升目の複体 as TS>::decl(&設定),
        <crate::建物の格子の装飾 as TS>::decl(&設定),
        <crate::建物の格子 as TS>::decl(&設定),
        <crate::建物の格子の一覧項目 as TS>::decl(&設定),
        <crate::位置3次元 as TS>::decl(&設定),
        <crate::チャンク座標 as TS>::decl(&設定),
        <crate::世界の区画割り as TS>::decl(&設定),
        <crate::広域道路 as TS>::decl(&設定),
        <crate::チャンクの道路 as TS>::decl(&設定),
        <crate::建物の配置 as TS>::decl(&設定),
        <crate::散布の設定 as TS>::decl(&設定),
        <crate::散布の個体 as TS>::decl(&設定),
        <crate::マテリアル定義 as TS>::decl(&設定),
        <crate::層割当 as TS>::decl(&設定),
        <crate::マテリアル台帳 as TS>::decl(&設定),
        <crate::大域世界構造 as TS>::decl(&設定),
        <crate::チャンク構造 as TS>::decl(&設定),
        <crate::造成筆致種別 as TS>::decl(&設定),
        <crate::造成筆致 as TS>::decl(&設定),
        <crate::地表材質層 as TS>::decl(&設定),
        <crate::材質の筆致 as TS>::decl(&設定),
        <crate::道路対象 as TS>::decl(&設定),
        <crate::道路を追加する as TS>::decl(&設定),
        <crate::道路を削除する as TS>::decl(&設定),
        <crate::道路点を追加する as TS>::decl(&設定),
        <crate::道路点を挿入する as TS>::decl(&設定),
        <crate::道路点を移動する as TS>::decl(&設定),
        <crate::道路点を削除する as TS>::decl(&設定),
        <crate::建物を配置する as TS>::decl(&設定),
        <crate::建物を移動する as TS>::decl(&設定),
        <crate::建物を削除する as TS>::decl(&設定),
        <crate::散布設定を変更する as TS>::decl(&設定),
        <crate::道路に合わせて切土盛土する as TS>::decl(&設定),
        <crate::建物基礎を平坦化する as TS>::decl(&設定),
        <crate::急勾配を岩肌へベイクする as TS>::decl(&設定),
        <crate::道路下を泥へベイクする as TS>::decl(&設定),
        <crate::編集コマンド as TS>::decl(&設定),
    ];
    let mut 本文 = 本文を組み立てる(&型宣言一覧);
    本文.push_str(&format!(
        "export const 建物外形カタログ形式版 = {} as const;\n",
        crate::建物外形カタログの現在の形式版
    ));
    本文
}

fn 本文を組み立てる(型宣言一覧: &[String]) -> String {
    let mut 本文 = String::from(手書き禁止の注記);
    本文.push('\n');
    for 宣言 in 型宣言一覧 {
        本文.push_str("export ");
        本文.push_str(宣言);
        本文.push('\n');
    }
    本文
}
