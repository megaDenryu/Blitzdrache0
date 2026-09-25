import { button, div, span, ButtonC, DivC, SpanC, LV2HtmlComponentBase } from 'sengen-ui'
import {
    サイドバーの縦積みの見た目,
    名前の行の横並びの見た目,
    type 永続化の操作列の見た目,
} from './永続化の操作列.css.ts'

// 保存・読み込みのボタンと通信結果・エラー文言の表示をひとまとめにした最小の並び。
// 永続化パネル(右サイドバーの枠)と楽曲名の欄(名前の行)がこれを1フィールドで保持する。
// 置き場ごとに並べ方が違うため、置き場の名前を持つ生成の口を2つ置き、見た目の組はその口が選ぶ。
// 進行状況表示パネルの形は構造的な型の一致で満たす。画面の層はツールルートの型を取り込めないため implements を書かない。
export class 永続化の操作列 extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    private readonly _保存ボタン: ButtonC
    private readonly _読込ボタン: ButtonC
    private readonly _状態表示: SpanC

    private constructor(private readonly _見た目: 永続化の操作列の見た目) {
        super()
        this._保存ボタン = button({ class: _見た目.保存ボタン, text: '保存' }).setTooltip('保存')
        this._読込ボタン = button({ class: _見た目.読込ボタン, text: '読み込み' }).setTooltip('読み込み')
        this._状態表示 = span({ class: _見た目.状態文言, text: '未保存' }).setTooltip('未保存')
        this._componentRoot = div({ class: _見た目.枠 }).childs([
            div({ class: _見た目.ボタン行 }).childs([this._保存ボタン, this._読込ボタン]),
            this._状態表示,
        ])
    }

    // 右サイドバーの永続化パネルに置く。ボタン2つが幅を分け合い、状態文言は下の行に出る。
    public static サイドバーの枠に置く形で作る(): 永続化の操作列 {
        return new 永続化の操作列(サイドバーの縦積みの見た目)
    }

    // 楽曲名の欄のような1行の中に置く。ボタンと状態文言を横1列に詰める。
    public static 名前の行に置く形で作る(): 永続化の操作列 {
        return new 永続化の操作列(名前の行の横並びの見た目)
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

    // 注意: 見た目のクラスは合成で空白区切りの複数の名前になるため、配列でなく1つずつ文字列で外す。
    // SengenUIは文字列だけを空白で分けて外し、配列の要素に空白があるとDOMが例外を投げる。
    public 状態文言を更新する(文言: string, エラーか: boolean = false): void {
        this._状態表示
            .setTooltip(文言)
            .removeClass(this._見た目.状態文言)
            .removeClass(this._見た目.エラーの状態文言)
            .addClass(エラーか ? this._見た目.エラーの状態文言 : this._見た目.状態文言)
            .setTextContent(文言)
    }

    public override delete(): void {
        this._保存ボタン.delete()
        this._読込ボタン.delete()
        this._状態表示.delete()
        super.delete()
    }
}
