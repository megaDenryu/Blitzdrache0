//! 骨格方式の部品と、面に属さない接合点の綴りの正本。Blender側が骨格1件と壁3種と切妻屋根と床板へ与えた
//! 名前を、この1箇所だけが持つ。骨格の側面と2つの口の対応は`frame_bay_face`が持つ。
//!
//! 手順(`frame_recipe`)から分けるのは、綴りが手順の数だけ写ると、Blender側が名前を変えたときに
//! 1つだけ直し忘れる形の食い違いが起きるためである。
//! 参照: `_doc/設計/部品カタログと接合点.md`「段7: 一間四方の骨格へ壁をはめる」

use blitz_assembly::{接合点名, 部品ID};

const 骨格の綴り: &str = "Mod_Frame_Bay_Single";
const 平壁の綴り: &str = "Mod_Wall_HalfTimber_Solid";
const 窓壁の綴り: &str = "Mod_Wall_HalfTimber_Window";
const 扉枠付きの壁の綴り: &str = "Mod_Wall_HalfTimber_DoorFrame";
const 切妻屋根の綴り: &str = "Mod_Frame_Roof_Gable";
const 床板の綴り: &str = "Mod_Frame_Floor";

/// はめ口へ入れる壁の種類。壁3種の外枠は同じ位置と姿勢と寸法で宣言されているため、
/// 接合には現れず、違うのはメッシュと材質スロットの数だけである。
#[derive(Debug, Clone, Copy)]
pub(super) enum はめる壁の種類 {
    平壁,
    窓壁,
    扉枠付きの壁,
}

impl はめる壁の種類 {
    pub(super) fn 部品id(self) -> Result<部品ID, String> {
        部品idを作る(match self {
            Self::平壁 => 平壁の綴り,
            Self::窓壁 => 窓壁の綴り,
            Self::扉枠付きの壁 => 扉枠付きの壁の綴り,
        })
    }
}

pub(super) fn 骨格の部品id() -> Result<部品ID, String> {
    部品idを作る(骨格の綴り)
}

/// 1ベイぶんの切妻屋根。宣言する接合点は下面積層1件だけであり、骨格の上面へ載ることしかできない。
pub(super) fn 切妻屋根の部品id() -> Result<部品ID, String> {
    部品idを作る(切妻屋根の綴り)
}

/// 1ベイぶんの床板。宣言する接合点は床の外枠1件だけであり、骨格の床のはめ口へ収まることしかできない。
pub(super) fn 床板の部品id() -> Result<部品ID, String> {
    部品idを作る(床板の綴り)
}

pub(super) fn 屋根の下面の接合点名() -> Result<接合点名, String> {
    接合点名を作る("屋根の下面")
}

/// 壁3種が共通で持つ、骨格のはめ口と噛み合う接合点の名前。
pub(super) fn 壁の外枠の接合点名() -> Result<接合点名, String> {
    接合点名を作る("壁の外枠")
}

pub(super) fn 骨格の上面の接合点名() -> Result<接合点名, String> {
    接合点名を作る("骨格の上面")
}

pub(super) fn 骨格の下面の接合点名() -> Result<接合点名, String> {
    接合点名を作る("骨格の下面")
}

/// 骨格が土台の梁の上面へ宣言する、床板を受ける口。
pub(super) fn 床のはめ口の接合点名() -> Result<接合点名, String> {
    接合点名を作る("床のはめ口")
}

pub(super) fn 床の外枠の接合点名() -> Result<接合点名, String> {
    接合点名を作る("床の外枠")
}

pub(super) fn 接合点名を作る(綴り: &str) -> Result<接合点名, String> {
    接合点名::生成する(綴り).map_err(|誤り| 誤り.to_string())
}

fn 部品idを作る(綴り: &str) -> Result<部品ID, String> {
    部品ID::生成する(綴り).map_err(|誤り| 誤り.to_string())
}
