//! Foxのスキンとアニメーションを版1の実行時形式で決定的に往復する境界テスト。

use std::path::PathBuf;

use blitz_engine::{
    アセットID, アセット実行時形式エラー, カタログ, シーンを実行時形式へ格納する, シーンを読み込む, 実行時形式からシーンを読む
};

fn リポジトリルートからのパス(相対パス: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..").join(相対パス)
}

#[test]
fn foxを決定的な実行時形式で往復する() {
    let id = match アセットID::生成する("fox") {
        Ok(id) => id,
        Err(誤り) => panic!("試験用IDの生成に失敗した: {誤り}"),
    };
    let mut カタログ = カタログ::空を作る();
    カタログ.登録する(id.clone(), リポジトリルートからのパス("assets/samples/Fox/Fox.glb"));
    let mut 期待 = match シーンを読み込む(&カタログ, &id) {
        Ok(シーン) => シーン,
        Err(誤り) => panic!("Fox.glbの読込に失敗した: {誤り}"),
    };
    期待.参照ファイル一覧.clear();
    let 一回目 = match シーンを実行時形式へ格納する(&期待) {
        Ok(バイト列) => バイト列,
        Err(誤り) => panic!("Foxを実行時形式へ格納できなかった: {誤り}"),
    };
    let 二回目 = match シーンを実行時形式へ格納する(&期待) {
        Ok(バイト列) => バイト列,
        Err(誤り) => panic!("Foxを2回目の実行時形式へ格納できなかった: {誤り}"),
    };
    assert_eq!(一回目, 二回目);
    assert_eq!(実行時形式からシーンを読む(&一回目), Ok(期待.clone()));

    let mut 親順不正 = 期待.clone();
    let Some(スキン) = 親順不正.スキン.as_mut() else {
        panic!("Foxにはスキンがあるはず");
    };
    スキン.ジョイント一覧[1].親添字 = Some(1);
    assert!(matches!(
        シーンを実行時形式へ格納する(&親順不正),
        Err(アセット実行時形式エラー::親添字順序違反 { .. })
    ));
}
