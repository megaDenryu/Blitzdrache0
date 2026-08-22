import { div, CanvasC, DivC, LV2HtmlComponentBase } from 'sengen-ui'
import { 場面を作る, 平行光源を作る, 描画ループ, 環境光を作る, 透視カメラを作る } from 'SengenThree'
import { 立体の見取りを組み立てる, 見取りの外接箱を求める, type 表示する立体, type 建物の格子の編集モデル } from '../../編集モデル/index.ts'
import { セクション, セクション見出し, 横並び, 選択ボタン } from '../スタイル.css.ts'
import { 建物の立体メッシュ部品 } from './建物の立体メッシュ部品.ts'
import { 見回しの視点 } from './見回しの視点.ts'
import { 役割の凡例を作る } from './役割の凡例.ts'
import { 三次元のキャンバス, 三次元の枠 } from './三次元表示のスタイル.css.ts'

function HTMLキャンバスか(要素: HTMLElement): 要素 is HTMLCanvasElement {
    return 'getContext' in 要素
}

// 建物の格子を役割ごとの識別色で立体として見せるパネル。平面図と並べて出し、升目の並びが立体として
// どう積まれるかを人が確かめるためのものである。絵の真実はエンジンが担うため実部品のglbは読まず、
// 役割ごとの識別色の直方体だけを描く(判断9)。
export class 建物の三次元パネル extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    private readonly _キャンバス = new CanvasC({ class: 三次元のキャンバス })
    private readonly _立体 = new 建物の立体メッシュ部品()
    private readonly _視点 = new 見回しの視点()
    private readonly _カメラ = 透視カメラを作る({ 画角: 45, アスペクト比: 16 / 9, 奥クリップ距離: 500 })
    private readonly _場面 = 場面を作る()
    private readonly _描画ループ: 描画ループ
    private _いまの見取り: readonly 表示する立体[] = []

    public constructor() {
        super()
        this._場面.背景色を設定する(0x0b0f19).childs([
            this._カメラ,
            環境光を作る({ 色: 0xe2e8f0, 強さ: 0.7 }),
            平行光源を作る({ 色: 0xffedd5, 強さ: 1.1 }).位置を設定する(12, 20, 8),
            this._立体,
        ])
        this._componentRoot = div({ class: セクション }).childs([
            div({ class: セクション見出し, text: '三次元の識別色表示' }),
            div({ class: 三次元の枠 }).child(this._キャンバス),
            役割の凡例を作る(),
            div({ class: 横並び }).childs([
                div({ class: 選択ボタン, text: '左へ回す' }).setTooltip('左へ回す').onClick(() => this.左へ回す()),
                div({ class: 選択ボタン, text: '右へ回す' }).setTooltip('右へ回す').onClick(() => this.右へ回す()),
            ]),
        ])
        this._描画ループ = 描画ループ.キャンバスへ作る(this.キャンバスの実体を取り出す(), this._場面, this._カメラ)
        this._描画ループ.開始する()
        this.カメラを見取りへ合わせる()
    }

    public 再構築する(モデル: 建物の格子の編集モデル): void {
        this._いまの見取り = 立体の見取りを組み立てる(モデル.升目を昇順に並べる())
        this._立体.見取りを描き直す(this._いまの見取り)
        this.カメラを見取りへ合わせる()
    }

    // キャンバスが実際に占めている大きさを自分で測って描画の解像度を合わせる。枠の高さはCSSが決めるため、
    // 呼び出し側が寸法を持参すると、CSSの値と食い違った解像度で描いても誰も気づかない。
    public いまの枠の大きさへ合わせる(ピクセル比: number): void {
        const 枠 = this.キャンバスの実体を取り出す().getBoundingClientRect()
        if (枠.width <= 0 || 枠.height <= 0) return
        this._カメラ.アスペクト比を更新する(枠.width / 枠.height)
        this._描画ループ.寸法を合わせる(枠.width, 枠.height, ピクセル比)
    }

    public override delete(): void {
        this._描画ループ.破棄する()
        this._場面.破棄する()
        this._キャンバス.delete()
        super.delete()
    }

    private 左へ回す(): void {
        this._視点.左へ回す()
        this.カメラを見取りへ合わせる()
    }

    private 右へ回す(): void {
        this._視点.右へ回す()
        this.カメラを見取りへ合わせる()
    }

    private カメラを見取りへ合わせる(): void {
        const 置き場 = this._視点.置き場を求める(見取りの外接箱を求める(this._いまの見取り))
        this._カメラ.位置を設定する(置き場.位置メートル.x, 置き場.位置メートル.y, 置き場.位置メートル.z)
        this._カメラ.注視点を設定する(置き場.注視点メートル.x, 置き場.注視点メートル.y, 置き場.注視点メートル.z)
    }

    private キャンバスの実体を取り出す(): HTMLCanvasElement {
        const 要素 = this._キャンバス.dom.element
        if (!HTMLキャンバスか(要素)) throw new Error('三次元表示のキャンバス要素がHTMLCanvasElementではない')
        return 要素
    }
}
