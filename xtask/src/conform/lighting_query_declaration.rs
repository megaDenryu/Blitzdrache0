//! 照明問い合わせのセット(set3)の宣言が正本に閉じていることの検査。受け取るのは無し(対象の表がここにある)、
//! 返すのは自前でset3を宣言しているシェーダーと、正本の宣言が欠けた場合の違反一覧である。
//!
//! シーン描画と布描画が同じ光を別々の宣言で読むと、片方だけがレコードの並びや件数の読み方を変えても
//! コンパイルは通り、2つの絵が食い違う形でしか現れない。正本を1つに保つことを機械的に見る
//! (参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「段階導入」の段5の検収ゲート(iii))。
//!
//! 正本が2つあるのは、契約の枝ごとに宣言する番号が違うためである。直接光と影の4番号はどちらの契約も宣言し、
//! 遠方環境の3番号は遠方環境の契約だけが宣言する。定数近似の契約へ未使用のダミー束縛を強制しないための分け方であり、
//! それぞれの正本が自分の番号を全部持っていることを別々に確かめる。

mod table;

use std::path::{Path, PathBuf};

use super::violation::違反;
use table::{取り込む側一覧, 宣言の正本, 正本一覧};

pub fn 全シェーダーを検査する() -> Result<Vec<違反>, String> {
    let mut 違反一覧 = Vec::new();
    for 正本 in &正本一覧 {
        違反一覧.extend(正本の宣言を確かめる(正本)?);
    }
    for パス in 取り込む側一覧 {
        let 内容 = std::fs::read_to_string(Path::new(パス)).map_err(|誤り| format!("{パス}の読み取りに失敗した: {誤り}"))?;
        違反一覧.extend(自前の宣言を探す(パス, &内容));
    }
    Ok(違反一覧)
}

/// 宣言の消失を「一致した」と読み替えないため、正本に受け持つ番号が揃っていることを先に確かめる。
fn 正本の宣言を確かめる(正本: &宣言の正本) -> Result<Vec<違反>, String> {
    let パス = 正本.パス;
    let 内容 = std::fs::read_to_string(Path::new(パス)).map_err(|誤り| format!("{パス}の読み取りに失敗した: {誤り}"))?;
    let mut 違反一覧 = Vec::new();
    for 番号 in 正本.バインディング番号一覧 {
        if !内容.contains(&宣言の書き出し(*番号)) {
            違反一覧.push(違反::ファイル単位(
                PathBuf::from(パス),
                format!("照明問い合わせのセットのbinding{番号}の宣言が正本から消えている"),
            ));
        }
    }
    Ok(違反一覧)
}

fn 自前の宣言を探す(パス: &str, 内容: &str) -> Vec<違反> {
    let mut 違反一覧 = Vec::new();
    for 正本 in &正本一覧 {
        for 番号 in 正本.バインディング番号一覧 {
            let 宣言 = 宣言の書き出し(*番号);
            let 正本パス = 正本.パス;
            for (行番号, _) in 内容.lines().enumerate().filter(|(_, 行)| 行.trim_start().starts_with(&宣言)) {
                違反一覧.push(違反::行単位(
                    PathBuf::from(パス),
                    行番号 + 1,
                    format!("照明問い合わせのセット(binding{番号})を自前で宣言している。{正本パス}をimportする"),
                ));
            }
        }
    }
    違反一覧
}

fn 宣言の書き出し(番号: u32) -> String {
    format!("[[vk::binding({番号}, 3)]]")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 自前でset3を宣言した行を違反にする() {
        let 内容 = "[[vk::binding(2, 3)]] StructuredBuffer<DirectionalLightRecord> myRecords;\n";
        assert_eq!(自前の宣言を探す("shaders/scene.slang", 内容).len(), 1);
    }

    #[test]
    fn 遠方環境の番号を自前で宣言した行も違反にする() {
        let 内容 = "[[vk::binding(5, 3)]] TextureCube myPrefilter;\n";
        assert_eq!(自前の宣言を探す("shaders/scene.slang", 内容).len(), 1);
    }

    #[test]
    fn 取り込むだけの行は違反にしない() {
        assert!(自前の宣言を探す("shaders/scene.slang", "import lighting_query;\n").is_empty());
    }
}
