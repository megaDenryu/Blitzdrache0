import { div, DivC, LV2HtmlComponentBase } from 'sengen-ui'
import { 永続化パネル } from '../../../チャンク編集/画面/パネル/永続化/index.ts'
import { 役割の凡例を作る } from '../三次元/役割の凡例.ts'
import { セクション, セクション見出し, 建物インスペクター枠 } from '../スタイル.css.ts'
import { 入口の向きパネル } from './入口の向きパネル.ts'
import { 選んでいる升目の札 } from './選んでいる升目の札.ts'
import { 階の一覧パネル } from './階の一覧パネル.ts'

// 右サイドバーへ出す建物の設定一式。階の一覧・いま選んでいる升目・入口の向き・識別色の凡例と、
// 保存と読み込みを収める。エディタ領域には編集の対象そのもの(建物の形と平面図)だけを残すため、
// 設定はここへ集める(設計正本の判断14)。
export class 建物インスペクターパネル extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    public readonly 階の一覧: 階の一覧パネル = new 階の一覧パネル()
    public readonly 選んでいる升目: 選んでいる升目の札 = new 選んでいる升目の札()
    public readonly 入口の向き: 入口の向きパネル = new 入口の向きパネル()
    public readonly 永続化: 永続化パネル = new 永続化パネル()

    public constructor() {
        super()
        this._componentRoot = div({ class: 建物インスペクター枠 }).childs([
            this.階の一覧,
            this.選んでいる升目,
            this.入口の向き,
            div({ class: セクション }).childs([
                div({ class: セクション見出し, text: '役割の識別色' }),
                役割の凡例を作る(),
            ]),
            this.永続化,
        ])
    }

    public override delete(): void {
        this.階の一覧.delete()
        this.選んでいる升目.delete()
        this.入口の向き.delete()
        this.永続化.delete()
        super.delete()
    }
}
