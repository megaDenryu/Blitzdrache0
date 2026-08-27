// 画面が1コマ進むたびに起こされる側の規約。
export interface I画面の1コマごとに起こされる側 {
    画面が1コマ進んだ(): void
}

// 画面の描画に合わせて起こす見張り。音を鳴らす時刻はこの見張りが決めるのではなく、
// 音声の時計から導いた再生位置を画面へ映すためだけに使う。
export class 毎フレームの見張り {
    private _要求識別子: number | null = null
    private _動作中: boolean = false

    public constructor(private readonly _起こす相手: I画面の1コマごとに起こされる側) {}

    public 始める(): void {
        if (this._動作中) return
        this._動作中 = true
        this._次の1コマを頼む()
    }

    public やめる(): void {
        this._動作中 = false
        if (this._要求識別子 !== null) {
            window.cancelAnimationFrame(this._要求識別子)
            this._要求識別子 = null
        }
    }

    public get 動作中か(): boolean {
        return this._動作中
    }

    private _次の1コマを頼む(): void {
        this._要求識別子 = window.requestAnimationFrame(() => this._1コマ動く())
    }

    private _1コマ動く(): void {
        this._要求識別子 = null
        if (!this._動作中) return
        this._起こす相手.画面が1コマ進んだ()
        if (this._動作中) this._次の1コマを頼む()
    }
}
