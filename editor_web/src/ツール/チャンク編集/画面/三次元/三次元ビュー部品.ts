import { div, DivC, CanvasC, LV2HtmlComponentBase } from 'sengen-ui'
import { 描画ループ, レイキャスト入力 } from 'SengenThree'
import type { ワールド編集状態 } from '../../編集モデル/index.ts'
import type { チャンクシーン部品束 } from './三次元ビュー部品/シーン構築.ts'
import { チャンクシーンを構築する } from './三次元ビュー部品/シーン構築.ts'
import { 注視点マーカー表示制御器 } from './カメラ/注視点マーカー表示制御器.ts'
import type { 三次元の配色 } from './三次元の配色.ts'
import { コンテナ, キャンバス } from './三次元ビュー部品スタイル.css.ts'

function HTMLキャンバスか(要素: HTMLElement): 要素 is HTMLCanvasElement {
    return 'getContext' in 要素
}

export class 描画キャンバス要素 extends CanvasC {
    public constructor() {
        super({ class: キャンバス })
    }
}

// 三次元シーングラフ・描画ループ・レイキャスト・カメラ操作を束ねるビュー部品。
// シーンの内訳の組み立ては三次元ビュー部品/シーン構築.ts が持つ。
export class 三次元ビュー部品 extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    public readonly キャンバス要素: 描画キャンバス要素
    public readonly 場面: チャンクシーン部品束['場面']
    public readonly カメラ: チャンクシーン部品束['カメラ']
    public readonly カメラ制御: チャンクシーン部品束['カメラ制御']
    public readonly 描画ループ: 描画ループ
    public readonly レイキャスト: レイキャスト入力
    public readonly 地形: チャンクシーン部品束['地形']
    public readonly ブラシリング: チャンクシーン部品束['ブラシリング']
    public readonly 注視点マーカー: チャンクシーン部品束['注視点マーカー']
    public readonly 注視点表示制御: 注視点マーカー表示制御器
    public readonly 道路帯: チャンクシーン部品束['道路帯']
    public readonly 道路点マーカー: チャンクシーン部品束['道路点マーカー']
    public readonly 毎フレーム処理: チャンクシーン部品束['毎フレーム処理']
    public readonly 建物: チャンクシーン部品束['建物']
    public readonly 散布: チャンクシーン部品束['散布']

    public constructor(編集状態: ワールド編集状態, 初期背景色: string | number = 0x0b0f19) {
        super()
        const シーン = チャンクシーンを構築する(編集状態, 初期背景色)
        this.場面 = シーン.場面
        this.カメラ = シーン.カメラ
        this.カメラ制御 = シーン.カメラ制御
        this.地形 = シーン.地形
        this.ブラシリング = シーン.ブラシリング
        this.注視点マーカー = シーン.注視点マーカー
        this.注視点表示制御 = new 注視点マーカー表示制御器(this.注視点マーカー)
        this.道路帯 = シーン.道路帯
        this.道路点マーカー = シーン.道路点マーカー
        this.毎フレーム処理 = シーン.毎フレーム処理
        this.建物 = シーン.建物
        this.散布 = シーン.散布

        this.キャンバス要素 = new 描画キャンバス要素()
        this._componentRoot = div({ class: コンテナ }).child(this.キャンバス要素)

        const dom要素 = this.キャンバス要素.dom.element
        if (!HTMLキャンバスか(dom要素)) {
            throw new Error('キャンバス要素がHTMLCanvasElementではありません')
        }

        this.描画ループ = 描画ループ.キャンバスへ作る(dom要素, this.場面, this.カメラ)
        this.毎フレーム処理.描画ループへ差し込む(this.描画ループ)
        this.レイキャスト = new レイキャスト入力()
    }

    public 背景色を設定する(色: string | number): this {
        this.場面.背景色を設定する(色)
        return this
    }

    // テーマ切替時に植生・道路・道路点マーカーの材質色を差し替える。地表4層の識別色は
    // マテリアル台帳が正本のため、テーマ切替では触れない(判断9。チャンク編集地表材質色配線.ts参照)。
    public 三次元の配色を設定する(配色: 三次元の配色): this {
        this.散布.植生色を更新する(配色.植生色)
        this.道路帯.道路色を更新する(配色.道路色)
        this.道路点マーカー.配色を設定する(配色.道路点マーカー配色)
        return this
    }

    public 寸法を合わせる(幅: number, 高さ: number, ピクセル比: number = 1): void {
        this.描画ループ.寸法を合わせる(幅, 高さ, ピクセル比)
    }

    public override delete(): void {
        this.注視点表示制御.破棄する()
        this.レイキャスト.破棄する()
        this.描画ループ.破棄する()
        this.場面.破棄する()
        this.キャンバス要素.delete()
        super.delete()
    }
}
