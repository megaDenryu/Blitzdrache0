//! `編集資源契約.ts`の本文。編集資源と操作コマンドの型宣言の並びと、値として出す定数をこのモジュールが所有する。
//!
//! 形式版と既定のコード進行を値として出すのは、ブラウザが同じ定数の写しを持たなくて済むようにするためである。
//! 写しを持つと、片方だけが変わった食い違いが実行するまで出ない
//! (参照: `_doc/設計/楽曲エディター.md`「判断9」「判断10」)。

use ts_rs::{Config, TS};

pub fn 編集資源契約の本文を組み立てる() -> String {
    let 設定 = Config::new();
    let 型宣言一覧: [String; 66] = [
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
        <crate::既定のコード進行 as TS>::decl(&設定),
        <crate::楽器 as TS>::decl(&設定),
        <crate::打楽器の種類 as TS>::decl(&設定),
        <crate::音の並び as TS>::decl(&設定),
        <crate::トラックの種類 as TS>::decl(&設定),
        <crate::コード進行参照 as TS>::decl(&設定),
        <crate::トラック定義 as TS>::decl(&設定),
        <crate::ミキサー設定 as TS>::decl(&設定),
        <crate::和音の種類 as TS>::decl(&設定),
        <crate::和音 as TS>::decl(&設定),
        <crate::コード進行 as TS>::decl(&設定),
        <crate::トラックの格子 as TS>::decl(&設定),
        <crate::パターン as TS>::decl(&設定),
        <crate::曲の節 as TS>::decl(&設定),
        <crate::楽曲 as TS>::decl(&設定),
    ];
    let mut 本文 = super::本文を組み立てる(&型宣言一覧);
    本文.push_str(&format!(
        "export const 建物外形カタログ形式版 = {} as const;\n",
        crate::建物外形カタログの現在の形式版
    ));
    本文.push_str(&format!(
        "export const 建物の格子の形式版 = {} as const;\n",
        crate::建物の格子の現在の形式版
    ));
    本文.push_str(&format!("export const 楽曲の形式版 = {} as const;\n", crate::楽曲の現在の形式版));
    本文.push_str(&format!(
        "export const 既定のコード進行一覧: 既定のコード進行[] = {};\n",
        既定のコード進行一覧のjson()
    ));
    本文
}

/// 進行1件を1行へ畳むのは、生成物の差分が進行の単位で出るようにするためである。
fn 既定のコード進行一覧のjson() -> String {
    let 行一覧: Vec<String> = crate::既定のコード進行一覧().iter().map(進行1件のjson).collect();
    format!("[\n  {}\n]", 行一覧.join(",\n  "))
}

/// 直列化できない欄を1つも持たない型のため、失敗したら型契約の組み立ての不変条件が破れている。
fn 進行1件のjson(進行: &crate::既定のコード進行) -> String {
    serde_json::to_string(進行)
        .unwrap_or_else(|誤り| panic!("既定のコード進行を値として書き出せない。全ての欄がJSONへ写せるという不変条件に違反した: {誤り}"))
}
