import { div, DivC, CanvasC, LV2HtmlComponentBase } from 'sengen-ui'
import {
    場面を作る,
    透視カメラを作る,
    環境光を作る,
    平行光源を作る,
    描画ループ,
    レイキャスト入力,
} from 'SengenThree'
import { ワールド編集状態, 地表材質 } from '../../../ワールド/編集モデル/index.ts'
import { 地形メッシュ部品 } from '../../../ワールド/画面/三次元/地形/地形メッシュ部品.ts'
import { ブラシリング部品 } from '../../../ワールド/画面/三次元/ブラシ/ブラシリング部品.ts'
import { 道路帯メッシュ部品 } from '../../../ワールド/画面/三次元/道路/道路帯メッシュ部品.ts'
import { 道路ノードメッシュ部品 } from '../../../ワールド/画面/三次元/道路/道路ノードメッシュ部品.ts'
import { 軌道カメラ制御器 } from '../../../ワールド/画面/三次元/カメラ/軌道カメラ制御器.ts'
import { チャンク境界格子部品 } from './チャンク境界格子部品.ts'
import { コンテナ, キャンバス } from './大域三次元ビュー部品スタイル.css.ts'

function HTMLキャンバスか(要素: HTMLElement): 要素 is HTMLCanvasElement {
    return 'getContext' in 要素
}

export class 描画キャンバス要素 extends CanvasC {
    public constructor() {
        super({ class: キャンバス })
    }
}

// 1024m大域地形・4x4境界線・広域道路・マクロブラシを束ねる大域ビュー部品。
export class 大域三次元ビュー部品 extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    public readonly キャンバス要素: 描画キャンバス要素
    public readonly 場面: ReturnType<typeof 場面を作る>
    public readonly カメラ: ReturnType<typeof 透視カメラを作る>
    public readonly カメラ制御: 軌道カメラ制御器
    public readonly 描画ループ: 描画ループ
    public readonly レイキャスト: レイキャスト入力
    public readonly 地形: 地形メッシュ部品
    public readonly チャンク境界: チャンク境界格子部品
    public readonly ブラシリング: ブラシリング部品
    public readonly 道路帯: 道路帯メッシュ部品
    public readonly 道路ノード: 道路ノードメッシュ部品

    public constructor(編集状態: ワールド編集状態) {
        super()
        const 地表材質モデル = new 地表材質(編集状態.大域高さ場.解像度, 編集状態.大域高さ場.一辺のメートル)
        this.地形 = new 地形メッシュ部品(編集状態.大域高さ場, 地表材質モデル)
        this.チャンク境界 = new チャンク境界格子部品(
            編集状態.大域世界構造.区画割り.一辺のメートル,
            編集状態.大域世界構造.区画割り.軸あたりチャンク数,
        )
        this.ブラシリング = new ブラシリング部品()
        this.道路帯 = new 道路帯メッシュ部品()
        this.道路ノード = new 道路ノードメッシュ部品()

        this.カメラ = 透視カメラを作る({ 画角: 50, アスペクト比: 16 / 9, 奥クリップ距離: 5000 })
        this.カメラ制御 = new 軌道カメラ制御器(this.カメラ, 800, 20, 3000)

        this.場面 = 場面を作る()
            .背景色を設定する(0x070b14)
            .childs([
                this.カメラ,
                環境光を作る({ 色: 0xe2e8f0, 強さ: 0.5 }),
                平行光源を作る({ 色: 0xffedd5, 強さ: 1.2 }).位置を設定する(400, 800, 300),
                this.地形,
                this.チャンク境界,
                this.ブラシリング,
                this.道路帯,
                this.道路ノード,
            ])

        this.キャンバス要素 = new 描画キャンバス要素()
        this._componentRoot = div({ class: コンテナ }).child(this.キャンバス要素)

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
        this.キャンバス要素.delete()
        super.delete()
    }
}
