import { div, span, DivC, LV2HtmlComponentBase } from 'sengen-ui'
import type { はめ口の値, 升目の宣言, 升目の座標 } from '../../../生成/編集資源契約.ts'
import { 側面のはめ口を読む, 全側面, type 升目の側面 } from '../編集モデル/index.ts'
import { 升目中央, 升目枠, 右面ボタン, 左面ボタン, 正面ボタン, 背面ボタン, 面ボタン } from './スタイル.css.ts'

export interface I升目の配線 {
    readonly on升目を触る: (座標: 升目の座標) => void
    readonly on面を触る: (座標: 升目の座標, 側面: 升目の側面) => void
}

const 側面ごとの位置: Record<升目の側面, string> = {
    正面: 正面ボタン,
    背面: 背面ボタン,
    左面: 左面ボタン,
    右面: 右面ボタン,
}

// 平面図の1升目。中央が升目そのもの、周囲の4つが面である。升目が無い座標も枠として描き、
// そこを触ると升目が置かれる(置ける場所が画面から読めるようにするため)。
export class 平面図の升目部品 extends LV2HtmlComponentBase {
    protected _componentRoot: DivC

    public constructor(
        座標: 升目の座標,
        宣言: 升目の宣言 | undefined,
        根か: boolean,
        隣に升目があるか: (側面: 升目の側面) => boolean,
        配線: I升目の配線,
    ) {
        super()
        const 中央 = div({ class: 升目中央 })
            .child(span({ text: 宣言 === undefined ? `${座標.横},${座標.奥}` : 中央の綴り(宣言) }))
            .setTooltip(`横${座標.横}・奥${座標.奥}・階${座標.階}`)
            .setAttribute('data-升目あり', String(宣言 !== undefined))
            .setAttribute('data-根', String(根か))
            .onClick(() => 配線.on升目を触る(座標))
        this._componentRoot = div({ class: 升目枠 }).childs([
            中央,
            ...全側面.map((側面) => this.面のボタンを作る(座標, 宣言, 側面, 隣に升目があるか(側面), 配線)),
        ])
    }

    private 面のボタンを作る(
        座標: 升目の座標,
        宣言: 升目の宣言 | undefined,
        側面: 升目の側面,
        隣があるか: boolean,
        配線: I升目の配線,
    ): DivC {
        const 印 = 宣言 === undefined ? '空' : 隣があるか ? '継ぎ口' : 壁の印(側面のはめ口を読む(宣言, 側面))
        return div({ class: `${面ボタン} ${側面ごとの位置[側面]}` })
            .setTooltip(`${側面}: ${印}`)
            .setAttribute('data-壁', 印)
            .onClick(() => 配線.on面を触る(座標, 側面))
    }
}

function 壁の印(値: はめ口の値): string {
    return 値.種類 === '入れない' ? '空' : 値.値.壁の種類
}

function 中央の綴り(宣言: 升目の宣言): string {
    const 床 = 宣言.床 === '張る' ? '床' : ''
    const 屋根 = 宣言.屋根 === '載せる' ? '屋根' : ''
    return `${床}${屋根}` === '' ? '骨格' : `${床}${屋根}`
}
