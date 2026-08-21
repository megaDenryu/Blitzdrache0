//! 同じディレクトリの一時ファイルへ書いてから改名し、途中までの生成物を正本として見せない。

use std::path::{Path, PathBuf};

pub(crate) fn 一時ファイル経由で書き込む(保存先: &Path, 内容: &[u8]) -> std::io::Result<()> {
    let 一時ファイル = 一時ファイルパスを作る(保存先);
    std::fs::write(&一時ファイル, 内容)?;
    std::fs::rename(&一時ファイル, 保存先)?;
    Ok(())
}

fn 一時ファイルパスを作る(保存先: &Path) -> PathBuf {
    let mut ファイル名 = 保存先.as_os_str().to_os_string();
    ファイル名.push(blitz_engine::実行時アセットの公開完了印::公開前の一時ファイル接尾辞());
    PathBuf::from(ファイル名)
}
