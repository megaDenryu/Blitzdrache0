//! `cargo xtask gen-pending-short-definitions` の入口。裁定待ちの語の一覧(`_doc/計画/pending_ubiquitous_language.md`)から、
//! 語・区切られたドメイン・定義文の最初の文だけを並べた短い版(`_doc/計画/裁定待ちの語の短い定義.md`)を生成する。
//! 元の一覧は1件ごとに使っている場所や理由を持ち、全部を読み込むと記憶の小さいサブエージェントの文脈があふれるため、短い版を別に持つ。
//! 元の一覧は箇条書きの項目と、列の違う表が混ざる。読めない行を1件でも見つけたら、短い版を書かずに失敗で終わる。
//! 短い版が古くなっていないかは `cargo xtask conform` が確かめる(`xtask/src/conform/裁定待ちの語の短い版の古さの検査.rs`)。

#[path = "裁定待ちの語の短い版/ファイルの置き場.rs"]
mod ファイルの置き場;
#[path = "裁定待ちの語の短い版/元の一覧.rs"]
mod 元の一覧;
#[cfg(test)]
#[path = "裁定待ちの語の短い版/元の一覧の読み取りの試験.rs"]
mod 元の一覧の読み取りの試験;
#[path = "裁定待ちの語の短い版/元の一覧の読み手.rs"]
mod 元の一覧の読み手;
#[path = "裁定待ちの語の短い版/区切られたドメインの表記.rs"]
mod 区切られたドメインの表記;
#[path = "裁定待ちの語の短い版/原文の塊.rs"]
mod 原文の塊;
#[path = "裁定待ちの語の短い版/定義文の最初の文.rs"]
mod 定義文の最初の文;
#[cfg(test)]
#[path = "裁定待ちの語の短い版/文の読み取りの試験.rs"]
mod 文の読み取りの試験;
#[path = "裁定待ちの語の短い版/短い版の組み立て.rs"]
mod 短い版の組み立て;
#[path = "裁定待ちの語の短い版/生成した結果.rs"]
mod 生成した結果;
#[path = "裁定待ちの語の短い版/表の読み取り.rs"]
mod 表の読み取り;
#[path = "裁定待ちの語の短い版/裁定待ちの語.rs"]
mod 裁定待ちの語;
#[path = "裁定待ちの語の短い版/読めなかった行.rs"]
mod 読めなかった行;

use std::process::ExitCode;

pub(crate) use ファイルの置き場::裁定待ちの語のファイルの置き場;
pub(crate) use 生成した結果::短い定義一覧を書けない理由;

/// 生成のコマンドの名前。短い版の冒頭と、古さの検査の違反の文言がこの名前で生成し直す方法を案内する。
pub(crate) const 生成のコマンド: &str = "cargo xtask gen-pending-short-definitions";

pub fn 短い版を生成する() -> ExitCode {
    let 置き場 = 裁定待ちの語のファイルの置き場::リポジトリの根からの既定();
    let 元の一覧 = match 置き場.元の一覧を読む() {
        Ok(元の一覧) => 元の一覧,
        Err(誤り) => {
            eprintln!("[xtask] gen-pending-short-definitions: {} を読めなかった: {誤り}", 置き場.元の一覧のパス().display());
            return ExitCode::FAILURE;
        }
    };
    let 生成した結果 = 元の一覧.短い版を組む();
    println!("[xtask] 元の一覧から読んだ語: {}件、短い版に書いた語: {}件", 生成した結果.読んだ語の数(), 生成した結果.短い版().語の数を数える());
    if let Err(理由) = 生成した結果.短い版を書いてよいかを確かめる() {
        eprintln!("[xtask] gen-pending-short-definitions: {} から短い版を書かなかった: {理由}", 置き場.元の一覧のパス().display());
        return ExitCode::FAILURE;
    }
    match 置き場.短い版を書く(生成した結果.短い版()) {
        Ok(()) => {
            println!("[xtask] {} を書いた", 置き場.短い版のパス().display());
            ExitCode::SUCCESS
        }
        Err(誤り) => {
            eprintln!("[xtask] gen-pending-short-definitions: {} を書けなかった: {誤り}", 置き場.短い版のパス().display());
            ExitCode::FAILURE
        }
    }
}
