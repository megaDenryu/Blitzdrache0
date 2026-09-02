//! 布の固定刻み1本の長さ。値は時間の規律の正本(`crates/blitz_app/src/game/step_seconds.rs`)からコンポジションルートが注入し、
//! レンダラーは刻み幅の綴りを1つも持たない(`_doc/設計/XPBD共通拘束基盤.md`「判断9」)。`blitz_sim::刻み幅`の写しであり、
//! 値だけを運ぶ(blitz_renderはblitz_simに依存しないため型を共有しない)。GPUの積分が読むdtと、コンプライアンスから刻み依存量α̃を
//! 導く刻み幅の2乗の逆数はこの値から作るため、0と非有限を生成で拒む(0では逆数が無限大になり、布全体が非数になる)。

use blitz_math::秒;

use super::布素材エラー;

/// 布の固定刻み1本の長さ。正の有限の秒だけを持つ。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct 布の刻み幅(秒);

impl 布の刻み幅 {
    /// 0以下と非有限を型付きエラーで拒む。
    pub fn 生成する(長さ: 秒) -> Result<Self, 布素材エラー> {
        if !(長さ.値().is_finite() && 長さ.値() > 0.0) {
            return Err(布素材エラー::刻み幅が正の有限値でない { 秒: 長さ.値() });
        }
        Ok(Self(長さ))
    }

    pub fn 秒(&self) -> 秒 {
        self.0
    }

    /// 定数UBOの`misc.y`へ書く刻み幅の2乗の逆数。境界向けの生値であり、GPUのバイト列化だけが読む。
    pub(crate) fn 刻み幅の2乗の逆数(&self) -> f32 {
        let 刻み = self.0.値();
        1.0 / (刻み * 刻み)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 正の刻み幅だけを通し2乗の逆数を導く() {
        let 刻み幅 = 布の刻み幅::生成する(秒::生成する(0.5));
        assert!(matches!(刻み幅, Ok(刻み幅) if 刻み幅.刻み幅の2乗の逆数() == 4.0));
        assert!(布の刻み幅::生成する(秒::生成する(0.0)).is_err());
        assert!(布の刻み幅::生成する(秒::生成する(-1.0)).is_err());
        assert!(布の刻み幅::生成する(秒::生成する(f32::NAN)).is_err());
    }
}
