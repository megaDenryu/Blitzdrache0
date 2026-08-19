//! 骨格方式の部品と接合点の綴りの正本。Blender側が骨格1件と壁3種へ与えた名前を、この1箇所だけが持つ。
//!
//! 3つの手順(壁をはめた1棟・2段に積んだ骨格・横に2ベイ継いだ骨格)から分けるのは、綴りが3つへ写ると、
//! Blender側が名前を変えたときに1つだけ直し忘れる形の食い違いが起きるためである。
//!
//! **ベイの面が2つの口の名前を答えるのは、同じ物理面へ壁のはめ口とベイの継ぎ口が重ねて宣言されているためである。**
//! 面を選べば両方の綴りが決まるため、正面の壁を背面の継ぎ口へ繋ぐ取り違えを手順が書けない。
//! 参照: `_doc/設計/部品カタログと接合点.md`「役割「相互」」

use blitz_assembly::{接合点名, 部品ID};

const 骨格の綴り: &str = "Mod_Frame_Bay_Single";
const 平壁の綴り: &str = "Mod_Wall_HalfTimber_Solid";
const 窓壁の綴り: &str = "Mod_Wall_HalfTimber_Window";
const 扉枠付きの壁の綴り: &str = "Mod_Wall_HalfTimber_DoorFrame";

/// 骨格の4つの側面。1つの面が壁のはめ口とベイの継ぎ口を重ねて持ち、どちらか一方だけを使う。
#[derive(Debug, Clone, Copy)]
pub(super) enum ベイの面 {
    正面,
    背面,
    左面,
    右面,
}

impl ベイの面 {
    pub(super) fn 壁のはめ口の接合点名(self) -> Result<接合点名, String> {
        接合点名を作る(match self {
            Self::正面 => "正面のはめ口",
            Self::背面 => "背面のはめ口",
            Self::左面 => "左面のはめ口",
            Self::右面 => "右面のはめ口",
        })
    }

    pub(super) fn ベイの継ぎ口の接合点名(self) -> Result<接合点名, String> {
        接合点名を作る(match self {
            Self::正面 => "正面の継ぎ口",
            Self::背面 => "背面の継ぎ口",
            Self::左面 => "左面の継ぎ口",
            Self::右面 => "右面の継ぎ口",
        })
    }
}

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

fn 接合点名を作る(綴り: &str) -> Result<接合点名, String> {
    接合点名::生成する(綴り).map_err(|誤り| 誤り.to_string())
}

fn 部品idを作る(綴り: &str) -> Result<部品ID, String> {
    部品ID::生成する(綴り).map_err(|誤り| 誤り.to_string())
}
