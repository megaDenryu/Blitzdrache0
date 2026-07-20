//! smoke実行前の一時コピー: shaders/scene.slangとassets/smoke/一式を
//! target/配下へ複製する。スモークシナリオが監視対象ファイルを書き換えるため、
//! リポジトリ本体は汚さない。

use std::path::PathBuf;

const アセットファイル一覧: [&str; 4] =
    ["quad.gltf", "quad.bin", "quad_base_color.png", "quad_alt_color.png"];

pub(super) fn シェーダーを一時コピーする() -> Result<PathBuf, String> {
    let コピー先ディレクトリ = PathBuf::from("target/smoke_shaders");
    std::fs::create_dir_all(&コピー先ディレクトリ)
        .map_err(|誤り| format!("コピー先ディレクトリの作成に失敗した: {誤り}"))?;

    let コピー先 = コピー先ディレクトリ.join("scene.slang");
    std::fs::copy("shaders/scene.slang", &コピー先)
        .map_err(|誤り| format!("shaders/scene.slangのコピーに失敗した: {誤り}"))?;
    Ok(コピー先)
}

pub(super) fn アセットを一時コピーする() -> Result<PathBuf, String> {
    let ルート = PathBuf::from("target/smoke_assets");
    let コピー先ディレクトリ = ルート.join("smoke");
    std::fs::create_dir_all(&コピー先ディレクトリ)
        .map_err(|誤り| format!("コピー先ディレクトリの作成に失敗した: {誤り}"))?;

    for ファイル名 in アセットファイル一覧 {
        let 元 = PathBuf::from("assets/smoke").join(ファイル名);
        let 先 = コピー先ディレクトリ.join(ファイル名);
        std::fs::copy(&元, &先)
            .map_err(|誤り| format!("{}のコピーに失敗した: {誤り}", 元.display()))?;
    }
    Ok(ルート)
}
