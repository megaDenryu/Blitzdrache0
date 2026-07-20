//! Khronos標準サンプルFox.glb(判断42)の往復テスト: スキン・アニメーションが実際に
//! 読み込め、ジョイント添字の不変条件(トポロジカル順・JOINTS_0の再解決)が成立することを
//! 確認する。
//!
//! 注意: `cargo test`のテストバイナリはパッケージディレクトリを作業ディレクトリとして
//! 実行される（リポジトリルートではない）ため、`CARGO_MANIFEST_DIR`からの相対パスで
//! リポジトリルート直下のassets/を参照する。`cargo xtask fetch-assets`で取得済みが前提。

use std::path::PathBuf;

use blitz_engine::{アセットID, カタログ, シーンを読み込む};

fn リポジトリルートからのパス(相対パス: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(相対パス)
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

// 注意: assets/samples/Fox/Fox.glbの唯一のプリミティブはPOSITION/TEXCOORD_0/
// JOINTS_0/WEIGHTS_0のみを持ちNORMAL/TANGENTが無い。判断46の計算充填(面積重み
// スムース法線・法線に直交する任意接線)により読込が成立することも併せて確認する。
#[test]
fn foxのスキンとアニメーションを読み込める() {
    let (カタログ, id) = 試験用カタログ();

    let シーン = match シーンを読み込む(&カタログ, &id) {
        Ok(シーン) => シーン,
        Err(誤り) => panic!(
            "assets/samples/Fox/Fox.glbの読込に失敗した(cargo xtask fetch-assetsで取得済みか確認): {誤り}"
        ),
    };

    let スキン = match &シーン.スキン {
        Some(スキン) => スキン,
        None => panic!("Fox.glbにはスキンがあるはず"),
    };
    let ジョイント数 = スキン.ジョイント一覧.len();
    assert!(ジョイント数 > 1, "Foxのスキンは複数ジョイントを持つはず: {ジョイント数}");

    // 不変条件: トポロジカル順(親添字 < 自分の添字)。ルート(親添字None)が1つ以上ある。
    let mut ルート数 = 0;
    for (添字, ジョイント) in スキン.ジョイント一覧.iter().enumerate() {
        match ジョイント.親添字 {
            Some(親) => assert!(親 < 添字, "親添字{親}は自分の添字{添字}より小さいはず"),
            None => ルート数 += 1,
        }
    }
    assert!(ルート数 >= 1, "トポロジカル順のスキンにはルートが1つ以上あるはず");

    // JOINTS_0がトポロジカル順への再解決後もジョイント数の範囲に収まる。
    let スキン頂点属性一覧 = match &シーン.メッシュ.スキン頂点属性一覧 {
        Some(一覧) => 一覧,
        None => panic!("スキン付きメッシュはスキン頂点属性を持つはず"),
    };
    assert_eq!(スキン頂点属性一覧.len(), シーン.メッシュ.頂点一覧.len());
    for 属性 in スキン頂点属性一覧 {
        for &添字 in &属性.ジョイント {
            assert!(
                usize::from(添字) < ジョイント数,
                "JOINTS_0の再解決後添字{添字}はジョイント数{ジョイント数}未満のはず"
            );
        }
    }

    // Foxの標準サンプルは3本のアニメーション(Survey/Walk/Run)を持つ。
    assert_eq!(シーン.アニメーション一覧.len(), 3, "Foxは3本のアニメーションを持つはず");
    for クリップ in &シーン.アニメーション一覧 {
        assert_eq!(
            クリップ.ジョイントチャンネル一覧.len(),
            ジョイント数,
            "クリップ「{}」のジョイントチャンネル数はスキンのジョイント数と一致するはず",
            クリップ.名前
        );
        assert!(クリップ.継続秒.値() > 0.0, "クリップ「{}」の継続秒は正のはず", クリップ.名前);
    }
}
