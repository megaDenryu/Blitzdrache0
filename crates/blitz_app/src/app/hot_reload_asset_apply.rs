//! 生成台帳で公開が完了した実行時カタログ・チャンク目録・起動時シーンを1組として反映する。
//! 触れるフィールドはゲーム配線・レンダラー・永続束・ストリーミング・ホットリローダーに限る。

use super::アプリ;
use crate::hot_reload::公開済みの実行時アセット一式;

impl アプリ {
    pub(super) fn 公開済みの実行時アセット一式を反映する(&mut self, 一式: 公開済みの実行時アセット一式) {
        if let Some(配線) = &self.ストリーミング
            && let Err(誤り) = 配線.チャンク目録の一辺を検査する(&一式.チャンク目録)
        {
            eprintln!("[hot-reload] 実行時アセット一式を採用しなかった: {誤り}");
            return;
        }
        if let Err(誤り) = self.ゲーム配線.カタログから高さ場を据える(&一式.カタログ) {
            eprintln!("[hot-reload] 新しい高さ場を据えられなかった: {誤り}");
            return;
        }
        let 一辺 = self.ストリーミング.as_ref().map(super::streaming::ストリーミング配線::一辺);
        let Some(レンダラー) = &mut self.レンダラー else {
            return;
        };
        let Some(登録一式) = 起動時シーンを差し替える(
            レンダラー,
            &一式.起動時シーン,
            self.描画対象の並べ方,
            self.大域ずらし量,
            一辺,
            &self.ゲーム配線,
        ) else {
            return;
        };
        if let Err(誤り) = self.永続束.再登録する(
            レンダラー,
            &mut self.可視判定,
            &mut self.プリミティブ描画項目台帳,
            登録一式,
            self.大域ずらし量,
            一辺,
        ) {
            eprintln!("[hot-reload] 永続束を再登録できなかった: {誤り}");
            return;
        }
        if let Some(配線) = &mut self.ストリーミング
            && let Err(誤り) = 配線.チャンク目録を差し替える(一式.チャンク目録)
        {
            eprintln!("[hot-reload] チャンク目録を差し替えられなかった: {誤り}");
            return;
        }
        self.ホットリローダー.カタログを採用する(一式.カタログ);
    }
}

fn 起動時シーンを差し替える(
    レンダラー: &mut blitz_render::レンダラー,
    シーン: &blitz_engine::シーンデータ,
    並べ方: crate::cli::描画対象の並べ方,
    大域平行移動: blitz_math::大域ワールド位置,
    一辺: Option<blitz_engine::チャンク一辺>,
    ゲーム配線: &crate::game::ゲーム配線,
) -> Option<super::scene_load::束の登録一式> {
    let 束座標 = super::scene_load::起動時シーンの所有チャンク;
    let mut 描画入力 = match super::scene_load::シーンをレンダラー入力に変換する(シーン, 並べ方, 束座標, 大域平行移動, 一辺)
    {
        Ok(入力) => 入力,
        Err(誤り) => {
            eprintln!("[hot-reload] 再読込したシーンの変換に失敗した: {誤り}");
            return None;
        }
    };
    if let Err(誤り) =
        ゲーム配線.束の描画シーン素材へ動く個体を宣言する(super::scene_load::起動時シーンの束ID, &mut 描画入力.描画シーン)
    {
        eprintln!("[hot-reload] 再読込したシーンへ動く個体を宣言できなかった: {誤り}");
        return None;
    }
    match レンダラー.シーンを差し替える(描画入力.描画シーン) {
        Ok(()) => Some(描画入力.登録一式),
        Err(誤り) => {
            eprintln!("[hot-reload] シーン差し替えに失敗した: {誤り}");
            None
        }
    }
}
