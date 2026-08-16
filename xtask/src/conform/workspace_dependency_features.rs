//! ワークスペースの依存宣言の機能一覧の台帳。根のCargo.tomlの`[workspace.dependencies]`が宣言する機能が、
//! この台帳と一致することを確かめる。依存を足すときと同じく、機能を足すときも台帳の更新を強制する。
//!
//! 依存白リストは各クレートのCargo.tomlの依存の名前だけを読み、根のCargo.tomlを一度も開かない。
//! そのため`gltf = { version = "1", features = ["extras"] }`のような機能の追加が、依存の追加と違って
//! 二重台帳を通らず無検査で入る。機能はビルドの結果を変えるため、名前と同じく台帳に持たせる。
//! 参照: `_doc/設計/部品カタログと接合点.md`「機械強制の手段」

mod declaration;
#[cfg(test)]
mod declaration_tests;
mod feature_list;
mod ledger;

use std::path::PathBuf;

use super::cargo_toml_parse::パッケージ宣言のファイル名;
use super::error::規約検査の破れ;
use super::violation::違反;
use declaration::依存節の行;
use ledger::機能台帳;

const 依存節の見出し: &str = "[workspace.dependencies]";

pub fn 全依存を検査する() -> Result<Vec<違反>, 規約検査の破れ> {
    let パス = PathBuf::from(パッケージ宣言のファイル名);
    let 内容 = std::fs::read_to_string(&パス).map_err(|誤り| 規約検査の破れ::ファイルを読めなかった(&パス, 誤り))?;
    Ok(内容から違反を集める(&パス, &内容))
}

/// 台帳に無い依存と、台帳にあるのに宣言が無い依存の両方を違反にする。片方だけだと、依存を消したときに台帳が腐る。
fn 内容から違反を集める(パス: &std::path::Path, 内容: &str) -> Vec<違反> {
    let mut 違反一覧 = Vec::new();
    let mut 見つけた依存名一覧: Vec<String> = Vec::new();
    for 行 in 依存節の行一覧を切り出す(内容) {
        match 依存節の行::読み取る(行) {
            依存節の行::宣言でない => {}
            依存節の行::一行で閉じていない { 名前 } => 違反一覧.push(違反::ファイル単位(
                パス.to_path_buf(),
                format!("依存{名前}の宣言が1行で閉じていない。機能一覧を読み取れないため、1行で書く"),
            )),
            依存節の行::宣言(宣言) => {
                見つけた依存名一覧.push(宣言.名前().to_string());
                match 機能台帳.iter().find(|項| 項.依存名 == 宣言.名前()) {
                    Some(項) => 違反一覧.extend(
                        宣言
                            .台帳と食い違う点一覧(項)
                            .into_iter()
                            .map(|説明| 違反::ファイル単位(パス.to_path_buf(), 説明)),
                    ),
                    None => 違反一覧.push(違反::ファイル単位(
                        パス.to_path_buf(),
                        format!("機能台帳に未登録の依存: {}", 宣言.名前()),
                    )),
                }
            }
        }
    }
    for 項 in 機能台帳.iter().filter(|項| !見つけた依存名一覧.iter().any(|名前| 名前 == 項.依存名)) {
        違反一覧.push(違反::ファイル単位(
            パス.to_path_buf(),
            format!("機能台帳にあるのに宣言が無い依存: {}。依存を消したなら台帳からも消す", 項.依存名),
        ));
    }
    違反一覧
}

fn 依存節の行一覧を切り出す(内容: &str) -> impl Iterator<Item = &str> {
    内容
        .lines()
        .skip_while(|行| 行.trim() != 依存節の見出し)
        .skip(1)
        .take_while(|行| !行.trim().starts_with('['))
}
