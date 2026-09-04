# xtask conform 第1区画 行数と分割の質の検査

- 検査員: 行数と分割の質チェック員(sonnet・effort medium)
- 対象: `xtask/src/conform/` 配下、パスの辞書順で先頭40ファイル
- 検査日: 2026-09-04
- 時点のコミット: `271e8530`
- 直読: 40 / 40(行数を数えただけのファイル 0)
- 作業ツリーの汚染: **無し**

`xtask` は既存識別子のローラー検査自体が未実施の領域であり、この軸(2026-08-27制定)は台帳に表が無く全域が未検査であった。

## 総括

対象40ファイルすべてを直読し、コードの行(空行とコメントだけの行を除く)を数えた。**100行を超えるファイルは1件も無い**(最大は `duplicate_file_literal/tests.rs` の83行)。よって超過と超過の根拠の軸は該当なしであり、判定の主軸は分割の質になる。

## 判定結果(40件全件)

| ファイル | コード行数 | 判定 | 根拠 |
| --- | ---: | --- | --- |
| allow_lint.rs | 60 | 合法 | 不正なallowの検出という単一の責務。分割なし |
| cargo_toml_parse.rs | 74 | 合法 | Cargo.tomlの最小の構文解析という単一の責務 |
| declaration_comment_line.rs | 57 | 合法 | 宣言の間のコメントの検査の本体 |
| declaration_comment_line/tests.rs | 54 | 合法 | Rustの標準の形。固定したい仕様(注釈の例外の境界)を独立して所有 |
| dependency_whitelist.rs | 48 | 合法 | 検査の走査と実行の工程 |
| dependency_whitelist/ledger.rs | 58 | 合法(宣言的データの別枠) | クレートごとの許可依存の宣言的な台帳。触る理由(依存の追加)が工程と別 |
| depth_contract.rs | 29 | 合法 | 接点の照合の工程の本体 |
| depth_contract/table.rs | 39 | 合法 | 深度の契約2種の型と束ね |
| depth_contract/table/camera.rs | 63 | 合法(別枠) | カメラの逆向き深度のうち消去とNDC端点の契約の宣言的データ |
| depth_contract/table/camera_compare.rs | 63 | 合法(別枠) | 同じくカメラの比較演算子と奥行きの復元の契約。camera.rsとは扱う接点の性質が異なる |
| depth_contract/table/shadow.rs | 45 | 合法(別枠) | 光源の影の深度という別のドメインの契約データ |
| depth_contract/tests.rs | 16 | 合法 | 綴りの照合のロジックの固定 |
| doc_reference.rs | 57 | 合法 | 参照パスの実在の検査という単一の責務 |
| doc_section.rs | 76 | 合法 | 節の参照の実在の検査という単一の責務 |
| drop_impl.rs | 63 | 合法 | Drop実装の配置の検査という単一の責務 |
| duplicate_file_literal.rs | 65 | 合法 | 差配の本体。9個の子モジュールへ委譲するが、各子は独立した責務を持つ |
| duplicate_file_literal/allowance.rs | 49 | 合法 | 寄せられない綴りの許可の判定という工程 |
| duplicate_file_literal/allowance/build_output.rs | 14 | 合法 | ビルド成果物の対の判定という独立した規則 |
| duplicate_file_literal/allowance/table.rs | 9 | 合法 | 台帳の型と領域の束ねのみ |
| duplicate_file_literal/allowance/table/other_files.rs | 22 | 合法(別枠) | シェーダー以外の寄せられない綴りの宣言的データ |
| duplicate_file_literal/allowance/table/shader_files.rs | 57 | 合法(別枠) | シェーダー関連の宣言的データ(other_filesとはドメインが別) |
| duplicate_file_literal/builtin_include_bytes.rs | 28 | 合法 | 名前の乗っ取りの宣言の検出という別の責務 |
| duplicate_file_literal/builtin_include_bytes_tests.rs | 29 | 合法 | 上記の固定 |
| duplicate_file_literal/extract.rs | 16 | 合法 | 拡張子の検出という独立した責務 |
| duplicate_file_literal/include_bytes_argument.rs | 14 | 合法 | 引数の区間の判定の工程 |
| duplicate_file_literal/include_bytes_argument/code_sequence.rs | 61 | 合法 | 独自の状態を持つ型(文字列の走査の緩衝)を所有。3段目の入れ子だが親を丸ごと受け取らず自己完結 |
| duplicate_file_literal/scan_scope.rs | 10 | 合法 | 走査の範囲の決定という独立した責務 |
| duplicate_file_literal/self_reference.rs | 77 | 合法 | 台帳のファイル自身を数から除く台帳と、その陳腐化の検査という独立した責務 |
| duplicate_file_literal/tally.rs | 50 | 合法 | 集計の状態を持つ型を所有 |
| duplicate_file_literal/test_item_skip.rs | 40 | 合法 | `#[cfg(test)]` の範囲の判定という独立した責務 |
| duplicate_file_literal/tests.rs | 83 | 合法 | 規則だけを見る固定 |
| error.rs | 76 | 合法 | 型付きエラーの列挙の定義 |
| error/display.rs | 32 | 合法 | パーシャル規約 条1(外部トレイト `Display` の実装の分離)に明確に該当し、冒頭コメントに根拠を明記 |
| forbidden_strings.rs | 76 | 合法 | 禁止語・経緯語・絵文字の検出という単一の責務 |
| free_function_whole_type.rs | 22 | 合法 | 差配の本体 |
| free_function_whole_type/index.rs | 57 | 合法 | 型の定義の索引と判定の操作を持つ型を所有 |
| free_function_whole_type/ledger.rs | 62 | 合法 | 台帳の照合という独立した工程 |
| free_function_whole_type/ledger/entry.rs | 37 | 合法 | 台帳の値オブジェクトを所有。冒頭コメントに1行に収める理由を明記 |
| free_function_whole_type/ledger/table.rs | 19 | 合法 | 7区画の束ねのみ。区画分けの理由を明記 |
| free_function_whole_type/ledger/table/blitz_app_app.rs | 49 | 合法(別枠) | 名前の付く未是正領域の宣言的な台帳 |

