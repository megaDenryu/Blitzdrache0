//! Graphiteの生成物の照合が失敗した理由と、その人が読む文。
//! 照合の工程から分けるのは、こちらが持つのは失敗の選択肢と `Display` の実装(CLAUDE.md「切り出しの根拠義務」の1号のトレイト実装の分離)であり、工程は失敗を作って返すだけだからである。

/// 照合が失敗した理由。
#[derive(Debug)]
pub(super) enum Graphiteの生成物の照合の失敗 {
    パッケージの置き場を読めなかった { 置き場: String, 誤り: std::io::Error },
    コマンドを起動できなかった { コマンド: &'static str, 誤り: std::io::Error },
    生成器が入っていない,
    生成物が宣言と一致しない { パッケージ: String, 終了状態: std::process::ExitStatus },
}

impl std::fmt::Display for Graphiteの生成物の照合の失敗 {
    fn fmt(&self, 書き先: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::パッケージの置き場を読めなかった { 置き場, 誤り } => write!(書き先, "{置き場} を読めなかった({誤り})"),
            Self::コマンドを起動できなかった { コマンド, 誤り } => write!(書き先, "{コマンド} を起動できなかった({誤り})"),
            Self::生成器が入っていない => write!(書き先, "Graphiteの生成器(cargo-graphite)が入っていない"),
            Self::生成物が宣言と一致しない { パッケージ, 終了状態 } => write!(
                書き先,
                "{パッケージ} の cargo graphite generate --check が失敗した({終了状態})。生成物が宣言から生成し直した結果と一致しないか、生成器が古く宣言を読めない。宣言を変えたなら、そのパッケージで cargo graphite generate を実行して生成物をコミットする"
            ),
        }
    }
}
