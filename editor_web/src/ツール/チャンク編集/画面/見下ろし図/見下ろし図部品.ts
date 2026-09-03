import { div, DivC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { 高さ場, 地表材質 } from '../../編集モデル/index.ts'
import { キャンバス部品 } from './キャンバス部品.ts'
import { 見下ろし図の下地の管理 } from './見下ろし図の下地の管理.ts'
import { 見下ろし図のポインタ操作係, type I見下ろし図の左ボタン配線 } from './見下ろし図のポインタ操作係.ts'
import { 見下ろし図の視点 } from './見下ろし図の視点.ts'
import type { 下地の配色 } from './下地の配色.ts'
import { コンテナ } from './見下ろし図スタイル.css.ts'

// 下地の上に重ねて描く手順。等高線・大升の重ね描きがここへ登録される。文脈の座標はCSS画素であり、
// ワールド座標からの変換に視点を使う。
export type 見下ろし図の重ね描き = (文脈: CanvasRenderingContext2D, 視点: 見下ろし図の視点) => void

// 1チャンクを真上から見た二次元の編集面(設計正本の語彙「見下ろし図」)。キャンバス部品・視点・下地を束ね、
// 高さ場と地表材質を受け取って下地を作り直し、ズームに応じて下地を拡大してチャンクの外枠を描く。
// 参照: `_doc/設計/見下ろし図による地形編集.md`「判断6」
export class 見下ろし図部品 extends LV2HtmlComponentBase implements I配線可能<I見下ろし図の左ボタン配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I見下ろし図の左ボタン配線> = new 配線ポート<I見下ろし図の左ボタン配線>('見下ろし図部品')
    private readonly _キャンバス: キャンバス部品 = new キャンバス部品()
    private readonly _下地: 見下ろし図の下地の管理 = new 見下ろし図の下地の管理()
    private readonly _重ね描き一覧: 見下ろし図の重ね描き[] = []
    private readonly _ポインタ購読の解除: () => void
    private _視点: 見下ろし図の視点 | null = null
    private _ピクセル比: number = 1
    private _表示中: boolean = false

    public constructor() {
        super()
        this._componentRoot = div({ class: コンテナ }).child(this._キャンバス)
        const 操作係 = new 見下ろし図のポインタ操作係(
            () => this._視点,
            (視点) => {
                this._視点 = 視点
                this.再描画する()
            },
            () => this._配線.先,
        )
        this._ポインタ購読の解除 = this._キャンバス.ポインタ事象を配線する({
            on押下: (事象) => 操作係.押された(事象),
            on移動: (事象) => 操作係.動かされた(事象),
            on離し: (事象) => 操作係.離された(事象),
            onホイール: (事象) => 操作係.ホイールが回された(事象),
        })
    }

    public 配線する(配線: I見下ろし図の左ボタン配線): this {
        this._配線.配線する(配線)
        return this
    }

    // 等高線と大升の操作が当たりの半径を画素で決めるために倍率を読む。視点の書き換えはポインタ操作係だけが行う。
    public get 視点(): 見下ろし図の視点 | null {
        return this._視点
    }

    // 重ね描きが層の識別色を下地と同じ出どころから取るための口。
    public get 下地の配色(): 下地の配色 {
        return this._下地.配色
    }

    public 重ね描きを登録する(手順: 見下ろし図の重ね描き): void {
        this._重ね描き一覧.push(手順)
    }

    // 地形が変わるたびに呼ばれる。隠れている間は作り直しを先送りし、表示されたときに1回だけ作る。
    public 地形を受け取る(高さ場モデル: 高さ場, 地表材質モデル: 地表材質): void {
        this._下地.地形を受け取る(高さ場モデル, 地表材質モデル)
        if (this._表示中) this._下地を作り直して描く()
    }

    public 配色を設定する(配色: Partial<下地の配色>): void {
        this._下地.配色を設定する(配色)
        if (this._表示中) this._下地を作り直して描く()
    }

    public 表示するか設定する(表示中: boolean): void {
        this._表示中 = 表示中
        this.setStyleCSS({ display: 表示中 ? '' : 'none' })
        if (!表示中) return
        this.いまの枠の大きさへ合わせる(this._ピクセル比)
        this._下地を作り直して描く()
    }

    // 隠れている間は枠が0になるため、その測定は捨てて表示されたときに測り直す。
    public いまの枠の大きさへ合わせる(ピクセル比: number): void {
        this._ピクセル比 = ピクセル比
        const 枠 = this._キャンバス.いまの枠の大きさ()
        if (枠.width <= 0 || 枠.height <= 0) return
        this._キャンバス.寸法を合わせる(枠.width, 枠.height, ピクセル比)
        this._視点 ??= this._初期の視点を作る()
        this.再描画する()
    }

    public 再描画する(): void {
        const 視点 = this._視点
        if (視点 === null) return
        this._キャンバス.描く((文脈) => {
            this._下地.描く(文脈, 視点)
            for (const 手順 of this._重ね描き一覧) 手順(文脈, 視点)
        })
    }

    private _下地を作り直して描く(): void {
        if (!this._下地.作り直す()) return
        this._視点 ??= this._初期の視点を作る()
        this.再描画する()
    }

    private _初期の視点を作る(): 見下ろし図の視点 | null {
        const 一辺 = this._下地.一辺のメートル
        if (一辺 === null || this._キャンバス.幅 <= 0) return null
        return 見下ろし図の視点.チャンク全体が収まる視点を作る(一辺, this._キャンバス.幅, this._キャンバス.高さ)
    }

    public override delete(): void {
        this._ポインタ購読の解除()
        this._キャンバス.delete()
        super.delete()
    }
}
