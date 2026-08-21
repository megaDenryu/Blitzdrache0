//! 画素段が属する照明問い合わせ契約と材質変種の組と、その組のソースファイル・一時出力名・報告名の対応。
//! 触れるのは監視先のパスだけであり、slangcの起動には関わらない。
//!
//! 組を型で持つのは、4本の画素段の入口名がどれも`fragmentMain`で同じだからである。失敗の報告に入口名しか載せないと、
//! どの組のコンパイルが落ちたのかが読み手に決められない。ソースの解決も報告名もこの1つが答える。

use std::path::{Path, PathBuf};

use crate::hot_reload::compile_error::シェーダー再コンパイルエラー;

/// 遠方環境の契約の標準金属粗さPBRの画素段のファイル名。監視先と同じ親ディレクトリから解決する。
const 遠方環境のソースファイル名: &str = "scene_distant_environment.slang";
/// 定数近似の契約の地表の層の重ね合わせの画素段のファイル名。
const 地表の層のソースファイル名: &str = "scene_surface_layer.slang";
/// 遠方環境の契約の地表の層の重ね合わせの画素段のファイル名。
const 地表の層の遠方環境のソースファイル名: &str = "scene_surface_layer_distant_environment.slang";

#[derive(Debug, Clone, Copy)]
pub(super) enum 画素段の組 {
    定数近似の標準金属粗さpbr,
    定数近似の地表の層の重ね合わせ,
    遠方環境の標準金属粗さpbr,
    遠方環境の地表の層の重ね合わせ,
}

impl 画素段の組 {
    /// 組み立ての漏れを型で塞ぐため、呼び出し元はこの並びを走査して4本すべてを作る。
    pub(super) const 全ての組: [Self; 4] = [
        Self::定数近似の標準金属粗さpbr,
        Self::定数近似の地表の層の重ね合わせ,
        Self::遠方環境の標準金属粗さpbr,
        Self::遠方環境の地表の層の重ね合わせ,
    ];

    /// 失敗の報告へ載せる組の名前。
    pub(super) const fn 名前(self) -> &'static str {
        match self {
            Self::定数近似の標準金属粗さpbr => "定数近似の標準金属粗さPBR",
            Self::定数近似の地表の層の重ね合わせ => "定数近似の地表の層の重ね合わせ",
            Self::遠方環境の標準金属粗さpbr => "遠方環境の標準金属粗さPBR",
            Self::遠方環境の地表の層の重ね合わせ => "遠方環境の地表の層の重ね合わせ",
        }
    }

    /// 一時ディレクトリへ書き出すSPIR-Vのファイル名。組ごとに分けるのは、同じ入口名の4本が同じ名前へ上書きし合わないためである。
    pub(super) const fn 出力ファイル名(self) -> &'static str {
        match self {
            Self::定数近似の標準金属粗さpbr => "fragment.spv",
            Self::定数近似の地表の層の重ね合わせ => "scene_surface_layer_fragment.spv",
            Self::遠方環境の標準金属粗さpbr => "scene_distant_environment_fragment.spv",
            Self::遠方環境の地表の層の重ね合わせ => "scene_surface_layer_distant_environment_fragment.spv",
        }
    }

    /// 監視先からこの組の画素段のソースを決める。定数近似の標準金属粗さPBR以外は監視先と同じ親ディレクトリの別ファイルにあるため、
    /// 監視先を`--shader-source`で移した実行でも同じディレクトリの複製を読む。
    pub(super) fn ソースパスを解決する(self, 監視先: &Path) -> Result<PathBuf, シェーダー再コンパイルエラー> {
        let ファイル名 = match self {
            Self::定数近似の標準金属粗さpbr => return Ok(監視先.to_path_buf()),
            Self::定数近似の地表の層の重ね合わせ => 地表の層のソースファイル名,
            Self::遠方環境の標準金属粗さpbr => 遠方環境のソースファイル名,
            Self::遠方環境の地表の層の重ね合わせ => 地表の層の遠方環境のソースファイル名,
        };
        let 親 = 監視先
            .parent()
            .ok_or_else(|| シェーダー再コンパイルエラー::監視先に親ディレクトリが無い {
                監視先: 監視先.to_path_buf(),
            })?;
        Ok(親.join(ファイル名))
    }
}
