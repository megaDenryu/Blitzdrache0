import { div, DivC, LV2部品集約Base } from 'sengen-ui'
import type { 編集モード } from '../パネル/モード切替/モード定義.ts'
import { 棚の枠, 棚の列 } from '../パネル/共通/棚.css.ts'
import { 筆と置くものの棚の部品 } from './筆と置くものの棚の部品.ts'

// 下パネルへ出す、これから使うものの棚。地形の筆・地表の材質・置ける建物・引き始める道を横へ並べる。
// 選んでいる道や建物の設定をここへ置かないのは、それが右サイドバーの持ち物だからである。
// 棚は常に見えているため、どれかを選んだときは同期サービスがモードもそちらへ移し、
// 押した結果が必ず画面へ返るようにする(設計正本の判断14、エディター制作スキル第5条)。
// 参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断14」
export class 筆と置くものの棚 extends LV2部品集約Base<筆と置くものの棚の部品> {
    protected _componentRoot: DivC
    public readonly 部品: 筆と置くものの棚の部品

    public constructor(初期モード: 編集モード) {
        super()
        this.部品 = 筆と置くものの棚の部品.作る(初期モード)
        this._componentRoot = this._ルートを構築する(this.部品)
    }

    public モードの案内を更新する(モード: 編集モード): void {
        this.部品.案内.モードを更新する(モード)
    }

    protected _ルートを構築する(部品: 筆と置くものの棚の部品): DivC {
        return (
            div({ class: 棚の枠 }).childs([
                部品.案内,
                div({ class: 棚の列 }).childs([
                    部品.地形の筆,
                    部品.地表の材質,
                    部品.置ける建物,
                    部品.道を引き始める])])
        )
    }

    public override delete(): void {
        this.部品.delete()
        super.delete()
    }
}
