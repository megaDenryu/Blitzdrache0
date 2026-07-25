//! Fox.glb(判断46)の計算充填した法線・接線が単位長・直交という基本性質を満たすことを
//! 確認する。Fox.glbの唯一のプリミティブはPOSITION/TEXCOORD_0/JOINTS_0/WEIGHTS_0のみを
//! 持ちNORMAL/TANGENTが無いため、全頂点が計算充填の対象になる。
//!
//! 注意: `cargo test`のテストバイナリはパッケージディレクトリを作業ディレクトリとして
//! 実行される（リポジトリルートではない）ため、`CARGO_MANIFEST_DIR`からの相対パスで
//! リポジトリルート直下のassets/を参照する。`cargo xtask fetch-assets`で取得済みが前提。

use std::path::PathBuf;

use blitz_asset_compiler::ソースシーンを読み込む;
use blitz_engine::{アセットID, カタログ, チャンク座標};

fn リポジトリルートからのパス(相対パス: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..").join(相対パス)
}

fn 試験用カタログ() -> (カタログ, アセットID) {
    let id = match アセットID::生成する("fox") {
        Ok(id) => id,
        Err(誤り) => panic!("試験用IDの生成に失敗した: {誤り}"),
    };
    let mut カタログ = カタログ::空を作る();
    カタログ.登録する(id.clone(), リポジトリルートからのパス("assets/samples/Fox/Fox.glb"));
    (カタログ, id)
}

fn 内積(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[test]
fn foxの計算充填法線と接線は単位長かつ直交する() {
    let (カタログ, id) = 試験用カタログ();
    let シーン = match ソースシーンを読み込む(&カタログ, &id, チャンク座標::生成する(0, 0)) {
        Ok(シーン) => シーン,
        Err(誤り) => panic!("assets/samples/Fox/Fox.glbの読込に失敗した(cargo xtask fetch-assetsで取得済みか確認): {誤り}"),
    };

    for 頂点 in &シーン.先頭の描画対象().メッシュ().頂点一覧 {
        let 法線長さ二乗 = 内積(頂点.法線, 頂点.法線);
        assert!(
            (法線長さ二乗 - 1.0).abs() < 1e-3,
            "計算充填した法線は単位長のはず: {:?}(長さ二乗={法線長さ二乗})",
            頂点.法線
        );

        let 接線ベクトル = [頂点.接線[0], 頂点.接線[1], 頂点.接線[2]];
        let 接線長さ二乗 = 内積(接線ベクトル, 接線ベクトル);
        assert!(
            (接線長さ二乗 - 1.0).abs() < 1e-3,
            "計算充填した接線は単位長のはず: {接線ベクトル:?}(長さ二乗={接線長さ二乗})"
        );
        assert!(
            内積(頂点.法線, 接線ベクトル).abs() < 1e-3,
            "計算充填した接線は法線と直交するはず: 法線={:?} 接線={接線ベクトル:?}",
            頂点.法線
        );
    }
}
