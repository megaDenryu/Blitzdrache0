import { 注視点マーカー表示状態 } from './注視点マーカー表示状態.ts'

// 表示先が満たすべき最小の口。実装は注視点マーカー部品だが、テストでは記録用の偽物を渡す。
export interface 注視点表示先 {
    地点へ表示する(x: number, y: number, z: number): unknown
    非表示にする(): unknown
}

// window.setTimeout/clearTimeoutを予約・取り消しの1組として抽象化したもの。
// テストでは実時間を進めずにコールバックを手動発火できる偽物へ差し替える。
export interface 遅延実行 {
    予約する(遅延ミリ秒: number, コールバック: () => void): number
    取り消す(識別子: number): void
}

function 既定の遅延実行を作る(): 遅延実行 {
    return {
        予約する: (遅延ミリ秒, コールバック) => window.setTimeout(コールバック, 遅延ミリ秒),
        取り消す: (識別子) => window.clearTimeout(識別子),
    }
}

const 既定の非表示猶予ミリ秒 = 500

// カメラ操作(回転・パン・WASDQE)の間だけ注視点マーカーを表示し、操作が止まって
// 一定時間が経つと非表示に戻す。可視/非可視の判断は注視点マーカー表示状態(純粋)へ委ね、
// 実時間の管理(タイマーの起動・取消)はここが遅延実行として保持する。
export class 注視点マーカー表示制御器 {
    private readonly _状態 = new 注視点マーカー表示状態()
    private _保留タイマー識別子: number | null = null

    public constructor(
        private readonly _マーカー: 注視点表示先,
        private readonly _遅延実行: 遅延実行 = 既定の遅延実行を作る(),
        private readonly _非表示猶予ミリ秒: number = 既定の非表示猶予ミリ秒,
    ) {}

    public 操作された(x: number, y: number, z: number): void {
        this._状態.操作された()
        this._マーカー.地点へ表示する(x, y, z)
        this._タイマーを再設定する()
    }

    public 破棄する(): void {
        if (this._保留タイマー識別子 !== null) {
            this._遅延実行.取り消す(this._保留タイマー識別子)
            this._保留タイマー識別子 = null
        }
    }

    private _タイマーを再設定する(): void {
        if (this._保留タイマー識別子 !== null) {
            this._遅延実行.取り消す(this._保留タイマー識別子)
        }
        this._保留タイマー識別子 = this._遅延実行.予約する(this._非表示猶予ミリ秒, () => {
            this._状態.非表示にする()
            this._マーカー.非表示にする()
            this._保留タイマー識別子 = null
        })
    }
}
