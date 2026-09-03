import { div, DivC, ButtonC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import { モードの並び, モードボタン } from '../パネル/モード切替/スタイル.css.ts'
import { 表示面一覧, type 表示面 } from './表示面.ts'

export interface I表示面の切替配線 {
    readonly on表示面変更: (面: 表示面) => void
}

class 表示面ボタン extends ButtonC {
    public constructor(面: 表示面, 選択中: boolean) {
        super({ class: モードボタン, text: 面 })
        this.選択状態を設定する(選択中)
        this.setTooltip(面 === '三次元' ? '地形を三次元で見て筆でなでる' : '真上から見て等高線と粗マスで地形を描く')
    }

    public 選択状態を設定する(選択中: boolean): this {
        this.setAttribute('data-selected', 選択中 ? 'true' : 'false')
        return this
    }
}

// 操作帯に置く「三次元 / 見下ろし図」の切替。見た目はモードの切替と同じ並びにし、押した面を配線先へ伝える。
// いまどの面かの正本はツールルートのUI状態であり、この部品は表示を更新されるまで自分で選択を変えない。
// 参照: `_doc/設計/見下ろし図による地形編集.md`「判断6」「操作契約」
export class 表示面の切替 extends LV2HtmlComponentBase implements I配線可能<I表示面の切替配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I表示面の切替配線> = new 配線ポート<I表示面の切替配線>('表示面の切替')
    private readonly _ボタン表: Map<表示面, 表示面ボタン> = new Map<表示面, 表示面ボタン>()

    public constructor(初期: 表示面) {
        super()
        this._componentRoot = div({ class: モードの並び }).childs(
            表示面一覧.map((面) => {
                const ボタン = new 表示面ボタン(面, 面 === 初期)
                this._ボタン表.set(面, ボタン)
                return ボタン.onClick(() => this._配線.先.on表示面変更(面))
            }),
        )
    }

    public 配線する(配線: I表示面の切替配線): this {
        this._配線.配線する(配線)
        return this
    }

    public 表示を更新する(面: 表示面): void {
        for (const [候補, ボタン] of this._ボタン表.entries()) {
            ボタン.選択状態を設定する(候補 === 面)
        }
    }
}
