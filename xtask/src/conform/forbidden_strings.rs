//! 禁止文字列検査: 変更履歴の書き捨てを誘発する語、切り出しの経緯だけを述べる語、絵文字を検出する。
//! 注意: 検査対象の綴りそのものをこのファイルに連続して書くと、conformが
//! 自分自身を違反として検出してしまう。分割リテラルの連結で回避する。

use std::path::Path;

use super::split_debt;
use super::violation::違反;

const 禁止語一覧: [&str; 3] = [concat!("TO", "DO"), concat!("FIX", "ME"), concat!("HA", "CK")];

/// そのファイルが何を担当するかではなく分割の経緯だけを述べる語。
/// 参照: CLAUDE.md「切り出しの根拠義務(パーシャル規約)」。
const 経緯語一覧: [&str; 3] = [concat!("行数", "分割"), concat!("行数", "のため"), concat!("行数制限", "のため")];

pub fn 行に禁止語を含むか(行: &str) -> Option<&'static str> {
    禁止語一覧.iter().find(|語| 行.contains(*語)).copied()
}

pub fn 行に経緯語を含むか(行: &str) -> Option<&'static str> {
    経緯語一覧.iter().find(|語| 行.contains(*語)).copied()
}

pub fn コードポイントが絵文字か(コードポイント: u32) -> bool {
    (0x1F300..=0x1FAFF).contains(&コードポイント) || (0x2600..=0x27BF).contains(&コードポイント)
}

pub fn 行に絵文字を含むか(行: &str) -> bool {
    行.chars().any(|文字| コードポイントが絵文字か(u32::from(文字)))
}

pub fn 検査する(パス: &Path, 内容: &str) -> Vec<違反> {
    let mut 違反一覧 = Vec::new();
    let 未是正既知 = split_debt::既知の未是正ファイルか(パス);
    let mut 経緯語を見つけた = false;
    for (行番号, 行) in 内容.lines().enumerate() {
        if let Some(禁止語) = 行に禁止語を含むか(行) {
            違反一覧.push(違反::行単位(パス.to_path_buf(), 行番号 + 1, format!("禁止語({禁止語})")));
        }
        if let Some(語) = 行に経緯語を含むか(行) {
            経緯語を見つけた = true;
            if !未是正既知 {
                違反一覧.push(違反::行単位(
                    パス.to_path_buf(),
                    行番号 + 1,
                    format!("切り出しの経緯だけを述べる語({語})"),
                ));
            }
        }
        if 行に絵文字を含むか(行) {
            違反一覧.push(違反::行単位(パス.to_path_buf(), 行番号 + 1, "絵文字".to_string()));
        }
    }
    違反一覧.extend(split_debt::台帳の陳腐化を検査する(パス, 未是正既知, 経緯語を見つけた));
    違反一覧
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn 禁止語を検出する() {
        assert_eq!(行に禁止語を含むか(concat!("// ", "TO", "DO: あとで直す")), Some(禁止語一覧[0]));
        assert_eq!(行に禁止語を含むか("普通のコメント"), None);
    }

    #[test]
    fn 小文字は対象外() {
        assert_eq!(行に禁止語を含むか(concat!("to", "do")), None);
    }

    #[test]
    fn 経緯語を検出する() {
        assert_eq!(行に経緯語を含むか(concat!("//! ", "行数", "分割のための切り出し")), Some(経緯語一覧[0]));
        assert_eq!(行に経緯語を含むか(concat!("//! ", "行数", "のために切り出した")), Some(経緯語一覧[1]));
        assert_eq!(行に経緯語を含むか("//! 触れるフィールドは布とdeviceに限る"), None);
    }

    #[test]
    fn 台帳にないファイルの経緯語は違反になる() {
        let 違反一覧 = 検査する(
            Path::new("crates/blitz_render/src/新規.rs"),
            concat!("//! ", "行数", "分割のための切り出し\n"),
        );
        assert_eq!(違反一覧.len(), 1);
    }

    #[test]
    fn 絵文字コードポイントを判定する() {
        assert!(コードポイントが絵文字か(0x1F600));
        assert!(コードポイントが絵文字か(0x2708));
        assert!(!コードポイントが絵文字か(0x0041));
    }
}
