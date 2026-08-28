//! メニューが選んだコマンドから組み立てる実行の1行。所有するのは、先頭語がASCIIコマンド名であることと
//! 語の並び順に意味があることを保つ責務、その並びを端末へ打ち直せる綴りへ写す責務、そしてdispatchの
//! 対応表へ渡す語の並びを貸す責務である。
//!
//! 生成の口を1つに絞るのは、先頭語がコマンド名であることを構築の時点で保証するためである。
//! 空白を含む語を引用符で囲むのは、そのまま打ち直したときに語が割れないためであり、囲みの規則は
//! `argument_line`が行を語へ分ける規則の逆にあたる。

pub(super) struct コマンド行 {
    語一覧: Vec<String>,
}

impl コマンド行 {
    /// ASCIIコマンド名を先頭に置き、その後ろへ引数の語を並びの順のまま足す。
    pub(super) fn コマンド名と引数の語から生成する(ascii名: &str, 引数の語一覧: Vec<String>) -> Self {
        let mut 語一覧 = vec![ascii名.to_string()];
        語一覧.extend(引数の語一覧);
        Self { 語一覧 }
    }

    /// 端末へそのまま打ち直せる1行へ写す。メニューが実行の直前にこの行を出すのは、次からメニューを
    /// 通さずに同じ実行ができるようにするためである。
    pub(super) fn 打ち直せる行にする(&self) -> String {
        let 囲んだ語一覧: Vec<String> = self.語一覧.iter().map(|語| 必要なら引用符で囲む(語)).collect();
        format!("cargo xtask {}", 囲んだ語一覧.join(" "))
    }

    /// dispatchの対応表へ渡すための語の並び。生の文字列の並びへ戻すのはこの1箇所だけである。
    pub(super) fn 語の並び(&self) -> &[String] {
        &self.語一覧
    }
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

    fn 引数の語一覧にする(語一覧: &[&str]) -> Vec<String> {
        語一覧.iter().map(|語| (*語).to_string()).collect()
    }

    #[test]
    fn 引数なしのコマンドはコマンド名だけを並べる() {
        let コマンド行 = コマンド行::コマンド名と引数の語から生成する("verify", Vec::new());
        assert_eq!(コマンド行.打ち直せる行にする(), "cargo xtask verify");
    }

    #[test]
    fn 引数はそのまま空白で繋ぐ() {
        let 引数 = 引数の語一覧にする(&["--frames", "160"]);
        let コマンド行 = コマンド行::コマンド名と引数の語から生成する("indirect-cost", 引数);
        assert_eq!(コマンド行.打ち直せる行にする(), "cargo xtask indirect-cost --frames 160");
    }

    #[test]
    fn 空白を含む語は引用符で囲む() {
        let コマンド行 = コマンド行::コマンド名と引数の語から生成する("check-glb", 引数の語一覧にする(&["小屋 の 部品"]));
        assert_eq!(コマンド行.打ち直せる行にする(), "cargo xtask check-glb \"小屋 の 部品\"");
    }

    #[test]
    fn 語の並びはコマンド名から始まる() {
        let コマンド行 = コマンド行::コマンド名と引数の語から生成する("menu", 引数の語一覧にする(&["--frames", "8"]));
        assert_eq!(コマンド行.語の並び(), ["menu", "--frames", "8"]);
    }
}
