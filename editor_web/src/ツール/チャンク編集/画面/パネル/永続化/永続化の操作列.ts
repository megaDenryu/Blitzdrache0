import { button, div, span, ButtonC, DivC, SpanC, LV2HtmlComponentBase } from 'sengen-ui'
import type { 進行状況表示パネル } from '../../../永続化パネル操作.ts'
import { 操作列枠, アクションボタン, 副アクションボタン, 状態メッセージ, エラー状態メッセージ } from './永続化の操作列.css.ts'

// 保存・読み込みのボタンと通信結果・エラー文言の表示をひとまとめにした最小の並び。
// 永続化パネル(チャンク編集の右サイドバーの枠)と楽曲名の欄(名前の行)がこれを
// 1フィールドで保持し、置き場ごとの見た目は各自の外側の枠のスタイルで調整する。
export class 永続化の操作列 extends LV2HtmlComponentBase implements 進行状況表示パネル {
    protected _componentRoot: DivC
    private readonly _保存ボタン: ButtonC
    private readonly _読込ボタン: ButtonC
    private readonly _状態表示: SpanC

    public constructor() {
        super()
        this._保存ボタン = button({ class: アクションボタン, text: '保存' }).setTooltip('保存')
        this._読込ボタン = button({ class: 副アクションボタン, text: '読み込み' }).setTooltip('読み込み')
        this._状態表示 = span({ class: 状態メッセージ, text: '未保存' }).setTooltip('未保存')
        this._componentRoot = div({ class: 操作列枠 }).childs([this._保存ボタン, this._読込ボタン, this._状態表示])
    }

    public on保存クリック(コールバック: () => void): void {
        this._保存ボタン.onClick(() => コールバック())
    }

    public on読込クリック(コールバック: () => void): void {
        this._読込ボタン.onClick(() => コールバック())
    }

    public ボタン活性状態を更新する(活性: boolean): void {
        this._保存ボタン.setStyleCSS({ opacity: 活性 ? '1' : '0.5', pointerEvents: 活性 ? 'auto' : 'none' })
        this._読込ボタン.setStyleCSS({ opacity: 活性 ? '1' : '0.5', pointerEvents: 活性 ? 'auto' : 'none' })
    }

    public 状態文言を更新する(文言: string, エラーか: boolean = false): void {
        this._状態表示
            .setTooltip(文言)
            .removeClass([状態メッセージ, エラー状態メッセージ])
            .addClass(エラーか ? エラー状態メッセージ : 状態メッセージ)
            .setTextContent(文言)
    }

    public override delete(): void {
        this._保存ボタン.delete()
        this._読込ボタン.delete()
        this._状態表示.delete()
        super.delete()
    }
}
