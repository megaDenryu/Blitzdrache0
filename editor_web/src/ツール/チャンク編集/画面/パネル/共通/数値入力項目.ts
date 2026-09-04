import { div, span, input, DivC, InputC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import { 行コンテナ, ラベル行, 数値入力 } from './スタイル.css.ts'

export interface I数値入力配線 {
    readonly on確定: (値: number) => void
}

// 項目名と数値の入力欄を一体化したLV2素部品。打っている間は何も知らせず、欄から離れたかEnterで入力が決まった
// ときに1回だけ知らせる。1文字打つたびにコマンドを積むと取り消しの段が文字数ぶん増えるためである
// (文書の表示名の編集と同じ規律。参照: `_doc/設計/楽曲エディター.md`「判断13」)。
// 数に読めない入力は知らせず、直前の値へ欄を戻す(無言のゼロ値の適用の禁止)。
export class 数値入力項目 extends LV2HtmlComponentBase implements I配線可能<I数値入力配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I数値入力配線> = new 配線ポート<I数値入力配線>('数値入力項目')
    private readonly _入力欄: InputC
    private _直前の値: number

    public constructor(ラベル名: string, 初期値: number, 刻み幅: number, 接尾辞: string = '') {
        super()
        this._直前の値 = 初期値
        this._入力欄 = input({ class: 数値入力, type: 'number', value: 初期値.toString() }).setRangeParam({ step: 刻み幅 })
        this._componentRoot = div({ class: 行コンテナ }).childs([
            div({ class: ラベル行 }).childs([
                span({ text: ラベル名 }).setTooltip(ラベル名),
                span({ text: 接尾辞 })]),
            this._入力欄.onChange(() => this._入力が決まった())])
    }

    public 配線する(配線: I数値入力配線): this {
        this._配線.配線する(配線)
        return this
    }

    // 正本の値へ欄を合わせる。人が打った値ではないため知らせない。
    public 値を設定する(値: number): void {
        this._直前の値 = 値
        this._入力欄.setValue(値.toString())
    }

    public 操作できるか設定する(操作できるか: boolean): void {
        this._入力欄.setDisabled(!操作できるか)
    }

    private _入力が決まった(): void {
        const 数値 = Number.parseFloat(this._入力欄.getValue())
        if (!Number.isFinite(数値)) {
            this._入力欄.setValue(this._直前の値.toString())
            return
        }
        if (数値 === this._直前の値) return
        this._直前の値 = 数値
        this._配線.先.on確定(数値)
    }
}
