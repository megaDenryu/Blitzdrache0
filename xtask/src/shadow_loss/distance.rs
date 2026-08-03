//! 候補として与える距離。担当するのは、綴りの検証と保持である。
//!
//! 綴りを捨てないのは、値をどう解釈するかの正本を`blitz_app`側へ置いたままにするためである。

pub(super) struct 距離メートル {
    綴り: String,
}

impl 距離メートル {
    /// 0以下と非有限を拒むのは、どちらも「影を落とす個体が1体も無い」か「比較が常に偽になる」のどちらかになり、
    /// 距離の水準として意味を持たないためである。
    pub(super) fn 生成する(綴り: &str) -> Result<Self, String> {
        let 値 = 綴り.parse::<f64>().map_err(|誤り| format!("距離を数として読めない({綴り}): {誤り}"))?;
        if !値.is_finite() || 値 <= 0.0 {
            return Err(format!("距離は正の有限値でなければならない({綴り})"));
        }
        Ok(Self { 綴り: 綴り.to_string() })
    }

    pub(super) fn 綴り(&self) -> &str {
        &self.綴り
    }
}
