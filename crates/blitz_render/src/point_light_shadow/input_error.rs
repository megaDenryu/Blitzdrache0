//! 点光源の影が世界の宣言を受け取れないときの型付きの失敗。

use thiserror::Error;

#[derive(Debug, Clone, Copy, Error, PartialEq)]
pub enum 点光源の影の入力エラー {
    #[error("影を落とす点光源の影響半径{影響半径}メートルが、立方体の面の近面{近面}メートル以下である")]
    影響半径が近面以下 { 影響半径: f32, 近面: f32 },
}
