import { div, CanvasC, DivC, LV2HtmlComponentBase } from 'sengen-ui'
import { レイキャスト入力, 描画ループ } from 'SengenThree'
import type { 升目の座標 } from '../../../../生成/編集資源契約.ts'
import { 立体の見取りを組み立てる, 見取りの外接箱を求める, type 表示する立体, type 建物の格子の編集モデル } from '../../編集モデル/index.ts'
import { 建物のシーンを構築する, type 建物のシーン部品束 } from './建物のシーン構築.ts'
import { 建物の三次元の入力係, type I三次元の触り } from './建物の三次元の入力係.ts'
import { 操作の案内, 三次元のキャンバス, 三次元の枠 } from './三次元表示のスタイル.css.ts'

function HTMLキャンバスか(要素: HTMLElement): 要素 is HTMLCanvasElement {
    return 'getContext' in 要素
}

// 外接箱の対角に掛けて距離にする係数。1より大きくして、建物の端が画面の縁へ触れないようにする。
const 距離の余裕 = 1.3

// 建物の格子を立体として見せるパネル。右ドラッグで回し、中ドラッグで平行移動し、ホイールで寄り引きし、
// 左クリックで升目を選ぶ(判断13)。視点を動かすためのモードのボタンは持たない。
// 表示の切替(識別色の重ね・建物ぜんたいを写す)のボタンをこのパネルが持たないのは、それらが
// エディタ領域の上部の固定の行にある建物ぜんたいの操作帯の持ち物だからである(判断14)。
// シーンの内訳は`建物のシーン構築`が、入力の配線は`建物の三次元の入力係`が持つ。
// 参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断13」「判断14」
export class 建物の三次元パネル extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    private readonly _キャンバス = new CanvasC({ class: 三次元のキャンバス })
    private readonly _シーン: 建物のシーン部品束 = 建物のシーンを構築する()
    private readonly _レイキャスト = new レイキャスト入力()
    private readonly _描画ループ: 描画ループ
    private _入力係: 建物の三次元の入力係 | undefined = undefined
    private _いまの見取り: readonly 表示する立体[] = []

    public constructor() {
        super()
        this._componentRoot = div({ class: 三次元の枠 }).childs([
            this._キャンバス,
            div({ class: 操作の案内, text: '右ドラッグで回す。中ドラッグで平行移動。ホイールで寄り引き。左クリックで升目を選ぶ。' }),
        ])
        // 描画ループはここでは回さない。回し始めるのはタブが前面になったときであり、止めるのは背面になったときである
        // (チャンク編集の三次元ビューと同じ運びである)。描画ループは二重開始を例外で拒む。
        this._描画ループ = 描画ループ.キャンバスへ作る(this.キャンバスの実体を取り出す(), this._シーン.場面, this._シーン.カメラ)
        this.カメラを見取りへ合わせる()
    }

    // 人が三次元へ行える触りを結ぶ。ポインタの購読を始めるのもここであり、結ぶのは道具の配線の1回だけである。
    public 触りを結ぶ(触り: I三次元の触り): void {
        if (this._入力係 !== undefined) throw new Error('建物の三次元パネルの触りは既に結ばれている')
        this._入力係 = new 建物の三次元の入力係(
            {
                キャンバス要素: this._キャンバス,
                カメラ: this._シーン.カメラ,
                カメラ制御: this._シーン.カメラ制御,
                レイキャスト: this._レイキャスト,
                立体: this._シーン.立体,
            },
            触り,
        )
    }

    public 再構築する(モデル: 建物の格子の編集モデル, 選んでいる升目: 升目の座標 | undefined, 識別色を重ねるか: boolean): void {
        this._いまの見取り = 立体の見取りを組み立てる(モデル)
        this._シーン.立体.見取りを描き直す(this._いまの見取り, 識別色を重ねるか)
        this._シーン.立体.選んだ升目を示す(選んでいる升目)
        if (this._入力係 === undefined || !this._入力係.視点を人が動かしたか) this.カメラを見取りへ合わせる()
    }

    // 視点を建物ぜんたいが収まる位置へ戻す。人が視点を動かしたあとに呼び戻す口であり、
    // 押しボタンは建物ぜんたいの操作帯が持つ。
    public 建物ぜんたいを写す(): void {
        this.カメラを見取りへ合わせる()
    }

    // キャンバスが実際に占めている大きさを自分で測って描画の解像度を合わせる。枠の高さはCSSが決めるため、
    // 呼び出し側が寸法を持参すると、CSSの値と食い違った解像度で描いても誰も気づかない。
    public いまの枠の大きさへ合わせる(ピクセル比: number): void {
        const 枠 = this.キャンバスの実体を取り出す().getBoundingClientRect()
        if (枠.width <= 0 || 枠.height <= 0) return
        this._シーン.カメラ.アスペクト比を更新する(枠.width / 枠.height)
        this._描画ループ.寸法を合わせる(枠.width, 枠.height, ピクセル比)
    }

    public 描画を始める(): void {
        this._描画ループ.開始する()
    }

    // 背面のタブで描き続けると、見えていない絵のためにGPUを回し続けることになる。
    public 描画を止める(): void {
        this._描画ループ.停止する()
    }

    public override delete(): void {
        this._入力係?.解除する()
        this._レイキャスト.破棄する()
        this._描画ループ.破棄する()
        this._シーン.場面.破棄する()
        this._キャンバス.delete()
        super.delete()
    }

    private カメラを見取りへ合わせる(): void {
        const 外接箱 = 見取りの外接箱を求める(this._いまの見取り)
        this._シーン.カメラ制御.注視点を設定する(外接箱.中心メートル.x, 外接箱.中心メートル.y, 外接箱.中心メートル.z)
        this._シーン.カメラ制御.距離を設定する(外接箱.対角の長さメートル * 距離の余裕)
    }

    private キャンバスの実体を取り出す(): HTMLCanvasElement {
        const 要素 = this._キャンバス.dom.element
        if (!HTMLキャンバスか(要素)) throw new Error('三次元表示のキャンバス要素がHTMLCanvasElementではない')
        return 要素
    }
}
