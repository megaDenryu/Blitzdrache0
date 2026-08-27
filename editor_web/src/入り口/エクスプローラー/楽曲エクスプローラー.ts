import { div, DivC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { 楽曲ID } from '../../境界/通信/index.ts'
import type { タブ識別子 } from '../タブ識別子.ts'
import { コンテナ, セクション見出し } from './スタイル.css.ts'
import type { 領域エクスプローラー } from './領域エクスプローラー.ts'
import { 楽曲木 } from './木/楽曲木.ts'

export interface I楽曲エクスプローラー配線 {
    readonly on楽曲を開く: (楽曲ID: 楽曲ID, 表示名: string) => void
    readonly on楽曲を作る: () => void
}

// 編集領域「楽曲」のエクスプローラー。保存済みの楽曲の一覧と、新しい楽曲を作る口を並べる。
export class 楽曲エクスプローラー extends LV2HtmlComponentBase implements I配線可能<I楽曲エクスプローラー配線>, 領域エクスプローラー {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I楽曲エクスプローラー配線> = new 配線ポート<I楽曲エクスプローラー配線>('楽曲エクスプローラー')
    private readonly _楽曲木: 楽曲木

    public constructor() {
        super()
        this._楽曲木 = new 楽曲木(
            (楽曲ID, 表示名) => this._配線.先.on楽曲を開く(楽曲ID, 表示名),
            () => this._配線.先.on楽曲を作る(),
        )
        this._componentRoot = div({ class: コンテナ }).childs([
            div({ class: セクション見出し, text: '楽曲' }),
            ...this._楽曲木.ルート要素一覧,
        ])
    }

    public 配線する(配線: I楽曲エクスプローラー配線): this {
        this._配線.配線する(配線)
        return this
    }

    public 一覧を作り直す(一覧: readonly 楽曲ID[]): void {
        this._楽曲木.一覧を作り直す(一覧)
    }

    public 前面のタブに合わせて選択表示する(タブ: タブ識別子): void {
        const 楽曲ID = タブ.楽曲IDを復元する()
        if (楽曲ID !== null) this._楽曲木.選択表示する(楽曲ID)
    }

    public 選択表示を解除する(): void {
        this._楽曲木.選択解除する()
    }
}
