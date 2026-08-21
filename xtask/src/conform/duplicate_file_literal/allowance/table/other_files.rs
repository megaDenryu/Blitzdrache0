//! シェーダー以外で、正本を1箇所へ寄せられない綴りの台帳。判定の手順は`allowance`が、台帳の型と束ねは親が持つ。

use super::寄せられない綴り;

const ビルドスクリプトとの契約: &str = "blitz_appのビルドスクリプトが読む入力の名前であり、本体も同じ入力を実行中に読む。build_supportはビルドスクリプトのモジュールで本体から参照できないため、両側が同じ綴りを持つほかない";
const 書き手が非公開: &str = "blitz_asset_compilerのworld_source_directoryが持つ正本の定数はpub(super)で非公開であり、editor_serverクレートから届かない。blitz_*クレート本体の変更はゲーム開発用エディター段の対象外(参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断5」)であるため、editor_server側に同じ綴りの書き手を独立に持つ";

/// 注意: この一覧への追加は、正本を1箇所へ寄せられないと示せたときだけ許す。減らす方向にのみ動かす。
pub(super) const 一覧: [寄せられない綴り; 2] = [
    寄せられない綴り {
        綴り: "slangc.exe",
        現れてよい場所一覧: &[
            "crates/blitz_app/build_support/slangc_locate.rs",
            "crates/blitz_app/src/hot_reload/slangc.rs",
        ],
        寄せられない理由: ビルドスクリプトとの契約,
    },
    寄せられない綴り {
        綴り: "chunk_directory.txt",
        現れてよい場所一覧: &[
            "crates/blitz_asset_compiler/src/asset_layout/world_source_directory.rs",
            "crates/editor_server/src/export/chunk_directory_text.rs",
        ],
        寄せられない理由: 書き手が非公開,
    },
];
