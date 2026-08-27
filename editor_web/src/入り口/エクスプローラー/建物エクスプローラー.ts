import { div, DivC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { 建物の格子の一覧項目, 建物定義ID } from '../../境界/通信/index.ts'
import type { タブ識別子 } from '../タブ識別子.ts'
import { コンテナ, セクション見出し } from './スタイル.css.ts'
import type { 領域エクスプローラー } from './領域エクスプローラー.ts'
import { 建物木 } from './木/建物木.ts'

export interface I建物エクスプローラー配線 {
    readonly on建物を開く: (建物定義ID: 建物定義ID, 表示名: string) => void
    readonly on建物を作る: () => void
}

// 編集領域「建物」のエクスプローラー。保存済みの建物定義の一覧と、新しい建物を作る口を並べる。
// 一覧の正本はサーバーが持つ台帳であり、この型は渡された並びを描くだけで自分では持たない。
export class 建物エクスプローラー extends LV2HtmlComponentBase implements I配線可能<I建物エクスプローラー配線>, 領域エクスプローラー {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I建物エクスプローラー配線> = new 配線ポート<I建物エクスプローラー配線>('建物エクスプローラー')
    private readonly _建物木: 建物木

    public constructor() {
        super()
        this._建物木 = new 建物木(
            (建物定義ID, 表示名) => this._配線.先.on建物を開く(建物定義ID, 表示名),
            () => this._配線.先.on建物を作る(),
        )
        this._componentRoot = div({ class: コンテナ }).childs([
            div({ class: セクション見出し, text: '建物' }),
            ...this._建物木.ルート要素一覧,
        ])
    }

    public 配線する(配線: I建物エクスプローラー配線): this {
        this._配線.配線する(配線)
        return this
    }

    public 一覧を作り直す(一覧: readonly 建物の格子の一覧項目[]): void {
        this._建物木.一覧を作り直す(一覧)
    }

    public 前面のタブに合わせて選択表示する(タブ: タブ識別子): void {
        const 建物定義ID = タブ.建物定義IDを復元する()
        if (建物定義ID !== null) this._建物木.選択表示する(建物定義ID)
    }

    public 選択表示を解除する(): void {
        this._建物木.選択解除する()
    }
}
