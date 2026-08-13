//! 検収用のルート: 生成の決定性を確かめるために使い捨てるディレクトリを表す型。4つの役割ごとに作り、
//! 掃除と突き合わせのための畳み込みを自分で行う。
//!
//! 既定のルートと分けるのは、遊ぶ世界を検収が消して作り直さないためである。役割ごとに構築の入口を分けるのは、
//! ソースの置き場と実行時形式の置き場を取り違えても裸のパスでは型が通るためである。
//!
//! 台帳を突き合わせから除くのは、生成台帳が生成器の実行ファイルの内容ハッシュと、ルートごとに違うソースのパスから
//! 導いた値を持つためである。別のルートへの生成どうしで一致しないのは当然であり、台帳は生成物ではなく増分の途中状態である。

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use blitz_asset_compiler::{ソースルート, 場所巡りの世界のソースディレクトリ, 生成の出力ルート};

mod cleanup;
mod parent;
mod source_fixture;
#[cfg(test)]
mod tests;

pub(super) use cleanup::{大規模世界の計測入力パス, 大規模世界の計測入力以外を掃除する};
pub(super) use parent::生成検収の親ディレクトリ;

use super::super::error::場所巡りの通しの検収エラー;
use super::file_digest::ディレクトリの全ファイルを畳む;

#[repr(transparent)]
pub(super) struct 検収用のルート(PathBuf);

impl 検収用のルート {
    pub(super) fn ソースの一度目(親: 生成検収の親ディレクトリ) -> Self {
        Self::末端の名前から作る(親, "source_a")
    }

    pub(super) fn ソースの二度目(親: 生成検収の親ディレクトリ) -> Self {
        Self::末端の名前から作る(親, "source_b")
    }

    pub(super) fn 実行時形式の一度目(親: 生成検収の親ディレクトリ) -> Self {
        Self::末端の名前から作る(親, "runtime_x")
    }

    pub(super) fn 実行時形式の二度目(親: 生成検収の親ディレクトリ) -> Self {
        Self::末端の名前から作る(親, "runtime_y")
    }

    pub(super) fn 増分のソース(親: 生成検収の親ディレクトリ) -> Self {
        Self::末端の名前から作る(親, "incremental_source")
    }

    pub(super) fn 増分の実行時形式(親: 生成検収の親ディレクトリ) -> Self {
        Self::末端の名前から作る(親, "incremental_runtime")
    }

    /// 増分を無効にするため、走らせる前にルートごと消す。台帳も生成物も残っていない状態から焼き直させる。
    pub(super) fn 掃除する(&self) -> Result<(), 場所巡りの通しの検収エラー> {
        match std::fs::remove_dir_all(&self.0) {
            Ok(()) => Ok(()),
            Err(誤り) if 誤り.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(誤り) => Err(場所巡りの通しの検収エラー::検収用のルートを消せなかった {
                パス: self.0.clone(), 誤り
            }),
        }
    }

    /// 突き合わせに使う畳んだ値。生成台帳は取り除く。台帳かどうかの判定はアセットコンパイラの側が持つため、
    /// ここは台帳のファイル名の綴りを1文字も持たない。
    pub(super) fn 全ファイルを畳む(&self) -> Result<BTreeMap<PathBuf, u64>, 場所巡りの通しの検収エラー> {
        let mut 対応表 = ディレクトリの全ファイルを畳む(&self.0)?;
        対応表.retain(|相対パス, _| !生成の出力ルート::生成台帳を指すパスか(相対パス));
        Ok(対応表)
    }

    pub(super) fn 場所巡りの生成物を畳む(&self) -> Result<BTreeMap<PathBuf, u64>, 場所巡りの通しの検収エラー> {
        let ソースルート = self.ソースルートとして読む();
        let 世界 = 場所巡りの世界のソースディレクトリ::ソースルートの下を開く(&ソースルート)
            .map_err(|誤り| 場所巡りの通しの検収エラー::ソースの置き場を作れなかった(誤り.to_string()))?;
        let mut 対応表 = ディレクトリの全ファイルを畳む(世界.ファイル走査へ貸すディレクトリ())?;
        対応表.retain(|相対パス, _| !生成の出力ルート::生成台帳を指すパスか(相対パス));
        Ok(対応表)
    }

    /// 生値へ戻す唯一の口。生成器とコンパイラをプロセスとして起動するとき、引数の綴りとして渡すためだけに使う。
    pub(super) fn プロセスへ渡すパス(&self) -> &Path {
        &self.0
    }

    /// この置き場をソースルートとして読む。生成器はソースの置き場を役割の型で受けるため、ここで着せ替える。
    pub(super) fn ソースルートとして読む(&self) -> ソースルート {
        ソースルート::生成する(self.0.clone())
    }

    pub(super) fn 実行時アセットルートとして読む(&self) -> crate::acceptance::実行時アセットルート {
        crate::acceptance::実行時アセットルート::パスから生成する(self.0.clone())
    }

    fn 末端の名前から作る(親: 生成検収の親ディレクトリ, 末端の名前: &str) -> Self {
        Self(親.ディレクトリ().join(末端の名前))
    }
}
