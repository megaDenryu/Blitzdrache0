import { div, DivC, CanvasC, LV2HtmlComponentBase } from 'sengen-ui'
import { 描画ループ, レイキャスト入力 } from 'SengenThree'
import { ワールド編集状態 } from '../../../チャンク編集/編集モデル/index.ts'
import type { 大域シーン部品束 } from './大域三次元ビュー部品/シーン構築.ts'
import { 大域シーンを構築する } from './大域三次元ビュー部品/シーン構築.ts'
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
// シーンの内訳の組み立ては大域三次元ビュー部品/シーン構築.ts が持つ。
export class 大域三次元ビュー部品 extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    public readonly キャンバス要素: 描画キャンバス要素
    public readonly 場面: 大域シーン部品束['場面']
    public readonly カメラ: 大域シーン部品束['カメラ']
    public readonly カメラ制御: 大域シーン部品束['カメラ制御']
    public readonly 描画ループ: 描画ループ
    public readonly レイキャスト: レイキャスト入力
    public readonly 地形: 大域シーン部品束['地形']
    public readonly チャンク境界: 大域シーン部品束['チャンク境界']
    public readonly ブラシリング: 大域シーン部品束['ブラシリング']
    public readonly 道路帯: 大域シーン部品束['道路帯']
    public readonly 道路ノード: 大域シーン部品束['道路ノード']

    public constructor(編集状態: ワールド編集状態, 初期背景色: string | number = 0x070b14) {
        super()
        const シーン = 大域シーンを構築する(編集状態, 初期背景色)
        this.場面 = シーン.場面
        this.カメラ = シーン.カメラ
        this.カメラ制御 = シーン.カメラ制御
        this.地形 = シーン.地形
        this.チャンク境界 = シーン.チャンク境界
        this.ブラシリング = シーン.ブラシリング
        this.道路帯 = シーン.道路帯
        this.道路ノード = シーン.道路ノード

        this.キャンバス要素 = new 描画キャンバス要素()
        this._componentRoot = div({ class: コンテナ }).child(this.キャンバス要素)

        const dom要素 = this.キャンバス要素.dom.element
        if (!HTMLキャンバスか(dom要素)) {
            throw new Error('キャンバス要素がHTMLCanvasElementではありません')
        }

        this.描画ループ = 描画ループ.キャンバスへ作る(dom要素, this.場面, this.カメラ)
        this.レイキャスト = new レイキャスト入力()
    }

    public 背景色を設定する(色: string | number): this {
        this.場面.背景色を設定する(色)
        return this
    }

    // テーマ切替時に地形・道路の材質色を差し替える(大域三次元ビューに植生は無い)。
    public 地形色を設定する(色: number): this {
        this.地形.地形色を更新する(色)
        return this
    }

    public 道路色を設定する(色: number): this {
        this.道路帯.道路色を更新する(色)
        return this
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
