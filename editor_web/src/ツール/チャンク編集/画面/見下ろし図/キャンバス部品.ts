import { CanvasC } from 'sengen-ui'
import type { 画素位置 } from './見下ろし図の視点.ts'
import { キャンバス } from './見下ろし図スタイル.css.ts'

// キャンバスの上のポインタの出来事。位置はキャンバスの左上を原点にしたCSS画素で、視点の変換へそのまま渡せる。
export interface キャンバスのポインタ事象 {
    readonly 位置: 画素位置
    readonly 原初事象: PointerEvent
}

export interface キャンバスのホイール事象 {
    readonly 位置: 画素位置
    readonly 原初事象: WheelEvent
}

export interface Iキャンバス部品配線 {
    readonly on押下: (事象: キャンバスのポインタ事象) => void
    readonly on移動: (事象: キャンバスのポインタ事象) => void
    readonly on離し: (事象: キャンバスのポインタ事象) => void
    readonly onホイール: (事象: キャンバスのホイール事象) => void
}

// HTMLのcanvas要素を包む二次元描画の部品(LV1拡張)。CanvasRenderingContext2D はこの部品の中に閉じ、
// 外へは寸法合わせ・描く・ポインタ事象の配線だけを見せる(設計正本の判断6)。
// 描く側の座標はCSS画素であり、デバイスピクセル比による拡大は setTransform でこの部品が吸収する。
export class キャンバス部品 extends CanvasC {
    private readonly _文脈: CanvasRenderingContext2D
    private _ピクセル比: number = 1
    private _幅: number = 0
    private _高さ: number = 0

    public constructor() {
        super({ class: キャンバス })
        const 文脈 = this.getContext2D()
        if (文脈 === null) throw new Error('canvas要素から2Dの描画文脈を取得できない')
        this._文脈 = 文脈
    }

    public get 幅(): number {
        return this._幅
    }

    public get 高さ(): number {
        return this._高さ
    }

    // 幅と高さはCSS画素で受け取り、実際の画素数はピクセル比を掛けた値にする。
    public 寸法を合わせる(幅: number, 高さ: number, ピクセル比: number): void {
        this._幅 = 幅
        this._高さ = 高さ
        this._ピクセル比 = ピクセル比
        this.setWidth(Math.max(1, Math.round(幅 * ピクセル比)))
        this.setHeight(Math.max(1, Math.round(高さ * ピクセル比)))
    }

    public いまの枠の大きさ(): DOMRect {
        return this.dom.element.getBoundingClientRect()
    }

    // 描画手順は全面を消した後の文脈を受け取る。setTransform で毎回リセットするため、手順の中で
    // 変換を積んでも次の描画へ持ち越さない。
    public 描く(描画手順: (文脈: CanvasRenderingContext2D) => void): void {
        this._文脈.setTransform(this._ピクセル比, 0, 0, this._ピクセル比, 0, 0)
        this._文脈.clearRect(0, 0, this._幅, this._高さ)
        描画手順(this._文脈)
    }

    // 購読を解除する関数を返す。部品の delete では解除しないため、配線した側が寿命を管理する。
    public ポインタ事象を配線する(配線: Iキャンバス部品配線): () => void {
        const 要素 = this.dom.element
        const 位置を求める = (事象: MouseEvent): 画素位置 => {
            const 枠 = 要素.getBoundingClientRect()
            return { x: 事象.clientX - 枠.left, y: 事象.clientY - 枠.top }
        }
        const 押下 = (事象: PointerEvent): void => 配線.on押下({ 位置: 位置を求める(事象), 原初事象: 事象 })
        const 移動 = (事象: PointerEvent): void => 配線.on移動({ 位置: 位置を求める(事象), 原初事象: 事象 })
        const 離し = (事象: PointerEvent): void => 配線.on離し({ 位置: 位置を求める(事象), 原初事象: 事象 })
        const ホイール = (事象: WheelEvent): void => 配線.onホイール({ 位置: 位置を求める(事象), 原初事象: 事象 })
        要素.addEventListener('pointerdown', 押下)
        要素.addEventListener('pointermove', 移動)
        要素.addEventListener('pointerup', 離し)
        要素.addEventListener('pointercancel', 離し)
        要素.addEventListener('wheel', ホイール, { passive: false })
        return () => {
            要素.removeEventListener('pointerdown', 押下)
            要素.removeEventListener('pointermove', 移動)
            要素.removeEventListener('pointerup', 離し)
            要素.removeEventListener('pointercancel', 離し)
            要素.removeEventListener('wheel', ホイール)
        }
    }
}
