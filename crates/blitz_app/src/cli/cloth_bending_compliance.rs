//! `--cloth-xpbd-reference-bending <曲げのコンプライアンス>`が受け取る布の曲げのコンプライアンス(毎ニュートンメートル)。
//! 0以上の有限値だけを受け取り、blitz_simの`曲げのコンプライアンス`へ写す。距離のコンプライアンス(メートル毎ニュートン)の指定と別の型にするのは、
//! 次元が違う2つの値が起動設定の中で取り違えられないようにするためである。

use super::argument_error::起動引数エラー;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(transparent)]
pub(crate) struct 布の曲げのコンプライアンス指定(f32);

impl 布の曲げのコンプライアンス指定 {
    pub(crate) fn 綴りから解析する(綴り: &str) -> Result<Self, 起動引数エラー> {
        let 値: f32 = 綴り
            .parse()
            .map_err(|_| 起動引数エラー::布の曲げのコンプライアンス不正(format!("数として読めない({綴り})")))?;
        if !値.is_finite() || 値 < 0.0 {
            return Err(起動引数エラー::布の曲げのコンプライアンス不正(
                format!("0以上の有限値でない({値})"),
            ));
        }
        Ok(Self(値))
    }

    /// `--cloth-xpbd-reference-bending`を指定しない参照比較の布が採る曲げのコンプライアンス。吊るし布のプリセットと同じ10毎ニュートンメートルである。
    pub(crate) fn 既定() -> Self {
        Self(10.0)
    }

    /// blitz_simの物性へ写す境界。
    pub(crate) fn 値(self) -> f32 {
        self.0
    }
}
