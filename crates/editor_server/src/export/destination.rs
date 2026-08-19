//! 世界ソース出力先: `<リポジトリルート>/assets/<世界名>/`という、書き出し先ディレクトリを表す
//! 役割を持つパスの型。配下のファイルパスの導出と、一時ファイル経由での原子的な書き込みを
//! この型のメソッドの1箇所へ閉じる(参照: グローバルCLAUDE.md「役割の型は自分の配置を知る」)。
//! 書き込みの技法自体(一時ファイルへ書いてから改名)は`storage::file_repository`の保存規律と
//! 同じであり、綴り(".tmp")を2箇所へ書かないよう`crate::atomic_file_write`へ寄せてある。

use std::path::{Path, PathBuf};

use super::world_name::出力世界名;
use crate::atomic_file_write::一時ファイル経由で書き込む;

const アセットの親ディレクトリ名: &str = "assets";

/// 世界ソース出力先とは、1回の書き出しが対象とする、世界名で決まる出力先ディレクトリのことである。
pub struct 世界ソース出力先(PathBuf);

impl 世界ソース出力先 {
    pub fn 生成する(リポジトリルート: &Path, 世界名: &出力世界名) -> Self {
        Self(リポジトリルート.join(アセットの親ディレクトリ名).join(世界名.綴り()))
    }

    pub fn 表示の綴り(&self) -> String {
        self.0.to_string_lossy().into_owned()
    }

    pub fn ディレクトリを用意する(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.0)
    }

    /// 直下のファイル名を1つ受け取り、そこへ一時ファイル経由で書き込む。呼び出し側は
    /// パスの組み立てに触れないため、書き出し先の綴りがこのメソッドの外へ散らばらない。
    pub fn 直下へ書き込む(&self, ファイル名: &str, 内容: &[u8]) -> std::io::Result<()> {
        一時ファイル経由で書き込む(&self.0.join(ファイル名), 内容)
    }
}
