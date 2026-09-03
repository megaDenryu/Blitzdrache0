import { div, span, button, input, select, DivC, SpanC, ButtonC, InputC, SelectC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { 地表材質層 } from '../../../../../生成/編集資源契約.ts'
import { ラベル行, 値ラベル, 数値入力 } from '../共通/スタイル.css.ts'
import { 選択の詳細区画, 選択の案内文, 危険ボタン, 選択欄 } from './スタイル.css.ts'

export interface I選択中の大升欄配線 {
    readonly on高さ確定: (高さメートル: number | null) => void
    readonly on層確定: (層: 地表材質層 | null) => void
    readonly on消す: () => void
}

export interface 選択中の大升情報 {
    readonly 列: number
    readonly 行: number
    readonly 塗った高さメートル: number | null
    readonly 塗った層: 地表材質層 | null
    readonly 現在の平均高さメートル: number
    readonly 現在の重みが最大の層: 地表材質層
}

const なしの選択値 = 'なし'
const 層の選択肢: readonly string[] = [なしの選択値, '草', '泥', '岩', '砂']

function 層を選択値にする(層: 地表材質層 | null): string {
    return 層 ?? なしの選択値
}

function 選択値を層にする(値: string): 地表材質層 | null {
    if (値 === なしの選択値) return null
    if (値 === '草' || 値 === '泥' || 値 === '岩' || 値 === '砂') return 値
    throw new Error(`選択欄の値が地表材質層でもなしでもない: ${値}`)
}

// 大升パネルの一番上に置く「選択中の大升」欄。見下ろし図で大升をクリックして選んだときだけ詳細を出す
// (設計正本の判断7)。高さと層はnullを「なし」として扱い、確定するとその1升だけへ`大升を塗る`を積む。
export class 選択中の大升欄 extends LV2HtmlComponentBase implements I配線可能<I選択中の大升欄配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I選択中の大升欄配線> = new 配線ポート<I選択中の大升欄配線>('選択中の大升欄')
    private readonly _列行値: SpanC = span({ class: 値ラベル })
    private readonly _高さ入力: InputC = input({ class: 数値入力, type: 'number', value: '' }).setRangeParam({ step: 0.5 })
    private readonly _層選択: SelectC = select({ class: 選択欄, options: 層の選択肢.map((値) => ({ value: 値, text: 値 })) })
    private readonly _現在の平均値: SpanC = span({ class: 値ラベル })
    private readonly _消すボタン: ButtonC = button({ class: 危険ボタン, text: '塗りを消す' }).setTooltip('選んだ大升の高さと層を外す')
    private readonly _詳細区画: DivC
    private readonly _無選択の案内: SpanC = span({ class: 選択の案内文, text: '升をクリックして選ぶ' })
    private _直前の高さ: number | null = null

    public constructor() {
        super()
        this._詳細区画 = div({ class: 選択の詳細区画 }).childs([
            div({ class: ラベル行 }).childs([span({ text: '(列,行)' }), this._列行値]),
            div({ class: ラベル行 }).childs([span({ text: '塗った高さ' }), this._高さ入力]),
            div({ class: ラベル行 }).childs([span({ text: '塗った層' }), this._層選択]),
            div({ class: ラベル行 }).childs([span({ text: '現在の平均' }), this._現在の平均値]),
            this._消すボタン])
        this._componentRoot = div().childs([this._詳細区画, this._無選択の案内])
    }

    public 配線する(配線: I選択中の大升欄配線): this {
        this._配線.配線する(配線)
        this._高さ入力.onChange(() => this._高さが決まった())
        this._層選択.onSelectChange(() => this._配線.先.on層確定(選択値を層にする(this._層選択.getValue())))
        this._消すボタン.onClick(() => this._配線.先.on消す())
        return this
    }

    public 表示を更新する(情報: 選択中の大升情報 | null): void {
        this._詳細区画.setStyleCSS({ display: 情報 === null ? 'none' : '' })
        this._無選択の案内.setStyleCSS({ display: 情報 === null ? '' : 'none' })
        if (情報 === null) return
        this._列行値.setTextContent(`(${情報.列},${情報.行})`)
        this._直前の高さ = 情報.塗った高さメートル
        this._高さ入力.setValue(情報.塗った高さメートル === null ? '' : 情報.塗った高さメートル.toString())
        this._層選択.setValue(層を選択値にする(情報.塗った層))
        this._現在の平均値.setTextContent(`${情報.現在の平均高さメートル.toFixed(1)}m / ${情報.現在の重みが最大の層}`)
    }

    // 空欄はnull(塗りを消す)として送る。数に読めない入力は知らせず直前の値へ戻す。
    private _高さが決まった(): void {
        const 文字 = this._高さ入力.getValue().trim()
        if (文字 === '') {
            if (this._直前の高さ === null) return
            this._直前の高さ = null
            this._配線.先.on高さ確定(null)
            return
        }
        const 数値 = Number.parseFloat(文字)
        if (!Number.isFinite(数値)) {
            this._高さ入力.setValue(this._直前の高さ === null ? '' : this._直前の高さ.toString())
            return
        }
        if (数値 === this._直前の高さ) return
        this._直前の高さ = 数値
        this._配線.先.on高さ確定(数値)
    }
}
