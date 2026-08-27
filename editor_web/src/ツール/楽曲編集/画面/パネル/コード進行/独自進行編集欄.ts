import { div, textInput, button, DivC, ButtonC, TextInputC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import { 和音の根音の下限, type コード進行, type 和音 } from '../../../../../生成/編集資源契約.ts'
import { テキスト入力, 副ボタン, 横並び行 } from '../共通/スタイル.css.ts'
import { 和音の既定の続くステップ数 } from './和音の欄の値.ts'
import { 和音編集行部品 } from './和音編集行部品.ts'
import { 独自進行の保存ボタン } from './独自進行の保存ボタン.ts'
import { 一覧枠, 編集枠 } from './スタイル.css.ts'

export interface I独自進行編集欄配線 {
    readonly on保存: (進行: コード進行) => void
}

// 独自コード進行を組み立てる欄。編集中の和音の並びを保持し、名前と和音が揃ってはじめて保存できる。
export class 独自進行編集欄 extends LV2HtmlComponentBase implements I配線可能<I独自進行編集欄配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I独自進行編集欄配線> = new 配線ポート<I独自進行編集欄配線>('独自進行編集欄')
    private readonly _名前入力: TextInputC
    private readonly _和音追加ボタン: ButtonC
    private readonly _保存ボタン: 独自進行の保存ボタン = new 独自進行の保存ボタン()
    private readonly _和音行コンテナ: DivC = div({ class: 一覧枠 })
    private _和音行一覧: 和音編集行部品[] = []
    private _編集中の和音一覧: 和音[] = []

    public constructor() {
        super()
        this._名前入力 = textInput({ class: テキスト入力, placeholder: '独自進行の名前' })
            .setTooltip('保存する独自進行の名前')
        this._和音追加ボタン = button({ class: 副ボタン, text: '+ 和音を追加' }).setTooltip('和音を末尾へ追加')
        this._componentRoot = div({ class: 編集枠 }).childs([
            div({ class: 横並び行 }).childs([this._名前入力, this._和音追加ボタン, this._保存ボタン]),
            this._和音行コンテナ,
        ])
    }

    public 配線する(配線: I独自進行編集欄配線): this {
        this._配線.配線する(配線)
        this._名前入力.onInput(() => this._保存できるかを見直す())
        this._和音追加ボタン.onClick(() => this._末尾へ和音を足す())
        this._保存ボタン.onClick(() => this._編集中の進行の保存を伝える())
        return this
    }

    public 進行を読み込む(名前: string, 和音一覧: readonly 和音[]): void {
        this._名前入力.setValue(名前)
        this._編集中の和音一覧 = 和音一覧.map((和音) => ({ ...和音 }))
        this._和音行一覧を組み直す()
    }

    public override delete(): void {
        for (const 行 of this._和音行一覧) 行.delete()
        this._和音行一覧 = []
        this._名前入力.delete()
        this._和音追加ボタン.delete()
        this._保存ボタン.delete()
        super.delete()
    }

    private _末尾へ和音を足す(): void {
        this._編集中の和音一覧.push({
            根音: 和音の根音の下限,
            種類: '長三和音',
            続くステップ数: 和音の既定の続くステップ数,
        })
        this._和音行一覧を組み直す()
    }

    private _編集中の進行の保存を伝える(): void {
        if (!this._配線.配線済みか) return
        this._配線.先.on保存({
            名前: this._名前入力.getValue(),
            和音一覧: this._編集中の和音一覧.map((和音) => ({ ...和音 })),
        })
    }

    private _保存できるかを見直す(): void {
        this._保存ボタン.編集中の内容を反映する(this._名前入力.getValue(), this._編集中の和音一覧.length)
    }

    private _和音行一覧を組み直す(): void {
        for (const 行 of this._和音行一覧) 行.delete()
        this._和音行コンテナ.clearChildren()
        this._和音行一覧 = this._編集中の和音一覧.map((和音, 位置) => {
            const 行 = new 和音編集行部品(和音, 位置).配線する({
                on和音変更: (新和音) => this._編集中の和音一覧.splice(位置, 1, 新和音),
                on削除: () => this._位置の和音を取り除く(位置),
            })
            this._和音行コンテナ.child(行)
            return 行
        })
        this._保存できるかを見直す()
    }

    private _位置の和音を取り除く(位置: number): void {
        this._編集中の和音一覧.splice(位置, 1)
        this._和音行一覧を組み直す()
    }
}
