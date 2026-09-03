import type { 大升の塗り } from '../../../../../生成/編集資源契約.ts'
import type { ワールドXZ } from '../見下ろし図の視点.ts'
import type { 見下ろし図の編集状態 } from '../見下ろし図の編集状態.ts'
import { ワールドから大升へ, 同じ大升か, type 大升の格子, type 大升の番地 } from './大升の座標変換.ts'

export interface I大升の操作の相手 {
    readonly 格子を読む: () => 大升の格子 | null
    readonly 大升を塗る: (一辺の升目数: number, 塗り一覧: readonly 大升の塗り[]) => void
    readonly 再描画する: () => void
}

// 大升モードの左ボタン。押してから離すまでに通った大升を集め、離したときに1つの`大升を塗る`へまとめる
// (設計正本の判断2)。同じ大升は1回だけ数える。置く高さ・層・塗りを消すかは右サイドバーの設定に従う。
export class 大升のポインタ操作係 {
    private readonly _通った番地一覧: 大升の番地[] = []
    private _押下中: boolean = false

    public constructor(
        private readonly _状態: 見下ろし図の編集状態,
        private readonly _相手: I大升の操作の相手,
    ) {}

    public get 通った番地一覧(): readonly 大升の番地[] {
        return this._通った番地一覧
    }

    public 押された(位置: ワールドXZ): void {
        this._押下中 = true
        this._通る(位置)
    }

    public 動かされた(位置: ワールドXZ, 押下中: boolean): void {
        if (!this._押下中 || !押下中) return
        this._通る(位置)
    }

    public 離された(): void {
        if (!this._押下中) return
        this._押下中 = false
        const 塗り一覧 = this._通った番地一覧.map((番地) => this._塗りを組む(番地))
        this._通った番地一覧.length = 0
        // 高さも層も置かず消しもしない設定では、塗りを消す項目を積んでしまうため何もしない。
        if (塗り一覧.length > 0 && (this._状態.大升の塗りを消すか || this._状態.大升に高さを置くか || this._状態.大升に層を置くか)) {
            this._相手.大升を塗る(this._状態.大升の一辺の升目数, 塗り一覧)
        }
        this._相手.再描画する()
    }

    private _通る(位置: ワールドXZ): void {
        const 格子 = this._相手.格子を読む()
        if (格子 === null) return
        const 番地 = ワールドから大升へ(位置, 格子)
        if (番地 === null || this._通った番地一覧.some((既) => 同じ大升か(既, 番地))) return
        this._通った番地一覧.push(番地)
        this._相手.再描画する()
    }

    // 高さも層も無い項目は塗りを消す意味になる(設計正本の判断2の表)。
    private _塗りを組む(番地: 大升の番地): 大升の塗り {
        if (this._状態.大升の塗りを消すか) return { 列: 番地.列, 行: 番地.行, 高さメートル: null, 層: null }
        return {
            列: 番地.列,
            行: 番地.行,
            高さメートル: this._状態.大升に高さを置くか ? this._状態.大升に置く高さメートル : null,
            層: this._状態.大升に層を置くか ? this._状態.大升に置く層 : null,
        }
    }
}
