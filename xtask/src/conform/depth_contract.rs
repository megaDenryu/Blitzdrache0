//! カメラの逆向き深度と光源影の標準深度を、消去・NDC端点・書き込み比較・標本比較の組として検査する。
//! 数値と演算を別々の定数対にしないのは、深度領域を取り違えた組だけが成立することを防ぐためである。

mod table;
#[cfg(test)]
mod tests;

use std::path::{Path, PathBuf};

use super::error::規約検査の破れ;
use super::violation::違反;

pub(super) fn 全接点を検査する() -> Result<Vec<違反>, 規約検査の破れ> {
    let mut 違反一覧 = Vec::new();
    for 接点 in table::全接点() {
        let パス = Path::new(接点.パス);
        let 内容 = std::fs::read_to_string(パス).map_err(|誤り| 規約検査の破れ::ファイルを読めなかった(パス, 誤り))?;
        if !指定の綴りがあるか(&内容, 接点.期待する綴り) {
            違反一覧.push(違反::ファイル単位(
                PathBuf::from(接点.パス),
                format!(
                    "{}の深度契約({})が組と食い違う: 消去{}・近NDC{}・遠NDC{}・書込{}・標本{}",
                    接点.契約.領域名, 接点.項目, 接点.契約.消去値, 接点.契約.近面ndc, 接点.契約.遠面ndc, 接点.契約.書込比較, 接点.契約.標本比較,
                ),
            ));
        }
    }
    Ok(違反一覧)
}

fn 指定の綴りがあるか(内容: &str, 期待する綴り: &str) -> bool {
    空白を均す(内容).contains(&空白を均す(期待する綴り))
}

fn 空白を均す(内容: &str) -> String {
    内容.split_whitespace().collect::<Vec<_>>().join(" ")
}
