import { canvas, div, DivC, LV2HtmlComponentBase } from 'sengen-ui'
import type { CanvasC } from 'sengen-ui'
import {
    場面を作る,
    透視カメラを作る,
    環境光を作る,
    平行光源を作る,
    描画ループ,
    レイキャスト入力,
} from 'SengenThree'
import type { ワールド編集状態 } from '../../編集モデル/index.ts'
import { 地形メッシュ部品 } from './地形/地形メッシュ部品.ts'
import { ブラシリング部品 } from './ブラシ/ブラシリング部品.ts'
import { 道路帯メッシュ部品 } from './道路/道路帯メッシュ部品.ts'
import { 道路ノードメッシュ部品 } from './道路/道路ノードメッシュ部品.ts'
import { 建物メッシュ部品 } from './建物/建物メッシュ部品.ts'
import { 散布個体群部品 } from './散布/散布個体群部品.ts'
import { 軌道カメラ制御器 } from './カメラ/軌道カメラ制御器.ts'
import * as styles from './三次元ビュー部品スタイル.css.ts'

function HTMLキャンバスか(要素: HTMLElement): 要素 is HTMLCanvasElement {
    return 'getContext' in 要素
}

// 三次元シーングラフ・描画ループ・レイキャスト・カメラ操作を束ねるビュー部品。
export class 三次元ビュー部品 extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    public readonly キャンバス要素: CanvasC
    public readonly 場面: ReturnType<typeof 場面を作る>
    public readonly カメラ: ReturnType<typeof 透視カメラを作る>
    public readonly カメラ制御: 軌道カメラ制御器
    public readonly 描画ループ: 描画ループ
    public readonly レイキャスト: レイキャスト入力
    public readonly 地形: 地形メッシュ部品
    public readonly ブラシリング: ブラシリング部品
    public readonly 道路帯: 道路帯メッシュ部品
    public readonly 道路ノード: 道路ノードメッシュ部品
    public readonly 建物: 建物メッシュ部品
    public readonly 散布: 散布個体群部品

    public constructor(編集状態: ワールド編集状態) {
        super()
        const チャンク = 編集状態.チャンク一覧マップ.values().next().value
        if (!チャンク) throw new Error('初期チャンクが登録されていません')

        this.地形 = new 地形メッシュ部品(チャンク.高さ場, チャンク.地表材質)
        this.ブラシリング = new ブラシリング部品()
        this.道路帯 = new 道路帯メッシュ部品()
        this.道路ノード = new 道路ノードメッシュ部品()
        this.建物 = new 建物メッシュ部品()
        this.散布 = new 散布個体群部品()

        this.カメラ = 透視カメラを作る({ 画角: 50, アスペクト比: 16 / 9, 奥クリップ距離: 2000 })
        this.カメラ制御 = new 軌道カメラ制御器(this.カメラ)

        this.場面 = 場面を作る()
            .背景色を設定する(0x0b0f19)
            .childs([
                this.カメラ,
                環境光を作る({ 色: 0xe2e8f0, 強さ: 0.6 }),
                平行光源を作る({ 色: 0xffedd5, 強さ: 1.2 }).位置を設定する(120, 200, 80),
                this.地形,
                this.ブラシリング,
                this.道路帯,
                this.道路ノード,
                this.建物,
                this.散布,
            ])

        let キャンバス参照: CanvasC | null = null
        this._componentRoot = div({ class: styles.コンテナ }).child(
            canvas({ class: styles.キャンバス }).tap((c: CanvasC) => {
                キャンバス参照 = c
            }),
        )
        if (キャンバス参照 === null) throw new Error('キャンバス生成に失敗しました')
        this.キャンバス要素 = キャンバス参照

        const dom要素 = this.キャンバス要素.dom.element
        if (!HTMLキャンバスか(dom要素)) {
            throw new Error('キャンバス要素がHTMLCanvasElementではありません')
        }

        this.描画ループ = 描画ループ.キャンバスへ作る(dom要素, this.場面, this.カメラ)
        this.レイキャスト = new レイキャスト入力()
    }

    public 寸法を合わせる(幅: number, 高さ: number, ピクセル比: number = 1): void {
        this.描画ループ.寸法を合わせる(幅, 高さ, ピクセル比)
    }

    public override delete(): void {
        this.レイキャスト.破棄する()
        this.描画ループ.破棄する()
        this.場面.破棄する()
        super.delete()
    }
}
