import { div, span, DivC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import { パネル, 見出し行, アクション区画, 平坦化ボタン, 行ボタン群, 接地ボタン } from './スタイル.css.ts'
import { 建物件数ラベル } from './置いた建物のパネル/建物件数ラベル.ts'
import { 建物削除ボタン } from './置いた建物のパネル/建物削除ボタン.ts'
import { 選んだ建物への操作ボタン } from './置いた建物のパネル/選んだ建物への操作ボタン.ts'

export interface I置いた建物のパネル配線 {
    readonly on基礎平坦化: () => void
    readonly on地面接地: () => void
    readonly on建物削除: () => void
}

// このチャンクへ置いた建物の件数と、いま選んでいる建物への操作(基礎の平坦化・接地・削除)を
// まとめたLV2素部品。右サイドバーへ置く。これから置ける建物の一覧は下パネルの棚が持つ
// (選ぶ対象と使う道具を同じ区画へ混ぜないため。設計正本の判断14)。
export class 置いた建物のパネル extends LV2HtmlComponentBase implements I配線可能<I置いた建物のパネル配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I置いた建物のパネル配線> = new 配線ポート<I置いた建物のパネル配線>('置いた建物のパネル')
    private readonly _件数表示: 建物件数ラベル = new 建物件数ラベル(0)
    private readonly _平坦化ボタン: 選んだ建物への操作ボタン = new 選んだ建物への操作ボタン('選択建物の基礎に合わせて地形造成', 平坦化ボタン)
    private readonly _接地ボタン: 選んだ建物への操作ボタン = new 選んだ建物への操作ボタン('地面に接地', 接地ボタン)
    private readonly _削除ボタン: 建物削除ボタン = new 建物削除ボタン()

    public constructor() {
        super()
        this._componentRoot = this._ルートを構築する()
    }

    public 配線する(配線: I置いた建物のパネル配線): this {
        this._配線.配線する(配線)
        return this
    }

    public 件数を更新する(件数: number): void {
        this._件数表示.件数を更新する(件数)
    }

    // 3つの操作はどれも選んでいる建物にだけ効くため、選択の有無で一緒に押せるかが決まる。
    public 選択建物有効状態を設定する(選択あり: boolean): void {
        this._平坦化ボタン.有効状態を設定する(選択あり)
        this._接地ボタン.有効状態を設定する(選択あり)
        this._削除ボタン.有効状態を設定する(選択あり)
    }

    public override delete(): void {
        this._件数表示.delete()
        this._平坦化ボタン.delete()
        this._接地ボタン.delete()
        this._削除ボタン.delete()
        super.delete()
    }

    private _ルートを構築する(): DivC {
        return (
            div({ class: パネル }).childs([
                div({ class: 見出し行 }).childs([
                    span({ text: '置いた建物' }).setTooltip('置いた建物'),
                    this._件数表示]),
                div({ class: アクション区画 }).childs([
                    this._平坦化ボタン.onClick(() => this._配線.先.on基礎平坦化()),
                    div({ class: 行ボタン群 }).childs([
                        this._接地ボタン.onClick(() => this._配線.先.on地面接地()),
                        this._削除ボタン.onClick(() => this._配線.先.on建物削除())])])])
        )
    }
}
