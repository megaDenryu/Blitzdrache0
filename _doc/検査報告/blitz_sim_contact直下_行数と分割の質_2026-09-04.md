# blitz_sim contact 直下 行数と分割の質の検査

- 検査員: 行数と分割の質チェック員(sonnet・effort medium)
- 対象: `crates/blitz_sim/src/contact/` の直下にある37個の `.rs`(子ディレクトリは対象外)
- 検査日: 2026-09-04
- 時点のコミット: `271e8530`
- 直読: 37 / 37(行数を数えただけのファイル 0)

## この検査は共有した作業ツリーの上で行われた

**検査の時点で、`crates/blitz_sim/src/contact/` に Issue #40 の3つ目の作業の未コミットの差分が載っていた**(37ファイル中13ファイルが未コミットまたは未追跡)。この報告の冒頭に書いた時点のコミットは、実際に検査した内容と一致しない。

**この領域は [#67](https://github.com/megaDenryu/Blitzdrache0/pull/67) が `main` へ入った後に再検査が要る。**

## この軸は台帳に表が無かった

1ファイル100行の原則と分割の質の軸は2026-08-27に制定されたが、`_doc/計画/検査の網羅台帳.md` にその表が無く、全域が未検査であった。本検査がこの軸の最初の記録である。

## 判定結果(37件全件)

| ファイル | コードの行数 | 判定 | 根拠 |
| --- | ---: | --- | --- |
| mod.rs | 91 | 合法 | モジュールの宣言と再輸出のみ。契約の一覧を1箇所に持つ責務 |
| batch_builder.rs | 75 | 合法 | 生成の局面(受け皿の確保)を担当。ライフサイクル局面の分離(条2) |
| batch_builder_push.rs | 82 | 合法 | 積む局面を担当。上と局面が異なる(生成1回 に対して候補の対ごとに複数回)。`&mut self` のみを受け親型を丸ごとは受けない |
| batch_generation_order_tests.rs | 26 | 合法 | 参加者の順の不変条件だけを固定する試験 |
| batch_generation_static_tests.rs | 64 | 合法 | 剛体と静的世界の生成の性質を固定する試験 |
| batch_generation_tests.rs | 87 | 合法 | 剛体どうしの生成の性質を固定する試験 |
| body_body_contact_parameters.rs | 18 | 合法 | 値オブジェクト(細分の間で不変の引数) |
| body_static_contact_parameters.rs | 19 | 合法 | 同上、静的世界の版 |
| contact_batches.rs | 72 | 合法 | 2つのバッチを持つ集約型 |
| contact_property.rs | 28 | 合法 | 値オブジェクト |
| contact_property_limits.rs | 18 | 合法 | 接触物性への追加の impl。触れるのは摩擦係数2つだけに限定(条3) |
| contact_test_fixtures.rs | 93 | 合法 | 生成の試験が共有する材料の工房 |
| contact_thresholds.rs | 20 | 合法 | 閾値の集約(値の根拠を1箇所に) |
| contacting_body.rs | 41 | 合法 | 値オブジェクト+局所座標の変換メソッド |
| feature_identity.rs | 16 | 合法 | 列挙 |
| friction_coefficient.rs | 27 | 合法 | 値オブジェクト |
| generation_error.rs | 25 | 合法 | エラー型 |
| manifold_range.rs | 21 | 合法 | 値オブジェクト |
| material_id.rs | 11 | 合法 | 値オブジェクト |
| material_pair.rs | 27 | 合法 | 値オブジェクト |
| minimum_thickness.rs | 19 | 合法 | 値オブジェクト |
| mixing_rule.rs | 35 | 合法 | 集約型 |
| mixing_rule_builder.rs | 55 | 合法 | ビルダー(不変条件を最後に1度検査) |
| mixing_rule_builder_tests.rs | 58 | 合法 | ビルダーの拒否の性質を固定する試験 |
| mixing_rule_tests.rs | 40 | 合法 | 混合則の値の性質を固定する試験(ビルダーの試験と責務が異なる) |
| penetration_depth.rs | 12 | 合法 | 値オブジェクト |
| property_error.rs | 16 | 合法 | エラー型 |
| property_test_fixtures.rs | 38 | 合法 | 接触物性の試験が共有する材料 |
| restitution_coefficient.rs | 30 | 合法 | 値オブジェクト |
| shared_frame.rs | 25 | 合法 | 自由関数だが根拠が明記されている(入力が他のクレートの型でありメソッドにできない) |
| solver_quality.rs | 26 | 合法 | 値オブジェクト |
| solver_quality_error.rs | 8 | 合法 | エラー型 |
| solver_quality_tests.rs | 17 | 合法 | 品質の設定の値域の試験 |
| stacked_box_fixture.rs | 28 | 合法 | 試験用の場面の材料 |
| static_world_partner.rs | 39 | 合法 | 値オブジェクト |
| static_world_partner_id.rs | 11 | 合法 | 値オブジェクト |
| surface_property.rs | 48 | 合法 | 値オブジェクト |

行数の数え方は、空行とコメントだけの行を除いたコードの行である。

## 各観点の判定

1. **100行超過**: なし(最大は `contact_test_fixtures.rs` の93行)。150行の上限に関わる例外の台帳の要否も無い
2. **超過の根拠のコメント**: 該当するファイルが無いため判定の対象外
3. **分割が責務を所有しているか**: 全37ファイルが所有する側である。値オブジェクト・エラー型・ビルダー・集約型・試験の各ファイルはいずれも名前の付く責務を持ち、フィールドと状態を限定した操作を持つ。試験のファイル群も内容が重複せず、それぞれ別の不変条件を固定している
4. **不正な分割の形**: 連番の分割は無い。呼び出しの連鎖の深さで切ったものも無い。委譲だけの薄いラッパーも無い(`shared_frame.rs` は自由関数だが、責務は「他のクレートの型をこちらの型へ写す境界」であり薄いラッパーではない)
5. **親型を丸ごと受け取る引数**: 全ファイルとも `&self` か `&mut self`(自分の型)か、名前の付いた個別の値を引数に取る。組み立ての型を丸ごと引数に取る自由関数は見当たらない
6. **引数が5個を超える箇所**: `batch_builder_push.rs` の2つのメソッドが `&mut self` に加えて3引数(剛体a・剛体b・接触点集合)で最多。5個を超える箇所なし

## 指摘

**指摘なし。** 37ファイル全件が責務を所有しており、100行を超えたファイルも無い。

## 親による裏取り

親のセッションが同じ数え方で機械的に数え直し、報告の行数と一致することを確認した。あわせて `crates` と `xtask` の全域を数えたところ、**100行を超えるファイルは1件も無い。** この軸は台帳に表が無かったが、実態としては守られている。
