import { div, button, DivC, ButtonC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import { モード一覧, モードヒント写像, type 編集モード } from './モード定義.ts'
import * as styles from './スタイル.css.ts'

export interface Iモード切替配線 {
    readonly onモード変更: (新モード: 編集モード) => void
}

// 6種類の編集モード切り替えボタンとヒント文を表示するLV2素部品。
export class モード切替パネル extends LV2HtmlComponentBase implements I配線可能<Iモード切替配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<Iモード切替配線> = new 配線ポート<Iモード切替配線>('モード切替パネル')
    private readonly _ボタンマップ: Map<編集モード, ButtonC> = new Map<編集モード, ButtonC>()
    private readonly _ヒント要素: DivC

    public constructor(初期モード: 編集モード) {
        super()
        this._ヒント要素 = div({ class: styles.ヒント枠, text: モードヒント写像[初期モード] })
        this._componentRoot = this._ルートを構築する(初期モード)
    }

    public 配線する(配線: Iモード切替配線): this {
        this._配線.配線する(配線)
        return this
    }

    public モードを更新する(選択モード: 編集モード): void {
        for (const [モード, ボタン] of this._ボタンマップ.entries()) {
            ボタン.setAttribute('data-selected', モード === 選択モード ? 'true' : 'false')
        }
        this._ヒント要素.setTextContent(モードヒント写像[選択モード])
    }

    private _ルートを構築する(初期モード: 編集モード): DivC {
        return (
            div({ class: styles.コンテナ }).childs([
                div({ class: styles.グリッド }).childs(
                    モード一覧.map((モード) =>
                        button({
                            class: styles.モードボタン,
                            text: モード,
                        })
                            .setAttribute('data-selected', モード === 初期モード ? 'true' : 'false')
                            .tap((btn: ButtonC) => {
                                this._ボタンマップ.set(モード, btn)
                            })
                            .onClick(() => {
                                this.モードを更新する(モード)
                                this._配線.先.onモード変更(モード)
                            }),
                    ),
                ),
                this._ヒント要素])
        )
    }
}
