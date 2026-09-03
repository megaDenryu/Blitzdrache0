import type { チャンク座標 } from '../../../生成/編集資源契約.ts'
import type { ワールド編集状態 } from '../編集モデル/index.ts'
import { 三次元ビュー部品 } from './三次元/三次元ビュー部品.ts'
import { チャンクぜんたいの操作帯 } from './操作帯/チャンクぜんたいの操作帯.ts'
import { お知らせの帯 } from './操作帯/お知らせの帯.ts'
import { 筆と置くものの棚 } from './下パネル/筆と置くものの棚.ts'
import { インスペクターパネル } from './パネル/インスペクター/インスペクターパネル.ts'
import { 初期の編集モード } from './パネル/モード切替/モード定義.ts'
import { 見下ろし図部品 } from './見下ろし図/見下ろし図部品.ts'
import type { 表示面 } from './操作帯/表示面.ts'

// チャンク編集画面が集約する部品の型契約(部品DTO)。エディタ領域へ置く3つ(操作帯・三次元ビュー・見下ろし図)に
// 加えて、外殻の別の区画へ渡す2つ(右サイドバーのインスペクター・下パネルの筆と置くものの棚)も
// ここが所有する。区画は違っても、寿命と作り方は1つの画面のものだからである。
export class チャンク編集画面部品 {
    private constructor(
        public readonly 操作帯: チャンクぜんたいの操作帯,
        public readonly お知らせ: お知らせの帯,
        public readonly 三次元ビュー: 三次元ビュー部品,
        public readonly 見下ろし図: 見下ろし図部品,
        public readonly インスペクター: インスペクターパネル,
        public readonly 棚: 筆と置くものの棚,
    ) {}

    public static 作る(編集状態: ワールド編集状態, 対象座標: チャンク座標): チャンク編集画面部品 {
        return new チャンク編集画面部品(
            new チャンクぜんたいの操作帯(対象座標, 初期の編集モード),
            new お知らせの帯(),
            new 三次元ビュー部品(編集状態),
            new 見下ろし図部品(),
            new インスペクターパネル(),
            new 筆と置くものの棚(初期の編集モード),
        )
    }

    // 三次元ビューと見下ろし図は同じ枠を占めるため片方だけを表示する(設計正本の判断6)。三次元の描画ループは
    // 止めない。切り戻したときに描画の再開を待たせないためである。
    public 表示面を切り替える(面: 表示面): void {
        this.三次元ビュー.setStyleCSS({ display: 面 === '三次元' ? '' : 'none' })
        this.見下ろし図.表示するか設定する(面 === '見下ろし図')
    }

    public delete(): void {
        this.操作帯.delete()
        this.お知らせ.delete()
        this.三次元ビュー.delete()
        this.見下ろし図.delete()
        this.インスペクター.delete()
        this.棚.delete()
    }
}
