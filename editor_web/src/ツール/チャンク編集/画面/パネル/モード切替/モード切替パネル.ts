import { div, DivC, ButtonC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import { 地形追従切替 } from '../共通/地形追従切替.ts'
import { モード一覧, モードヒント写像, type 編集モード } from './モード定義.ts'
import { コンテナ, グリッド, モードボタン, ヒント枠, キー操作説明 } from './スタイル.css.ts'

export interface Iモード切替配線 {
    readonly onモード変更: (新モード: 編集モード) => void
    readonly on地形追従変更: (有効: boolean) => void
}

class モードヒント表示 extends DivC {
    public constructor(初期ヒント: string) {
        super({ class: ヒント枠, text: 初期ヒント })
    }

    public ヒントを更新する(新ヒント: string): this {
        this.setTextContent(新ヒント)
        return this
    }
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

// 6種類の編集モード切り替えボタンとヒント文を表示するLV2素部品。
export class モード切替パネル extends LV2HtmlComponentBase implements I配線可能<Iモード切替配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<Iモード切替配線> = new 配線ポート<Iモード切替配線>('モード切替パネル')
    private readonly _ボタンマップ: Map<編集モード, モード選択ボタン> = new Map<編集モード, モード選択ボタン>()
    private readonly _ヒント要素: モードヒント表示

    public constructor(初期モード: 編集モード) {
        super()
        this._ヒント要素 = new モードヒント表示(モードヒント写像[初期モード])
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
        this._ヒント要素.ヒントを更新する(モードヒント写像[選択モード])
    }

    private _ルートを構築する(初期モード: 編集モード): DivC {
        return (
            div({ class: コンテナ }).childs([
                div({ class: グリッド }).childs(
                    モード一覧.map((モード) => {
                        const btn = new モード選択ボタン(モード, モード === 初期モード)
                        this._ボタンマップ.set(モード, btn)
                        return btn.onClick(() => {
                            this.モードを更新する(モード)
                            this._配線.先.onモード変更(モード)
                        })
                    }),
                ),
                div({ class: キー操作説明, text: 'Alt: 次のモード / Shift+Alt: 前のモード' }),
                new 地形追従切替().切替時((有効) => this._配線.先.on地形追従変更(有効)),
                this._ヒント要素])
        )
    }
}
