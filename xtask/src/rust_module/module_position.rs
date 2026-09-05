//! 1つのRustのファイルが占めるモジュールの位置。そのファイルが本体になっているモジュールと、
//! そのモジュールが子を置くディレクトリの組である。
//!
//! 経路の解決の起点(`self`と`super`が指す先、`crate`が指すクレートの根)を、この型が答える。

use std::path::{Path, PathBuf};

use super::module_directory::{モジュールのディレクトリ, モジュールの本体の語幹か};

/// クレートの根のディレクトリの名前。`crate::`から始まる経路の起点であり、Cargoの既定の配置に従う。
const クレートの根のディレクトリ名: &str = "src";

pub struct モジュールの位置 {
    定義ファイル: PathBuf,
    子のモジュールのディレクトリ: PathBuf,
}

impl モジュールの位置 {
    pub fn 定義ファイルから生成する(定義ファイル: &Path) -> Self {
        Self {
            定義ファイル: 定義ファイル.to_path_buf(),
            子のモジュールのディレクトリ: 子のモジュールのディレクトリを求める(定義ファイル),
        }
    }

    /// このモジュールの木に属するファイルか。本体のファイル自身と、子のモジュールのディレクトリの配下が属する。
    pub fn この木の中のファイルか(&self, パス: &Path) -> bool {
        パス == self.定義ファイル || パス.starts_with(&self.子のモジュールのディレクトリ)
    }

    /// `self`が指す先。このファイルのモジュールが子のモジュールを置くディレクトリである。
    pub fn 自分のモジュールのディレクトリ(&self) -> モジュールのディレクトリ {
        モジュールのディレクトリ::パスから生成する(&self.子のモジュールのディレクトリ)
    }

    /// `crate`が指す先。`src`という名前の最も近い祖先のディレクトリをクレートの根とする。
    pub fn クレートの根のディレクトリ(&self) -> Option<モジュールのディレクトリ> {
        self.定義ファイル
            .ancestors()
            .find(|祖先| 祖先.file_name().and_then(|名前| 名前.to_str()) == Some(クレートの根のディレクトリ名))
            .map(モジュールのディレクトリ::パスから生成する)
    }
}

/// `x/mod.rs`の子は`x/`配下であり、`x/y.rs`の子は`x/y/`配下である。Rustのモジュールの綴り方に従う。
fn 子のモジュールのディレクトリを求める(定義ファイル: &Path) -> PathBuf {
    let 親 = 定義ファイル.parent().unwrap_or(Path::new("")).to_path_buf();
    let Some(語幹) = 定義ファイル.file_stem().and_then(|語幹| 語幹.to_str()) else {
        return 親;
    };
    if モジュールの本体の語幹か(語幹) {
        return 親;
    }
    親.join(語幹)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn モジュールの木は本体のファイルの子孫だけを含む() {
        let 位置 = モジュールの位置::定義ファイルから生成する(Path::new("crates/blitz_app/src/app/mod.rs"));
        assert!(位置.この木の中のファイルか(Path::new("crates/blitz_app/src/app/frame/draw_input.rs")));
        assert!(!位置.この木の中のファイルか(Path::new("crates/blitz_app/src/input/ingest.rs")));
    }

    #[test]
    fn 自分のディレクトリは語幹が本体かで変わる() {
        let 枝 = モジュールの位置::定義ファイルから生成する(Path::new("a/src/near/impl.rs"));
        assert_eq!(
            枝.自分のモジュールのディレクトリ(),
            モジュールのディレクトリ::パスから生成する(Path::new("a/src/near/impl"))
        );
        let 本体 = モジュールの位置::定義ファイルから生成する(Path::new("a/src/near/mod.rs"));
        assert_eq!(
            本体.自分のモジュールのディレクトリ(),
            モジュールのディレクトリ::パスから生成する(Path::new("a/src/near"))
        );
    }

    #[test]
    fn クレートの根は最も近いsrcの祖先になる() {
        let 位置 = モジュールの位置::定義ファイルから生成する(Path::new("crates/blitz_collision/src/triangle/sweep.rs"));
        assert_eq!(
            位置.クレートの根のディレクトリ().unwrap(),
            モジュールのディレクトリ::パスから生成する(Path::new("crates/blitz_collision/src"))
        );
        assert!(
            モジュールの位置::定義ファイルから生成する(Path::new("build.rs"))
                .クレートの根のディレクトリ()
                .is_none()
        );
    }
}
