//! scene.slangの2エントリ(頂点+画素段)と、残り3本のシーンの画素段をSPIR-Vへコンパイルする。
//! 実際のslangc呼び出しは`slangc_entry_compile`に集約する。
//!
//! 頂点段を契約ごとにも材質変種ごとにも作らないのは、頂点段が照明問い合わせのセットを1つも読まず、
//! 材質変種でも変わらないためである。残り3本のパイプラインは`scene.slang`の頂点段と、それぞれのファイルの画素段を組み合わせる。

use std::path::Path;

use super::slangc_entry_compile::{エントリ一覧をコンパイルする, エントリ指定};
use super::slangc_locate::スランガー位置;

const エントリ一覧: [エントリ指定; 2] = [
    エントリ指定 {
        エントリ名: "vertexMain",
        ステージ: "vertex",
        出力ファイル名: "vertex.spv",
    },
    エントリ指定 {
        エントリ名: "fragmentMain",
        ステージ: "fragment",
        出力ファイル名: "fragment.spv",
    },
];

/// `scene.slang`以外のシーンの画素段。ソースのファイル名と焼き上がりの名前の対応をここが正本として持つ。
/// ホットリロードの側は同じ綴りを別に持つ(出力先が一時ディレクトリで別物であり、寄せると片方の意味が壊れる)。
const 残りのシーンの画素段: [(&str, &str); 3] = [
    ("scene_distant_environment.slang", "scene_distant_environment_fragment.spv"),
    ("scene_surface_layer.slang", "scene_surface_layer_fragment.spv"),
    (
        "scene_surface_layer_distant_environment.slang",
        "scene_surface_layer_distant_environment_fragment.spv",
    ),
];

/// 画素段1本ぶんのエントリ指定を作る。入口名とステージは4本とも同じであり、違うのは出力ファイル名だけである。
const fn 画素段のエントリ(出力ファイル名: &'static str) -> [エントリ指定; 1] {
    [エントリ指定 {
        エントリ名: "fragmentMain",
        ステージ: "fragment",
        出力ファイル名,
    }]
}

pub(super) fn 頂点と画素段をコンパイルする(
    slangc: &スランガー位置,
    ソース絶対パス: &Path,
    出力先ディレクトリ: &Path,
) -> Result<(), String> {
    エントリ一覧をコンパイルする(slangc, ソース絶対パス, 出力先ディレクトリ, &エントリ一覧)
}

pub(super) fn 残りのシーンの画素段をコンパイルする(
    slangc: &スランガー位置,
    シェーダーディレクトリ絶対パス: &Path,
    出力先ディレクトリ: &Path,
) -> Result<(), String> {
    for (ソースファイル名, 出力ファイル名) in 残りのシーンの画素段 {
        let ソース絶対パス = シェーダーディレクトリ絶対パス.join(ソースファイル名);
        エントリ一覧をコンパイルする(slangc, &ソース絶対パス, 出力先ディレクトリ, &画素段のエントリ(出力ファイル名))?;
    }
    Ok(())
}
