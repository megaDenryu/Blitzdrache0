//! 生成器→ローダの往復テスト: `cargo xtask gen-smoke-asset`で生成済みの
//! assets/smoke/quad.gltf を実際に読み込み、形状・法線・ベースカラーを検証する。
//!
//! 注意: `cargo test`のテストバイナリはパッケージディレクトリを作業ディレクトリとして
//! 実行される（リポジトリルートではない）ため、`CARGO_MANIFEST_DIR`からの相対パスで
//! リポジトリルート直下のassets/を参照する。

use std::path::PathBuf;

use blitz_engine::{アセットID, カタログ, シーンを読み込む};

fn リポジトリルートからのパス(相対パス: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(相対パス)
}

fn 試験用カタログ() -> (カタログ, アセットID) {
    let id = match アセットID::生成する("quad") {
        Ok(id) => id,
        Err(誤り) => panic!("試験用IDの生成に失敗した: {誤り}"),
    };
    let mut カタログ = カタログ::空を作る();
    カタログ.登録する(id.clone(), リポジトリルートからのパス("assets/smoke/quad.gltf"));
    (カタログ, id)
}

#[test]
fn 極小四角形アセットを読み込める() {
    let (カタログ, id) = 試験用カタログ();

    let シーン = match シーンを読み込む(&カタログ, &id) {
        Ok(シーン) => シーン,
        Err(誤り) => panic!(
            "assets/smoke/quad.gltfの読込に失敗した(cargo xtask gen-smoke-assetで生成済みか確認): {誤り}"
        ),
    };

    assert_eq!(シーン.メッシュ.頂点一覧.len(), 4);
    assert_eq!(シーン.メッシュ.インデックス一覧.len(), 6);

    for 頂点 in &シーン.メッシュ.頂点一覧 {
        assert_eq!(頂点.法線, [0.0, 0.0, 1.0]);
        assert_eq!(頂点.接線, [1.0, 0.0, 0.0, 1.0]);
    }

    let ベースカラー = match シーン.マテリアル.ベースカラー {
        Some(テクスチャ) => テクスチャ,
        None => panic!("quad.gltfにはbaseColorテクスチャが設定されているはず"),
    };
    assert_eq!(ベースカラー.幅, 4);
    assert_eq!(ベースカラー.高さ, 4);

    let 期待ピクセル = [0u8, 0, 255, 255];
    for ピクセル in ベースカラー.rgba8.chunks_exact(4) {
        assert_eq!(ピクセル, 期待ピクセル);
    }

    assert!(
        シーン.参照ファイル一覧.len() >= 2,
        "参照ファイル一覧は主ファイル・外部バッファ・外部画像を含み2件以上のはず: {:?}",
        シーン.参照ファイル一覧
    );
}
