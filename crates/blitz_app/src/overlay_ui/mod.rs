//! 画面へ重ねるUI(egui)の統合(判断31〜34)。egui/egui-winitへの依存はblitz_appにのみ閉じ、
//! blitz_renderは自前のUI描画語彙のみを知る(判断32)。重ねる中身は開発パネルが`panel`にある。
//! 開発パネルはF3で表示を切り替え、既定は非表示、`--dev-ui`で起動時から表示する。
//! 重ねるものが1つも無いフレームはeguiを一切実行しない。

mod font_setup;
mod frame_time;
mod mesh_convert;
mod panel;
mod scissor_convert;
mod texture_id_map;
mod texture_mirror;
mod texture_sync;

pub(crate) mod stats;

use winit::event::WindowEvent;
use winit::window::Window;

use crate::error::起動エラー;
use frame_time::フレーム時間計測;
use stats::開発UI統計;
use texture_mirror::テクスチャミラー;

pub(crate) struct 画面へ重ねるUI {
    コンテキスト: egui::Context,
    winit統合: egui_winit::State,
    ミラー: テクスチャミラー,
    フレーム時間: フレーム時間計測,
    開発パネルを表示するか: bool,
}

impl 画面へ重ねるUI {
    pub(crate) fn 生成する(window: &Window, 開発パネルの初期表示: bool) -> Self {
        let コンテキスト = egui::Context::default();
        font_setup::日本語フォントを追加する(&コンテキスト);
        let 初期ppp = egui_winit::pixels_per_point(&コンテキスト, window);
        let winit統合 = egui_winit::State::new(コンテキスト.clone(), egui::ViewportId::ROOT, window, Some(初期ppp), window.theme(), None);
        Self {
            コンテキスト,
            winit統合,
            ミラー: テクスチャミラー::新規(),
            フレーム時間: フレーム時間計測::生成する(),
            開発パネルを表示するか: 開発パネルの初期表示,
        }
    }

    pub(crate) fn 開発パネルの表示を切り替える(&mut self) {
        self.開発パネルを表示するか = !self.開発パネルを表示するか;
    }

    /// eguiが消費したイベントは呼び出し元(入力層)へ渡さない。開発パネルを表示していないフレームは
    /// eguiがマウスもキーも受け取らないため常にfalse(消費しない)を返す。ゲームの画面はキー操作を1つも持たず、
    /// 操作は共通入力層が確定するため、この判定は開発パネルの表示だけで決まる。
    pub(crate) fn winitイベントを取り込む(&mut self, window: &Window, event: &WindowEvent) -> bool {
        if !self.開発パネルを表示するか {
            return false;
        }
        self.winit統合.on_window_event(window, event).consumed
    }

    /// このフレームぶんのegui実行と、UIテクスチャ反映・メッシュ変換までを行う。
    /// 無効時はeguiを実行せず`None`を返す(判断34)。
    /// `露出`・`ブレンド`はスライダーが書き換えるため可変参照で受ける(判断39・45)。
    pub(crate) fn 描画データを作る(
        &mut self,
        window: &Window,
        レンダラー: &mut blitz_render::レンダラー,
        統計: 開発UI統計,
        露出: &mut f32,
        ブレンド: &mut f32,
    ) -> Result<Option<blitz_render::UI描画データ>, 起動エラー> {
        if !self.開発パネルを表示するか {
            return Ok(None);
        }
        let raw_input = self.winit統合.take_egui_input(window);
        let full_output = self.コンテキスト.run(raw_input, |ctx| panel::内容を描く(ctx, &統計, 露出, ブレンド));
        self.winit統合.handle_platform_output(window, full_output.platform_output);

        texture_sync::反映する(レンダラー, &full_output.textures_delta, &mut self.ミラー)?;

        let 図形一覧 = self.コンテキスト.tessellate(full_output.shapes, full_output.pixels_per_point);
        Ok(Some(mesh_convert::変換する(&図形一覧, full_output.pixels_per_point)))
    }

    /// このフレームのCPU側経過時間を記録し、更新後の移動平均(ミリ秒)を返す。
    pub(crate) fn フレーム時間を記録する(&mut self) -> f64 {
        self.フレーム時間.記録する()
    }
}
