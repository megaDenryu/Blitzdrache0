import { div, span, select, DivC, SelectC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { 層割当, 地表材質層 } from '../../../生成/編集資源契約.ts'
import { 地表材質層一覧 } from '../編集モデル/index.ts'
import { 層割当行, 層ラベル, 層セレクト } from './スタイル.css.ts'

export interface I層割当パネル配線 {
    readonly on割当変更: (層: 地表材質層, 材質名: string) => void
}

// 地表材質4層(草・泥・岩・砂)それぞれがどのエンジン材質名を参照するかを選ぶセレクト群。
export class 層割当パネル extends LV2HtmlComponentBase implements I配線可能<I層割当パネル配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I層割当パネル配線> = new 配線ポート<I層割当パネル配線>('層割当パネル')
    private readonly _選択群: Record<地表材質層, SelectC>

    public constructor() {
        super()
        this._選択群 = { 草: select({ class: 層セレクト }), 泥: select({ class: 層セレクト }), 岩: select({ class: 層セレクト }), 砂: select({ class: 層セレクト }) }
        this._componentRoot = div().childs(
            地表材質層一覧.map((層) =>
                div({ class: 層割当行 }).childs([span({ class: 層ラベル, text: 層 }).setTooltip(層), this._選択群[層]]),
            ),
        )
    }

    public 配線する(配線: I層割当パネル配線): this {
        this._配線.配線する(配線)
        for (const 層 of 地表材質層一覧) {
            this._選択群[層].onSelectChange(() => this._配線.先.on割当変更(層, this._選択群[層].getValue()))
        }
        return this
    }

    public 選択肢を更新する(材質名一覧: ReadonlyArray<string>, 現在の割当: 層割当): void {
        for (const 層 of 地表材質層一覧) {
            this._選択群[層].setOptions(
                材質名一覧.map((材質名) => ({ value: 材質名, text: 材質名, selected: 材質名 === 現在の割当[層] })),
            )
        }
    }

    public override delete(): void {
        for (const 層 of 地表材質層一覧) this._選択群[層].delete()
        super.delete()
    }
}
