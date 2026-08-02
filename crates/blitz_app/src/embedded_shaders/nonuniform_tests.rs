//! 材質テクスチャ表を参照する添字へ、最終SPIR-VでNonUniformの装飾が付いていることの検査。
//! 原文の`NonUniformResourceIndex`は上流の最適化や書き換えで落ちうるため、埋め込み済みのSPIR-Vそのものを見る
//! (参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「段階導入」の段4bの検収ゲート(i))。
//! SPIR-Vの読み解きは`spirv_module`、参照の数え方は`nonuniform_scan`にある。どちらもこの検査だけが使う。

mod nonuniform_scan;
mod spirv_module;

use super::画素段SPIRV;
use nonuniform_scan::集計する;

/// 材質テクスチャ表の変数名。`shaders/scene.slang`の宣言が正本である。
const 材質テクスチャ表の変数名: &str = "materialTextures";

#[test]
fn 材質テクスチャ表への参照はすべて非一様な添字として装飾される() {
    let 集計 = match 集計する(画素段SPIRV, 材質テクスチャ表の変数名) {
        Ok(集計) => 集計,
        Err(誤り) => panic!("シーンの画素段のSPIR-Vを読めない: {誤り}"),
    };
    assert!(集計.参照件数 > 0, "材質テクスチャ表を参照するアクセス連鎖が1件も無い");
    assert_eq!(
        集計.非一様装飾付きの件数, 集計.参照件数,
        "材質テクスチャ表への参照{}件のうち非一様の装飾が付いたのは{}件だった",
        集計.参照件数, 集計.非一様装飾付きの件数
    );
}
