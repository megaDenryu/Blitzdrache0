//! 開発用UIテクスチャの登録・削除(公開API、判断33)。egui由来のテクスチャデルタを
//! 呼び出し側(blitz_app)がこのAPIへ写像する。

use super::レンダラー;
use crate::error::レンダラーエラー;
use crate::ui_texture_id::UIテクスチャID;
use crate::ui_texture_material::UIテクスチャ素材;

impl レンダラー {
    /// `id`のテクスチャを新規登録する、または既存なら新しい内容で置き換える。
    pub fn uiテクスチャを登録する(
        &mut self, id: UIテクスチャID, 素材: UIテクスチャ素材
    ) -> Result<(), レンダラーエラー> {
        let 確保係 = self.環境.資源の確保係を貸す();
        self.ui一式.テクスチャを反映する(&確保係, &self.転送環境, id, &素材)
    }

    /// `id`のテクスチャを削除する。未登録IDへの呼び出しは無視する
    /// (egui側の解放イベントが既に反映済みのIDを重複して指す場合がある)。
    pub fn uiテクスチャを削除する(&mut self, id: UIテクスチャID) {
        self.ui一式.テクスチャを削除する(self.環境.device(), id);
    }
}
