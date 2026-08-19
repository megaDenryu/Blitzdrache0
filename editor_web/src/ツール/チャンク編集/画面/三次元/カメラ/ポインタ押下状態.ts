export interface ポインタの移動量 {
    readonly 横: number
    readonly 縦: number
}

// ポインタのどのボタンが押し下がっているかと、直前のポインタ位置を持つ器。
// ドラッグの移動量の計算と、いま何のドラッグの最中かの判断に使う。
// チャンク編集と大域編集のポインタ配線が同じ形で使う。
export class ポインタ押下状態 {
    private _左ボタン押下中: boolean = false
    private _右ボタン押下中: boolean = false
    private _中ボタン押下中: boolean = false
    private _直前の横位置: number = 0
    private _直前の縦位置: number = 0

    public get 左ボタン押下中(): boolean {
        return this._左ボタン押下中
    }

    // 押し下がっているボタンのうち、カメラ操作の割り当てで優先する1つを返す。
    // 右(回転)・中(平行移動)・左(主作業)の順に見る。どれも押されていなければnullを返す。
    public get 押しているボタン(): number | null {
        if (this._右ボタン押下中) return 2
        if (this._中ボタン押下中) return 1
        if (this._左ボタン押下中) return 0
        return null
    }

    public 押された(事象: MouseEvent): void {
        this._直前の横位置 = 事象.clientX
        this._直前の縦位置 = 事象.clientY
        if (事象.button === 0) this._左ボタン押下中 = true
        if (事象.button === 2) this._右ボタン押下中 = true
        if (事象.button === 1) {
            this._中ボタン押下中 = true
            // 中ボタンの既定動作(自動スクロール)が始まると以降のポインタ事象が届かなくなる。
            事象.preventDefault()
        }
    }

    public 離された(事象: MouseEvent): void {
        if (事象.button === 0) this._左ボタン押下中 = false
        if (事象.button === 2) this._右ボタン押下中 = false
        if (事象.button === 1) this._中ボタン押下中 = false
    }

    // 直前のポインタ位置からの移動量を返し、直前の位置を今の位置へ進める。
    public 移動量を取り出す(事象: MouseEvent): ポインタの移動量 {
        const 横 = 事象.clientX - this._直前の横位置
        const 縦 = 事象.clientY - this._直前の縦位置
        this._直前の横位置 = 事象.clientX
        this._直前の縦位置 = 事象.clientY
        return { 横, 縦 }
    }
}
