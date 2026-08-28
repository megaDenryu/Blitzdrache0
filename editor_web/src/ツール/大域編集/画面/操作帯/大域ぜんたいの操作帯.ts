import { div, span, DivC, LV2部品集約Base, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import { 大域世界表示名 } from '../../../../境界/index.ts'
import { 地形追従切替 } from '../../../チャンク編集/画面/パネル/共通/地形追従切替.ts'
import { 取り消しボタン } from '../../../チャンク編集/画面/パネル/共通/取り消しボタン.ts'
import { 操作帯の枠, 対象の名前, 対象の大きさの札 } from '../../../チャンク編集/画面/パネル/共通/操作帯.css.ts'
import { 大域モード切替パネル } from '../パネル/モード切替/大域モード切替パネル.ts'
import type { 大域編集モード } from '../パネル/モード切替/大域モード定義.ts'

export interface I大域ぜんたいの操作帯配線 {
    readonly onモード変更: (モード: 大域編集モード) => void
    readonly on取り消す: () => void
    readonly on地形追従変更: (有効: boolean) => void
}

// エディタ領域の上部へ固定して置く帯。いま編集している世界の名前と大きさ、編集のモード、
// 世界ぜんたいに効く操作(取り消し・カメラを地形に沿わせるか)を1行へ収める。
// チャンク編集の操作帯と同じ姿にするのは、同じ種類の編集をする道具どうしで操作を揃えるためである。
// 参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断13」「判断14」
export class 大域ぜんたいの操作帯 extends LV2部品集約Base<大域モード切替パネル> implements I配線可能<I大域ぜんたいの操作帯配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I大域ぜんたいの操作帯配線> = new 配線ポート<I大域ぜんたいの操作帯配線>('大域ぜんたいの操作帯')
    private readonly _モード切替: 大域モード切替パネル
    private readonly _取り消し: 取り消しボタン = new 取り消しボタン()
    private readonly _地形追従: 地形追従切替 = new 地形追従切替()

    public constructor(初期モード: 大域編集モード) {
        super()
        this._モード切替 = new 大域モード切替パネル(初期モード)
        this._componentRoot = this._ルートを構築する(this._モード切替)
    }

    public 配線する(配線: I大域ぜんたいの操作帯配線): this {
        this._配線.配線する(配線)
        this._モード切替.配線する({ onモード変更: (モード) => this._配線.先.onモード変更(モード) })
        this._地形追従.切替時((有効) => this._配線.先.on地形追従変更(有効))
        return this
    }

    public 表示を更新する(モード: 大域編集モード, 取り消せるか: boolean): void {
        this._モード切替.モードを更新する(モード)
        this._取り消し.押せるか設定する(取り消せるか)
    }

    protected _ルートを構築する(モード切替: 大域モード切替パネル): DivC {
        return (
            div({ class: 操作帯の枠 }).childs([
                div({ class: 対象の名前, text: 大域世界表示名 }).setTooltip(大域世界表示名),
                span({ class: 対象の大きさの札, text: '世界 1024m (4×4)' }).setTooltip('世界 1024m (4×4)'),
                モード切替,
                this._取り消し.onClick(() => this._配線.先.on取り消す()),
                this._地形追従])
        )
    }

    public override delete(): void {
        this._モード切替.delete()
        this._取り消し.delete()
        this._地形追従.delete()
        super.delete()
    }
}
