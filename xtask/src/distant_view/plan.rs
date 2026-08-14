//! 引数を固定構図の対照採取・候補採取・計画表示へ写す。

use super::error::遠景構図の検収エラー;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum 実行の別 {
    対照を採る,
    候補を採る,
    計画を表示する,
}

pub(super) fn 引数を読む(引数一覧: &[String]) -> Result<実行の別, 遠景構図の検収エラー> {
    match 引数一覧 {
        [引数] if 引数 == "--capture-reference" => Ok(実行の別::対照を採る),
        [引数] if 引数 == "--capture-candidate" => Ok(実行の別::候補を採る),
        [引数] if 引数 == "--print-plan" => Ok(実行の別::計画を表示する),
        _ => Err(遠景構図の検収エラー::引数が不正(引数一覧.join(" "))),
    }
}

pub(super) fn 計画を表示する() -> String {
    "scene=terrain_fox_tour frames=360 streaming-radius=8 camera-yaw=180 camera-pitch=-10 time-of-day=43200 taa=off auto-exposure=off depth-prepass=equal color=RGBA8 depth=D32"
        .to_string()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::{実行の別, 引数を読む};

    #[test]
    fn 三つの実行の別を一つずつ受け付ける() {
        for (綴り, 期待) in [
            ("--capture-reference", 実行の別::対照を採る),
            ("--capture-candidate", 実行の別::候補を採る),
            ("--print-plan", 実行の別::計画を表示する),
        ] {
            assert_eq!(引数を読む(&[綴り.to_string()]).unwrap(), 期待);
        }
    }

    #[test]
    fn 複数の実行指定を拒む() {
        let 引数 = ["--capture-reference".to_string(), "--capture-candidate".to_string()];
        assert!(引数を読む(&引数).is_err());
    }
}
