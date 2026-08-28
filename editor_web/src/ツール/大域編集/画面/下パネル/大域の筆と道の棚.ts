import { div, DivC, LV2部品集約Base } from 'sengen-ui'
import { 棚の枠, 棚の列 } from '../../../チャンク編集/画面/パネル/共通/棚.css.ts'
import type { 大域編集モード } from '../パネル/モード切替/大域モード定義.ts'
import { 大域の筆と道の棚の部品 } from './大域の筆と道の棚の部品.ts'

// 下パネルへ出す、これから使うものの棚。大域造成の筆と、これから引き始める道を横へ並べる。
// 選んでいる道の設定をここへ置かないのは、それが右サイドバーの持ち物だからである。
// 棚は常に見えているため、どれかを選んだときは操作サービスがモードもそちらへ移し、
// 押した結果が必ず画面へ返るようにする(設計正本の判断14、エディター制作スキル第5条)。
// 参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断14」
export class 大域の筆と道の棚 extends LV2部品集約Base<大域の筆と道の棚の部品> {
    protected _componentRoot: DivC
    public readonly 部品: 大域の筆と道の棚の部品

    public constructor(初期モード: 大域編集モード) {
        super()
        this.部品 = 大域の筆と道の棚の部品.作る(初期モード)
        this._componentRoot = this._ルートを構築する(this.部品)
    }

    public モードの案内を更新する(モード: 大域編集モード): void {
        this.部品.案内.モードを更新する(モード)
    }

    protected _ルートを構築する(部品: 大域の筆と道の棚の部品): DivC {
        return (
            div({ class: 棚の枠 }).childs([
                部品.案内,
                div({ class: 棚の列 }).childs([
                    部品.大域の筆,
                    部品.道を引き始める])])
        )
    }

    public override delete(): void {
        this.部品.delete()
        super.delete()
    }
}
