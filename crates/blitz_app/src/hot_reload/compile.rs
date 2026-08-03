//! ホットリロード時の再コンパイル。一時ディレクトリへ共有頂点段1本と契約ごとの画素段2本を書き出して読み戻す。
//!
//! 3本すべてが成功したときにだけ束を返すのは、片方の契約だけが新しい混成の束を作らないためである。
//! 途中で失敗すると呼び出し元は差し替えを行わず、旧いパイプラインがそのまま描き続ける。
//! どの契約がどのソースから来るかは`fragment_contract`が答える。

mod fragment_contract;

use std::path::Path;
use std::process::Command;

use blitz_render::indirect_lighting::契約別の描画シェーダー;

use super::slangc::{self, スランガー位置};
use fragment_contract::画素段の契約;

pub(super) fn 契約別の描画シェーダーをコンパイルする(監視先: &Path) -> Result<契約別の描画シェーダー, String> {
    let slangc = slangc::発見する()?;
    let 頂点spirv = エントリを1つコンパイルする(&slangc, 監視先, "vertexMain", "vertex", "vertex.spv")
        .map_err(|誤り| format!("共有頂点段({})のコンパイルに失敗した: {誤り}", 監視先.display()))?;
    let 定数近似の画素段 = 画素段をコンパイルする(&slangc, 画素段の契約::定数近似, 監視先)?;
    let 遠方環境の画素段 = 画素段をコンパイルする(&slangc, 画素段の契約::遠方環境, 監視先)?;
    契約別の描画シェーダー::生成する(頂点spirv, 定数近似の画素段, 遠方環境の画素段).map_err(|誤り| 誤り.to_string())
}

/// 契約名とソースパスを失敗へ載せるのは、両方の画素段の入口名が`fragmentMain`で同じであり、
/// slangcの出す文面だけではどちらの契約が落ちたのか読み手に決められないためである。
fn 画素段をコンパイルする(slangc: &スランガー位置, 契約: 画素段の契約, 監視先: &Path) -> Result<Vec<u8>, String> {
    let ソースパス = 契約.ソースパスを解決する(監視先)?;
    エントリを1つコンパイルする(slangc, &ソースパス, "fragmentMain", "fragment", 契約.出力ファイル名())
        .map_err(|誤り| format!("{}の契約の画素段({})のコンパイルに失敗した: {誤り}", 契約.名前(), ソースパス.display()))
}

fn エントリを1つコンパイルする(
    slangc: &スランガー位置,
    ソースパス: &Path,
    エントリ名: &str,
    ステージ: &str,
    出力ファイル名: &str,
) -> Result<Vec<u8>, String> {
    let 出力パス = std::env::temp_dir().join("blitzdrache0_hot_reload").join(出力ファイル名);
    if let Some(親) = 出力パス.parent() {
        std::fs::create_dir_all(親).map_err(|誤り| format!("一時ディレクトリの作成に失敗した: {誤り}"))?;
    }

    let 実行結果 = Command::new(slangc.プログラム名())
        .arg(ソースパス)
        .args(["-entry", エントリ名])
        .args(["-stage", ステージ])
        .args(["-target", "spirv"])
        .arg("-fvk-use-entrypoint-name")
        .arg("-o")
        .arg(&出力パス)
        .output()
        .map_err(|起動誤り| format!("slangcの起動に失敗した: {起動誤り}"))?;

    if !実行結果.status.success() {
        let stderr = String::from_utf8_lossy(&実行結果.stderr);
        return Err(format!("slangcが{エントリ名}のコンパイルに失敗した:\n{stderr}"));
    }

    std::fs::read(&出力パス).map_err(|誤り| format!("コンパイル結果の読み込みに失敗した: {誤り}"))
}
