//! 参照実装の作業コピーが固定リビジョンであることを確かめる工程。受け取るのは作業コピーのパス、返すのは実測した2つのリビジョンである。
//!
//! 固定リビジョンを定数で持ち、焼くたびに`git rev-parse HEAD`の実測値と突き合わせるのは、別のリビジョンで焼いた
//! 数値に「このリビジョンで焼いた」という見出しが付くと、生成物の出自の表示がそのまま嘘になるためである。
//! 必要ファイルの存在検査では、同じ名前の別の版を取り違えたことを検出できない。

use std::path::Path;
use std::process::Command;

/// 焼き出しの対象として固定した参照実装のリビジョン。
const 参照実装のリビジョン: &str = "34f14e745cff948f4ca3157d1b62a445ffa7286f";
/// 参照実装が部分モジュールとして持つ、単位付き数値ライブラリのリビジョン。
const 部分モジュールのリビジョン: &str = "84e1e53320cd89119d31ec87cfdc2b51825b0c4b";
const 参照実装の名前: &str = "precomputed_atmospheric_scattering";
const 部分モジュールの位置: &str = "external/dimensional_types";

/// 作業コピーから読み取ったリビジョン。生成物の見出しへはこの実測値を焼く。
pub(super) struct 実測リビジョン {
    pub(super) 参照実装: String,
    pub(super) 部分モジュール: String,
}

pub(super) fn 固定リビジョンを確かめる(参照パス: &Path) -> Result<実測リビジョン, String> {
    let 参照実装 = 先頭リビジョンを読む(参照パス)?;
    照合する(参照実装の名前, &参照実装, 参照実装のリビジョン)?;
    let 部分モジュール = 先頭リビジョンを読む(&参照パス.join(部分モジュールの位置))?;
    照合する(部分モジュールの位置, &部分モジュール, 部分モジュールのリビジョン)?;
    Ok(実測リビジョン {
        参照実装, 部分モジュール
    })
}

fn 照合する(名前: &str, 実測: &str, 固定: &str) -> Result<(), String> {
    if 実測 != 固定 {
        return Err(format!(
            "{名前}のHEADは{実測}であり、焼き出しが固定している{固定}と違う。出自の表示が実際と食い違うため生成を止める"
        ));
    }
    Ok(())
}

/// 作業コピーが指している先頭のコミット、すなわちgitのHEADのリビジョンを読む。
fn 先頭リビジョンを読む(作業コピー: &Path) -> Result<String, String> {
    let 出力 = Command::new("git")
        .arg("-C")
        .arg(作業コピー)
        .args(["rev-parse", "HEAD"])
        .output()
        .map_err(|誤り| format!("{}でgitを起動できない: {誤り}", 作業コピー.display()))?;
    if !出力.status.success() {
        return Err(format!(
            "{}のHEADを読めない。gitの作業コピーであることと、部分モジュールが初期化済みであることを確かめること",
            作業コピー.display()
        ));
    }
    let 文字列 = String::from_utf8(出力.stdout).map_err(|誤り| format!("gitの出力がUTF-8でない: {誤り}"))?;
    Ok(文字列.trim().to_string())
}
