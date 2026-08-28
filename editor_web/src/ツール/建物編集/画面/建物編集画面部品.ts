import type { 建物定義ID } from '../../../境界/建物定義ID.ts'
import { 建物の三次元パネル } from './三次元/建物の三次元パネル.ts'
import { 建物インスペクターパネル } from './インスペクター/建物インスペクターパネル.ts'
import { 部品の棚 } from './下パネル/部品の棚.ts'
import { 平面図パネル } from './平面図パネル.ts'
import { 建物ぜんたいの操作帯 } from './建物ぜんたいの操作帯.ts'
import { 建物名の欄 } from './建物名の欄.ts'
import { 触りの知らせの札 } from './触りの知らせの札.ts'

// 建物編集画面が集約する部品の型契約(部品DTO)。エディタ領域へ置く部品(建物名・操作帯・三次元・平面図・
// 触りの知らせ)に加えて、外殻の別の区画へ渡す2つ(右サイドバーのインスペクター・下パネルの部品の棚)も
// ここが所有する。区画は違っても、寿命と作り方は1つの画面のものだからである。
export class 建物編集画面部品 {
    private constructor(
        public readonly 建物名: 建物名の欄,
        public readonly 操作帯: 建物ぜんたいの操作帯,
        public readonly 三次元: 建物の三次元パネル,
        public readonly 平面図: 平面図パネル,
        public readonly 触りの知らせ: 触りの知らせの札,
        public readonly インスペクター: 建物インスペクターパネル,
        public readonly 部品の棚: 部品の棚,
    ) {}

    public static 作る(建物定義ID: 建物定義ID): 建物編集画面部品 {
        return new 建物編集画面部品(
            new 建物名の欄(建物定義ID),
            new 建物ぜんたいの操作帯(),
            new 建物の三次元パネル(),
            new 平面図パネル(),
            new 触りの知らせの札(),
            new 建物インスペクターパネル(),
            new 部品の棚(),
        )
    }

    public delete(): void {
        this.建物名.delete()
        this.操作帯.delete()
        this.三次元.delete()
        this.平面図.delete()
        this.触りの知らせ.delete()
        this.インスペクター.delete()
        this.部品の棚.delete()
    }
}
