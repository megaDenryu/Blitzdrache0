//! 引用形「X」節・下の「X」の参照から節名を読み取り、助詞形と合わせて1行分の節名を返す。
//! 検出するのは節名が明示された形だけとする。「後述」「前述」のように節名を伴わない参照は
//! 参照先を機械的に特定できないため対象外とする(偽陽性を出さないほうを優先する)。

use super::particle_reference;

const 括弧対一覧: [(char, char); 2] = [('「', '」'), ('『', '』')];
const 指示語接頭一覧: [&str; 4] = ["下の", "上の", "次の", "前の"];

pub fn 節名を抽出する(行: &str) -> Vec<String> {
    let mut 結果 = 引用形から抽出する(行);
    結果.extend(particle_reference::節名を抽出する(行));
    結果
}

fn 引用形から抽出する(行: &str) -> Vec<String> {
    let mut 結果 = Vec::new();
    for (開き, 閉じ) in 括弧対一覧 {
        let mut 走査位置 = 0;
        while let Some(相対開始) = 行[走査位置..].find(開き) {
            let 中身開始 = 走査位置 + 相対開始 + 開き.len_utf8();
            let Some(相対終了) = 行[中身開始..].find(閉じ) else { break };
            let 中身終了 = 中身開始 + 相対終了;
            if 参照とみなすか(行, 走査位置 + 相対開始, 中身終了 + 閉じ.len_utf8()) {
                結果.push(行[中身開始..中身終了].to_string());
            }
            走査位置 = 中身終了 + 閉じ.len_utf8();
        }
    }
    結果
}

/// 引用の直後が「節」であるか、直前が指示語であるときだけ同一文書内の参照とみなす。
/// 直前がバッククォートの場合は他文書のパスに続く引用であり、同一文書内の参照ではない。
fn 参照とみなすか(行: &str, 開き位置: usize, 閉じ終端: usize) -> bool {
    if 行[..開き位置].ends_with('`') {
        return false;
    }
    行[閉じ終端..].starts_with('節') || 指示語接頭一覧.iter().any(|指示語| 行[..開き位置].ends_with(指示語))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn 鉤括弧に節が続く形を拾う() {
        assert_eq!(節名を抽出する("詳細は「機械的強制」節を読む。"), vec!["機械的強制".to_string()]);
    }

    #[test]
    fn 指示語付きの引用を拾う() {
        assert_eq!(節名を抽出する("所在は下の「既知の逸脱」で管理する。"), vec!["既知の逸脱".to_string()]);
    }

    #[test]
    fn 他文書のパスに続く引用は対象外() {
        assert!(節名を抽出する("参照: `_doc/計画/開発計画.md`「定点」節").is_empty());
    }
}
