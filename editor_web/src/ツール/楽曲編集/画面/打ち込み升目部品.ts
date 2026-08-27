import { div, DivC, LV2HtmlComponentBase } from 'sengen-ui'
import type { セル } from '../編集モデル/index.ts'
import { 升目 } from './スタイル.css.ts'

// 格子の1升目を表す素部品。セルの種類、進行追従、和音による許可状態、拍の区切りを表示する。
export class 打ち込み升目部品 extends LV2HtmlComponentBase {
    protected _componentRoot: DivC

    public constructor(ステップ: number) {
        super()
        this._componentRoot = div({ class: 升目 })
        const 境界 = ステップ % 4 === 0 ? 'measure' : ステップ % 2 === 0 ? 'beat' : 'step'
        this._componentRoot.setAttribute('data-boundary', 境界)
        this._componentRoot.setAttribute('data-step', String(ステップ))
    }

    public 表示を更新する(対象セル: セル, 許されるか: boolean): void {
        this._componentRoot.setAttribute('data-allowed', String(許されるか))
        switch (対象セル.種類) {
            case '打点なし':
                this._componentRoot.setAttribute('data-kind', 'none')
                this._componentRoot.setAttribute('data-follow', 'none')
                break
            case '音の始まり':
                this._componentRoot.setAttribute('data-kind', 'start')
                this._componentRoot.setAttribute('data-follow', String(対象セル.進行に従うか))
                break
            case '音の継続':
                this._componentRoot.setAttribute('data-kind', 'hold')
                this._componentRoot.setAttribute('data-follow', String(対象セル.進行に従うか))
                break
        }
    }
}
