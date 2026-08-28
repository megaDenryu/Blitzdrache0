import type { ワールド編集状態 } from '../../チャンク編集/編集モデル/index.ts'
import { 大域三次元ビュー部品 } from './三次元/大域三次元ビュー部品.ts'
import { 大域ぜんたいの操作帯 } from './操作帯/大域ぜんたいの操作帯.ts'
import { 大域の筆と道の棚 } from './下パネル/大域の筆と道の棚.ts'
import { 大域インスペクターパネル } from './パネル/インスペクター/大域インスペクターパネル.ts'
import { 初期の大域編集モード } from './パネル/モード切替/大域モード定義.ts'

// 大域編集画面が集約する部品の型契約(部品DTO)。エディタ領域へ置く2つ(操作帯・三次元ビュー)に
// 加えて、外殻の別の区画へ渡す2つ(右サイドバーのインスペクター・下パネルの大域の筆と道の棚)も
// ここが所有する。区画は違っても、寿命と作り方は1つの画面のものだからである。
export class 大域編集画面部品 {
    private constructor(
        public readonly 操作帯: 大域ぜんたいの操作帯,
        public readonly 三次元ビュー: 大域三次元ビュー部品,
        public readonly インスペクター: 大域インスペクターパネル,
        public readonly 棚: 大域の筆と道の棚,
    ) {}

    public static 作る(編集状態: ワールド編集状態): 大域編集画面部品 {
        return new 大域編集画面部品(
            new 大域ぜんたいの操作帯(初期の大域編集モード),
            new 大域三次元ビュー部品(編集状態),
            new 大域インスペクターパネル(),
            new 大域の筆と道の棚(初期の大域編集モード),
        )
    }

    public delete(): void {
        this.操作帯.delete()
        this.三次元ビュー.delete()
        this.インスペクター.delete()
        this.棚.delete()
    }
}
