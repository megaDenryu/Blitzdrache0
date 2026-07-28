//! F3キーによる開発用UIの表示切替(判断34)。触れるフィールドは`開発ui`だけであり、
//! winitイベントを読んで押下の立ち上がりだけを拾う。

use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

use crate::app::アプリ;

impl アプリ {
    /// F3キー押下(リピートでない立ち上がりのみ)で開発用UIをトグルする。
    pub(super) fn f3押下を確認する(&mut self, event: &WindowEvent) {
        let WindowEvent::KeyboardInput {
            event:
                KeyEvent {
                    physical_key: PhysicalKey::Code(KeyCode::F3),
                    state: ElementState::Pressed,
                    repeat: false,
                    ..
                },
            ..
        } = event
        else {
            return;
        };
        if let Some(開発ui) = &mut self.開発ui {
            開発ui.トグルする();
        }
    }
}
