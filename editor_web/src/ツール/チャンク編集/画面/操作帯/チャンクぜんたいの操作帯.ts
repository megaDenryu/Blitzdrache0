import { div, span, DivC, LV2部品集約Base, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { チャンク座標 } from '../../../../生成/編集資源契約.ts'
import { チャンク表示名を生成する } from '../../../../境界/index.ts'
import { 地形追従切替 } from '../パネル/共通/地形追従切替.ts'
import { 取り消しボタン } from '../パネル/共通/取り消しボタン.ts'
import { 操作帯の枠, 対象の名前, 対象の大きさの札 } from '../パネル/共通/操作帯.css.ts'
import { モード切替パネル } from '../パネル/モード切替/モード切替パネル.ts'
import type { 編集モード } from '../パネル/モード切替/モード定義.ts'
import { 表示面の切替 } from './表示面の切替.ts'
import { 初期の表示面, type 表示面 } from './表示面.ts'

export interface Iチャンクぜんたいの操作帯配線 {
    readonly onモード変更: (モード: 編集モード) => void
    readonly on取り消す: () => void
    readonly on地形追従変更: (有効: boolean) => void
    readonly on表示面変更: (面: 表示面) => void
}

// エディタ領域の上部へ固定して置く帯。いま編集しているチャンクの名前と大きさ、編集のモード、
// チャンクぜんたいに効く操作(取り消し・カメラを地形に沿わせるか)と、三次元と見下ろし図の切替を1行へ収める。
// 地形の三次元と一緒にスクロールして消えると、いま何をしているかが分からなくなるため固定する。
// 参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断14」
export class チャンクぜんたいの操作帯 extends LV2部品集約Base<モード切替パネル> implements I配線可能<Iチャンクぜんたいの操作帯配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<Iチャンクぜんたいの操作帯配線> = new 配線ポート<Iチャンクぜんたいの操作帯配線>('チャンクぜんたいの操作帯')
    private readonly _モード切替: モード切替パネル
    private readonly _取り消し: 取り消しボタン = new 取り消しボタン()
    private readonly _地形追従: 地形追従切替 = new 地形追従切替()
    private readonly _表示面切替: 表示面の切替 = new 表示面の切替(初期の表示面)
    private readonly _対象座標: チャンク座標

    public constructor(対象座標: チャンク座標, 初期モード: 編集モード) {
        super()
        this._対象座標 = 対象座標
        this._モード切替 = new モード切替パネル(初期モード)
        this._componentRoot = this._ルートを構築する(this._モード切替)
    }

    public 配線する(配線: Iチャンクぜんたいの操作帯配線): this {
        this._配線.配線する(配線)
        this._モード切替.配線する({ onモード変更: (モード) => this._配線.先.onモード変更(モード) })
        this._地形追従.切替時((有効) => this._配線.先.on地形追従変更(有効))
        this._表示面切替.配線する({ on表示面変更: (面) => this._配線.先.on表示面変更(面) })
        return this
    }

    public 表示を更新する(モード: 編集モード, 取り消せるか: boolean, 面: 表示面): void {
        this._モード切替.モードを更新する(モード)
        this._取り消し.押せるか設定する(取り消せるか)
        this._表示面切替.表示を更新する(面)
    }

    protected _ルートを構築する(モード切替: モード切替パネル): DivC {
        const 表示名 = チャンク表示名を生成する(this._対象座標)
        return (
            div({ class: 操作帯の枠 }).childs([
                div({ class: 対象の名前, text: 表示名 }).setTooltip(表示名),
                span({ class: 対象の大きさの札, text: 'チャンク: 256m' }).setTooltip('チャンク: 256m'),
                モード切替,
                this._取り消し.onClick(() => this._配線.先.on取り消す()),
                this._地形追従,
                this._表示面切替])
        )
    }

    public override delete(): void {
        this._モード切替.delete()
        this._取り消し.delete()
        this._地形追従.delete()
        this._表示面切替.delete()
        super.delete()
    }
}
