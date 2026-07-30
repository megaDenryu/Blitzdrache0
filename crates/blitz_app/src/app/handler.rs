//! `ApplicationHandler` 実装。winit所有ループ(パターンA)の受け口。参照: `_doc/設計/イベントループとフレームペーシング.md`
//! 開発用UIの表示切替だけは触れるフィールドが`開発ui`に閉じるため`dev_ui_toggle`にある。

mod dev_ui_toggle;

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
        let 実表示計測要求 = super::measurement_setup::実表示計測要求を決める(self);
        match super::window_setup::ウィンドウとレンダラーを作る(
            event_loop,
            &self.シーン名,
            &self.アセットルート,
            self.描画対象の並べ方,
            &mut self.ホットリローダー,
            self.粒子表示,
            self.空の方式,
            self.開発ui初期有効,
            self.フレーム構成,
            self.布モード,
            実表示計測要求,
            self.大域オフセット,
        ) {
            Ok((window, mut レンダラー, 開発ui, アニメーション, 布プリセット, 可視材料一覧)) => {
                // 起動時シーンをディスクから読んだのはこの1回である。
                self.シーン読込計数.読み込んだ(self.現在フレーム);
                if let Some(状況) = super::measurement_setup::レンダラーの計測を有効にする(&mut レンダラー, self) {
                    println!("実表示時刻計測: {}", 状況.名称());
                }
                self.可視判定.束を登録する(super::scene_load::起動時シーンの束ID, 可視材料一覧);
                self.window = Some(window);
                self.レンダラー = Some(レンダラー);
                self.開発ui = Some(開発ui);
                self.アニメーション = アニメーション;
                self.布プリセット = 布プリセット;
            }
            Err(誤り) => {
                self.起動時エラー = Some(誤り);
                event_loop.exit();
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        let egui消費済みか = self
            .window
            .as_ref()
            .zip(self.開発ui.as_mut())
            .is_some_and(|(window, 開発ui)| 開発ui.winitイベントを取り込む(window, &event));
        self.f3押下を確認する(&event);

        // 入力層はwinitイベントを蓄積するだけで、以降のmatchが既存の責務を続ける
        // （カメラインテントへの写像は`入力状態`内部で完結し、blitz_engineはwinitを知らない）。
        // eguiが消費したイベント(ポインタ/キーボードがUI操作中)はカメラ入力へ流さない。
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
