import { div, span, p, DivC, LV2部品集約Base } from 'sengen-ui'
import type { 大域編集モード } from '../モード切替/index.ts'
import { 大域インスペクター部品 } from './大域インスペクター部品.ts'
import { インスペクター枠, ヘッダー, タイトル, サブタイトル, バッジ } from './スタイル.css.ts'

// 大域インスペクター枠・ヘッダー・モードに応じたサブパネルの表示切り替えを統括する部品集約。
export class 大域インスペクターパネル extends LV2部品集約Base<大域インスペクター部品> {
    protected _componentRoot: DivC
    public readonly 部品: 大域インスペクター部品

    public constructor(初期モード: 大域編集モード = '大域カメラ') {
        super()
        this.部品 = 大域インスペクター部品.作る(初期モード)
        this._componentRoot = this._ルートを構築する(this.部品)
        this.表示モードを切り替える(初期モード)
    }

    public 表示モードを切り替える(モード: 大域編集モード): void {
        this.部品.モード切替.モードを更新する(モード)
        this.部品.ヒント.モードを更新する(モード)
        this.部品.造成.setStyleCSS({ display: モード === '大域造成' ? 'flex' : 'none' })
        this.部品.道路.setStyleCSS({
            display: モード === '大域カメラ' || モード === '広域道路作成' || モード === '広域道路編集' ? 'flex' : 'none',
        })
    }

    public override delete(): void {
        this.部品.delete()
        super.delete()
    }

    protected _ルートを構築する(部品: 大域インスペクター部品): DivC {
        return (
            div({ class: インスペクター枠 }).childs([
                div({ class: ヘッダー }).childs([
                    div().childs([
                        div({ class: タイトル, text: '大域グリッドマネージャー' }),
                        p({ class: サブタイトル, text: 'マクロ造成＆シームレススライサー' }),
                    ]),
                    span({ class: バッジ, text: 'World: 1024m (4x4)' }),
                ]),
                部品.モード切替,
                部品.ヒント,
                部品.造成,
                部品.道路,
                部品.スライス,
            ])
        )
    }
}
