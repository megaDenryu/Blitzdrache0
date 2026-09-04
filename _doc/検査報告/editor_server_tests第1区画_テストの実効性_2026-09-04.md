# editor_server tests 第1区画 テストの実効性の検査

- 検査員: テスト実効性チェック員(sonnet・effort high)
- 対象: `crates/editor_server/tests/` の辞書順で先頭21ファイル
- 検査日: 2026-09-04
- 時点のコミット: `271e8530`
- 直読: 21 / 21(grepだけで済ませたファイル 0)
- 作業ツリーの汚染: **無し**

対象: `building_grid_routes.rs` `building_grid_save_failure.rs` `building_outline_catalog_contract.rs` `building_outline_catalog_routes.rs` `chunk_grid_routes.rs` `chunk_heightmap_mother_cutout.rs` `chunk_heightmap_mother_cutout_export.rs` `chunk_storage_roundtrip.rs` `chunk_structure_building_routes.rs` `chunk_structure_routes.rs` `contract_freshness.rs` `editor_world_grid_building_bake.rs` `editor_world_scatter_bake.rs` `legacy_structure_conversion.rs` `legacy_structure_overwrite.rs` `legacy_structure_without_scatter.rs` `material_board_routes.rs` `material_board_storage_roundtrip.rs` `music_command_note.rs` `music_command_pattern.rs` `music_command_section.rs`

## 総評

21ファイル中、実効性を欠く重大な違反は少ない。この区画は全体として、実装をなぞらず独立に導出した期待値を使い、失敗経路で「正本が書き換わっていないこと」まで確かめる規律が徹底されている。指摘は主に「HTTPを通す必要が無い」と「実行されない試験」の2点に集中する。

## 指摘1: 標準の検証で一度も走らない試験がある

`contract_freshness.rs:5,10-28` の2つの試験(`生存確認契約tsは組み立てた本文と一致する` と `編集資源契約tsは組み立てた本文と一致する`)。

`#![cfg(feature = "typescript")]` でゲートされているが、`cargo xtask verify` が実行する `cargo test --workspace`(`xtask/src/verify.rs:19`)は `--features typescript` を渡していない。`editor_server/Cargo.toml` で `typescript` は既定featureに含まれない(`Cargo.toml:27-28`)。

**この2つの試験は標準の検証列では一切コンパイルもされず実行されない。** `契約ファイルの本文を組み立てる()` を変更してTypeScript側の生成物の再生成を忘れても、`cargo xtask verify` はこの乖離を検出しない。`cargo xtask contract-export` を明示的に叩く別経路でしか機能しない。

## 指摘2: HTTPの口を通す必要が無い試験が3本ある

`music_command_note.rs` `music_command_pattern.rs` `music_command_section.rs` の3ファイル。

3ファイルとも `mod common` は使うが `common::ルーターを作る` を一切呼ばず、`axum`・`tower`・`tokio::test` も未使用である。`楽曲編集コマンド::検証する(&共通の楽曲フィクスチャ)` という純粋関数の呼び出しだけで完結する(フィクスチャの `楽曲の例()` もファイル入出力の無い純粋構築、`tests/common/music_fixture.rs:77-97`)。

同じクレート内の `resource/material_board.rs:53-98` に既に同型の `#[cfg(test)] mod tests` が存在し、先例がある。この3ファイルは独立した実行ファイル1本としてリンクされる費用を負っているが、HTTPの口を通す理由が無い。**`crates/editor_server/src/resource/music/command/` 配下の `#[cfg(test)]` へ移す候補である。**

## 指摘3: 保管庫の層で足りる試験が1本ある

`legacy_structure_overwrite.rs:39-62` の `移行できない旧版が残るチャンクへの保存は拒まれ正本は残る`。

拒否のロジックの実体は `storage/file_repository/chunk.rs:27-35`(`構造を検証して保存する` が上書き前に既存の正本を読み直して失敗を伝播する)にあり、HTTP層(`routes/chunk_structure_put.rs`)は関与しない。CONFLICTへのステータス変換は `storage/response.rs:34` の機械的な対応づけだけである。同じ拒否の筋書きは `legacy_structure_conversion.rs` が `保管庫を作る` 経由の直接呼び出しで既に検証している形式である。

## 境界事例(違反に数えない)

`material_board_storage_roundtrip.rs:51-58,60-70` と `crates/editor_server/src/resource/material_board.rs:81-98`。

`マテリアル台帳::検証する()` が拒む条件(重複した材質名・存在しない参照)は、既に `resource/material_board.rs` の `#[cfg(test)]` 内で同一のデータ形で確認済みである。ただし統合試験の側は追加で「保存が失敗したときファイルが書かれていないこと」を確かめており、これは `src` 側の単体試験には無い主張である。**検証の述語そのものの重複は実在するが、永続化の境界の主張が上乗せされているため、明確な水増しとは判定しない。**

## 反証性で問題無しと確認した代表例

- `building_grid_save_failure.rs:23-56`: `建物の格子の保存係::検査して保存する` の書き込みの順序(格子ファイル→カタログファイル→台帳の確定、`building_grid_store.rs:71-83`)を実際に読み、カタログのパスへディレクトリを置いて書き込みを強制的に失敗させる形が、書き込みの順序を入れ替える改変を実際に検出できることを確認した
- `chunk_storage_roundtrip.rs:54-82`: `なじみ半径 < 基礎半径` の拒否は `resource/building.rs:36-41` に実在する不変条件であり、比較の演算子を反転させれば落ちる
- `chunk_structure_building_routes.rs`: カタログの照合は `routes/chunk_structure_put.rs:32-39` というHTTPのハンドラ内にしか存在しない。HTTPが必要と判定した
- `chunk_heightmap_mother_cutout.rs` と `_export.rs`: 実装の内部の計算を書き写さず「同じ大域の格子点を指すか」という独立の主張で確認しており、座標変換の式を壊せば落ちる
- `editor_world_scatter_bake.rs`: `個体の水平位置一覧` は東西と南北の符号の取り違えを検出するよう意図的に非対称へ配置されたフィクスチャである(`tests/common/scatter_fixture.rs:1-5,27-28`)
- `building_outline_catalog_contract.rs`: 定義の件数5と家屋の件数3は生成元(`definition_source.rs`)の実データと一致させた値であり、クレートの境界を跨いだ生産側と消費側の型の食い違いを検出する

## 重複が無いことを確認した組

- `material_board_routes.rs`(HTTP層)と `material_board_storage_roundtrip.rs`(保管庫層)は主張が異なる
- `chunk_grid_routes.rs` と `chunk_storage_roundtrip.rs` は同じ往復を別の境界で確認しており、層ごとの正当な二重化である
- `legacy_structure_conversion.rs` と `_without_scatter.rs` と `_overwrite.rs` は旧版の形がそれぞれ異なる

## 是正のissue

[#60 統合試験の実行ファイル62本を2本へまとめ、tests/直下の分割の費用を規約へ書く](https://github.com/megaDenryu/Blitzdrache0/issues/60)(指摘1・2・3を含む)
