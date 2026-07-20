//! `ApplicationHandler` 実装。winit所有ループ（パターンA）の受け口。
//! 参照: `_doc/設計/イベントループとフレームペーシング.md`

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;

use super::アプリ;
use blitz_render::ウィンドウ寸法;

impl ApplicationHandler for アプリ {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.レンダラー.is_some() {
            return;
        }
        match super::window_setup::ウィンドウとレンダラーを作る(event_loop) {
            Ok((window, レンダラー)) => {
                self.window = Some(window);
                self.レンダラー = Some(レンダラー);
            }
            Err(誤り) => {
                self.起動時エラー = Some(誤り);
                event_loop.exit();
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        // 入力層はwinitイベントを蓄積するだけで、以降のmatchが既存の責務を続ける
        // （カメラインテントへの写像は`入力状態`内部で完結し、blitz_engineはwinitを知らない）。
        self.入力状態.winitイベントを取り込む(&event);

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
