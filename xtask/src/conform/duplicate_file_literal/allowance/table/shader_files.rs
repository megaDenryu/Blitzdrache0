//! シェーダーの綴りのうち、正本を1箇所へ寄せられないものの台帳。ビルドスクリプトの入力と出力の名前、
//! およびホットリロードが一時ディレクトリへ焼く同名の成果物がここに並ぶ。
//! 判定の手順は`allowance`が、台帳の型と束ねは親が持つ。

use super::寄せられない綴り;

const ビルドスクリプトとの契約: &str = "blitz_appのビルドスクリプトが読む入力の名前であり、本体も同じ入力を実行中に読む。build_supportはビルドスクリプトのモジュールで本体から参照できないため、両側が同じ綴りを持つほかない";
const ホットリロードの一時出力: &str = "綴りは同じだが別のファイルを指す。ビルドスクリプトの出力はOUT_DIRへ焼く成果物、ホットリロードの出力は実行中に一時ディレクトリへ焼く成果物であり、どちらの綴りを変えても他方は壊れない";

const 焼く側の入口: &str = "crates/blitz_app/build_support/spirv_compile.rs";
const 取り込む側の入口: &str = "crates/blitz_app/src/embedded_shaders/scene_shaders.rs";
const ホットリロードの入口: &str = "crates/blitz_app/src/hot_reload/compile/fragment_contract.rs";

/// 注意: この一覧への追加は、正本を1箇所へ寄せられないと示せたときだけ許す。減らす方向にのみ動かす。
pub(super) const 一覧: [寄せられない綴り; 9] = [
    寄せられない綴り {
        綴り: "vertex.spv",
        現れてよい場所一覧: &[焼く側の入口, 取り込む側の入口, "crates/blitz_app/src/hot_reload/compile.rs"],
        寄せられない理由: ホットリロードの一時出力,
    },
    寄せられない綴り {
        綴り: "fragment.spv",
        現れてよい場所一覧: &[焼く側の入口, 取り込む側の入口, ホットリロードの入口],
        寄せられない理由: ホットリロードの一時出力,
    },
    寄せられない綴り {
        綴り: "scene_distant_environment_fragment.spv",
        現れてよい場所一覧: &[焼く側の入口, 取り込む側の入口, ホットリロードの入口],
        寄せられない理由: ホットリロードの一時出力,
    },
    寄せられない綴り {
        綴り: "scene.slang",
        現れてよい場所一覧: &[
            "crates/blitz_app/build_support/mod.rs",
            "xtask/src/shader_copy.rs",
            "crates/blitz_app/src/cli/types/default.rs",
        ],
        寄せられない理由: "blitz_appのビルドスクリプトとxtaskが別のクレートに在り、xtaskからビルドスクリプトへ依存を張れない",
    },
    寄せられない綴り {
        綴り: "scene_surface_layer_fragment.spv",
        現れてよい場所一覧: &[焼く側の入口, 取り込む側の入口, ホットリロードの入口],
        寄せられない理由: ホットリロードの一時出力,
    },
    寄せられない綴り {
        綴り: "scene_surface_layer_distant_environment_fragment.spv",
        現れてよい場所一覧: &[焼く側の入口, 取り込む側の入口, ホットリロードの入口],
        寄せられない理由: ホットリロードの一時出力,
    },
    寄せられない綴り {
        綴り: "scene_distant_environment.slang",
        現れてよい場所一覧: &[焼く側の入口, ホットリロードの入口],
        寄せられない理由: ビルドスクリプトとの契約,
    },
    寄せられない綴り {
        綴り: "scene_surface_layer.slang",
        現れてよい場所一覧: &[焼く側の入口, ホットリロードの入口],
        寄せられない理由: ビルドスクリプトとの契約,
    },
    寄せられない綴り {
        綴り: "scene_surface_layer_distant_environment.slang",
        現れてよい場所一覧: &[焼く側の入口, ホットリロードの入口],
        寄せられない理由: ビルドスクリプトとの契約,
    },
];
