//! 2つの診断世界の実行時アセットを焼き、指定した条件で同じ構図の絵を1枚ずつ描く工程。
//! 受け取るのは条件の名前と追加の起動指定、返すのは粗い側と細かい側の実行結果である。
//!
//! アセットを焼くのは1度だけであり、条件を増やしても焼き直さない。
//! 焼き直すと、比べている2枚が別々に作り直したアセットから描かれることになる。

use std::path::{Path, PathBuf};

use super::run;

pub(super) struct 診断の対 {
    出力先: PathBuf,
    粗いルート: PathBuf,
    細かいルート: PathBuf,
}

impl 診断の対 {
    pub(super) fn 焼く(出力先: &Path, チャンクあたり個体数: usize) -> Result<Self, String> {
        let 粗いルート = 一世界を焼く(出力先, "coarse", crate::compile_assets::頂点診断の粗い世界, チャンクあたり個体数)?;
        let 細かいルート = 一世界を焼く(出力先, "fine", crate::compile_assets::頂点診断の細かい世界, チャンクあたり個体数)?;
        Ok(Self {
            出力先: 出力先.to_path_buf(),
            粗いルート,
            細かいルート,
        })
    }

    pub(super) fn 描き比べる(&self, 条件名: &str, 追加引数: &[&str]) -> Result<(run::実行結果, run::実行結果), String> {
        let 粗い = run::描画する(&self.出力先, &format!("coarse_{条件名}"), &self.粗いルート, 追加引数)?;
        let 細かい = run::描画する(&self.出力先, &format!("fine_{条件名}"), &self.細かいルート, 追加引数)?;
        Ok((粗い, 細かい))
    }
}

fn 一世界を焼く(出力先: &Path, 名前: &str, 世界名: &str, チャンクあたり個体数: usize) -> Result<PathBuf, String> {
    let ルート = 出力先.join(format!("assets_{名前}"));
    if crate::compile_assets::世界を個体数指定で生成する(&ルート, 世界名, チャンクあたり個体数) {
        return Ok(ルート);
    }
    Err(format!("{世界名}の実行時アセット生成に失敗した"))
}
