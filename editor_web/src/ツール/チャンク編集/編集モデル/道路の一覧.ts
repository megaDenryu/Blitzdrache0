import type { チャンクの道路, 広域道路 } from '../../../生成/編集資源契約.ts'
import type { 高さ場 } from './高さ場.ts'
import { 道路スプライン } from './道路スプライン.ts'

// 1つのチャンク、または大域世界が持つ道路の一覧。道の分岐は「先頭の制御点を分かれ元と
// 同じ位置に置いた別の道路」として表すため、道路は何本でも並ぶ。
// 添字は先頭を0とする位置であり、削除すると後ろの道路が1つずつ前へ詰まる。
export class 道路の一覧 {
    private readonly _道路列: 道路スプライン[]

    public constructor(初期の道路一覧: ReadonlyArray<チャンクの道路 | 広域道路>) {
        this._道路列 = 初期の道路一覧.map((道路) => new 道路スプライン(道路))
    }

    public get 件数(): number {
        return this._道路列.length
    }

    public get 全ての道路(): readonly 道路スプライン[] {
        return this._道路列
    }

    public 道路が在るか(道路添字: number): boolean {
        return 道路添字 >= 0 && 道路添字 < this._道路列.length
    }

    public 道路を取り出す(道路添字: number): 道路スプライン {
        const 道路 = this._道路列[道路添字]
        if (道路 === undefined) {
            throw new Error(`道路添字が範囲外: 道路添字=${道路添字}, 本数=${this._道路列.length}`)
        }
        return 道路
    }

    // 末尾へ1本足し、足した位置の道路添字を返す。
    public 末尾へ追加する(道路: 道路スプライン): number {
        this._道路列.push(道路)
        return this._道路列.length - 1
    }

    // 削除の差し戻しで、消える前と同じ位置へ道路を戻すときに使う。
    public 添字へ挿入する(道路添字: number, 道路: 道路スプライン): void {
        if (道路添字 < 0 || 道路添字 > this._道路列.length) {
            throw new Error(`道路添字が範囲外: 道路添字=${道路添字}, 本数=${this._道路列.length}`)
        }
        this._道路列.splice(道路添字, 0, 道路)
    }

    public 削除する(道路添字: number): 道路スプライン {
        const 削除した道路 = this.道路を取り出す(道路添字)
        this._道路列.splice(道路添字, 1)
        return 削除した道路
    }

    public 全て削除する(): void {
        this._道路列.length = 0
    }

    public チャンクの道路一覧として取り出す(): Array<チャンクの道路> {
        return this._道路列.map((道路) => 道路.チャンクの道路として取り出す())
    }

    public 広域道路一覧として取り出す(): Array<広域道路> {
        return this._道路列.map((道路) => 道路.広域道路として取り出す())
    }

    public 全ての道路に合わせて切土盛土する(対象高さ場: 高さ場): void {
        for (const 道路 of this._道路列) {
            道路.道路に合わせて切土盛土する(対象高さ場)
        }
    }
}
