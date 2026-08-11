//! ホットリロードが監視するシェーダーの入口ファイル。担当するのは、`--shader-source`が指す1つの`.slang`ファイルを
//! 値として保持し、その中身の読み書きを自分のメソッドで行うことである。
//!
//! 裸の`PathBuf`で持ち回らないのは、入口ファイル・実行時アセットの置き場・フレームダンプ先が同じ`PathBuf`の姿を
//! しており、取り違えを型が止められないためである。`import`で参照される他の`.slang`はこの入口と同じディレクトリから
//! 解決されるため、監視の対象はこのファイルではなくこのファイルの属するディレクトリになる。

use std::path::{Path, PathBuf};

/// `--shader-source`が指すシェーダーの入口ファイル。
#[derive(Debug, Clone)]
pub(crate) struct 監視するシェーダーの入口ファイル {
    パス: PathBuf,
}

impl 監視するシェーダーの入口ファイル {
    pub(crate) fn 綴りから生成する(綴り: &str) -> Self {
        Self {
            パス: PathBuf::from(綴り)
        }
    }

    /// 入口ファイルの本文を読む。ファイルが無い起動(監視無効)もあるため、失敗は呼び出し側が扱う。
    pub(crate) fn 本文を読む(&self) -> std::io::Result<String> {
        std::fs::read_to_string(&self.パス)
    }

    /// 入口ファイルの本文を差し替える。
    pub(crate) fn 本文を書き出す(&self, 本文: &str) -> std::io::Result<()> {
        std::fs::write(&self.パス, 本文)
    }

    /// 監視の構築だけがこの姿で受け取る。監視は入口の属するディレクトリの全`.slang`を走査するため、
    /// ファイルの中身でなくパスそのものを要る。
    pub(crate) fn パス(&self) -> &Path {
        &self.パス
    }
}
