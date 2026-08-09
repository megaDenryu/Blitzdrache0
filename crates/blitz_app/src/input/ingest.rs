//! winitの`WindowEvent`を`入力状態`へ反映する。カーソル座標のf64→f32変換は
//! winitの`PhysicalPosition::cast`（境界向けの安全な変換API）を使い、asキャストを避ける。

use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

use super::入力状態;

/// PixelDelta(トラックパッド等)を1行相当のホイール量へ換算する係数。
const ピクセルあたりの行相当: f32 = 20.0;

pub(super) fn 取り込む(状態: &mut 入力状態, event: &WindowEvent) {
    match event {
        WindowEvent::MouseInput {
            state,
            button: MouseButton::Left,
            ..
        } => 左ボタンを反映する(状態, *state),
        WindowEvent::MouseInput {
            state,
            button: MouseButton::Right,
            ..
        } => 状態.右ボタン押下中 = *state == ElementState::Pressed,
        WindowEvent::CursorMoved { position, .. } => カーソル移動を反映する(状態, *position),
        WindowEvent::MouseWheel { delta, .. } => ホイールを反映する(状態, delta),
        WindowEvent::KeyboardInput { event, .. } => キー入力を反映する(状態, event),
        _ => {}
    }
}

fn 左ボタンを反映する(状態: &mut 入力状態, state: ElementState) {
    状態.左ボタン押下中 = state == ElementState::Pressed;
    if !状態.左ボタン押下中 {
        状態.直前カーソル位置 = None;
    }
}

fn カーソル移動を反映する(状態: &mut 入力状態, 位置: PhysicalPosition<f64>) {
    let 位置f32 = 位置.cast::<f32>();
    状態.現在カーソル位置 = Some((位置f32.x, 位置f32.y));
    if !状態.左ボタン押下中 {
        状態.直前カーソル位置 = Some((位置f32.x, 位置f32.y));
        return;
    }
    if let Some((前x, 前y)) = 状態.直前カーソル位置 {
        状態.ドラッグ蓄積.0 += 位置f32.x - 前x;
        状態.ドラッグ蓄積.1 += 位置f32.y - 前y;
    }
    状態.直前カーソル位置 = Some((位置f32.x, 位置f32.y));
}

fn ホイールを反映する(状態: &mut 入力状態, delta: &MouseScrollDelta) {
    match delta {
        MouseScrollDelta::LineDelta(_, y) => 状態.ホイール蓄積 += y,
        MouseScrollDelta::PixelDelta(位置) => {
            状態.ホイール蓄積 += 位置.cast::<f32>().y / ピクセルあたりの行相当;
        }
    }
}

fn キー入力を反映する(状態: &mut 入力状態, event: &KeyEvent) {
    let PhysicalKey::Code(コード) = event.physical_key else {
        return;
    };
    let 押下中 = event.state == ElementState::Pressed;
    match コード {
        KeyCode::KeyA => 状態.a押下中 = 押下中,
        KeyCode::KeyD => 状態.d押下中 = 押下中,
        KeyCode::KeyW => 状態.w押下中 = 押下中,
        KeyCode::KeyS => 状態.s押下中 = 押下中,
        KeyCode::KeyQ => 状態.q押下中 = 押下中,
        KeyCode::KeyE => 状態.e押下中 = 押下中,
        カメラが読まないコード => 状態.ゲーム操作のキー.キーコードを反映する(カメラが読まないコード, 押下中),
    }
}
