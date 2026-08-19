import { button, div, span, DivC, ButtonC, LV2HtmlComponentBase } from 'sengen-ui'
import { 永続化枠, ボタン行, アクションボタン, 副アクションボタン, 状態メッセージ, エラー状態メッセージ } from './スタイル.css.ts'

// 保存・読み込みの操作ボタンと現在の通信結果・エラー文言を表示するパネル。
export class 永続化パネル extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    private readonly _保存ボタン: ButtonC
    private readonly _読込ボタン: ButtonC
    private readonly _状態表示コンテナ: DivC

    public constructor() {
        super()
        this._保存ボタン = button({ class: アクションボタン, text: '保存' }).setTooltip('保存')
        this._読込ボタン = button({ class: 副アクションボタン, text: '読み込み' }).setTooltip('読み込み')
        this._状態表示コンテナ = div({ class: 状態メッセージ }).setTooltip('未保存').child(span({ text: '未保存' }))

        this._componentRoot = div({ class: 永続化枠 }).childs([
            div({ class: ボタン行 }).childs([this._保存ボタン, this._読込ボタン]),
            this._状態表示コンテナ,
        ])
    }

    public on保存クリック(コールバック: () => void): void {
        this._保存ボタン.onClick(() => コールバック())
    }

    public on読込クリック(コールバック: () => void): void {
        this._読込ボタン.onClick(() => コールバック())
    }

    public ボタン活性状態を更新する(活性: boolean): void {
        this._保存ボタン.setStyleCSS({
            opacity: 活性 ? '1' : '0.5',
            pointerEvents: 活性 ? 'auto' : 'none',
        })
        this._読込ボタン.setStyleCSS({
            opacity: 活性 ? '1' : '0.5',
            pointerEvents: 活性 ? 'auto' : 'none',
        })
    }

    public 状態文言を更新する(文言: string, エラーか: boolean = false): void {
        this._状態表示コンテナ.setTooltip(文言).clearChildren()
        this._状態表示コンテナ.child(
            span({ class: エラーか ? エラー状態メッセージ : 状態メッセージ, text: 文言 }),
        )
    }

    public override delete(): void {
        this._保存ボタン.delete()
        this._読込ボタン.delete()
        this._状態表示コンテナ.delete()
        super.delete()
    }
}
