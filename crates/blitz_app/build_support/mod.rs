//! ビルド時シェーダーコンパイルの入口。slangcの発見と2エントリのコンパイルを束ねる。

mod slangc_locate;
mod spirv_compile;

use std::env;
use std::path::PathBuf;

const シェーダーソース相対パス: &str = "../../shaders/triangle.slang";

pub(crate) fn シェーダーをビルドする() -> Result<(), String> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")
        .map_err(|誤り| format!("CARGO_MANIFEST_DIR環境変数が取得できない: {誤り}"))?;
    let ソース絶対パス = PathBuf::from(&manifest_dir).join(シェーダーソース相対パス);
    println!("cargo:rerun-if-changed={}", ソース絶対パス.display());

    let out_dir =
        env::var("OUT_DIR").map_err(|誤り| format!("OUT_DIR環境変数が取得できない: {誤り}"))?;
    let 出力先ディレクトリ = PathBuf::from(out_dir);

    let slangc = slangc_locate::発見する()?;
    spirv_compile::頂点とフラグメントをコンパイルする(&slangc, &ソース絶対パス, &出力先ディレクトリ)
}
