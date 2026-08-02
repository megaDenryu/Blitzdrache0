//! シーンの差し替えの経路がGPUの全作業完了待ちを使っていないことの検査。受け取るのは無し(対象のファイルと語がここにある)、
//! 返すのは待ちを呼んでいる行の違反一覧である。
//!
//! 材質は完全に構築した資源表世代を公開してから旧世代をフェンス通過後に退役させ、ジオメトリは束の解除予約と同じ
//! 破棄待ちの規律で解放するため、差し替えで進行中のフレームを止める必要がない。待ちを1行足すだけで元へ戻ってしまい、
//! 絵は同じに見えるため、不在を機械的に守る
//! (参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「段階導入」の段4bの検収ゲート(v))。

use std::path::{Path, PathBuf};

use super::violation::違反;

const 対象ファイル: &str = "crates/blitz_render/src/renderer/replace_scene.rs";

/// 禁じる呼び出し。エンジンの語彙とashの関数名の両方を見る。
const 待ちの語一覧: [&str; 2] = ["gpuの全作業完了を待つ()", "device_wait_idle"];

pub fn 検査する() -> Result<Vec<違反>, String> {
    let 内容 = std::fs::read_to_string(対象ファイル).map_err(|誤り| format!("{対象ファイル}の読み取りに失敗した: {誤り}"))?;
    Ok(ファイル1つを検査する(Path::new(対象ファイル), &内容))
}

fn ファイル1つを検査する(パス: &Path, 内容: &str) -> Vec<違反> {
    let mut 違反一覧 = Vec::new();
    for (行番号, 行) in 内容.lines().enumerate() {
        // コメント行を外すのは、なぜ待たないのかをその場に書いた説明が違反にならないようにするためである。
        if 行.trim_start().starts_with("//") {
            continue;
        }
        for 語 in 待ちの語一覧 {
            if 行.contains(語) {
                違反一覧.push(違反::行単位(
                    PathBuf::from(パス),
                    行番号 + 1,
                    format!("シーンの差し替えがGPUの全作業完了待ち({語})を呼んでいる"),
                ));
            }
        }
    }
    違反一覧
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 全作業完了待ちの呼び出しを違反にする() {
        let 違反一覧 = ファイル1つを検査する(Path::new(対象ファイル), "        self.環境.gpuの全作業完了を待つ()?;\n");
        assert_eq!(違反一覧.len(), 1);
    }

    #[test]
    fn 束の解除と追加への委譲は違反にしない() {
        let 違反一覧 = ファイル1つを検査する(Path::new(対象ファイル), "        self.描画束を解除する(起動シーンの束ID);\n");
        assert!(違反一覧.is_empty());
    }
}
