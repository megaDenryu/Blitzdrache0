import type { 部品交差情報 } from 'SengenThree'
import type { 位置3次元, 道路対象 } from '../../生成/編集資源契約.ts'
import type { ワールド編集状態, 道路スプライン } from './編集モデル/index.ts'
import { 道路対象の道路スプラインを取り出す } from './編集モデル/index.ts'
import type {
    道路点の選択状態,
    道路点編集の同期先,
    道路点編集の操作先,
    道路点編集対象ビュー,
} from './道路点編集の相手.ts'

// 道路点マーカーを左ボタンでつかみ、地表に沿って動かし、離した位置で確定する操作サービス。
// ドラッグの途中は編集モデルを直に書き換えて帯とマーカーを追従させ、コマンドとして積むのは
// 離したときの1回だけにする。取り消し1回でつかむ前の位置へ戻るようにするためである。
export class 道路点ドラッグハンドラ {
    private _つかんだ時の位置: 位置3次元 | null = null
    private _つかんでから動かしたか: boolean = false
    private _ドラッグ直後のクリックか: boolean = false

    public constructor(
        private readonly _モデル: ワールド編集状態,
        private readonly _状態: 道路点の選択状態,
        private readonly _ビュー: 道路点編集対象ビュー,
        private readonly _操作: 道路点編集の操作先,
        private readonly _同期: 道路点編集の同期先,
        private readonly _道路対象: 道路対象,
    ) {}

    // 動かして離した直後には、ブラウザがクリックも続けて配る。そのクリックを選択や挿入として
    // 扱うと、動かした先で意図しない編集が起きるため、呼び出し元が1回だけ読み飛ばす。
    public get ドラッグ直後のクリックか(): boolean {
        return this._ドラッグ直後のクリックか
    }

    public クリックの読み飛ばしを終える(): void {
        this._ドラッグ直後のクリックか = false
    }

    // 道路点をつかめたらtrueを返す。呼び出し元はそのとき筆致もカメラ操作も走らせない。
    public 押された(ボタン: number, 当たり一覧: readonly 部品交差情報[]): boolean {
        if (ボタン !== 0) return false
        const 添字 = this._当たった道路点の添字を探す(当たり一覧)
        const 点 = 添字 === null ? undefined : this._道路スプライン.制御点列[添字]
        if (添字 === null || 点 === undefined) return false

        this._つかんだ時の位置 = { ...点 }
        this._つかんでから動かしたか = false
        this._状態.つかんでいる道路点の添字 = 添字
        this._状態.選択中ノード添字 = 添字
        this._同期.道路を同期する()
        this._同期.UIを同期する()
        return true
    }

    // つかんでいる間はtrueを返し、呼び出し元にブラシ表示や筆致を飛ばさせる。
    public 動かされた(当たり一覧: readonly 部品交差情報[]): boolean {
        const 添字 = this._状態.つかんでいる道路点の添字
        if (添字 === null) return false
        const 地形の当たり = 当たり一覧.find((当たり) => 当たり.部品 === this._ビュー.地形)
        if (地形の当たり !== undefined) {
            const 交差点 = 地形の当たり.交差点
            this._道路スプライン.点を移動する(添字, { x: 交差点.x, y: 交差点.y, z: 交差点.z })
            this._つかんでから動かしたか = true
            this._同期.道路を同期する()
        }
        return true
    }

    public 離された(ボタン: number): void {
        const 添字 = this._状態.つかんでいる道路点の添字
        const つかんだ時の位置 = this._つかんだ時の位置
        if (ボタン !== 0 || 添字 === null) return
        this._状態.つかんでいる道路点の添字 = null
        this._つかんだ時の位置 = null

        const 離した位置 = this._道路スプライン.制御点列[添字]
        if (!this._つかんでから動かしたか || つかんだ時の位置 === null || 離した位置 === undefined) {
            this._同期.道路を同期する()
            return
        }
        this._ドラッグ直後のクリックか = true
        const 新しい位置 = { ...離した位置 }
        // 取り消しの断片がつかむ前の位置を覚えるよう、いったん元へ戻してからコマンドとして積み直す。
        this._道路スプライン.点を移動する(添字, つかんだ時の位置)
        this._操作.コマンドを実行する({ 種類: '道路点を移動する', 値: { 対象: this._道路対象, 添字, 新しい位置 } })
    }

    private get _道路スプライン(): 道路スプライン {
        return 道路対象の道路スプラインを取り出す(this._モデル, this._道路対象)
    }

    private _当たった道路点の添字を探す(当たり一覧: readonly 部品交差情報[]): number | null {
        for (const 当たり of 当たり一覧) {
            if (当たり.部品 !== this._ビュー.道路点マーカー) continue
            const 添字 = this._ビュー.道路点マーカー.当たった道路点の添字を求める(当たり.原初交差情報.object)
            if (添字 !== null) return 添字
        }
        return null
    }
}
