//! 検収用ソースルートへ、生成対象以外の既存素材を複写する。

use std::path::Path;

use super::検収用のルート;
use crate::game_fox_tour::error::場所巡りの通しの検収エラー;

impl 検収用のルート {
    pub(in crate::game_fox_tour) fn 既存素材を複写する(&self) -> Result<(), 場所巡りの通しの検収エラー> {
        let 元 = crate::compile_assets::ソースルート();
        self.ディレクトリを複写する(元, &self.0, true)
    }

    fn ディレクトリを複写する(
        &self,
        元: &Path,
        先: &Path,
        場所巡りの世界を除く: bool,
    ) -> Result<(), 場所巡りの通しの検収エラー> {
        std::fs::create_dir_all(先).map_err(|誤り| 場所巡りの通しの検収エラー::ファイルを複写できなかった {
            元: 元.to_path_buf(),
            先: 先.to_path_buf(),
            誤り,
        })?;
        for 項目 in std::fs::read_dir(元).map_err(|誤り| 場所巡りの通しの検収エラー::ディレクトリを開けなかった {
            パス: 元.to_path_buf(),
            誤り,
        })? {
            let 項目 = 項目.map_err(|誤り| 場所巡りの通しの検収エラー::ディレクトリの走査に失敗した {
                パス: 元.to_path_buf(),
                誤り,
            })?;
            if 場所巡りの世界を除く
                && blitz_asset_compiler::場所巡りの世界のソースディレクトリ::この世界のディレクトリ名か(
                    &項目.file_name(),
                )
            {
                continue;
            }
            let 先の項目 = 先.join(項目.file_name());
            if 項目.path().is_dir() {
                self.ディレクトリを複写する(&項目.path(), &先の項目, false)?;
            } else {
                self.ファイルを複写する(&項目.path(), &先の項目)?;
            }
        }
        Ok(())
    }

    fn ファイルを複写する(&self, 元: &Path, 先: &Path) -> Result<(), 場所巡りの通しの検収エラー> {
        std::fs::copy(元, 先)
            .map(|_| ())
            .map_err(|誤り| 場所巡りの通しの検収エラー::ファイルを複写できなかった {
                元: 元.to_path_buf(),
                先: 先.to_path_buf(),
                誤り,
            })
    }
}
