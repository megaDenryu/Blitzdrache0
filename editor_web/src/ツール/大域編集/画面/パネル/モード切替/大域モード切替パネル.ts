import { div, ButtonC, DivC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import { 地形追従切替 } from '../../../../チャンク編集/画面/パネル/共通/地形追従切替.ts'
import type { 大域編集モード } from './大域モード定義.ts'
import { コンテナ, グリッド, モードボタン, キー操作説明 } from './スタイル.css.ts'

export interface I大域モード切替配線 {
    readonly onモード変更: (モード: 大域編集モード) => void
    readonly on地形追従変更: (有効: boolean) => void
}

const モード一覧: readonly { readonly モード: 大域編集モード; readonly ラベル: string }[] = [
    { モード: '選択', ラベル: '選択' },
    { モード: '大域造成', ラベル: '大域造成' },
    { モード: '広域道路作成', ラベル: '広域道路作成' },
    { モード: '広域道路編集', ラベル: '広域道路編集' },
]

class モード選択ボタン extends ButtonC {
    public constructor(ラベル: string, 選択中: boolean) {
        super({ class: モードボタン, text: ラベル })
        this.setAttribute('data-selected', 選択中 ? 'true' : 'false')
        this.setTooltip(ラベル)
    }

    public 選択状態を設定する(選択中: boolean): this {
        this.setAttribute('data-selected', 選択中 ? 'true' : 'false')
        return this
    }
}

// 4つの大域編集モードを切り替えるLV2素部品。
export class 大域モード切替パネル extends LV2HtmlComponentBase implements I配線可能<I大域モード切替配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I大域モード切替配線> = new 配線ポート<I大域モード切替配線>('大域モード切替パネル')
    private readonly _ボタンマップ: Map<大域編集モード, モード選択ボタン> = new Map<大域編集モード, モード選択ボタン>()

    public constructor(初期モード: 大域編集モード = '選択') {
        super()
        this._componentRoot = this._ルートを構築する(初期モード)
    }

    public 配線する(配線: I大域モード切替配線): this {
        this._配線.配線する(配線)
        return this
    }

    public モードを更新する(選択モード: 大域編集モード): void {
        for (const [モード, ボタン] of this._ボタンマップ.entries()) {
            ボタン.選択状態を設定する(モード === 選択モード)
        }
    }

    private _ルートを構築する(初期モード: 大域編集モード): DivC {
        return (
            div({ class: コンテナ }).childs([
                div({ class: グリッド }).childs(
                    モード一覧.map(({ モード, ラベル }) => {
                        const btn = new モード選択ボタン(ラベル, モード === 初期モード)
                        this._ボタンマップ.set(モード, btn)
                        return btn.onClick(() => {
                            this.モードを更新する(モード)
                            this._配線.先.onモード変更(モード)
                        })
                    }),
                ),
                div({ class: キー操作説明, text: 'Alt: 次のモード / Shift+Alt: 前のモード' }),
                new 地形追従切替().切替時((有効) => this._配線.先.on地形追従変更(有効)),
            ])
        )
    }
}
