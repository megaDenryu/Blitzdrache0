//! 大気の期待値の焼き出しが返す型付きエラー。破れうる前提を枝で数え上げ、どの前提が破れたかを型で言う。
//!
//! xtaskは外部のクレートへ依存しないため、thiserrorを使わず手書きのenumと`Display`で書く。
//! 様式は`acceptance/error.rs`に倣う。
//!
//! この工程は検収の器を1つも通らないため、検収エラーの枝を持たない。破れるのは出自の照合と外部の道具の起動と
//! ファイルの書き出しの3種であり、直す側が見る場所が枝ごとに違う。人が読む1文へ写すのは`display`が持つ。

mod display;

use std::path::PathBuf;

#[derive(Debug)]
pub(super) enum 大気の期待値の焼き出しエラー {
    固定したリビジョンと食い違う {
        名前: &'static str,
        実測: String,
        固定: String,
    },
    作業コピーに未コミットの変更がある {
        名前: &'static str,
        変更一覧: String,
    },
    作業コピーでgitを起こせなかった {
        作業コピー: PathBuf,
        誤り: std::io::Error,
    },
    作業コピーでgitが失敗して終わった {
        作業コピー: PathBuf,
        引数の並び: String,
    },
    作業コピーの照合の標準出力がUTF8でない {
        誤り: std::string::FromUtf8Error,
    },
    参照実装の必要ファイルが無い {
        パス: PathBuf,
    },
    出力先を作れなかった {
        パス: PathBuf,
        誤り: std::io::Error,
    },
    現在のディレクトリを読めなかった {
        誤り: std::io::Error,
    },
    構築手順を起こせなかった {
        構築手順: PathBuf,
        誤り: std::io::Error,
    },
    構築手順が失敗して終わった {
        構築手順: PathBuf,
    },
    焼き出しを起こせなかった {
        実行ファイル: PathBuf,
        誤り: std::io::Error,
    },
    焼き出しが異常終了した {
        実行ファイル: PathBuf,
    },
    焼き出しの出力がUTF8でない {
        誤り: std::string::FromUtf8Error,
    },
    /// 1行も出さない焼き出しは、参照実装が何も計算していないか出力先が変わっている。
    焼き出しが1行も出さなかった,
    生成物を書けなかった {
        パス: PathBuf,
        誤り: std::io::Error,
    },
}

impl std::error::Error for 大気の期待値の焼き出しエラー {}
