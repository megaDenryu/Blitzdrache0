//! 1回の実行で振る軸そのものと、その綴り。担当するのは軸の一覧と、綴りと軸の相互の変換である。
//!
//! 綴りを網羅的な`match`で持つのは、軸を1つ足したときに綴りの追加をコンパイラへ強制させるためである。
//! 選べる軸の一覧だけは配列で持つため、足した軸を`全軸`へ入れ忘れると選べないままになる。
//! この抜けは下のテストが綴りの往復で捕まえる。

/// 振る軸。1回の実行で1つの軸だけを振り、他は既定へ固定する。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::shadow_probe) enum 振る軸 {
    解像度,
    キャスター,
    余白,
    視点,
    頂点,
    太陽高度,
    最大影距離,
    影の視距離,
}

impl 振る軸 {
    /// 引数で選べる軸の一覧。使い方の表示もこの並びで出す。
    pub(in crate::shadow_probe) const 全軸: [Self; 8] = [
        Self::解像度,
        Self::キャスター,
        Self::余白,
        Self::視点,
        Self::頂点,
        Self::太陽高度,
        Self::最大影距離,
        Self::影の視距離,
    ];

    /// 軸を選ぶ引数の綴り。生値と実行ログの置き場になる軸ごとのディレクトリ名にも同じ綴りを使う。
    /// 軸を続けて回しても前の軸の証拠を上書きせず、実行したコマンドと残った証拠の場所を読み手が1対1で結べる。
    pub(in crate::shadow_probe) fn 綴り(self) -> &'static str {
        match self {
            Self::解像度 => "resolution",
            Self::キャスター => "casters",
            Self::余白 => "margin",
            Self::視点 => "camera",
            Self::頂点 => "vertex",
            Self::太陽高度 => "sun",
            Self::最大影距離 => "distance",
            Self::影の視距離 => "range",
        }
    }
}

pub(in crate::shadow_probe) fn 綴りから読む(語: &str) -> Result<振る軸, String> {
    match 振る軸::全軸.into_iter().find(|軸| 軸.綴り() == 語) {
        Some(軸) => Ok(軸),
        None => Err(format!("知らない軸である({語})。{}のいずれかを指定する", 綴りを並べる())),
    }
}

pub(in crate::shadow_probe) fn 綴りを並べる() -> String {
    振る軸::全軸.iter().map(|軸| 軸.綴り()).collect::<Vec<&str>>().join(" / ")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 綴りが往復し、かつ重複しないこと。軸を足して`全軸`へ入れ忘れると往復が壊れる。
    #[test]
    fn 全軸の綴りは往復して重複しない() {
        for 軸 in 振る軸::全軸 {
            assert_eq!(綴りから読む(軸.綴り()), Ok(軸));
        }
        let mut 綴り一覧: Vec<&str> = 振る軸::全軸.iter().map(|軸| 軸.綴り()).collect();
        綴り一覧.sort_unstable();
        let 件数 = 綴り一覧.len();
        綴り一覧.dedup();
        assert_eq!(綴り一覧.len(), 件数);
    }
}
