import { div, span, p, DivC, LV2部品集約Base } from 'sengen-ui'
import type { 編集モード } from '../モード切替/モード定義.ts'
import { インスペクター部品 } from './インスペクター部品.ts'
import { インスペクター枠, ヘッダー, タイトル群, タイトル, サブタイトル, バッジ } from './スタイル.css.ts'

// インスペクター枠・ヘッダー・モードに応じたサブパネルの表示切り替えを統括する部品集約。
export class インスペクターパネル extends LV2部品集約Base<インスペクター部品> {
    protected _componentRoot: DivC
    public readonly 部品: インスペクター部品

    public constructor(初期モード: 編集モード = 'カメラ') {
        super()
        this.部品 = インスペクター部品.作る(初期モード)
        this._componentRoot = this._ルートを構築する(this.部品)
        this.表示モードを切り替える(初期モード)
    }

    public 表示モードを切り替える(モード: 編集モード): void {
        this.部品.モード切替.モードを更新する(モード)
        this.部品.造成.setStyleCSS({ display: モード === '造成' ? 'flex' : 'none' })
        this.部品.地表ペイント.setStyleCSS({ display: モード === '地表ペイント' ? 'flex' : 'none' })
        this.部品.道路.setStyleCSS({
            display: モード === 'カメラ' || モード === '道作成' || モード === '道編集' ? 'flex' : 'none',
        })
        this.部品.建物.setStyleCSS({ display: モード === '建物' ? 'flex' : 'none' })
    }

    public override delete(): void {
        this.部品.delete()
        super.delete()
    }

    protected _ルートを構築する(部品: インスペクター部品): DivC {
        return (
            div({ class: インスペクター枠 }).childs([
                div({ class: ヘッダー }).childs([
                    div({ class: タイトル群 }).childs([
                        div({ class: タイトル, text: 'ワールドパイプラインエディター' }).setTooltip('ワールドパイプラインエディター'),
                        p({ class: サブタイトル, text: 'チャンク編集パイプライン' }).setTooltip('チャンク編集パイプライン')]),
                    span({ class: バッジ, text: 'チャンク: 256m' }).setTooltip('チャンク: 256m')]),
                部品.モード切替,
                部品.造成,
                部品.地表ペイント,
                部品.道路,
                部品.建物,
                部品.散布,
                部品.永続化])
        )
    }
}
