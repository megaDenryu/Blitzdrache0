import { div, DivC, LV2HtmlComponentBase } from 'sengen-ui'
import { 煙突の段数, 選べる煙突の段数 } from '../編集モデル/index.ts'
import { 全ての升目への筆, 全ての面への筆, type 升目への筆, type 面への筆 } from '../操作コマンド/index.ts'
import { セクション見出し, 横並び, 棚の1組, 棚の列, 選択ボタン } from './スタイル.css.ts'

export interface I筆パネル配線 {
    readonly on升目への筆を選ぶ: (筆: 升目への筆) => void
    readonly on面への筆を選ぶ: (筆: 面への筆) => void
    readonly on煙突の段数を選ぶ: (段数: 煙突の段数) => void
}

// これから配置する部品を選ぶ筆の並び。升目そのものへ効く筆と面へ効く筆を別の組にするのは、
// 触る場所が違うためである(中央か周囲か)。煙突の段数を別の組にするのは、段数が筆そのものではなく、
// 煙突を立てる筆が使う値だからである。
// 横長の3つの組として並べるのは、この並びが下パネルの棚に載るためである(設計正本の判断14)。
export class 筆パネル extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    private readonly _升目の列: DivC = div({ class: 横並び })
    private readonly _面の列: DivC = div({ class: 横並び })
    private readonly _煙突の段数の列: DivC = div({ class: 横並び })

    public constructor() {
        super()
        this._componentRoot = div({ class: 棚の列 }).childs([
            div({ class: 棚の1組 }).childs([
                div({ class: セクション見出し, text: '升目への筆(升目の中央を触る)' }),
                this._升目の列,
            ]),
            div({ class: 棚の1組 }).childs([
                div({ class: セクション見出し, text: '面への筆(升目の周りの帯を触る)' }),
                this._面の列,
            ]),
            div({ class: 棚の1組 }).childs([
                div({ class: セクション見出し, text: '煙突の段数' }),
                this._煙突の段数の列,
            ]),
        ])
    }

    public 再構築する(
        選んだ升目への筆: 升目への筆,
        選んだ面への筆: 面への筆,
        選んだ煙突の段数: 煙突の段数,
        配線: I筆パネル配線,
    ): void {
        this._升目の列.clearChildren()
        for (const 筆 of 全ての升目への筆) {
            this._升目の列.child(この筆のボタン(筆, 筆 === 選んだ升目への筆, () => 配線.on升目への筆を選ぶ(筆)))
        }
        this._面の列.clearChildren()
        for (const 筆 of 全ての面への筆) {
            this._面の列.child(この筆のボタン(筆, 筆 === 選んだ面への筆, () => 配線.on面への筆を選ぶ(筆)))
        }
        this._煙突の段数の列.clearChildren()
        for (const 段数 of 選べる煙突の段数) {
            this._煙突の段数の列.child(
                この筆のボタン(`${段数.段数}段`, 段数.同じか(選んだ煙突の段数), () => 配線.on煙突の段数を選ぶ(段数)),
            )
        }
    }
}

function この筆のボタン(綴り: string, 選んでいるか: boolean, 押されたら: () => void): DivC {
    return div({ class: 選択ボタン, text: 綴り })
        .setTooltip(綴り)
        .setAttribute('data-selected', String(選んでいるか))
        .onClick(押されたら)
}
