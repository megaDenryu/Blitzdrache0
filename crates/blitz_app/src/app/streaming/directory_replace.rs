//! 世界差し替え時に旧チャンクの描画束を3台帳から解除し、その後で調停の目録と台帳を置き換える。

use super::{bundle_sync::束idを作る, ストリーミング配線};
use crate::error::起動エラー;

impl ストリーミング配線 {
    pub(in crate::app) fn チャンク目録と描画束を差し替える(
        &mut self,
        新しい目録: blitz_engine::チャンク目録,
        レンダラー: &mut blitz_render::レンダラー,
        可視判定: &mut crate::app::visibility::可視判定配線,
        項目台帳: &mut crate::app::primitive_draw_item_registry::プリミティブ描画項目台帳,
    ) -> Result<(), 起動エラー> {
        self.チャンク目録の一辺を検査する(&新しい目録)?;
        let mut 座標一覧 = Vec::new();
        self.調停.gpu資源を持つ座標を集める(&mut 座標一覧);
        let 束一覧 = 座標一覧.into_iter().map(束idを作る).collect::<Result<Vec<_>, _>>()?;
        for 束id in 束一覧 {
            レンダラー.描画束を解除する(束id);
            可視判定.束を解除する(束id);
            項目台帳.束を解除する(束id);
            // 同じ座標を含む世界へ連続で差し替えると同じ束IDの破棄完了が複数届くため、完了1件につき1件を残す。
            self.差し替え前の実破棄待ち束.push(束id);
        }
        self.調停.チャンク目録を差し替える(新しい目録)?;
        Ok(())
    }
}
