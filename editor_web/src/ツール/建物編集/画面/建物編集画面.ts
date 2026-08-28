import { div, DivC, LV2部品集約Base } from 'sengen-ui'
import type { 建物定義ID } from '../../../境界/建物定義ID.ts'
import { 建物編集画面部品 } from './建物編集画面部品.ts'
import { コンテナ, 三次元の柱, 固定の行, 平面図の巻き取り枠, 平面図の柱, 編集面 } from './スタイル.css.ts'

// 建物1件を編集する文書タブのエディタ領域。上に固定の1行(建物の名前と建物ぜんたいに効く操作)を置き、
// その下へ編集の対象そのものだけを並べる。左が建物の形の三次元、右が升目を編む平面図である。
// 設定と一覧は インスペクター(右サイドバー)へ、これから配置する部品の棚は 部品の棚(下パネル)へ渡す
// (設計正本の判断14)。三次元の視点はモードのボタンではなくポインタで直に操る(判断13)。
// 参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断13」「判断14」
export class 建物編集画面 extends LV2部品集約Base<建物編集画面部品> {
    protected _componentRoot: DivC
    private readonly _部品: 建物編集画面部品

    public constructor(建物定義ID: 建物定義ID) {
        super()
        this._部品 = 建物編集画面部品.作る(建物定義ID)
        this._componentRoot = this._ルートを構築する(this._部品)
    }

    public get 部品(): 建物編集画面部品 {
        return this._部品
    }

    protected _ルートを構築する(部品: 建物編集画面部品): DivC {
        return (
            div({ class: コンテナ }).childs([
                div({ class: 固定の行 }).childs([部品.建物名, 部品.操作帯]),
                div({ class: 編集面 }).childs([
                    div({ class: 三次元の柱 }).child(部品.三次元),
                    div({ class: 平面図の柱 }).childs([
                        div({ class: 平面図の巻き取り枠 }).child(部品.平面図),
                        部品.触りの知らせ])])])
        )
    }

    public override delete(): void {
        this._部品.delete()
        super.delete()
    }
}
