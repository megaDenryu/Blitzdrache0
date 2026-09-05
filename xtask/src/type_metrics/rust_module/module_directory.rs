//! モジュールが子のモジュールを置くディレクトリ。段を1つ進む・上位へ戻る・本体のファイルの綴りへ変える、の
//! 3つの操作をこの型が持つ。
//!
//! 裸のパスで運ばないのは、経路の解決がこの3つの操作だけで書けることを型で示すためである。綴りは斜線区切りへ
//! 揃えて持ち、走査が実行環境ごとの区切りで得たパスと突き合わせられるようにする。

use std::path::Path;

/// モジュールの本体になれるファイルの語幹。拡張子を除いた語幹で持つのは、ファイル名らしい綴りの重複の検査に
/// この規則の内側の綴りを数えさせないためである。`lib`と`main`はクレートの根の本体の語幹である。
const モジュールの本体の語幹一覧: [&str; 3] = ["mod", "lib", "main"];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct モジュールのディレクトリ(String);

impl モジュールのディレクトリ {
    pub fn パスから生成する(ディレクトリ: &Path) -> Self {
        Self(ディレクトリ.to_string_lossy().replace('\\', "/"))
    }

    /// 名前の段を1つ進んだ先の、子のモジュールのディレクトリ。
    pub fn 子のモジュールへ進む(&self, 名前: &str) -> Self {
        if self.0.is_empty() {
            return Self(名前.to_string());
        }
        Self(format!("{}/{名前}", self.0))
    }

    /// 上位のモジュールのディレクトリを指定の段数だけ辿る。クレートの根より上へは辿れないため、辿り切れなければ返さない。
    pub fn 上位のモジュールへ戻る(&self, 段数: usize) -> Option<Self> {
        let mut 位置 = self.clone();
        for _ in 0..段数 {
            let (上位, _) = 位置.0.rsplit_once('/')?;
            位置 = Self(上位.to_string());
        }
        Some(位置)
    }

    /// このディレクトリのモジュールの本体になりうるファイルの綴り。`x`のモジュールは`x.rs`か`x/mod.rs`に書かれ、
    /// クレートの根なら`lib.rs`か`main.rs`である。走査で見つけた定義ファイルと突き合わせるためだけに使う。
    pub fn 本体になりうるファイルの綴り一覧(&self) -> Vec<String> {
        let mut 一覧 = vec![format!("{}.rs", self.0)];
        一覧.extend(モジュールの本体の語幹一覧.iter().map(|語幹| format!("{}/{語幹}.rs", self.0)));
        一覧
    }
}

pub(super) fn モジュールの本体の語幹か(語幹: &str) -> bool {
    モジュールの本体の語幹一覧.contains(&語幹)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn 段を進むと子のモジュールのディレクトリになる() {
        let 根 = モジュールのディレクトリ::パスから生成する(Path::new(r"crates\blitz_collision\src"));
        assert_eq!(
            根.子のモジュールへ進む("triangle"),
            モジュールのディレクトリ::パスから生成する(Path::new("crates/blitz_collision/src/triangle"))
        );
    }

    #[test]
    fn 上位へ戻る段数だけディレクトリを遡る() {
        let 位置 = モジュールのディレクトリ::パスから生成する(Path::new("a/src/near/deep"));
        assert_eq!(
            位置.上位のモジュールへ戻る(2).unwrap(),
            モジュールのディレクトリ::パスから生成する(Path::new("a/src"))
        );
        assert!(
            モジュールのディレクトリ::パスから生成する(Path::new("a"))
                .上位のモジュールへ戻る(1)
                .is_none()
        );
    }

    #[test]
    fn 本体になりうるファイルは語幹の分だけ挙がる() {
        let 綴り一覧 = モジュールのディレクトリ::パスから生成する(Path::new("a/src/far")).本体になりうるファイルの綴り一覧();
        assert!(綴り一覧.contains(&"a/src/far.rs".to_string()));
        assert!(綴り一覧.contains(&"a/src/far/mod.rs".to_string()));
        assert!(綴り一覧.contains(&"a/src/far/lib.rs".to_string()));
    }
}
