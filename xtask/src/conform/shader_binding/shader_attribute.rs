//! シェーダーの宣言に付いた束縛の属性を読み取る工程。受け取るのは写しのパスと宣言されている変数の名前、
//! 返すのは束縛番号とセット番号の組か、読めなかった理由である。
//!
//! 属性が宣言と同じ行に書かれる形と、宣言の1行前だけに書かれる形の両方が実在するため、どちらも対象にする。
//! 名前の照合を語の境界付きで行うのは、短い名前を探して、それを先頭に含む別の名前の宣言に当たらないためである。

use std::path::Path;

const 属性の書き出し: &str = "[[vk::binding(";

/// 1つの宣言に付いた束縛の属性。番号とセット番号を名前で持つのは、2つの数の並びを取り違えないためである。
pub(super) struct 束縛の属性 {
    pub(super) 番号: u32,
    pub(super) セット番号: u32,
}

pub(super) fn 写しの束縛の属性を読む(パス: &str, 変数名: &str) -> Result<束縛の属性, String> {
    let 内容 = std::fs::read_to_string(Path::new(パス)).map_err(|誤り| format!("{パス}の読み取りに失敗した: {誤り}"))?;
    属性を探す(&内容, 変数名).ok_or_else(|| format!("{パス}に{変数名}の宣言と束縛の属性の組が無い"))
}

/// 変数名が語として現れる最初の行を宣言とみなし、その行と、その直前の空でない行から属性を読む。
/// 見つからない場合を違反でなく失敗として扱うのは、宣言の消失を「一致した」と読み替えないためである。
fn 属性を探す(内容: &str, 変数名: &str) -> Option<束縛の属性> {
    let 行一覧: Vec<&str> = 内容.lines().collect();
    let 宣言の添字 = 行一覧.iter().position(|行| 語として含むか(行, 変数名))?;
    let 直前の行 = 行一覧[..宣言の添字].iter().rev().find(|行| !行.trim().is_empty()).copied();
    属性を読む(行一覧[宣言の添字]).or_else(|| 属性を読む(直前の行?))
}

fn 属性を読む(行: &str) -> Option<束縛の属性> {
    let 数の並びの開始 = 行.find(属性の書き出し)? + 属性の書き出し.len();
    let 数の並びの終了 = 数の並びの開始 + 行.get(数の並びの開始..)?.find(')')?;
    let mut 数の並び = 行.get(数の並びの開始..数の並びの終了)?.split(',');
    let 番号 = 数の並び.next()?.trim().parse().ok()?;
    let セット番号 = 数の並び.next()?.trim().parse().ok()?;
    Some(束縛の属性 { 番号, セット番号 })
}

/// 前後が識別子を作る文字でない出現だけを、その名前そのものの出現として数える。
fn 語として含むか(行: &str, 語: &str) -> bool {
    行.match_indices(語).any(|(位置, _)| {
        let 前の文字 = 行.get(..位置).and_then(|手前| 手前.chars().next_back());
        let 後の文字 = 行.get(位置 + 語.len()..).and_then(|残り| 残り.chars().next());
        !前の文字.is_some_and(識別子を作る文字か) && !後の文字.is_some_and(識別子を作る文字か)
    })
}

fn 識別子を作る文字か(文字: char) -> bool {
    文字.is_alphanumeric() || 文字 == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn 名前を先頭に含む別の宣言には当たらない() {
        let 内容 = "[[vk::binding(9, 1)]] SamplerState materialSamplerState;\n[[vk::binding(2, 2)]] SamplerState materialSampler;\n";
        assert_eq!(番号とセット番号を並べる(内容, "materialSampler"), Some((2, 2)));
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
}
