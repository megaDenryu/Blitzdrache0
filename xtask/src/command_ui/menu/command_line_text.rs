//! 組み立てたコマンド行を、そのまま端末へ打ち直せる1行の綴りへ写す工程。受け取るのはコマンド名から
//! 始まる語の並び、返すのは`cargo xtask`を前に置いた1行である。
//!
//! メニューが実行の直前にこの行を出すのは、次からメニューを通さずに同じ実行ができるようにするためである。
//! 空白を含む語を引用符で囲むのは、そのまま打ち直したときに語が割れないためであり、囲みの規則は
//! `argument_line`が行を語へ分ける規則の逆にあたる。

pub(super) fn 打ち直せる行にする(コマンド行: &[String]) -> String {
    let 語一覧: Vec<String> = コマンド行.iter().map(|語| 必要なら引用符で囲む(語)).collect();
    format!("cargo xtask {}", 語一覧.join(" "))
}

fn 必要なら引用符で囲む(語: &str) -> String {
    if 語.is_empty() || 語.chars().any(char::is_whitespace) {
        return format!("\"{語}\"");
    }
    語.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn 語一覧にする(語一覧: &[&str]) -> Vec<String> {
        語一覧.iter().map(|語| (*語).to_string()).collect()
    }

    #[test]
    fn 引数なしのコマンドはコマンド名だけを並べる() {
        assert_eq!(打ち直せる行にする(&語一覧にする(&["verify"])), "cargo xtask verify");
    }

    #[test]
    fn 引数はそのまま空白で繋ぐ() {
        let 行 = 打ち直せる行にする(&語一覧にする(&["indirect-cost", "--frames", "160"]));
        assert_eq!(行, "cargo xtask indirect-cost --frames 160");
    }

    #[test]
    fn 空白を含む語は引用符で囲む() {
        let 行 = 打ち直せる行にする(&語一覧にする(&["check-glb", "小屋 の 部品"]));
        assert_eq!(行, "cargo xtask check-glb \"小屋 の 部品\"");
    }
}
