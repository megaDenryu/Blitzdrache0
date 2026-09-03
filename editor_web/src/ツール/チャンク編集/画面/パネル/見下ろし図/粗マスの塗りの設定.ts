import { div, span, checkbox, select, DivC, CheckboxInputC, SelectC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { 地表材質層 } from '../../../../../生成/編集資源契約.ts'
import { 数値入力項目 } from '../共通/数値入力項目.ts'
import { チェック行, チェック入力 } from '../共通/スタイル.css.ts'
import { 選択欄 } from './スタイル.css.ts'

export interface I粗マスの塗りの設定配線 {
    readonly on高さ確定: (高さメートル: number) => void
    readonly on高さを置くか変更: (置くか: boolean) => void
    readonly on層変更: (層: 地表材質層) => void
    readonly on層を置くか変更: (置くか: boolean) => void
    readonly on塗りを消すか変更: (消すか: boolean) => void
}

// 選択欄の並び。材質重みの層の並び(草・泥・岩・砂)と同じ順にする。
const 層の一覧: readonly 地表材質層[] = ['草', '泥', '岩', '砂']

// 選択欄の値は文字列でしか戻らないため、層の一覧に照らして型へ戻す。一覧に無い値は選択欄が作れないためバグである。
function 層へ戻す(値: string): 地表材質層 {
    const 層 = 層の一覧.find((候補) => 候補 === 値)
    if (層 === undefined) throw new Error(`選択欄の値が地表材質層でない: ${値}`)
    return 層
}

export interface 粗マスの塗りの初期設定 {
    readonly 高さメートル: number
    readonly 高さを置くか: boolean
    readonly 層: 地表材質層
    readonly 層を置くか: boolean
    readonly 塗りを消すか: boolean
}

// 粗マス1つに置くもの(高さ・層・またはその両方、あるいは塗りを消す)の設定。粗マスパネルの中の1区画であり、
// 置く高さと層の有効・無効の切替を一緒に持つ。
export class 粗マスの塗りの設定 extends LV2HtmlComponentBase implements I配線可能<I粗マスの塗りの設定配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I粗マスの塗りの設定配線> = new 配線ポート<I粗マスの塗りの設定配線>('粗マスの塗りの設定')
    private readonly _高さ: 数値入力項目
    private readonly _高さを置くか: CheckboxInputC
    private readonly _層: SelectC
    private readonly _層を置くか: CheckboxInputC
    private readonly _塗りを消すか: CheckboxInputC

    public constructor(初期: 粗マスの塗りの初期設定) {
        super()
        this._高さ = new 数値入力項目('置く高さ', 初期.高さメートル, 0.5, 'm')
        this._高さを置くか = checkbox({ class: チェック入力, checked: 初期.高さを置くか })
        this._層 = select({
            class: 選択欄,
            options: 層の一覧.map((層) => ({ value: 層, text: 層, selected: 層 === 初期.層 })),
        })
        this._層を置くか = checkbox({ class: チェック入力, checked: 初期.層を置くか })
        this._塗りを消すか = checkbox({ class: チェック入力, checked: 初期.塗りを消すか })
        this._componentRoot = this._ルートを構築する()
    }

    public 配線する(配線: I粗マスの塗りの設定配線): this {
        this._配線.配線する(配線)
        this._高さ.配線する({ on確定: (v) => this._配線.先.on高さ確定(v) })
        this._高さを置くか.onCheckChange((置くか) => this._配線.先.on高さを置くか変更(置くか))
        this._層.onSelectChange(() => this._配線.先.on層変更(層へ戻す(this._層.getValue())))
        this._層を置くか.onCheckChange((置くか) => this._配線.先.on層を置くか変更(置くか))
        this._塗りを消すか.onCheckChange((消すか) => this._配線.先.on塗りを消すか変更(消すか))
        return this
    }

    public override delete(): void {
        this._高さ.delete()
        super.delete()
    }

    private _ルートを構築する(): DivC {
        return (
            div().setStyleCSS({ display: 'flex', flexDirection: 'column', gap: '6px' }).childs([
                div({ class: チェック行 }).childs([this._高さを置くか, span({ text: '高さを置く' })]),
                this._高さ,
                div({ class: チェック行 }).childs([this._層を置くか, span({ text: '層を置く' })]),
                this._層,
                div({ class: チェック行 }).childs([
                    this._塗りを消すか,
                    span({ text: '塗りを消す(通った粗マスの高さと層を外す)' }).setTooltip('オンのとき、なぞった粗マスの塗りを消す。高さと層の設定は使わない。')])])
        )
    }
}
