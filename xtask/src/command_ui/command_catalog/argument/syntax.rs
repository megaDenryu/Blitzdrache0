//! 使い方の行が見せている引数の構文。所有するのは、人が読む綴りを1つの値として保つことと、
//! その綴りの中で名指しされている引数(ハイフン2つで始まる語)を数え上げる規則である。
//!
//! 数え上げの規則をこの型の中へ置くのは、綴りの形と読み方が別々の場所で変わるのを防ぐためである。
//! 実行時の経路は綴りを使い方の行へ出すだけであり、綴りを読み直すのは引数定義との突き合わせだけである。
//! そのため数え上げと空の判定は試験の構成でだけ組み込む。

#[derive(Clone, Copy)]
pub(crate) struct 引数の構文 {
    綴り: &'static str,
}

impl 引数の構文 {
    pub(crate) const fn 生成する(綴り: &'static str) -> Self {
        Self { 綴り }
    }

    /// 引数を1つも解釈しないコマンドの構文は空になる。
    #[cfg(test)]
    pub(crate) fn 空か(self) -> bool {
        self.綴り.is_empty()
    }

    /// 使い方の表示に出す綴りそのもの。生の文字列へ戻すのはこの1箇所だけである。
    pub(crate) fn 綴り(self) -> &'static str {
        self.綴り
    }

    /// 構文の中で名指しされている引数の綴りを、現れた順に全部並べる。位置で意味が決まる引数は
    /// 名指しされないため、ここには現れない。
    #[cfg(test)]
    pub(crate) fn 名指しされた引数の綴り一覧(self) -> Vec<&'static str> {
        let mut 綴り一覧 = Vec::new();
        let mut 残り = self.綴り;
        while let Some(位置) = 残り.find("--") {
            let 続き = &残り[位置..];
            let 終わり = 続き.find(|文字: char| 文字 != '-' && !文字.is_ascii_alphanumeric()).unwrap_or(続き.len());
            綴り一覧.push(&続き[..終わり]);
            残り = &続き[終わり..];
        }
        綴り一覧
    }
}

#[cfg(test)]
mod tests {
    use super::引数の構文;

    #[test]
    fn 引数を取らない構文は空になる() {
        assert!(引数の構文::生成する("").空か());
        assert!(引数の構文::生成する("").名指しされた引数の綴り一覧().is_empty());
    }

    #[test]
    fn 角括弧に囲まれた名前も取り出す() {
        let 構文 = 引数の構文::生成する("[--large-world] [--frames <数>]");
        assert_eq!(構文.名指しされた引数の綴り一覧(), ["--large-world", "--frames"]);
    }

    #[test]
    fn 位置で渡す見出しは名指しに数えない() {
        let 構文 = 引数の構文::生成する("<部品名> [--out <経路>]");
        assert_eq!(構文.名指しされた引数の綴り一覧(), ["--out"]);
    }
}
