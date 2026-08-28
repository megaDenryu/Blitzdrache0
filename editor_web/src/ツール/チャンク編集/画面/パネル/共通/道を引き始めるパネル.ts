import { div, button, DivC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import { 棚のカード, 棚のカードの見出し, 棚の案内文 } from './棚.css.ts'
import { 副ボタン } from './道路の操作ボタン群.css.ts'

export interface I道を引き始めるパネル配線 {
    readonly on新しい道を始める: () => void
}

// 下パネルの棚へ置く、これから1本の道を引き始めるためのパネル。チャンク編集と大域編集が共有する。
// 既に在る道への操作(点を消す・道を消す)を持たないのは、それが「選んでいるものへの操作」であり
// 右サイドバーの持ち物だからである(設計正本の判断14)。
// 参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断14」
export class 道を引き始めるパネル extends LV2HtmlComponentBase implements I配線可能<I道を引き始めるパネル配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I道を引き始めるパネル配線> = new 配線ポート<I道を引き始めるパネル配線>('道を引き始めるパネル')

    public constructor() {
        super()
        this._componentRoot = this._ルートを構築する()
    }

    public 配線する(配線: I道を引き始めるパネル配線): this {
        this._配線.配線する(配線)
        return this
    }

    private _ルートを構築する(): DivC {
        return (
            div({ class: 棚のカード }).childs([
                div({ class: 棚のカードの見出し, text: '道を引く' }),
                button({ class: 副ボタン, text: '新しい道を始める' })
                    .setTooltip('新しい道を始める')
                    .onClick(() => this._配線.先.on新しい道を始める()),
                div({
                    class: 棚の案内文,
                    text: '押したあと地形を左クリックすると、その場所から新しい道が始まる。',
                })])
        )
    }
}
