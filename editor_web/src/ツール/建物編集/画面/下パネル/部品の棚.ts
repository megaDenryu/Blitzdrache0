import { div, DivC, LV2部品集約Base } from 'sengen-ui'
import { 筆パネル } from '../筆パネル.ts'
import { 棚の枠, 説明文 } from '../スタイル.css.ts'

// 下パネルへ出す、これから配置する部品の棚。壁・窓・扉・煙突といった部品を置く筆の並びを収める。
// 「いま選んでいるものの中の要素の一覧」(階)は右サイドバーへ置き、この棚には
// 「これから使うものの並び」だけを置く(設計正本の判断14)。
// 参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断14」
export class 部品の棚 extends LV2部品集約Base<筆パネル> {
    protected _componentRoot: DivC
    public readonly 筆: 筆パネル = new 筆パネル()

    public constructor() {
        super()
        this._componentRoot = this._ルートを構築する(this.筆)
    }

    protected _ルートを構築する(筆: 筆パネル): DivC {
        return (
            div({ class: 棚の枠 }).childs([
                div({
                    class: 説明文,
                    text: '棚から筆を選び、平面図を触って置く。升目の中央が升目への筆、周りの帯が面への筆に効く。右クリックで消す。',
                }),
                筆])
        )
    }

    public override delete(): void {
        this.筆.delete()
        super.delete()
    }
}
