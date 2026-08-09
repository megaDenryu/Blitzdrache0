//! `ApplicationHandler` 実装。winit所有ループ(パターンA)の受け口。参照: `_doc/設計/イベントループとフレームペーシング.md`
//! 開発用UIの表示切替だけは触れるフィールドが`開発ui`に閉じるため`dev_ui_toggle`にある。
//! 起動時に1回だけ走る生成と格納は`resume`にある。

mod dev_ui_toggle;
mod resume;

use super::アプリ;
use blitz_render::ウィンドウ寸法;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;

impl ApplicationHandler for アプリ {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.レンダラー.is_some() {
            return;
        }
        resume::生成してアプリへ格納する(self, event_loop);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        let egui消費済みか = self
            .window
            .as_ref()
            .zip(self.開発ui.as_mut())
            .is_some_and(|(window, 開発ui)| 開発ui.winitイベントを取り込む(window, &event));
        self.f3押下を確認する(&event);

        // 入力層はwinitイベントを蓄積するだけで、以降のmatchが既存の責務を続ける（カメラインテントへの写像は`入力状態`内部で完結し、blitz_engineはwinitを知らない）。eguiが消費したイベント(ポインタ/キーボードがUI操作中)はカメラ入力へ流さない。
        if !egui消費済みか {
            self.入力状態.winitイベントを取り込む(&event);
        }

        match event {
            WindowEvent::RedrawRequested => self.一フレーム実行する(event_loop),
            WindowEvent::Resized(寸法) => {
                if let Some(レンダラー) = &mut self.レンダラー {
                    レンダラー.サイズ変更を通知する(ウィンドウ寸法::生成する(寸法.width, 寸法.height));
                }
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}
