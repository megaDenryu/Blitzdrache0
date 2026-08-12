//! 束縛の属性の読み取り規則そのものの試験。属性が宣言と同じ行にある形・属性だけの1行前にある形・
//! 1行前が別の宣言である形・語の境界・読めない形の5系統を見る。
//!
//! 読み取りの実装と分けるのは、規則を足すたびに試験が増え、実装の側の見通しを試験が押し流すためである。

use super::属性を探す;

/// 読み取った2つの数を書いた順に並べて照合する。取り違えがあれば期待値と合わなくなる。
fn 番号とセット番号を並べる(内容: &str, 変数名: &str) -> Option<(u32, u32)> {
    属性を探す(内容, 変数名).map(|属性| (属性.番号, 属性.セット番号))
}

#[test]
fn 同じ行に属性がある宣言から番号とセット番号を読む() {
    let 内容 = "[[vk::binding(2, 3)]] StructuredBuffer<DirectionalLightRecord> directionalLightRecords;\n";
    assert_eq!(番号とセット番号を並べる(内容, "directionalLightRecords"), Some((2, 3)));
}

#[test]
fn 属性が1行前にある宣言からも読む() {
    let 内容 = "[[vk::binding(0, 2)]]\nStructuredBuffer<MaterialRecord> materialRecords;\n";
    assert_eq!(番号とセット番号を並べる(内容, "materialRecords"), Some((0, 2)));
}

#[test]
fn 属性が空行をまたいだ1行前にあっても読む() {
    let 内容 = "[[vk::binding(1, 0)]]\n\nConstantBuffer<CascadeShadowUniform> cascadeShadowUniform;\n";
    assert_eq!(番号とセット番号を並べる(内容, "cascadeShadowUniform"), Some((1, 0)));
}

#[test]
fn 属性が2つ並ぶだけの行からも読む() {
    let 内容 = "[[vk::binding(5, 3)]] [[vk::combinedImageSampler]]\nSamplerState specularPrefilterSampler;\n";
    assert_eq!(番号とセット番号を並べる(内容, "specularPrefilterSampler"), Some((5, 3)));
}

#[test]
fn 名前を先頭に含む別の宣言には当たらない() {
    let 内容 = "[[vk::binding(9, 1)]] SamplerState materialSamplerState;\n[[vk::binding(2, 2)]] SamplerState materialSampler;\n";
    assert_eq!(番号とセット番号を並べる(内容, "materialSampler"), Some((2, 2)));
}

#[test]
fn 直前の行が別の宣言なら属性を借りない() {
    let 内容 = "[[vk::binding(4, 3)]] TextureCube diffuseIrradianceTexture;\nSamplerState diffuseIrradianceSampler;\n";
    assert_eq!(番号とセット番号を並べる(内容, "diffuseIrradianceSampler"), None);
}

/// サンプラー付きの画像は同じ番号を2つの宣言が共有する。後者から属性が消えたとき、前者の番号を借りて
/// 期待値と一致してしまうと、宣言の消失を検査が見逃す。
#[test]
fn 同じ番号を共有する対の後者から属性が消えたら読めない() {
    let 内容 = "[[vk::binding(10, 3)]] [[vk::combinedImageSampler]] TextureCubeArray pointLightShadowTexture;\nSamplerComparisonState pointLightShadowSampler;\n";
    assert_eq!(番号とセット番号を並べる(内容, "pointLightShadowSampler"), None);
}

#[test]
fn 変数名の宣言が無ければ読めない() {
    let 内容 = "[[vk::binding(0, 3)]] Texture2D other;\n";
    assert_eq!(番号とセット番号を並べる(内容, "materialSampler"), None);
}

#[test]
fn 宣言はあっても属性が無ければ読めない() {
    let 内容 = "import lighting_query;\n\nTexture2D materialSampler;\n";
    assert_eq!(番号とセット番号を並べる(内容, "materialSampler"), None);
}
