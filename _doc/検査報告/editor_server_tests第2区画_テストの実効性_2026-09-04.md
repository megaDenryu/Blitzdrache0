# editor_server tests 第2区画 テストの実効性の検査

- 検査員: テスト実効性チェック員(sonnet・effort high)
- 対象: `crates/editor_server/tests/` の辞書順で後半21ファイル
- 検査日: 2026-09-04
- 時点のコミット: `271e8530`
- 直読: 21 / 21(grepだけで済ませたファイル 0)
- 作業ツリーの汚染: **無し**

対象: `music_command_song_and_track.rs` `music_preset_progression.rs` `music_routes.rs` `music_storage_roundtrip.rs` `music_validation_basic.rs` `music_validation_pattern.rs` `music_validation_progression.rs` `music_validation_track.rs` `music_value_range_contract.rs` `plan_view_command_serde.rs` `plan_view_draft_structure.rs` `project_info_routes.rs` `source_asset_export_building_http.rs` `source_asset_export_error_http.rs` `source_asset_export_geometry.rs` `source_asset_export_http.rs` `source_asset_export_scatter_http.rs` `source_asset_export_surface_layers_http.rs` `world_heightmap_routes.rs` `world_storage_roundtrip.rs` `world_structure_routes.rs`

## 設計正本との突き合わせ

読んだ設計正本は次の3つである。`_doc/設計/楽曲エディター.md`「検収」節、`_doc/設計/見下ろし図による地形編集.md`「検収の形」節、`_doc/設計/ゲーム開発用エディター基盤.md`「判断10・判断11」。

両文書の「検収の形」が要求するRust側の不変条件(楽曲の保存の往復、見下ろし図の下書きを持つチャンク構造の保存の往復、旧版の空の下書き化、粗マスの一辺の割り切りの拒否)は、いずれも本区画の試験で固定されていることを確認した。

楽曲エディターの残り5項目と、見下ろし図の「編集モデルの単体試験」「操作コマンドの適用と差し戻し」は、`src` 側にRustの実装が無い(`plan_view_ops.rs` が「適用と差し戻しの実装はTS側の編集モデルが持つ」と明記している)。editor_web(TypeScript)側の責務であるため、本区画の未固定としては扱わない。

## 指摘1: 名前が主張する内容を確かめていない試験がある

`music_preset_progression.rs:51-55` の `表示名は識別子で始まり和音の並びを添える`。

試験の名前は「和音の並びを添える」ことを主張するが、本文は `進行.表示名.starts_with(&進行.識別子)` しか検証していない。表示名の実体(`src/resource/music/preset_progression/table.rs:65` 等)は括弧の中へ和音の進行を書き出しているが、**この括弧の中の文字列は一切読まれていない。**

壊し方: `table.rs` の表示名の括弧の中を無関係な文字列へ書き換えても、識別子の前方一致さえ保たれていればこの試験は通り続ける。

## 指摘2: 同じ経路を2箇所で確かめている

`music_value_range_contract.rs:47-57` の `書き出したテンポの境目は検証の境目と一致する` と、`music_validation_basic.rs:12-19`・`:21-28`。

確かめている対象(`楽曲.検証する()` を40・300・39・301で呼ぶ)とコードの経路が完全に一致する。片方を削除しても検出の力は落ちない。ただし同じファイルの他の主張(定数が `編集資源契約の本文を組み立てる()` の出力の文字列に含まれることの検査)は独自の価値を持ち、この重複は「境目の一致」を確かめる1つの関数だけに限られる。

## 指摘3: HTTPの口が不要な試験が8本ある

8ファイルは `axum` のルーター・`tower::ServiceExt::oneshot`・`一時プロジェクト`(ファイルシステム)のいずれも使わず、`editor_server` の公開関数をプロセス内で直接呼ぶだけである(`oneshot` の出現件数が全て0であることを確認した)。

- `music_command_song_and_track.rs` — `楽曲編集コマンド::検証する()` を直接呼ぶだけ
- `music_preset_progression.rs` — `既定のコード進行一覧()` を直接呼ぶだけ
- `music_validation_basic.rs` / `music_validation_pattern.rs` / `music_validation_progression.rs` / `music_validation_track.rs` — `common::楽曲の例().検証する()` を直接呼ぶだけ
- `music_value_range_contract.rs` — `編集資源契約の本文を組み立てる()` と `検証する()` を直接呼ぶだけ(HTTPは不要で、`typescript` featureだけが要る)
- `plan_view_command_serde.rs` — `serde_json::to_value` と `from_value` の往復のみ(`一時プロジェクト` すら使わない)

これら8ファイルは `crates/editor_server/src/resource/music/` 配下と `src/resource/command/plan_view_ops.rs` の付近へ、`#[cfg(test)] mod tests` として移設できる。

ただし `music_command_song_and_track.rs` の各試験は、対応する `楽曲編集コマンド` の `検証する()` の実装(`song_settings_ops.rs`・`track_ops.rs`)が委譲先の関数を呼び忘れる退行だけを捕まえる独自の価値を持つ。`music_validation_*.rs` と統合はできず、別建てのまま(ただし単体試験として)残すべきである。

## 反証性と境界の要否を満たすと確認した主な例

- `source_asset_export_geometry.rs`: 隣り合うチャンクの境界の頂点の同値・世界の端での切り詰め・チャンク別の高さ格子のマザーへの優先の3件は、いずれも壊し方を1行で書ける。HTTPを経由しないのは意図的であり、冒頭のコメントに「RESTの応答の形は `source_asset_export_http.rs` が担当する」と明記されている
- `source_asset_export_building_http.rs`: 建物をY=999という異常な値で配置し、焼いた全部の配置のYが100未満であることを検証する形は、「保存したY成分を使わず高さ格子を参照して焼く」(判断10)を直接崩す設計になっている
- `source_asset_export_surface_layers_http.rs`: 既定の並び(草→泥→岩→砂)のままでは通ってしまう検査にならないよう、草と岩を意図的に入れ替えたデータで検証している(コメントで明言している)。模擬物を使わず、実際のファイルシステムと実際の `blitz_engine` の読み込みで確認している
- `source_asset_export_error_http.rs`: 出力先のディレクトリの位置に通常のファイルを置いて500の応答を強制する形は、実際の入出力の境界の失敗を偽物なしで再現している

## 決定性と分離

乱数を使う散布まわりのフィクスチャは全て固定の種(1・3・7・9)を使っており、非決定性は無い。時刻・並行処理・浮動小数の丸めに依存する試験は、この21ファイルの中に見当たらない(浮動小数の比較は `source_asset_export_scatter_http.rs` の許容差 `1e-4` 付きの比較だけであり、単精度の往復の丸めに対して妥当な範囲である)。

各 `#[test]` が `一時プロジェクト::生成する(識別子)` で個別の一時ディレクトリを作る設計であり、実行の順序に依存する箇所は見当たらない。

## 境界事例(違反に数えない)

`music_command_song_and_track.rs` の各コマンドの検証の試験と、`music_validation_*.rs` の対応する構造体の検証の試験は、最終的に同じ内部の関数へ到達する。ただし呼び出しの経路(コマンド経由と楽曲全体経由)が異なり、コマンド側の委譲の忘れという別の退行を捕まえるため、重複とは判定しなかった。

## 是正のissue

[#60 統合試験の実行ファイル62本を2本へまとめ、tests/直下の分割の費用を規約へ書く](https://github.com/megaDenryu/Blitzdrache0/issues/60)
