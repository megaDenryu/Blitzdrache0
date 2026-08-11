//! 2つの診断世界の実行時アセットを焼き、指定した条件で同じ構図の絵を1枚ずつ描く工程。
//! 受け取るのは条件の名前と追加の起動指定、返すのは粗い側と細かい側の実行結果である。
//!
//! アセットを焼くのは1度だけであり、条件を増やしても焼き直さない。
//! 焼き直すと、比べている2枚が別々に作り直したアセットから描かれることになる。

use std::path::{Path, PathBuf};

use super::run;
use crate::acceptance::描画検収の実行環境;

pub(super) struct 診断の対 {
    粗い世界の実行環境: 描画検収の実行環境,
    細かい世界の実行環境: 描画検収の実行環境,
}

impl 診断の対 {
    pub(super) fn 焼く(出力先: &Path, チャンクあたり個体数: usize) -> Result<Self, String> {
        let 粗いルート = 一世界を焼く(出力先, "coarse", crate::asset_generator::世界名::頂点診断の粗い世界, チャンクあたり個体数)?;
        let 細かいルート = 一世界を焼く(出力先, "fine", crate::asset_generator::世界名::頂点診断の細かい世界, チャンクあたり個体数)?;
        Ok(Self {
            粗い世界の実行環境: run::実行環境を作る(粗いルート, 出力先.to_path_buf())?,
            細かい世界の実行環境: run::実行環境を作る(細かいルート, 出力先.to_path_buf())?,
        })
    }

    pub(super) fn 描き比べる(&self, 条件名: &str, 追加の選択肢: &[&str]) -> Result<(run::実行結果, run::実行結果), String> {
        let 粗い = run::描画する(&self.粗い世界の実行環境, &format!("coarse_{条件名}"), 追加の選択肢)?;
        let 細かい = run::描画する(&self.細かい世界の実行環境, &format!("fine_{条件名}"), 追加の選択肢)?;
        Ok((粗い, 細かい))
    }
}

fn 一世界を焼く(
    出力先: &Path, 名前: &str, 世界: crate::asset_generator::世界名, チャンクあたり個体数: usize
) -> Result<PathBuf, String> {
    let ルート = 出力先.join(format!("assets_{名前}"));
    if crate::compile_assets::世界を個体数指定で生成する(&ルート, 世界, チャンクあたり個体数) {
        return Ok(ルート);
    }
    Err(format!("{世界}の実行時アセット生成に失敗した"))
}
