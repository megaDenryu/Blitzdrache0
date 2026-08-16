//! JSONの表からの型ガードつきの読み取り。担当するのは、キーを指定して文字列・数・3成分・入れ子の表を取り出すことと、
//! 期待した形でなかったことをキー名つきの型付きエラーで言うことである。
//!
//! 数をf64で受けてからf32へ狭めるのでなく、JSONが綴った数の表記をそのままf32として解くのは、
//! 標準ライブラリがf64からf32への型付き変換を持たず、`as`による縮小変換をリポジトリの規約が禁じているためである。
//! 綴りが正本であり、この読み方なら二重の丸めも起きない。
//!
//! 前提: `生成する`へ渡す値はJSONの表である。表でないことの判定は呼び出し側が先に済ませる。
//! 表でない値を渡すと、どのキーも「キーが無い」として返り、読み手が形の破れに気づけない。

use serde_json::Value;

use super::error::接合点読み取りエラー;

pub(super) struct 表から読む<'値> {
    本体: &'値 Value,
}

impl<'値> 表から読む<'値> {
    pub(super) fn 生成する(本体: &'値 Value) -> Self {
        Self { 本体 }
    }

    pub(super) fn 文字列(&self, キー名: &'static str) -> Result<&'値 str, 接合点読み取りエラー> {
        self.値(キー名)?.as_str().ok_or(接合点読み取りエラー::値が文字列でない { キー名 })
    }

    pub(super) fn 数(&self, キー名: &'static str) -> Result<f32, 接合点読み取りエラー> {
        数として解く(self.値(キー名)?, キー名)
    }

    pub(super) fn 三成分(&self, キー名: &'static str) -> Result<[f32; 3], 接合点読み取りエラー> {
        let 配列 = self.値(キー名)?.as_array().ok_or(接合点読み取りエラー::値が3成分の数でない { キー名 })?;
        let [x, y, z] = 配列.as_slice() else {
            return Err(接合点読み取りエラー::値が3成分の数でない { キー名 });
        };
        Ok([数として解く(x, キー名)?, 数として解く(y, キー名)?, 数として解く(z, キー名)?])
    }

    pub(super) fn 表(&self, キー名: &'static str) -> Result<&'値 Value, 接合点読み取りエラー> {
        let 値 = self.値(キー名)?;
        if 値.is_object() {
            return Ok(値);
        }
        Err(接合点読み取りエラー::値が表でない { キー名 })
    }

    fn 値(&self, キー名: &'static str) -> Result<&'値 Value, 接合点読み取りエラー> {
        self.本体.get(キー名).ok_or(接合点読み取りエラー::キーが無い { キー名 })
    }
}

fn 数として解く(値: &Value, キー名: &'static str) -> Result<f32, 接合点読み取りエラー> {
    let Value::Number(数) = 値 else {
        return Err(接合点読み取りエラー::値が数でない { キー名 });
    };
    数.to_string()
        .parse::<f32>()
        .ok()
        .filter(|解いた値| 解いた値.is_finite())
        .ok_or(接合点読み取りエラー::値が数でない { キー名 })
}
