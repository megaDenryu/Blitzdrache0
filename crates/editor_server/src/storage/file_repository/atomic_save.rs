//! 一時ファイルへ書いてから改名する保存の手続き。書き込み中の異常終了で正本を壊さないため、
//! JSON(文字列)とバイナリ(バイト列)の両方の保存経路が使う共通の工程。

use std::path::{Path, PathBuf};

pub(super) fn 一時ファイル経由で書き込む(保存先: &Path, 内容: &[u8]) -> std::io::Result<()> {
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
