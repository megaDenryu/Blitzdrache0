//! 実行時形式の出力ルート: 1つの世界の実行時形式一式を置くディレクトリを表す型。カタログ・チャンク目録・アセットの
//! ファイル名の綴りと、それらの書き出しと読み戻しを持つ。
//!
//! 綴りを型の中へ閉じるのは、拡張子やカタログの名前の写しが呼び出し側へ散ると、書く側と読む側で食い違ったときに
//! 「前回のカタログが読めないので全部焼き直す」という無言の劣化になるためである。
//!
//! 置き場のパスは包んだ生成の出力ルートが1回だけ所有する。台帳の置き場として渡すときも借用で貸し、複製を作らない。

use std::path::PathBuf;

use blitz_engine::{アセットID, カタログ, 実行時形式からカタログを読む};

use crate::generation_ledger::生成の出力ルート;

const 実行時カタログのファイル名: &str = "catalog.blitzcatalog";
const 実行時目録のファイル名: &str = "chunk_directory.blitzchunks";
const アセットの拡張子つきの綴り: &str = ".blitzasset";

#[repr(transparent)]
pub struct 実行時形式の出力ルート(生成の出力ルート);

impl 実行時形式の出力ルート {
    pub fn 作る(ディレクトリ: PathBuf) -> Result<Self, String> {
        std::fs::create_dir_all(&ディレクトリ).map_err(|誤り| format!("出力ディレクトリ{}を作れない: {誤り}", ディレクトリ.display()))?;
        Ok(Self(生成の出力ルート::生成する(ディレクトリ)))
    }

    pub fn アセットの出力パス(&self, 安定id: &アセットID) -> PathBuf {
        self.0.直下のファイルのパス(&Self::アセットのファイル名(安定id))
    }

    /// アセット1件を書き出し、カタログが実行時パスとして持つファイル名を返す。ファイル名を1回だけ組んで
    /// 書き出しと戻り値の両方に使う。
    pub fn アセットを書き出す(&self, 安定id: &アセットID, バイト列: &[u8]) -> Result<PathBuf, String> {
        let ファイル名 = Self::アセットのファイル名(安定id);
        let パス = self.0.直下のファイルのパス(&ファイル名);
        std::fs::write(&パス, バイト列).map_err(|誤り| format!("{}を書き出せない: {誤り}", パス.display()))?;
        println!("[compile_assets] {}: {}バイト", パス.display(), バイト列.len());
        Ok(PathBuf::from(ファイル名))
    }

    fn アセットのファイル名(安定id: &アセットID) -> String {
        format!("{安定id}{アセットの拡張子つきの綴り}")
    }

    /// 前回の実行時カタログ。読めなければ値なしを返す。初回の生成にはカタログが無く、
    /// 壊れたカタログを読み違えるよりも全部焼き直すほうが安全である。
    pub fn 前回の実行時カタログを読む(&self) -> Option<カタログ> {
        let バイト列 = std::fs::read(self.0.直下のファイルのパス(実行時カタログのファイル名)).ok()?;
        実行時形式からカタログを読む(&バイト列).ok()
    }

    pub fn 実行時カタログを書き出す(&self, バイト列: &[u8]) -> Result<(), String> {
        let パス = self.直下へ書き込む(実行時カタログのファイル名, バイト列)?;
        println!("[compile_assets] {}: {}バイト", パス.display(), バイト列.len());
        Ok(())
    }

    pub fn チャンク目録を書き出す(&self, バイト列: &[u8], チャンク数: usize) -> Result<(), String> {
        let パス = self.直下へ書き込む(実行時目録のファイル名, バイト列)?;
        println!("[compile_assets] {}: {}バイト({}チャンク)", パス.display(), バイト列.len(), チャンク数);
        Ok(())
    }

    /// 生成台帳の置き場としてのこの置き場。台帳の読み書きは生成の出力ルートの側が担う。
    pub fn 台帳の置き場(&self) -> &生成の出力ルート {
        &self.0
    }

    /// この置き場がファイルへ書く唯一の口。書いた先を返すのは、報せの行がパスを載せるためである。
    fn 直下へ書き込む(&self, ファイル名: &str, バイト列: &[u8]) -> Result<PathBuf, String> {
        let パス = self.0.直下のファイルのパス(ファイル名);
        std::fs::write(&パス, バイト列).map_err(|誤り| format!("{}を書き出せない: {誤り}", パス.display()))?;
        Ok(パス)
    }
}
