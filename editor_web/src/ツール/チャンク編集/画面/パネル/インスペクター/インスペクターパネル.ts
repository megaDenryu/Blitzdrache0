import { div, DivC, LV2部品集約Base } from 'sengen-ui'
import type { 編集モード } from '../モード切替/モード定義.ts'
import { インスペクター部品 } from './インスペクター部品.ts'
import { インスペクター枠 } from './スタイル.css.ts'

// 右サイドバーへ出すチャンクの設定一式。選んでいる道と建物の設定、散布と地表の焼き直し、
// 保存と読み込みを収める。この枠の中だけが縦にスクロールする。
//
// モードで表示を出し分けないのは、ここに並ぶのが「いま選んでいるものの設定」であって
// 「これから使う道具」ではないためである。道具の出し分けは下パネルの棚が受け持つ。
// 例外は見下ろし図の等高線と大升の設定であり、そのモードのときだけ出す。見下ろし図の設定を
// 三次元の筆のモードでも出すと、右サイドバーが縦に伸びて選んでいるものの設定が下へ押し出されるためである。
// いま編集しているチャンクの名前は、エディタ領域の上部の操作帯が持つ(設計正本の判断14)。
// 参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断14」
export class インスペクターパネル extends LV2部品集約Base<インスペクター部品> {
    protected _componentRoot: DivC
    public readonly 部品: インスペクター部品

    public constructor() {
        super()
        this.部品 = インスペクター部品.作る()
        this._componentRoot = this._ルートを構築する(this.部品)
    }

    public モードに応じて表示する(モード: 編集モード): void {
        this.部品.等高線.setStyleCSS({ display: モード === '等高線' ? '' : 'none' })
        this.部品.大升.setStyleCSS({ display: モード === '大升' ? '' : 'none' })
    }

    protected _ルートを構築する(部品: インスペクター部品): DivC {
        return (
            div({ class: インスペクター枠 }).childs([
                部品.等高線,
                部品.大升,
                部品.道路,
                部品.建物,
                部品.散布,
                部品.地表の焼き直し,
                部品.永続化])
        )
    }

    public override delete(): void {
        this.部品.delete()
        super.delete()
    }
}
