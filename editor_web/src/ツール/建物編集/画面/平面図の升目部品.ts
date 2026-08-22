import { div, span, DivC, LV2HtmlComponentBase } from 'sengen-ui'
import type { はめ口の値, 升目の宣言, 升目の座標 } from '../../../生成/編集資源契約.ts'
import { 側面のはめ口を読む, 全側面, type 升目の側面, type 升目を置けない理由 } from '../編集モデル/index.ts'
import { 升目中央, 升目枠, 右面ボタン, 左面ボタン, 正面ボタン, 背面ボタン, 面ボタン } from './スタイル.css.ts'

export interface I升目の配線 {
    readonly on升目を触る: (座標: 升目の座標) => void
    readonly on面を触る: (座標: 升目の座標, 側面: 升目の側面) => void
}

// 1升目を描くのに要る、格子から読んだ事実の組。触ったときに何が起きるかは筆が決めるため、この部品は持たない。
export interface I升目の見取り {
    readonly 座標: 升目の座標
    readonly 宣言: 升目の宣言 | undefined
    readonly 根か: boolean
    readonly 升目を置けない理由: 升目を置けない理由 | undefined
    readonly 隣に升目があるか: (側面: 升目の側面) => boolean
}

const 側面ごとの位置: Record<升目の側面, string> = {
    正面: 正面ボタン,
    背面: 背面ボタン,
    左面: 左面ボタン,
    右面: 右面ボタン,
}

// 平面図の1升目。中央が升目そのもの、周囲の4つが面である。升目が無い座標も枠として描くが、
// 置けない座標(真下に升目が無い上の階・格子から離れた座標)は不活性にして触れなくする。
// 置ける場所が画面から読めるようにするためである。
export class 平面図の升目部品 extends LV2HtmlComponentBase {
    protected _componentRoot: DivC

    public constructor(見取り: I升目の見取り, 配線: I升目の配線) {
        super()
        const 触れるか = 見取り.宣言 !== undefined || 見取り.升目を置けない理由 === undefined
        const 中央 = div({ class: 升目中央 })
            .child(span({ text: 見取り.宣言 === undefined ? `${見取り.座標.横},${見取り.座標.奥}` : 中央の綴り(見取り.宣言) }))
            .setTooltip(中央の説明(見取り))
            .setAttribute('data-升目あり', String(見取り.宣言 !== undefined))
            .setAttribute('data-根', String(見取り.根か))
            .setAttribute('data-触れる', String(触れるか))
        if (触れるか) 中央.onClick(() => 配線.on升目を触る(見取り.座標))
        this._componentRoot = div({ class: 升目枠 }).childs([
            中央,
            ...全側面.map((側面) => this.面のボタンを作る(見取り, 側面, 配線)),
        ])
    }

    private 面のボタンを作る(見取り: I升目の見取り, 側面: 升目の側面, 配線: I升目の配線): DivC {
        const 隣があるか = 見取り.隣に升目があるか(側面)
        const 値 = 見取り.宣言 === undefined ? undefined : 側面のはめ口を読む(見取り.宣言, 側面)
        const 印 = 見取り.宣言 === undefined ? '空' : 隣があるか ? '継ぎ口' : 壁の印(値)
        return div({ class: `${面ボタン} ${側面ごとの位置[側面]}` })
            .setTooltip(`${側面}: ${印}${飾りの説明(値)}`)
            .setAttribute('data-壁', 印)
            .onClick(() => 配線.on面を触る(見取り.座標, 側面))
    }
}

// 置けない事情の綴りは`格子の成り立ちの判定`が持つ1つの正本であり、ここでは括弧に入れて添えるだけである。
function 中央の説明(見取り: I升目の見取り): string {
    const 位置 = `横${見取り.座標.横}・奥${見取り.座標.奥}・階${見取り.座標.階}`
    if (見取り.宣言 !== undefined || 見取り.升目を置けない理由 === undefined) return 位置
    return `${位置}(${見取り.升目を置けない理由})`
}

function 壁の印(値: はめ口の値 | undefined): string {
    return 値 === undefined || 値.種類 === '入れない' ? '空' : 値.値.壁の種類
}

// 面に付いている飾りの説明。飾りを平面図に出さないと、煙突を立てた面と立てていない面が同じ姿に見える。
function 飾りの説明(値: はめ口の値 | undefined): string {
    if (値 === undefined || 値.種類 === '入れない' || 値.値.外面の飾り.種類 === '付けない') return ''
    const 飾り = 値.値.外面の飾り
    return 飾り.種類 === '煙突を立てる' ? `・煙突を立てる(${飾り.値.段数}段)` : `・${飾り.種類}`
}

function 中央の綴り(宣言: 升目の宣言): string {
    const 床 = 宣言.床 === '張る' ? '床' : ''
    const 屋根 = 宣言.屋根 === '載せる' ? '屋根' : ''
    return `${床}${屋根}` === '' ? '骨格' : `${床}${屋根}`
}
