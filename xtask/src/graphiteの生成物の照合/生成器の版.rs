//! Graphiteの生成器(`cargo-graphite`)の版を、Cargo.lock が固定した graphite の版へ結び付ける確かめ。
//! 受け取るのは Cargo.lock の本文と、cargo が入れた道具を記録するファイル(`<cargoの置き場>/.crates.toml`)の本文、返すのは一致か食い違いである。
//!
//! 生成器は自分の版を答える口(`--version` のような引数)を持たないため、生成器そのものには尋ねない。代わりに、`cargo install` が
//! 入れた道具ごとに出どころ(`git+<リポジトリ>?rev=<版>#<コミット>` のような表記)を書き残す記録を読み、そのコミットを Cargo.lock の固定の版と比べる。
//! パスから入れた生成器(`path+file://...`)は、どのコミットから作ったかが記録に残らず確かめられないため、食い違いとして扱う。
//! 生成器の版と宣言を展開するマクロの版が違うと、`cargo graphite generate --check` の照合の相手が別の生成規則になり、照合の意味が崩れるためである。

/// 依存の版を固定したファイル(Cargo.lock)の名前。リポジトリの根に置かれる。
pub(super) const 依存の版を固定したファイルの名前: &str = "Cargo.lock";

/// cargo が入れた道具の出どころを書き残すファイルの名前。cargo の置き場の直下に置かれる。
pub(super) const 入れた道具の記録のファイル名: &str = ".crates.toml";

const 生成器のリポジトリ: &str = "https://github.com/megaDenryu/Graphite";

/// Cargo.lock が固定した graphite のコミット(40桁の16進)。
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Graphiteの固定の版(String);

/// 入れた生成器の出どころが、固定の版と一致しなかった理由。
#[derive(Debug, PartialEq, Eq)]
pub(super) enum 生成器の版の食い違い {
    記録に生成器が無い,
    コミットが違う { 入れた出どころ: String },
    コミットを確かめられない出どころ { 入れた出どころ: String },
}

impl Graphiteの固定の版 {
    /// Cargo.lock の本文から、名前が graphite の包みの `source` の `#` の後ろのコミットを読む。git から取っていなければ無しである。
    pub(super) fn 依存の版を固定したファイルの本文から読む(本文: &str) -> Option<Self> {
        let mut graphiteの包みか = false;
        for 行 in 本文.lines().map(str::trim) {
            if 行 == "[[package]]" {
                graphiteの包みか = false;
            } else if 行 == "name = \"graphite\"" {
                graphiteの包みか = true;
            } else if let (true, Some(出どころ)) = (graphiteの包みか, 行.strip_prefix("source = \"").and_then(|残り| 残り.strip_suffix('"'))) {
                return git由来のコミット(出どころ).map(|コミット| Self(コミット.to_string()));
            }
        }
        None
    }

    /// この版の生成器を入れる(入れ直す)コマンド。同じ名前の道具が別の出どころで入っていても置き換えるため `--force` を付ける。
    pub(super) fn 生成器を入れるコマンド(&self) -> String {
        format!("cargo install --git {生成器のリポジトリ} --rev {} graphite-cli --force", self.コミット())
    }

    /// 入れた道具の記録の本文から graphite-cli の出どころを探し、そのコミットがこの版と一致するかを確かめる。
    pub(super) fn 入れた生成器と照らす(&self, 記録の本文: &str) -> Result<(), 生成器の版の食い違い> {
        let Some(出どころ) = 記録の本文.lines().find_map(graphite_cliの出どころ) else {
            return Err(生成器の版の食い違い::記録に生成器が無い);
        };
        match git由来のコミット(出どころ) {
            Some(コミット) if コミット == self.コミット() => Ok(()),
            Some(_) => Err(生成器の版の食い違い::コミットが違う {
                入れた出どころ: 出どころ.to_string()
            }),
            None => Err(生成器の版の食い違い::コミットを確かめられない出どころ {
                入れた出どころ: 出どころ.to_string()
            }),
        }
    }

    pub(super) fn コミット(&self) -> &str {
        &self.0
    }
}

// 記録の1行 `"graphite-cli 0.1.0 (<出どころ>)" = [...]` から、括弧の中の出どころを取り出す。
fn graphite_cliの出どころ(行: &str) -> Option<&str> {
    let 残り = 行.trim().strip_prefix("\"graphite-cli ")?;
    let 開き = 残り.find('(')?;
    let 閉じ = 残り.find(')')?;
    残り.get(開き + 1..閉じ)
}

// `git+<リポジトリ>...#<コミット>` の形の出どころから、コミットを取り出す。git 以外の出どころは無しである。
fn git由来のコミット(出どころ: &str) -> Option<&str> {
    出どころ.strip_prefix("git+")?.rsplit_once('#').map(|(_, コミット)| コミット)
}

#[cfg(test)]
mod tests {
    use super::{Graphiteの固定の版, 生成器の版の食い違い};

    const コミット: &str = "8d264e19bd7c44c54d45316a17c10aa03cb9d9e3";

    fn 固定の版() -> Graphiteの固定の版 {
        let 本文 = format!(
            "[[package]]\nname = \"graphite-codegen\"\nsource = \"git+https://github.com/megaDenryu/Graphite#ffff\"\n\n[[package]]\nname = \"graphite\"\nversion = \"0.1.0\"\nsource = \"git+https://github.com/megaDenryu/Graphite#{コミット}\"\n"
        );
        Graphiteの固定の版::依存の版を固定したファイルの本文から読む(&本文).unwrap_or_else(|| panic!("graphite の固定の版を読めなかった"))
    }

    #[test]
    fn 依存の版を固定したファイルから名前がgraphiteの包みのコミットだけを読み入れるコマンドに含める() {
        assert_eq!(固定の版().コミット(), コミット);
        assert!(固定の版().生成器を入れるコマンド().contains(&format!("--rev {コミット}")));
        assert_eq!(Graphiteの固定の版::依存の版を固定したファイルの本文から読む("[[package]]\nname = \"graphite\"\nsource = \"path+file:///C:/Graphite\"\n"), None);
    }

    #[test]
    fn 記録の出どころのコミットが固定の版と同じときだけ一致する() {
        let 記録 = |出どころ: &str| format!("[v1]\n\"graphite-cli 0.1.0 ({出どころ})\" = [\"cargo-graphite.exe\"]\n");
        let 版 = 固定の版();
        assert_eq!(版.入れた生成器と照らす(&記録(&format!("git+https://github.com/megaDenryu/Graphite?rev={コミット}#{コミット}"))), Ok(()));
        assert!(matches!(版.入れた生成器と照らす(&記録("git+https://github.com/megaDenryu/Graphite#0da8f5c0")), Err(生成器の版の食い違い::コミットが違う { .. })));
        assert!(matches!(
            版.入れた生成器と照らす(&記録("path+file:///C:/devs/Graphite/crates/graphite-cli")),
            Err(生成器の版の食い違い::コミットを確かめられない出どころ { .. })
        ));
        assert_eq!(版.入れた生成器と照らす("[v1]\n"), Err(生成器の版の食い違い::記録に生成器が無い));
    }
}
