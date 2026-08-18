//! 1始まりの番号を一覧の添字(0始まり)へ変換する規則の正本。数字入力中の選択(display_state)と
//! パイプ実行時の番号入力(simple)の両方が同じ規則を必要とするため、実体をここへ1本化する。

/// 1始まりの番号の文字列を解析し、0始まりの添字へ変換する。数として読めない・0以下・
/// 件数以上のいずれかであれば`None`にする。
pub(super) fn 添字へ変換する(一始まりの番号の文字列: &str, 件数: usize) -> Option<usize> {
    let 一始まりの番号: usize = 一始まりの番号の文字列.parse().ok()?;
    let 位置 = 一始まりの番号.checked_sub(1)?;
    (位置 < 件数).then_some(位置)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 一始まりの番号を添字へ変換する() {
        assert_eq!(添字へ変換する("1", 10), Some(0));
        assert_eq!(添字へ変換する("10", 10), Some(9));
    }

    #[test]
    fn 零は添字にならない() {
        assert_eq!(添字へ変換する("0", 10), None);
    }

    #[test]
    fn 件数以上は範囲外になる() {
        assert_eq!(添字へ変換する("11", 10), None);
    }

    #[test]
    fn 数として読めない文字列はnoneになる() {
        assert_eq!(添字へ変換する("abc", 10), None);
    }
}
