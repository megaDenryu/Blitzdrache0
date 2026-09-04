import { div, span, button, DivC, SpanC, ButtonC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import { 数値入力項目 } from '../共通/数値入力項目.ts'
import { ラベル行, 値ラベル } from '../共通/スタイル.css.ts'
import { 選択の詳細区画, 選択の案内文, 危険ボタン } from './スタイル.css.ts'

export interface I選択中の等高線欄配線 {
    readonly on高さ確定: (高さメートル: number) => void
    readonly on削除: () => void
}

export interface 選択中の等高線情報 {
    readonly 高さメートル: number
    readonly 頂点数: number
    readonly 閉じているか: boolean
    readonly 長さメートル: number
}

// 等高線パネルの一番上に置く「選択中の等高線」欄。見下ろし図で線をクリックして選んだときだけ詳細を出し、
// 選んでいないときは案内文だけを出す(設計正本の判断7)。高さの入力欄は既存の欄をここへ移したもので、
// 確定すると`等高線を変更する`を1つ積む。
export class 選択中の等高線欄 extends LV2HtmlComponentBase implements I配線可能<I選択中の等高線欄配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I選択中の等高線欄配線> = new 配線ポート<I選択中の等高線欄配線>('選択中の等高線欄')
    private readonly _高さ: 数値入力項目 = new 数値入力項目('高さ', 0, 0.5, 'm')
    private readonly _頂点数値: SpanC = span({ class: 値ラベル })
    private readonly _閉じているか値: SpanC = span({ class: 値ラベル })
    private readonly _長さ値: SpanC = span({ class: 値ラベル })
    private readonly _削除ボタン: ButtonC = button({ class: 危険ボタン, text: '削除' }).setTooltip('選んだ等高線を削除する')
    private readonly _詳細区画: DivC
    private readonly _無選択の案内: SpanC = span({ class: 選択の案内文, text: '線をクリックして選ぶ' })

    public constructor() {
        super()
        this._詳細区画 = div({ class: 選択の詳細区画 }).childs([
            this._高さ,
            div({ class: ラベル行 }).childs([span({ text: '頂点数' }), this._頂点数値]),
            div({ class: ラベル行 }).childs([span({ text: '閉じているか' }), this._閉じているか値]),
            div({ class: ラベル行 }).childs([span({ text: '長さ' }), this._長さ値]),
            this._削除ボタン])
        this._componentRoot = div().childs([this._詳細区画, this._無選択の案内])
    }

    public 配線する(配線: I選択中の等高線欄配線): this {
        this._配線.配線する(配線)
        this._高さ.配線する({ on確定: (v) => this._配線.先.on高さ確定(v) })
        this._削除ボタン.onClick(() => this._配線.先.on削除())
        return this
    }

    public 表示を更新する(情報: 選択中の等高線情報 | null): void {
        this._詳細区画.setStyleCSS({ display: 情報 === null ? 'none' : '' })
        this._無選択の案内.setStyleCSS({ display: 情報 === null ? '' : 'none' })
        if (情報 === null) return
        this._高さ.値を設定する(情報.高さメートル)
        this._頂点数値.setTextContent(情報.頂点数.toString())
        this._閉じているか値.setTextContent(情報.閉じているか ? '閉じている' : '開いている')
        this._長さ値.setTextContent(`${情報.長さメートル.toFixed(1)}m`)
    }

    public override delete(): void {
        this._高さ.delete()
        super.delete()
    }
}
