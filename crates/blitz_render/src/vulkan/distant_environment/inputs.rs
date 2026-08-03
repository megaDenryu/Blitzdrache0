//! 遠方環境の生成パス1フレームぶんの入力の記録と、計算の発行前へ押し込む定数のバイト並び。
//!
//! バイト並びをここが単独で持つのは、これがCPUとシェーダーの即時定数の構造体の間の唯一の契約だからである。
//! 並びが食い違えば型検査は通ったまま値が入れ替わるため、詰め方を1箇所に閉じて詰める側と読む側を1対1に保つ。
//!
//! 4つの条件をシェーダー定数でなく即時定数で渡すのは、媒体が変わらないまま時刻や空描画方針だけが動くフレームでも
//! この4つが書き換わるためである。シェーダー定数へ載せると書き換えの時機が媒体と別になる。

use ash::vk;

use crate::vulkan::sync::進行中フレーム数;

/// 1本のコンピュートパスが束縛する資源と計算の発行の大きさ。
pub(crate) struct 遠方環境の生成入力 {
    pub(crate) pipeline: vk::Pipeline,
    pub(crate) layout: vk::PipelineLayout,
    pub(crate) ディスクリプタセット: vk::DescriptorSet,
    pub(crate) 計算の班数: [u32; 3],
    pub(crate) 即時定数: 遠方環境の即時定数,
}

/// 焼く条件のうち、大気媒体から決まらない4つ。太陽の方位を持たないのは太陽相対座標で焼くためである。
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct 遠方環境の即時定数 {
    pub(crate) 観測半径: f32,
    pub(crate) 太陽天頂余弦: f32,
    pub(crate) 太陽放射照度: f32,
    pub(crate) 夜空放射輝度: [f32; 3],
}

/// 押し込むバイト数。`shaders/distant_environment.slang`の`DistantEnvironmentCondition`と一致させる。
/// 実数8個ぶんあるのは、シェーダー側の構造体が夜空放射輝度を16バイト境界へ置くため、前の3つの後ろに
/// 埋め草の実数が1つ入るからである。
pub(crate) const 遠方環境の即時定数のバイト数: u32 = 32;

impl 遠方環境の即時定数 {
    /// 計算の発行前に書くバイト列。並びはシェーダー側の即時定数の宣言と同じである。
    pub(crate) fn バイト列(self) -> Vec<u8> {
        let 埋め草 = 0.0_f32;
        let 値一覧 = [
            self.観測半径,
            self.太陽天頂余弦,
            self.太陽放射照度,
            埋め草,
            self.夜空放射輝度[0],
            self.夜空放射輝度[1],
            self.夜空放射輝度[2],
            埋め草,
        ];
        let mut バイト列 = Vec::with_capacity(値一覧.len() * 4);
        for 値 in 値一覧 {
            バイト列.extend_from_slice(&値.to_le_bytes());
        }
        バイト列
    }
}

/// 遠方環境ディスクリプタが結ぶ、大気のベイク済み画像一式から借りる束縛先。
pub(crate) struct 遠方環境が借りる束縛先<'a> {
    pub(crate) シェーダー定数一覧: &'a [vk::Buffer; 進行中フレーム数],
    pub(crate) スカイビュービュー: vk::ImageView,
}

/// グラフへ資源を登録し、コピーを積むのに要るものだけを取り出す。メモリのハンドルは外へ出さない。
pub(crate) struct 遠方環境の画像入力 {
    pub(crate) 画像: vk::Image,
    pub(crate) ビュー: vk::ImageView,
    pub(crate) 範囲: vk::Extent3D,
    pub(crate) 層数: u32,
    pub(crate) グラフへ渡す寸法: vk::Extent2D,
}

/// 1計算の班が受け持つテクセルの一辺。平面の大気のベイク済み画像と同じ8である。
/// 層は1計算の班につき1つであり、6層を奥行きの計算の班数で覆う。
const 計算の班の一辺: u32 = 8;

pub(crate) fn 計算の班数を求める(面の一辺: u32, 層数: u32) -> [u32; 3] {
    [面の一辺.div_ceil(計算の班の一辺), 面の一辺.div_ceil(計算の班の一辺), 層数]
}
