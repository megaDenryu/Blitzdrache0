import type { 大升の塗り } from '../../../../../生成/編集資源契約.ts'
import type { ワールドXZ } from '../見下ろし図の視点.ts'
import type { 見下ろし図の編集状態 } from '../見下ろし図の編集状態.ts'
import { 同じ大升か, type 大升の格子, type 大升の番地 } from './大升の座標変換.ts'

export interface I大升の操作の相手 {
    readonly 格子を読む: () => 大升の格子 | null
    readonly 大升を塗る: (一辺の升目数: number, 塗り一覧: readonly 大升の塗り[]) => void
    readonly 大升を選択する: (番地: 大升の番地) => void
    readonly 再描画する: () => void
}

// 左ボタンを押してから離すまでの画素での移動がこの値以内なら「クリック」とみなし1升を選ぶ。
// 超えれば通った大升を塗る従来の操作になる(エディター基盤の判断13、左クリック=選択・左ドラッグ=配置)。
const 選択とみなす移動画素の上限 = 4

// 大升モードの左ボタン。押してから離すまでに通った大升が1つだけで、かつ画素での移動が小さいときは
// その1升を選ぶ。それ以外は従来どおり、通った大升を集めて離したときに1つの`大升を塗る`へまとめる
// (設計正本の判断2、判断7)。同じ大升は1回だけ数える。置く高さ・層・塗りを消すかは右サイドバーの設定に従う。
export class 大升のポインタ操作係 {
    private readonly _通った番地一覧: 大升の番地[] = []
    private _押下中: boolean = false
    private _押した画素: { readonly x: number; readonly y: number } | null = null

    public constructor(
        private readonly _状態: 見下ろし図の編集状態,
        private readonly _相手: I大升の操作の相手,
    ) {}

    public get 通った番地一覧(): readonly 大升の番地[] {
        return this._通った番地一覧
    }

    public 押された(位置: ワールドXZ, 原初事象: PointerEvent): void {
        this._押下中 = true
        this._押した画素 = { x: 原初事象.clientX, y: 原初事象.clientY }
        this._通る(位置)
    }

    public 動かされた(位置: ワールドXZ, 押下中: boolean): void {
        if (!this._押下中 || !押下中) return
        this._通る(位置)
    }

    public 離された(原初事象: PointerEvent): void {
        if (!this._押下中) return
        this._押下中 = false
        const 押した画素 = this._押した画素
        this._押した画素 = null
        const 通った番地一覧 = [...this._通った番地一覧]
        this._通った番地一覧.length = 0
        const 移動画素 = 押した画素 === null ? Number.POSITIVE_INFINITY : Math.hypot(原初事象.clientX - 押した画素.x, 原初事象.clientY - 押した画素.y)
        const 唯一の番地 = 通った番地一覧.length === 1 ? 通った番地一覧[0] : undefined
        if (唯一の番地 !== undefined && 移動画素 <= 選択とみなす移動画素の上限) {
            this._相手.大升を選択する(唯一の番地)
            this._相手.再描画する()
            return
        }
        const 塗り一覧 = 通った番地一覧.map((番地) => this._塗りを組む(番地))
        // 高さも層も置かず消しもしない設定では、塗りを消す項目を積んでしまうため何もしない。
        if (塗り一覧.length > 0 && (this._状態.大升の塗りを消すか || this._状態.大升に高さを置くか || this._状態.大升に層を置くか)) {
            this._相手.大升を塗る(this._状態.大升の一辺の升目数, 塗り一覧)
        }
        this._相手.再描画する()
    }

    private _通る(位置: ワールドXZ): void {
        const 格子 = this._相手.格子を読む()
        if (格子 === null) return
        const 番地 = 格子.ワールドから大升へ(位置)
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
