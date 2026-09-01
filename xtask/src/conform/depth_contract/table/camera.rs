//! カメラの逆向き深度を持ち直す全接点。

use super::{カメラ深度, 接点};

pub(super) const 接点一覧: [接点; 9] = [
    接点 {
        契約: &カメラ深度,
        項目: "投影の近遠順",
        パス: "crates/blitz_math/src/frame/transform_construct.rs",
        期待する綴り: "Mat4::perspective_rh(縦視野角.値(), アスペクト比, 遠クリップ.値(), 近クリップ.値())",
    },
    接点 {
        契約: &カメラ深度,
        項目: "描画グラフの消去",
        パス: "crates/blitz_render/src/vulkan/graph/depth_attachment.rs",
        期待する綴り: "深度領域::カメラ => 0.0",
    },
    接点 {
        契約: &カメラ深度,
        項目: "CPU局所可視性の消去",
        パス: "crates/blitz_render/src/local_visibility/depth_image.rs",
        期待する綴り: "pub const 深度の消去値: f32 = 0.0;",
    },
    接点 {
        契約: &カメラ深度,
        項目: "GPU局所可視性の消去",
        パス: "shaders/local_visibility_depth.slang",
        期待する綴り: "localVisibilityDepthClearValue = 0.0;",
    },
    接点 {
        契約: &カメラ深度,
        項目: "空中遠近の背景",
        パス: "shaders/aerial_composite.slang",
        期待する綴り: "if (depth <= 0.0)",
    },
    接点 {
        契約: &カメラ深度,
        項目: "空の遠面",
        パス: "shaders/sky_frame.slang",
        期待する綴り: "output.position = float4(ndc, 0.0, 1.0);",
    },
    接点 {
        契約: &カメラ深度,
        項目: "空中遠近の視錐台の端点",
        パス: "crates/blitz_app/src/app/time_of_day/aerial_frustum.rs",
        期待する綴り: "const 遠面の深度: f32 = 0.0; const 近面の深度: f32 = 1.0;",
    },
    接点 {
        契約: &カメラ深度,
        項目: "多段影のカメラ近平面",
        パス: "crates/blitz_render/src/cascade/camera_frustum.rs",
        期待する綴り: "let 近隅一覧 = 平面(1.0);",
    },
    接点 {
        契約: &カメラ深度,
        項目: "多段影のカメラ遠平面",
        パス: "crates/blitz_render/src/cascade/camera_frustum.rs",
        期待する綴り: "let 遠隅一覧 = 平面(0.0);",
    },
];
