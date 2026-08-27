import { div, span, textInput, DivC, TextInputC, LV2HtmlComponentBase } from 'sengen-ui'
import type { 建物定義ID } from '../../../境界/建物定義ID.ts'
import { 永続化パネル } from '../../チャンク編集/画面/パネル/永続化/index.ts'
import { 建物の三次元パネル } from './三次元/建物の三次元パネル.ts'
import { 入口の向きパネル } from './入口の向きパネル.ts'
import { 平面図パネル } from './平面図パネル.ts'
import { 筆パネル } from './筆パネル.ts'
import { 階の切替パネル } from './階の切替パネル.ts'
import { コンテナ, セクション, セクション見出し, 名前入力, 断りの文言, 本文幅, 横並び, 触りの知らせ, 表題, 説明文, 選択ボタン } from './スタイル.css.ts'

// 建物1件を編集する文書タブの画面全体。平面図・筆・階の切替・入口の向き・取り消しやり直し・保存と、
// 建物の形を見せる三次元表示を1枚に並べる。三次元表示の視点はモードのボタンではなくポインタで直に操り、
// 識別色は形の上へ重ねる補助として使う(判断13)。
// 参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断13」
export class 建物編集画面 extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    public readonly 平面図: 平面図パネル = new 平面図パネル()
    public readonly 三次元: 建物の三次元パネル = new 建物の三次元パネル()
    public readonly 筆: 筆パネル = new 筆パネル()
    public readonly 入口の向き: 入口の向きパネル = new 入口の向きパネル()
    public readonly 階の切替: 階の切替パネル = new 階の切替パネル()
    public readonly 永続化: 永続化パネル = new 永続化パネル()
    public readonly 表示名入力: TextInputC
    public readonly 取り消しボタン: DivC
    public readonly やり直しボタン: DivC
    private readonly _表題: DivC
    private readonly _触りの知らせ: DivC

    public constructor(private readonly _建物定義ID: 建物定義ID) {
        super()
        this._表題 = div({ class: 表題, text: _建物定義ID })
        this._触りの知らせ = div({ class: 触りの知らせ, text: '' })
        this.表示名入力 = textInput({ class: 名前入力, value: '', placeholder: '表示名' }).setTooltip('表示名')
        this.取り消しボタン = div({ class: 選択ボタン, text: '取り消し' }).setTooltip('取り消し')
        this.やり直しボタン = div({ class: 選択ボタン, text: 'やり直し' }).setTooltip('やり直し')
        this._componentRoot = div({ class: コンテナ }).child(
            div({ class: 本文幅 }).childs([
                this._表題,
                div({ class: 説明文, text: '升目の中央を触ると升目への筆が、周りの帯を触ると面への筆が効く。' }),
                div({ class: セクション }).childs([
                    div({ class: セクション見出し, text: '表示名' }),
                    div({ class: 横並び }).child(this.表示名入力),
                ]),
                this.階の切替,
                this.入口の向き,
                this.筆,
                div({ class: セクション }).childs([
                    div({ class: セクション見出し, text: '平面図' }),
                    this.平面図,
                    this._触りの知らせ,
                    div({ class: 横並び }).childs([this.取り消しボタン, this.やり直しボタン]),
                ]),
                this.三次元,
                this.永続化,
            ]),
        )
    }

    // 表示名を変えたことが画面の見えへ返るように、表題を建物定義IDと表示名の並びにする。
    public 表題を更新する(表示名: string): void {
        this._表題.setTextContent(表示名 === '' ? this._建物定義ID : `${表示名}(${this._建物定義ID})`)
    }

    // 筆が断られた事情を平面図の下へ出す。断りを黙って捨てると、触ったのに何も起きない理由が人に読めない。
    public 触りの知らせを更新する(文言: string): void {
        this._触りの知らせ.setTooltip(文言).clearChildren()
        if (文言 !== '') this._触りの知らせ.child(span({ class: 断りの文言, text: 文言 }))
    }

    public 取り消しとやり直しの活性を更新する(取り消せるか: boolean, やり直せるか: boolean): void {
        this.取り消しボタン.setStyleCSS({ opacity: 取り消せるか ? '1' : '0.4' })
        this.やり直しボタン.setStyleCSS({ opacity: やり直せるか ? '1' : '0.4' })
    }

    public override delete(): void {
        this.平面図.delete()
        this.三次元.delete()
        this.筆.delete()
        this.入口の向き.delete()
        this.階の切替.delete()
        this.永続化.delete()
        this.表示名入力.delete()
        super.delete()
    }
}
