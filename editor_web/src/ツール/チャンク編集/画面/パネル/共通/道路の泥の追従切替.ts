import { span, checkbox, DivC, CheckboxInputC } from 'sengen-ui'
import type { 道路の泥の追従方針 } from '../../../編集モデル/index.ts'
import { チェック行, チェック入力 } from './スタイル.css.ts'

const 説明文 = 'オンのとき、道路の点を動かす・足す・消すたびに道路下の泥の帯を焼き直し、古い位置の泥を焼く前の材質へ戻します。オフのときは下の焼き直しボタンを押したときだけ泥が変わります。'

// 道路を変えたときに道路下の泥の帯を自動で焼き直すかどうかを切り替えるチェックボックス(LV1拡張)。
export class 道路の泥の追従切替 extends DivC {
    private readonly _チェック: CheckboxInputC

    public constructor(初期方針: 道路の泥の追従方針) {
        super({ class: チェック行 })
        this._チェック = checkbox({ class: チェック入力, checked: 初期方針 === '追従する' })
        this.childs([
            this._チェック,
            span({ text: '道路を動かしたら泥も焼き直す' }).setTooltip(説明文)])
    }

    public 切替時(コールバック: (方針: 道路の泥の追従方針) => void): this {
        this._チェック.onCheckChange((有効) => { コールバック(有効 ? '追従する' : '追従しない') })
        return this
    }
}
