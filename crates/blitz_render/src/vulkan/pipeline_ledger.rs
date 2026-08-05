//! 材質を読む描画族(シーン・シャドウ)のパイプライン状態オブジェクトの台帳。担当するのは、パイプラインキーから
//! VkPipelineへの唯一の対応と、族ごとのパイプラインレイアウトの所有である。
//!
//! 布・粒子・全画面・コンピュートはこの台帳の外にあり、それぞれの段階資源が自分のパイプラインを持ち続ける。
//! 材質を読まないパイプラインへ材質の軸を強制しないためである。
//! 深度プリパスは色パスと同じレイアウト・同じ`シェーダー一式`の頂点段から作るため、独立した族でなくこの台帳の3つ目の表として持つ。
//! キーの型は`key`、描画先の一意化は`render_target`、照明資源の束縛レイアウトは`lighting_binding_layout`、
//! 起動時に数え上げる必要キー集合は`required_keys`、台帳が持つ成分からのキーの組み立ては`key_build`、キーと実体の対応表は`entry_table`、実体の作り方は`device_supplier`、
//! レイアウトの所有は`layouts`にある。生成の局面は`create`、シェーダーの差し替えの取引は`reload`が持つ。
//! 参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「段階導入」の段6

mod build_tables;
mod create;
mod device_supplier;
mod entry_table;
mod key;
mod key_build;
mod layouts;
mod lighting_binding_layout;
mod reload;
mod render_target;
mod required_keys;
mod supplier;
#[cfg(test)]
mod table_tests;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::frame_composition::深度プリパス方式;

use entry_table::パイプライン記載表;

pub(crate) use key::パイプラインキー;
pub(crate) use lighting_binding_layout::照明束縛レイアウト;
pub(crate) use render_target::描画先の一意化;

pub(crate) struct 材質描画族パイプライン台帳 {
    レイアウト: layouts::材質描画族のレイアウト,
    /// 起動時に決めた描画先と照明の契約。キーを組み立てるのは台帳自身であり、呼び出し元が成分を持ち回らない。
    シーンの描画先: 描画先の一意化,
    シーンの照明束縛: 照明束縛レイアウト,
    /// 起動時の計測条件が選んだ深度プリパスの方式。色パスの深度の扱いをここから導くため、台帳が唯一の持ち主である。
    深度プリパス方式: 深度プリパス方式,
    シャドウの描画先: 描画先の一意化,
    深度プリパスの描画先: 描画先の一意化,
    シーン: パイプライン記載表<vk::Pipeline>,
    シャドウ: パイプライン記載表<vk::Pipeline>,
    深度プリパス: パイプライン記載表<vk::Pipeline>,
}

impl 材質描画族パイプライン台帳 {
    /// 未知のキーは型付きの失敗である。記録の途中で作る枝を持たない。
    pub(crate) fn 引く(&self, キー: パイプラインキー) -> Result<vk::Pipeline, レンダラーエラー> {
        let 表 = match キー {
            パイプラインキー::シーン { .. } => &self.シーン,
            パイプラインキー::シャドウ { .. } => &self.シャドウ,
            パイプラインキー::深度プリパス { .. } => &self.深度プリパス,
        };
        表.引く(キー).copied()
    }

    /// このレンダラーが積む深度プリパスの方式。フレームの組み立てが「プリパスを積むか」をこの1箇所から読む。
    pub(crate) const fn 深度プリパス方式(&self) -> 深度プリパス方式 {
        self.深度プリパス方式
    }

    pub(crate) const fn シーンlayout(&self) -> vk::PipelineLayout {
        self.レイアウト.シーン()
    }

    pub(crate) const fn シャドウlayout(&self) -> vk::PipelineLayout {
        self.レイアウト.シャドウ()
    }

    /// 前提: レンダラー全体の破棄順を持つ`renderer/destroy.rs`が、GPU待機の済んだ1段として呼ぶ。
    /// パイプラインを全部捨ててからレイアウトを捨てる。レイアウトを参照するパイプラインが残っていてはならないためである。
    pub(crate) fn 破棄する(&mut self, device: &ash::Device) {
        self.シーン.破棄する(|pipeline| device_supplier::pipelineを破棄する(device, pipeline));
        self.シャドウ.破棄する(|pipeline| device_supplier::pipelineを破棄する(device, pipeline));
        self.深度プリパス
            .破棄する(|pipeline| device_supplier::pipelineを破棄する(device, pipeline));
        self.レイアウト.破棄する(device);
    }
}
