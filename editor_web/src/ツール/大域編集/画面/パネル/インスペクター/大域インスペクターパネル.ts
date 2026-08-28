import { div, DivC, LV2部品集約Base } from 'sengen-ui'
import { 大域インスペクター部品 } from './大域インスペクター部品.ts'
import { インスペクター枠 } from './スタイル.css.ts'

// 右サイドバーへ出す大域世界の設定一式。選んでいる道の設定、チャンクへの切り出しての書き出し、
// 保存と読み込みを収める。この枠の中だけが縦にスクロールする。
//
// モードで表示を出し分けないのは、ここに並ぶのが「いま選んでいるものの設定」であって
// 「これから使う道具」ではないためである。道具の出し分けは下パネルの棚が受け持つ。
// いま編集している世界の名前は、エディタ領域の上部の操作帯が持つ(設計正本の判断14)。
// 参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断14」
export class 大域インスペクターパネル extends LV2部品集約Base<大域インスペクター部品> {
    protected _componentRoot: DivC
    public readonly 部品: 大域インスペクター部品

    public constructor() {
        super()
        this.部品 = 大域インスペクター部品.作る()
        this._componentRoot = this._ルートを構築する(this.部品)
    }

    protected _ルートを構築する(部品: 大域インスペクター部品): DivC {
        return (
            div({ class: インスペクター枠 }).childs([
                部品.道路,
                部品.スライス,
                部品.永続化])
        )
    }

    public override delete(): void {
        this.部品.delete()
        super.delete()
    }
}
