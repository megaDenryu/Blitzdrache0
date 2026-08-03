//! 画素段が属する照明問い合わせ契約と、その契約のソースファイル・一時出力名・報告名の対応。
//! 触れるのは監視先のパスだけであり、slangcの起動には関わらない。
//!
//! 契約を型で持つのは、両方の画素段の入口名が`fragmentMain`で同じだからである。失敗の報告に入口名しか載せないと、
//! どちらの契約のコンパイルが落ちたのかが読み手に決められない。ソースの解決も報告名もこの1つが答える。

use std::path::{Path, PathBuf};

/// 遠方環境の契約の画素段のファイル名。監視先と同じ親ディレクトリから解決する。
const 遠方環境のソースファイル名: &str = "scene_distant_environment.slang";

#[derive(Debug, Clone, Copy)]
pub(super) enum 画素段の契約 {
    定数近似,
    遠方環境,
}

impl 画素段の契約 {
    /// 失敗の報告へ載せる契約の名前。
    pub(super) const fn 名前(self) -> &'static str {
        match self {
            Self::定数近似 => "定数近似",
            Self::遠方環境 => "遠方環境",
        }
    }

    /// 一時ディレクトリへ書き出すSPIR-Vのファイル名。契約ごとに分けるのは、同じ入口名の2本が同じ名前へ上書きし合わないためである。
    pub(super) const fn 出力ファイル名(self) -> &'static str {
        match self {
            Self::定数近似 => "fragment.spv",
            Self::遠方環境 => "scene_distant_environment_fragment.spv",
        }
    }

    /// 監視先からこの契約の画素段のソースを決める。遠方環境の契約は監視先と同じ親ディレクトリの別ファイルにあるため、
    /// 監視先を`--shader-source`で移した実行でも同じディレクトリの複製を読む。
    pub(super) fn ソースパスを解決する(self, 監視先: &Path) -> Result<PathBuf, String> {
        match self {
            Self::定数近似 => Ok(監視先.to_path_buf()),
            Self::遠方環境 => {
                let 親 = 監視先
                    .parent()
                    .ok_or_else(|| format!("監視先{}に親ディレクトリが無く遠方環境の画素段を解決できない", 監視先.display()))?;
                Ok(親.join(遠方環境のソースファイル名))
            }
        }
    }
}
