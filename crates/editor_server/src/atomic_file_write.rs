//! 一時ファイルへ書いてから改名する、原子的なファイル書き込みの共通工程。書き込み中の異常終了で
//! 正本を壊さないための技法であり、`storage::file_repository`(構造化データ・格子)と
//! `export::destination`(ソースアセットの書き出し)の両方が使う。綴り(".tmp")の正本をここへ
//! 1箇所へ寄せるのは、2箇所に同じ綴りを書くと`conform`の重複検査が拒むためである。

use std::path::{Path, PathBuf};

pub(crate) fn 一時ファイル経由で書き込む(保存先: &Path, 内容: &[u8]) -> std::io::Result<()> {
    if let Some(親) = 保存先.parent() {
        std::fs::create_dir_all(親)?;
    }
    let 一時ファイル = 一時ファイルパスを作る(保存先);
    std::fs::write(&一時ファイル, 内容)?;
    std::fs::rename(&一時ファイル, 保存先)?;
    Ok(())
}

fn 一時ファイルパスを作る(保存先: &Path) -> PathBuf {
    let mut ファイル名 = 保存先.as_os_str().to_os_string();
    ファイル名.push(".tmp");
    PathBuf::from(ファイル名)
}
