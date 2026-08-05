//! 遠方環境から導く3つの派生表現の数学。拡散照度の立方体画像・鏡面畳込みの立方体画像・反射率積分表を、
//! 決定的な標本列で焼き上げる正本がここにある。Vulkanの資源も描画の順序も知らない純粋な計算だけである。
//!
//! CPU側に正本を置くのは、定数の環境を入れたときの解析解(拡散は円周率×放射輝度、鏡面はどの粗さ段も放射輝度そのもの)や
//! 標本数を上げたときの収束といった性質を、GPUを起動せずに検査できるようにするためである。
//! コンピュートシェーダーはこの式の写しであり、写しが正本からずれる欠陥は読み戻し検査だけが捉えられる。
//!
//! 内側は3層に分かれる。量と方針(一辺・粗さ・段・3つの解像度・遠方環境の内容)、標本の道具
//! (ハンマーズリー列・接基底・GGXの半ベクトル・立方体テクセルの参照)、焼き上げ(拡散照度・鏡面畳込み・反射率積分表)であり、
//! 依存はこの並びの向きにだけ流れる。
//! 参照: `_doc/設計/放射輝度問い合わせ階層.md`「派生表現(3-Ib)」

mod brdf_axis;
mod brdf_integration;
mod brdf_resolution;
mod check_environment;
mod cube_side;
mod cube_texel_lookup;
mod diffuse_irradiance;
mod diffuse_resolution;
mod environment_content;
mod error;
mod ggx_sample;
mod hammersley;
mod mip_level;
mod roughness;
mod specular_prefilter;
mod specular_resolution;
mod spherical_direction;
mod tangent_basis;

#[cfg(test)]
mod derived_tests;

pub use brdf_axis::{法線と視線の余弦を求める, 粗さを求める};
pub use brdf_integration::{反射率積分表のテクセル値, 反射率積分表の標本数, 反射率積分表を焼く, 積分する};
pub use brdf_resolution::反射率積分表の解像度;
pub use check_environment::{定数の遠方環境, 方向性の強い遠方環境};
pub use cube_side::立方体画像の一辺;
pub use cube_texel_lookup::{向きから立方体テクセルを求める, 立方体テクセルの通し番号};
pub use diffuse_irradiance::{拡散照度のテクセル値, 拡散照度の標本数, 拡散照度を焼く};
pub use diffuse_resolution::拡散照度の解像度;
pub use environment_content::遠方環境の内容;
pub use error::派生表現エラー;
pub use hammersley::ハンマーズリー標本を求める;
pub use mip_level::粗さ段;
pub use roughness::粗さ;
pub use specular_prefilter::{鏡面畳込みのテクセル値, 鏡面畳込みの標本数, 鏡面畳込みの段を焼く};
pub use specular_resolution::鏡面畳込みの解像度;
pub use tangent_basis::{接基底の向きを戻す, 接基底を組む};
