//! 寄せられない綴りと、その綴りが現れてよい場所と、載せた理由の台帳。
//! 判定の手順は親モジュールが持ち、どの綴りをどこまで許すかはこの台帳だけが決める。

/// この台帳自身の置き場所。違反の報告先として親が使う。
pub(super) const 台帳のファイル: &str = "xtask/src/conform/duplicate_file_literal/allowance/table.rs";

/// 寄せられない綴り1件。理由を綴りと並べて持つのは、台帳を減らすときに
/// 「何が変われば消せるのか」を読み手が判断できるようにするためである。
pub(super) struct 寄せられない綴り {
    pub(super) 綴り: &'static str,
    /// この綴りが現れてよいファイル。ここに無い場所へ書かれたら許さず、ここから消えたら陳腐化として報告する。
    pub(super) 現れてよい場所一覧: &'static [&'static str],
    pub(super) 寄せられない理由: &'static str,
}

const ビルドスクリプトとの契約: &str = "blitz_appのビルドスクリプトが読む入力の名前であり、本体も同じ入力を実行中に読む。build_supportはビルドスクリプトのモジュールで本体から参照できないため、両側が同じ綴りを持つほかない";
const ホットリロードの一時出力: &str = "綴りは同じだが別のファイルを指す。ビルドスクリプトの出力はOUT_DIRへ焼く成果物、ホットリロードの出力は実行中に一時ディレクトリへ焼く成果物であり、どちらの綴りを変えても他方は壊れない";

const 焼く側の入口: &str = "crates/blitz_app/build_support/spirv_compile.rs";
const 取り込む側の入口: &str = "crates/blitz_app/src/embedded_shaders/scene_shaders.rs";

/// 注意: この一覧への追加は、正本を1箇所へ寄せられないと示せたときだけ許す。減らす方向にのみ動かす。
pub(super) const 許した綴り一覧: [寄せられない綴り; 6] = [
    寄せられない綴り {
        綴り: "vertex.spv",
        現れてよい場所一覧: &[焼く側の入口, 取り込む側の入口, "crates/blitz_app/src/hot_reload/compile.rs"],
        寄せられない理由: ホットリロードの一時出力,
    },
    寄せられない綴り {
        綴り: "fragment.spv",
        現れてよい場所一覧: &[
            焼く側の入口,
            取り込む側の入口,
            "crates/blitz_app/src/hot_reload/compile/fragment_contract.rs",
        ],
        寄せられない理由: ホットリロードの一時出力,
    },
    寄せられない綴り {
        綴り: "scene_distant_environment_fragment.spv",
        現れてよい場所一覧: &[
            焼く側の入口,
            取り込む側の入口,
            "crates/blitz_app/src/hot_reload/compile/fragment_contract.rs",
        ],
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
        綴り: "scene_distant_environment.slang",
        現れてよい場所一覧: &[
            "crates/blitz_app/build_support/mod.rs",
            "crates/blitz_app/src/hot_reload/compile/fragment_contract.rs",
        ],
        寄せられない理由: ビルドスクリプトとの契約,
    },
    寄せられない綴り {
        綴り: "slangc.exe",
        現れてよい場所一覧: &[
            "crates/blitz_app/build_support/slangc_locate.rs",
            "crates/blitz_app/src/hot_reload/slangc.rs",
        ],
        寄せられない理由: ビルドスクリプトとの契約,
    },
];
