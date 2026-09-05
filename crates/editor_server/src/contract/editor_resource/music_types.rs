//! 楽曲の編集資源(楽器・和音・コード進行・トラック・パターン・曲の節・楽曲)と、その操作コマンドの
//! 型宣言の並び。`編集資源契約.ts`へ出す宣言のうち、楽曲の道具が使う分をこのモジュールが所有する。

use ts_rs::{Config, TS};

pub(super) fn 楽曲の型宣言の並び(設定: &Config) -> [String; 39] {
    [
        <crate::既定のコード進行 as TS>::decl(設定),
        <crate::楽器 as TS>::decl(設定),
        <crate::打楽器の種類 as TS>::decl(設定),
        <crate::音の並び as TS>::decl(設定),
        <crate::トラックの種類 as TS>::decl(設定),
        <crate::コード進行参照 as TS>::decl(設定),
        <crate::トラック定義 as TS>::decl(設定),
        <crate::ミキサー設定 as TS>::decl(設定),
        <crate::和音の種類 as TS>::decl(設定),
        <crate::和音 as TS>::decl(設定),
        <crate::コード進行 as TS>::decl(設定),
        <crate::トラックの格子 as TS>::decl(設定),
        <crate::パターン as TS>::decl(設定),
        <crate::曲の節 as TS>::decl(設定),
        <crate::楽曲 as TS>::decl(設定),
        <crate::打ち込みの対象 as TS>::decl(設定),
        <crate::打点を置く as TS>::decl(設定),
        <crate::打点を消す as TS>::decl(設定),
        <crate::音を伸ばす as TS>::decl(設定),
        <crate::範囲の打点を消す as TS>::decl(設定),
        <crate::パターンの打点を全部消す as TS>::decl(設定),
        <crate::テンポを変える as TS>::decl(設定),
        <crate::ミキサー設定を変える as TS>::decl(設定),
        <crate::楽曲の表示名を変える as TS>::decl(設定),
        <crate::トラックの楽器を変える as TS>::decl(設定),
        <crate::トラックの音量を変える as TS>::decl(設定),
        <crate::トラックの進行の割り当てを変える as TS>::decl(設定),
        <crate::パターンを追加する as TS>::decl(設定),
        <crate::パターンを削除する as TS>::decl(設定),
        <crate::パターンの進行を変える as TS>::decl(設定),
        <crate::パターンの表示名を変える as TS>::decl(設定),
        <crate::独自の進行を保存する as TS>::decl(設定),
        <crate::独自の進行を削除する as TS>::decl(設定),
        <crate::曲の節を追加する as TS>::decl(設定),
        <crate::曲の節を挿入する as TS>::decl(設定),
        <crate::曲の節を変える as TS>::decl(設定),
        <crate::曲の節を削除する as TS>::decl(設定),
        <crate::曲の節を並べ替える as TS>::decl(設定),
        <crate::楽曲編集コマンド as TS>::decl(設定),
    ]
}
