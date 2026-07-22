//! ブルームピラミッド(判断41)のシェーダーをSPIR-Vへコンパイルする。
//! bloom_down.slang(全画面頂点+前処理+縮小)とbloom_up.slang(全画面頂点+拡大)の2ファイル。
//! ポストプロセスの有効/無効は実行時のCLIで切り替わるため、常時ビルドする。

use std::path::Path;

use super::slangc_entry_compile::{エントリ一覧をコンパイルする, エントリ指定};
use super::slangc_locate::スランガー位置;

const 縮小側エントリ一覧: [エントリ指定; 3] = [
    エントリ指定 {
        エントリ名: "vertexMain",
        ステージ: "vertex",
        出力ファイル名: "bloom_down_vertex.spv",
    },
    エントリ指定 {
        エントリ名: "prefilterMain",
        ステージ: "fragment",
        出力ファイル名: "bloom_prefilter.spv",
    },
    エントリ指定 {
        エントリ名: "downsampleMain",
        ステージ: "fragment",
        出力ファイル名: "bloom_downsample.spv",
    },
];

const 拡大側エントリ一覧: [エントリ指定; 2] = [
    エントリ指定 {
        エントリ名: "vertexMain",
        ステージ: "vertex",
        出力ファイル名: "bloom_up_vertex.spv",
    },
    エントリ指定 {
        エントリ名: "upsampleMain",
        ステージ: "fragment",
        出力ファイル名: "bloom_upsample.spv",
    },
];

pub(super) fn 縮小側をコンパイルする(
    slangc: &スランガー位置, ソース絶対パス: &Path, 出力先ディレクトリ: &Path
) -> Result<(), String> {
    エントリ一覧をコンパイルする(slangc, ソース絶対パス, 出力先ディレクトリ, &縮小側エントリ一覧)
}

pub(super) fn 拡大側をコンパイルする(
    slangc: &スランガー位置, ソース絶対パス: &Path, 出力先ディレクトリ: &Path
) -> Result<(), String> {
    エントリ一覧をコンパイルする(slangc, ソース絶対パス, 出力先ディレクトリ, &拡大側エントリ一覧)
}
