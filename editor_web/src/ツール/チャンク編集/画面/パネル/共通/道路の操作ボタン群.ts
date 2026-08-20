import { div, span, ButtonC, DivC, SpanC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import { ボタン群の枠, 行ボタン群, 対象の道ラベル, 削除ボタン, 副ボタン } from './道路の操作ボタン群.css.ts'

export interface I道路の操作ボタン群配線 {
    readonly on選択中の道路点を削除: () => void
    readonly on対象の道を削除: () => void
    readonly on新しい道を始める: () => void
    readonly on全ての道を消す: () => void
}

class 操作ボタン extends ButtonC {
    public constructor(見出し: string, クラス: string, 初期は押せるか: boolean) {
        super({ class: クラス, text: 見出し, disabled: !初期は押せるか })
        this.setTooltip(見出し)
    }

    public 押せるか設定する(押せるか: boolean): this {
        this.setDisabled(!押せるか)
        return this
    }
}

// 道路の点と道そのものを消す・新しい道を始める・全部消すの4つのボタンと、いま設定が効く道の
// 案内を1組にしたLV2素部品。チャンク編集の道路パネルと大域編集の広域道路パネルが共有する。
export class 道路の操作ボタン群 extends LV2HtmlComponentBase implements I配線可能<I道路の操作ボタン群配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I道路の操作ボタン群配線> = new 配線ポート<I道路の操作ボタン群配線>('道路の操作ボタン群')
    private readonly _案内: SpanC
    private readonly _点削除ボタン: 操作ボタン
    private readonly _道削除ボタン: 操作ボタン

    public constructor() {
        super()
        this._案内 = span({ class: 対象の道ラベル, text: '対象の道: なし' })
        this._点削除ボタン = new 操作ボタン('選択点を削除', 削除ボタン, false)
        this._道削除ボタン = new 操作ボタン('この道を削除', 削除ボタン, false)
        this._componentRoot = this._ルートを構築する()
    }

    public 配線する(配線: I道路の操作ボタン群配線): this {
        this._配線.配線する(配線)
        return this
    }

    public 選択中の道路点があるか設定する(選択あり: boolean): void {
        this._点削除ボタン.押せるか設定する(選択あり)
    }

    // 設定と削除が効く道の案内を書き換える。道路番号は人が数える1始まりで示す。
    public 対象の道を設定する(道路添字: number | null, 本数: number): void {
        this._道削除ボタン.押せるか設定する(道路添字 !== null)
        const 案内文 = 道路添字 === null
            ? `対象の道: なし (全${本数}本。地形をクリックすると新しい道が始まります)`
            : `対象の道: ${道路添字 + 1}本目 / 全${本数}本`
        this._案内.setTextContent(案内文)
        this._案内.setTooltip(案内文)
    }

    public override delete(): void {
        this._点削除ボタン.delete()
        this._道削除ボタン.delete()
        super.delete()
    }

    private _ルートを構築する(): DivC {
        return (
            div({ class: ボタン群の枠 }).childs([
                this._案内,
                div({ class: 行ボタン群 }).childs([
                    this._点削除ボタン.onClick(() => this._配線.先.on選択中の道路点を削除()),
                    this._道削除ボタン.onClick(() => this._配線.先.on対象の道を削除())]),
                div({ class: 行ボタン群 }).childs([
                    new ButtonC({ class: 副ボタン, text: '新しい道' })
                        .setTooltip('新しい道')
                        .onClick(() => this._配線.先.on新しい道を始める()),
                    new ButtonC({ class: 副ボタン, text: '全ての道を消す' })
                        .setTooltip('全ての道を消す')
                        .onClick(() => this._配線.先.on全ての道を消す())])])
        )
    }
}
