import { div, textInput, colorInput, button, DivC, TextInputC, ColorInputC, ButtonC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { マテリアル定義 } from '../../../生成/編集資源契約.ts'
import { 行, 名前入力, 色入力, 色見本, 削除ボタン } from './スタイル.css.ts'

export interface I材質行配線 {
    readonly on名前変更: (新しい材質名: string) => void
    readonly on識別色変更: (新しい識別色: string) => void
    readonly on削除: () => void
}

// マテリアル一覧の1件ぶんの編集行。エンジン材質名の入力・識別色の入力・色見本・
// 削除ボタンを持つ。編集は自分のフィールドだけで完結し、一覧全体の再構築は要らない。
export class マテリアル定義行 extends LV2HtmlComponentBase implements I配線可能<I材質行配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I材質行配線> = new 配線ポート<I材質行配線>('マテリアル定義行')
    private readonly _名前入力: TextInputC
    private readonly _色入力: ColorInputC
    private readonly _色見本: DivC
    private readonly _削除ボタン: ButtonC

    public constructor(初期値: マテリアル定義) {
        super()
        this._名前入力 = textInput({ class: 名前入力, value: 初期値.エンジン材質名, placeholder: 'エンジン材質名' })
            .setTooltip('エンジン材質名')
        this._色入力 = colorInput({ class: 色入力, value: 初期値.識別色 }).setTooltip('識別色')
        this._色見本 = div({ class: 色見本 }).setStyleCSS({ backgroundColor: 初期値.識別色 }).setTooltip(初期値.識別色)
        this._削除ボタン = button({ class: 削除ボタン, text: '削除' }).setTooltip('この材質を削除')
        this._componentRoot = div({ class: 行 }).childs([this._名前入力, this._色入力, this._色見本, this._削除ボタン])
    }

    public 配線する(配線: I材質行配線): this {
        this._配線.配線する(配線)
        this._名前入力.onChange(() => this._配線.先.on名前変更(this._名前入力.getValue()))
        this._色入力.onColorChange((色) => {
            this._色見本.setStyleCSS({ backgroundColor: 色 }).setTooltip(色)
            this._配線.先.on識別色変更(色)
        })
        this._削除ボタン.onClick(() => this._配線.先.on削除())
        return this
    }

    public override delete(): void {
        this._名前入力.delete()
        this._色入力.delete()
        this._色見本.delete()
        this._削除ボタン.delete()
        super.delete()
    }
}
