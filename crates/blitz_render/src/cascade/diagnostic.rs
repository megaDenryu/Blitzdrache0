//! シーンの画素段が本番の色の代わりに出す診断出力の種別と、影の欠落計器が使う受光距離帯の刻み。
//! 担当するのは、どの診断を出すかの選択と、定数の正本を持つことである。描画も判定も行わない。
//!
//! 受光距離帯を多段の分割でなく固定の刻みで持つのは、最大影距離を振ると分割深度そのものが動き、
//! 距離区分を帯に使うと基準と候補で帯の意味が変わって「どこの影が失われたか」を比べられなくなるためである。
//! 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「シャドウ性能の是正(フェーズ2性能課題、2026-08-03着手)」

/// 受光距離帯1つの幅。キャスターの距離分布と同じ25メートル刻みにするのは、投げる側と受ける側の距離を同じ物差しで読むためである。
/// 注意: `shaders/pixel_diagnostic.slang`の`receivingBandWidthMeters`が写しであり、`cargo xtask conform`が値の一致を見る。
pub const 受光距離帯の幅メートル: f32 = 25.0;

/// 受光距離帯の数。最後の帯は上限を持たず、それより遠い受光点をすべて受ける。
/// 注意: `shaders/pixel_diagnostic.slang`の`receivingBandCount`が写しであり、`cargo xtask conform`が値の一致を見る。
pub const 受光距離帯数: u32 = 16;

/// 多段影定数へ載せる診断の種別の番号。
/// 注意: `shaders/pixel_diagnostic.slang`の`pixelDiagnosticCascadeBands`と`pixelDiagnosticShadowLoss`が写しであり、
/// `cargo xtask conform`が値の一致を見る。
const 距離区分の可視化の番号: f32 = 1.0;
const 影の欠落計器の番号: f32 = 2.0;

/// シーンの画素段が出す診断の種別。本番の絵を出す実行は`出さない`である。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum 画素診断 {
    #[default]
    出さない,
    /// どの距離区分を参照したかと影の中かを色で出す。継ぎ目の検収が読む。
    距離区分の可視化,
    /// 多段影の評価直後の影可視度と、多段の分割に依存しない固定刻みの受光距離帯を出す。影の欠落と余分の計器が読む。
    影の欠落計器,
}

impl 画素診断 {
    pub(crate) fn 番号(self) -> f32 {
        match self {
            Self::出さない => 0.0,
            Self::距離区分の可視化 => 距離区分の可視化の番号,
            Self::影の欠落計器 => 影の欠落計器の番号,
        }
    }
}
