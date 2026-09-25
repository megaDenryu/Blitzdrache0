//! Graphiteの生成物の照合が失敗した理由と、その人が読む文。
//! 照合の工程から分けるのは、こちらが持つのは失敗の選択肢と `Display` の実装(CLAUDE.md「切り出しの根拠義務」の1号のトレイト実装の分離)であり、工程は失敗を作って返すだけだからである。

use std::path::PathBuf;

use super::生成器の版::生成器の版の食い違い;

/// 照合が失敗した理由。
#[derive(Debug)]
pub(super) enum Graphiteの生成物の照合の失敗 {
    ファイルを読めなかった { パス: String, 誤り: std::io::Error },
    原文を読めなかった { 破れ: String },
    cargoの置き場が分からない,
    コマンドを起動できなかった { コマンド: &'static str, 誤り: std::io::Error },
    生成物を持つパッケージが照合の対象に無い { パッケージ一覧: Vec<PathBuf> },
    固定の版が無い,
    生成器の版が固定の版と食い違う { 食い違い: 生成器の版の食い違い, 入れるコマンド: String },
    生成物が宣言と一致しない { パッケージ: String, 終了状態: std::process::ExitStatus },
}

impl std::fmt::Display for Graphiteの生成物の照合の失敗 {
    fn fmt(&self, 書き先: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ファイルを読めなかった { パス, 誤り } => write!(書き先, "{パス} を読めなかった({誤り})"),
            Self::原文を読めなかった { 破れ } => write!(書き先, "Graphiteの生成物を見分けるための原文を読めなかった({破れ})"),
            Self::cargoの置き場が分からない => write!(書き先, "cargo の置き場が分からない(環境変数 CARGO_HOME も USERPROFILE も HOME も無い)。入れた生成器の記録を読めない"),
            Self::コマンドを起動できなかった { コマンド, 誤り } => write!(書き先, "{コマンド} を起動できなかった({誤り})"),
            Self::生成物を持つパッケージが照合の対象に無い { パッケージ一覧 } => {
                let 表記一覧: Vec<String> = パッケージ一覧.iter().map(|パッケージ| パッケージ.display().to_string()).collect();
                write!(
                    書き先,
                    "Graphiteの生成物を持つパッケージが照合の対象に無い: {}。そのパッケージの Cargo.toml の graphite への依存の書き方を、依存の判定(xtask/src/graphiteの生成物の照合/graphiteへの依存.rs)が読めていない",
                    表記一覧.join(" / ")
                )
            }
            Self::固定の版が無い => write!(書き先, "Cargo.lock に git から取った graphite の固定の版が無い。生成器をどのコミットから入れるべきか決められない"),
            Self::生成器の版が固定の版と食い違う { 食い違い, 入れるコマンド } => write!(書き先, "{}。`{入れるコマンド}` で入れ直してから再実行する", 食い違いの理由(食い違い)),
            Self::生成物が宣言と一致しない { パッケージ, 終了状態 } => write!(
                書き先,
                "{パッケージ} の cargo graphite generate --check が失敗した({終了状態})。生成物が宣言から生成し直した結果と一致しない。宣言を変えたなら、そのパッケージで cargo graphite generate を実行して生成物をコミットする"
            ),
        }
    }
}

fn 食い違いの理由(食い違い: &生成器の版の食い違い) -> String {
    match 食い違い {
        生成器の版の食い違い::記録に生成器が無い => "Graphiteの生成器(cargo-graphite)が cargo install で入れられていない".to_string(),
        生成器の版の食い違い::コミットが違う { 入れた出どころ } => format!("入れた生成器の出どころ {入れた出どころ} が、Cargo.lock の graphite の固定の版と違うコミットである"),
        生成器の版の食い違い::コミットを確かめられない出どころ { 入れた出どころ } => {
            format!("入れた生成器の出どころ {入れた出どころ} はコミットを記録しておらず、Cargo.lock の graphite の固定の版と同じか確かめられない")
        }
    }
}