区画外だが判定の根拠として直読したもの(任意参照): `module_tree.rs`・`parameter.rs`・`signature.rs`・`free_function_whole_type/tests.rs`。いずれも独立した責務であり、`index.rs` が委譲するだけの薄いラッパー群ではないことを確認した。

## 深い入れ子についての判定

`duplicate_file_literal/` と `free_function_whole_type/ledger/` の2つのまとまりについて、入れ子の深さが責務の所有によるものか、行数に収めるためのものかを個別に見た。

`duplicate_file_literal` は「走査の範囲」「拡張子の抽出」「取り込みの引数の区間の判定(さらに内部で文字列の走査の緩衝の型を持つ)」「試験項目の除外」「許可の台帳」「自己参照の台帳」「集計の状態」の7つから9つが、それぞれ触れる状態と入出力を持って独立している。

`free_function_whole_type/ledger/` の3段(`ledger.rs` → `entry.rs` と `table.rs` → `table/*.rs`)は「照合の工程」「値オブジェクト」「クレート別の区画のデータ」の層に対応し、`table/*.rs` はいずれも別の作業の単位(是正の対象の領域)を指す名の付く概念であって連番の分割ではない。

**行数に収めるためだけの分割ではなく、責務の所有によるものと判定する。**

## 指摘

**指摘なし。** 40ファイル全件が合法(責務を所有する側)と判定した。

境界事例として `depth_contract/table/camera.rs` と `camera_compare.rs` の2ファイルを挙げた。合算しても100行程度に収まるが、扱う接点の性質(定数の端点と、比較の演算子および復元の式)が異なる名の付く責務であるため違反には数えない。

## 親による裏取り

親のセッションが同じ数え方で機械的に数え直し、報告の行数と一致することを確認した(最大83行)。
