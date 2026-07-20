//! レンダラー生成に必要なシェーダーの束。個別引数の列挙をやめ、シェーダーの追加
//! (M7のトーンマップ等)で公開APIのシグネチャが伸び続けることを防ぐ(判断38)。

use crate::particle_shader_set::粒子シェーダー一式;
use crate::shader_set::シェーダー一式;

/// レンダラーが使う全シェーダー。各フィールドはビルド時にslangcでコンパイル済みのSPIR-V。
#[derive(Debug, Clone)]
pub struct シェーダー束 {
    pub シーン: シェーダー一式,
    pub シャドウ: シェーダー一式,
    pub トーンマップ: シェーダー一式,
    pub ui: シェーダー一式,
    /// `--particles`指定時のみ`Some`(判断29)。
    pub 粒子: Option<粒子シェーダー一式>,
}
