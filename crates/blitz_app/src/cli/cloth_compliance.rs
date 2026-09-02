//! `--cloth-xpbd-reference <コンプライアンス>`(と`--cloth-xpbd-reference-below-floor <コンプライアンス>`)が受け取る布のコンプライアンス(メートル毎ニュートン)。
//! 0以上の有限値だけを受け取り、blitz_simの`コンプライアンス`へ写す。裸の`f32`で運ばないのは、露出倍率やブレンド係数と同じ姿の値が
//! 起動設定の中で取り違えられないようにするためである。

use super::argument_error::起動引数エラー;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(transparent)]
pub(crate) struct 布のコンプライアンス指定(f32);

impl 布のコンプライアンス指定 {
    pub(crate) fn 綴りから解析する(綴り: &str) -> Result<Self, 起動引数エラー> {
        let 値: f32 = 綴り
            .parse()
            .map_err(|_| 起動引数エラー::布のコンプライアンス不正(format!("数として読めない({綴り})")))?;
        if !値.is_finite() || 値 < 0.0 {
            return Err(起動引数エラー::布のコンプライアンス不正(
                format!("0以上の有限値でない({値})"),
            ));
        }
        Ok(Self(値))
    }

    /// blitz_simの物性へ写す境界。
    pub(crate) fn 値(self) -> f32 {
        self.0
    }
}
