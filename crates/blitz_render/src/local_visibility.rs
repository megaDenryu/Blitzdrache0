//! 局所可視性補正(設計文書がSSAOと呼ぶもの)のCPU正本。担うのは、深度だけを入力にして画素ごとの
//! 局所可視度を決める数学を、GPUから独立に定義することである。Vulkanの資源もフレームの経路も知らず、
//! 同じ入力からは常に同じ出力を返す。
//!
//! これは放射輝度問い合わせ階層の契約Cであり、放射輝度の供給元ではない。求めた局所可視度は拡散間接照度
//! だけへ掛かる(`参照: _doc/設計/放射輝度問い合わせ階層.md`「IIa(SSAO)の設計」)。
//!
//! blitz_engineでなくこのクレートへ置くのは、入力の深度画像も画素の位置も透視投影の逆変換もすべて描画側の
//! 語彙であり、blitz_engineが深度バッファも画面の画素も知らないためである。深度プリパスの方式が既に
//! フレーム構成の軸としてこのクレートに在ることとも揃う。
//!
//! 内側は3層に分かれる。量と設定(可視度・寸法・半径・バイアス・減衰・標本数)、画素1つの工程(位置の復元・
//! 隣点の選択・法線の再構成・標本の向き・遮蔽の標本化)、画像1枚の工程(焼き上げ・両側ぼかし・合成深度の4つの形)であり、
//! 依存はこの並びの向きにだけ流れる。
//!
//! 計算を単精度で行うのは、写しがGPUの単精度で同じ式を踏むためである。倍精度で組み立てると、遮蔽の判定が
//! バイアスの境界に乗った画素で正本と写しの答えが分かれ、その差が写しの誤りと見分けられなくなる。
//! 単精度で表せない中間だけを倍精度で作り、狭める場所を1箇所に閉じる。
//!
//! ハンマーズリー列と接基底は派生表現の畳み込みが持つ正本を共有する。同じ数学へ2つの正本を置くと
//! シェーダー側の写しも2本になり、値の一致検査が二重になるためである。

mod acceptance_override;
mod bake;
mod bilateral_blur;
mod blur_settings;
mod code_value;
mod depth_bias;
mod depth_image;
mod diffuse_indirect_method;
mod distance_falloff;
mod draw_settings;
mod error;
mod hemisphere_sample;
mod image_size;
mod narrowing;
mod neighbor_selection;
mod normal_reconstruction;
mod occlusion_radius;
mod occlusion_sampling;
mod occlusion_settings;
mod pixel_hash;
mod projection;
mod rotation;
mod same_surface_tolerance;
mod sample_count;
mod scene_intersection;
mod scene_shape;
mod settings;
mod shader_set;
mod synthetic_depth;
mod synthetic_scene;
mod view_direction;
mod view_displacement;
mod view_position;
mod visibility;
mod visibility_image;

#[cfg(test)]
mod local_visibility_tests;

pub use acceptance_override::局所可視度の上書き;
pub use bake::{局所可視度画像を焼く, 生の局所可視度を焼く};
pub use bilateral_blur::両側ぼかしを掛ける;
pub use blur_settings::両側ぼかしの設定;
pub use code_value::局所可視度の符号値;
pub use depth_bias::遮蔽の深度バイアス;
pub use depth_image::{深度の消去値, 深度画像};
pub use diffuse_indirect_method::拡散間接方式;
pub use distance_falloff::遮蔽の距離減衰;
pub use draw_settings::局所可視性の描画設定;
pub use error::局所可視性エラー;
pub use hemisphere_sample::半球の標本の向きを求める;
pub use image_size::画像の寸法;
pub use neighbor_selection::{差分の軸, 軸の隣点を選ぶ, 隣点の選択};
pub use normal_reconstruction::{ビュー空間法線を再構成する, 法線の再構成結果};
pub use occlusion_radius::遮蔽半径;
pub use occlusion_sampling::画素の局所可視度を求める;
pub use occlusion_settings::遮蔽の設定;
pub use pixel_hash::画素位置の整数ハッシュ;
pub use projection::射影の復元;
pub use rotation::{全回転, 標本の回転, 標本の回転の本数, 画素位置から回転を選ぶ};
pub use same_surface_tolerance::同一面の許容倍率;
pub use sample_count::遮蔽の標本数;
pub use settings::局所可視性の設定;
pub use shader_set::局所可視性のシェーダー一式;
pub use synthetic_depth::合成深度を焼く;
pub use synthetic_scene::合成深度の場面;
pub use view_direction::{ビュー空間の向き, 外積の向き};
pub use view_displacement::ビュー空間の変位;
pub use view_position::{
    カメラからの奥行き, 位置が落ちる画素を求める, 画素が覆う幅を求める, 画素中心の位置を復元する
};
pub use visibility::局所可視度;
pub use visibility_image::局所可視度画像;
