import { div, span, button, DivC, ButtonC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { 楽曲, コード進行参照 } from '../../../../../生成/編集資源契約.ts'
import { 行コンテナ, 項目ラベル, パネル外枠, パネル見出し, 主ボタン } from '../共通/スタイル.css.ts'
import { 選択中のパターンを探す } from './パターン操作判定.ts'
import { パターン選択欄 } from './パターン選択欄.ts'
import { パターン削除ボタン } from './パターン削除ボタン.ts'
import { パターン表示名入力欄 } from './パターン表示名入力欄.ts'
import { パターンの進行選択欄 } from './パターンの進行選択欄.ts'
import { パターン操作行, 編集グリッド } from './スタイル.css.ts'

export interface Iパターンパネル配線 {
    readonly onパターン選択: (名乗り: string) => void
    readonly onパターン追加: () => void
    readonly onパターン削除: (名乗り: string) => void
    readonly on表示名変更: (名乗り: string, 新しい表示名: string) => void
    readonly on進行変更: (名乗り: string, 新しい進行の参照: コード進行参照) => void
}

// 編集対象パターンの切り替え・追加・削除・表示名および進行変更を行うパネル。
export class パターンパネル extends LV2HtmlComponentBase implements I配線可能<Iパターンパネル配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<Iパターンパネル配線> = new 配線ポート<Iパターンパネル配線>('パターンパネル')
    private readonly _パターン選択: パターン選択欄
    private readonly _追加ボタン: ButtonC
    private readonly _削除ボタン: パターン削除ボタン = new パターン削除ボタン()
    private readonly _表示名入力: パターン表示名入力欄 = new パターン表示名入力欄()
    private readonly _進行選択: パターンの進行選択欄 = new パターンの進行選択欄()
    private _選択中パターンの名乗り: string | null

    public constructor(初期楽曲: 楽曲, 初期選択名乗り: string | null) {
        super()
        this._選択中パターンの名乗り = 初期選択名乗り
        this._パターン選択 = new パターン選択欄(初期楽曲.パターン一覧, 初期選択名乗り)
        this._追加ボタン = button({ class: 主ボタン, text: '+ パターンを追加' }).setTooltip('新規パターンを追加')
        this._componentRoot = this._ルートを構築する()
        this.表示を更新する(初期楽曲, 初期選択名乗り)
    }

    public 配線する(配線: Iパターンパネル配線): this {
        this._配線.配線する(配線)
        this._パターン選択.onSelectChange(() => this._選び直されたパターンを伝える())
        this._追加ボタン.onClick(() => {
            if (this._配線.配線済みか) this._配線.先.onパターン追加()
        })
        this._削除ボタン.onClick(() => this._選択中パターンの削除を伝える())
        this._表示名入力.onChange(() => this._書き換えられた表示名を伝える())
        this._進行選択.onSelectChange(() => this._選び直された進行を伝える())
        return this
    }

    public 表示を更新する(楽曲: 楽曲, 選択中名乗り: string | null): void {
        const 対象パターン = 選択中のパターンを探す(楽曲.パターン一覧, 選択中名乗り)
        this._選択中パターンの名乗り = 対象パターン === null ? null : 対象パターン.名乗り
        this._パターン選択.パターン一覧を反映する(楽曲.パターン一覧, this._選択中パターンの名乗り)
        this._削除ボタン.パターン数を反映する(楽曲.パターン一覧.length)
        this._表示名入力.パターンを反映する(対象パターン)
        this._進行選択.パターンを反映する(対象パターン, 楽曲.独自進行一覧)
    }

    public override delete(): void {
        this._パターン選択.delete()
        this._追加ボタン.delete()
        this._削除ボタン.delete()
        this._表示名入力.delete()
        this._進行選択.delete()
        super.delete()
    }

    private _選び直されたパターンを伝える(): void {
        const 名乗り = this._パターン選択.選ばれた名乗り()
        if (名乗り !== null && this._配線.配線済みか) this._配線.先.onパターン選択(名乗り)
    }

    private _選択中パターンの削除を伝える(): void {
        if (this._選択中パターンの名乗り !== null && this._配線.配線済みか) {
            this._配線.先.onパターン削除(this._選択中パターンの名乗り)
        }
    }

    private _書き換えられた表示名を伝える(): void {
        if (this._選択中パターンの名乗り !== null && this._配線.配線済みか) {
            this._配線.先.on表示名変更(this._選択中パターンの名乗り, this._表示名入力.getValue())
        }
    }

    private _選び直された進行を伝える(): void {
        if (this._選択中パターンの名乗り !== null && this._配線.配線済みか) {
            this._配線.先.on進行変更(this._選択中パターンの名乗り, this._進行選択.選ばれた進行の参照())
        }
    }

    private _ルートを構築する(): DivC {
        return div({ class: パネル外枠 }).childs([
            div({ class: パネル見出し, text: 'パターンの編集' }),
            div({ class: パターン操作行 }).childs([this._パターン選択, this._追加ボタン, this._削除ボタン]),
            div({ class: 編集グリッド }).childs([
                div({ class: 行コンテナ }).childs([
                    span({ class: 項目ラベル, text: 'パターンの表示名' }),
                    this._表示名入力,
                ]),
                div({ class: 行コンテナ }).childs([
                    span({ class: 項目ラベル, text: 'コード進行' }),
                    this._進行選択,
                ]),
            ]),
        ])
    }
}
