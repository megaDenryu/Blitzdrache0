import { div, DivC, ButtonC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import { モード一覧, type 編集モード } from './モード定義.ts'
import { モードの並び, モードボタン } from './スタイル.css.ts'

export interface Iモード切替配線 {
    readonly onモード変更: (新モード: 編集モード) => void
}

class モード選択ボタン extends ButtonC {
    public constructor(モード名: string, 選択中: boolean) {
        super({ class: モードボタン, text: モード名 })
        this.setAttribute('data-selected', 選択中 ? 'true' : 'false')
        this.setTooltip(モード名)
    }

    public 選択状態を設定する(選択中: boolean): this {
        this.setAttribute('data-selected', 選択中 ? 'true' : 'false')
        return this
    }
}

// 6種類の編集モードを切り替えるボタンの並び。エディタ領域の上部の固定の行へ置くため横一列にする。
// モードごとの操作の案内をここへ持たないのは、案内が下パネルの棚の持ち物だからである(判断14)。
export class モード切替パネル extends LV2HtmlComponentBase implements I配線可能<Iモード切替配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<Iモード切替配線> = new 配線ポート<Iモード切替配線>('モード切替パネル')
    private readonly _ボタンマップ: Map<編集モード, モード選択ボタン> = new Map<編集モード, モード選択ボタン>()

    public constructor(初期モード: 編集モード) {
        super()
        this._componentRoot = this._ルートを構築する(初期モード)
    }

    public 配線する(配線: Iモード切替配線): this {
        this._配線.配線する(配線)
        return this
    }

    public モードを更新する(選択モード: 編集モード): void {
        for (const [モード, ボタン] of this._ボタンマップ.entries()) {
            ボタン.選択状態を設定する(モード === 選択モード)
        }
    }

    private _ルートを構築する(初期モード: 編集モード): DivC {
        return (
            div({ class: モードの並び }).childs(
                モード一覧.map((モード) => {
                    const ボタン = new モード選択ボタン(モード, モード === 初期モード)
                    this._ボタンマップ.set(モード, ボタン)
                    return ボタン.onClick(() => {
                        this.モードを更新する(モード)
                        this._配線.先.onモード変更(モード)
                    })
                }),
            )
        )
    }
}
