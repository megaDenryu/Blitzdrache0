import type { キャンバスのホイール事象, キャンバスのポインタ事象 } from './キャンバス部品.ts'
import type { ワールドXZ, 見下ろし図の視点 } from './見下ろし図の視点.ts'

// 左ボタンの出来事をワールド座標で受ける側。等高線と大升の操作がここへ配線される。
export interface I見下ろし図の左ボタン配線 {
    readonly on左ボタン押下: (位置: ワールドXZ, 原初事象: PointerEvent) => void
    readonly on左ボタン移動: (位置: ワールドXZ, 押下中: boolean, 原初事象: PointerEvent) => void
    readonly on左ボタン離し: (位置: ワールドXZ, 原初事象: PointerEvent) => void
}

const 左ボタン = 0
const 中ボタン = 1

// ホイール1刻み(deltaY=100)でおよそ1.1倍になる係数。三次元の視点操作のホイールと近い感触にする。
const ホイールのズーム係数 = 0.001

// 見下ろし図の上のポインタの割り当て。中ドラッグがパン、ホイールがカーソル位置を中心にしたズームであり、
// 左ボタンは編集としてワールド座標へ変換して配線先へ渡す(設計正本の判断6、エディター基盤の判断13)。
// 視点そのものは見下ろし図部品が持つため、読み書きは渡された関数を通す。
export class 見下ろし図のポインタ操作係 {
    private _パン中の直前の画素: { x: number; y: number } | null = null
    private _左ボタン押下中: boolean = false

    public constructor(
        private readonly _視点を読む: () => 見下ろし図の視点 | null,
        private readonly _視点を書く: (視点: 見下ろし図の視点) => void,
        private readonly _左ボタン配線: () => I見下ろし図の左ボタン配線,
    ) {}

    public 押された({ 位置, 原初事象 }: キャンバスのポインタ事象): void {
        const 視点 = this._視点を読む()
        if (視点 === null) return
        if (原初事象.button === 中ボタン) {
            原初事象.preventDefault()
            this._パン中の直前の画素 = { x: 位置.x, y: 位置.y }
            this._キャンバスにポインタを捕まえさせる(原初事象)
            return
        }
        if (原初事象.button === 左ボタン) {
            this._左ボタン押下中 = true
            this._キャンバスにポインタを捕まえさせる(原初事象)
            this._左ボタン配線().on左ボタン押下(視点.画素からワールドへ(位置), 原初事象)
        }
    }

    public 動かされた({ 位置, 原初事象 }: キャンバスのポインタ事象): void {
        const 視点 = this._視点を読む()
        if (視点 === null) return
        if (this._パン中の直前の画素 !== null) {
            this._視点を書く(視点.画素で平行移動する(位置.x - this._パン中の直前の画素.x, 位置.y - this._パン中の直前の画素.y))
            this._パン中の直前の画素 = { x: 位置.x, y: 位置.y }
            return
        }
        this._左ボタン配線().on左ボタン移動(視点.画素からワールドへ(位置), this._左ボタン押下中, 原初事象)
    }

    public 離された({ 位置, 原初事象 }: キャンバスのポインタ事象): void {
        if (原初事象.button === 中ボタン || 原初事象.type === 'pointercancel') {
            this._パン中の直前の画素 = null
        }
        if (原初事象.button !== 左ボタン && 原初事象.type !== 'pointercancel') return
        const 視点 = this._視点を読む()
        if (!this._左ボタン押下中 || 視点 === null) return
        this._左ボタン押下中 = false
        this._左ボタン配線().on左ボタン離し(視点.画素からワールドへ(位置), 原初事象)
    }

    public ホイールが回された({ 位置, 原初事象 }: キャンバスのホイール事象): void {
        const 視点 = this._視点を読む()
        if (視点 === null) return
        原初事象.preventDefault()
        this._視点を書く(視点.カーソルを中心にズームする(位置, Math.exp(-原初事象.deltaY * ホイールのズーム係数)))
    }

    // ドラッグ中にキャンバスの外へ出ても移動と離しが届くようにする。
    private _キャンバスにポインタを捕まえさせる(原初事象: PointerEvent): void {
        const 対象 = 原初事象.currentTarget
        if (対象 instanceof Element) 対象.setPointerCapture(原初事象.pointerId)
    }
}
