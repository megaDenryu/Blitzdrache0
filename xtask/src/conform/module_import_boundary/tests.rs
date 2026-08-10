//! 取り込みの境界の検査が、台帳の許した1箇所とそれ以外を分けることの固定。
//! 材質のシェーダーへ取り込みを1行足した時点で落ちることが、この境界が機械の側にあることの根拠である。

use std::path::Path;

use super::検査する;

fn 違反の数(パス: &str, 内容: &str) -> usize {
    検査する(Path::new(パス), 内容).len()
}

#[test]
fn 材質のシェーダーが束縛先を取り込むと違反になる() {
    assert_eq!(違反の数("shaders/direct_light_shading.slang", "import local_light_binding;\n"), 1);
    assert_eq!(違反の数("shaders/cloth_shading.slang", "import local_light_binding;\n"), 1);
    assert_eq!(違反の数("shaders/scene.slang", "import local_light_records;\n"), 1);
}

#[test]
fn 台帳が許した1箇所からの取り込みは違反にしない() {
    assert_eq!(違反の数("shaders/local_light_shading.slang", "import local_light_binding;\n"), 0);
    assert_eq!(
        違反の数("shaders/local_light_binding.slang", "__exported import local_light_records;\n"),
        0
    );
    assert_eq!(違反の数("shaders/cluster_light_assignment.slang", "import local_light_records;\n"), 0);
}

/// 再輸出は取り込みの並びを変えずに境界だけを消す。許した1箇所からでも落とす。
#[test]
fn 許した1箇所からでも束縛先の再輸出は違反になる() {
    assert_eq!(
        違反の数("shaders/local_light_shading.slang", "__exported import local_light_binding;\n"),
        1
    );
}

#[test]
fn 台帳に無いモジュールの取り込みは見ない() {
    assert_eq!(違反の数("shaders/scene.slang", "import lighting_query;\nimport pbr;\n"), 0);
}

/// 走査が返すパスの区切りは環境で変わる。台帳はスラッシュで書くため、そろえてから突き合わせる。
#[test]
fn 区切りが円記号のパスでも台帳と突き合わせる() {
    assert_eq!(違反の数(r"shaders\local_light_shading.slang", "import local_light_binding;\n"), 0);
    assert_eq!(違反の数(r"shaders\scene.slang", "import local_light_binding;\n"), 1);
}
