import {
    トラックに適用される和音一覧を解決する,
    トラックの行の音はステップで許されるか,
    type 楽曲編集状態,
} from './編集モデル/index.ts'
import type { 楽曲履歴適用サービス } from './操作コマンド/楽曲履歴適用サービス.ts'
import type { 楽曲編集UI状態 } from './楽曲編集UI状態.ts'
import { ドラッグ見込みを導出する } from './画面/ドラッグ見込み導出.ts'
import { 確定コマンドを生成する } from './画面/打ち込み確定コマンド生成.ts'
import type { I楽曲発音配線 } from './画面/発音配線.ts'
import type { 升目の当たりの記録 } from './画面/打ち込み見込み.ts'

export type { 升目の当たりの記録 } from './画面/打ち込み見込み.ts'

interface ドラッグ中情報 {
    readonly ボタン: number
    readonly 起点: 升目の当たりの記録
    現在: 升目の当たりの記録
    readonly 進行に従うか: boolean
}

// 升目のポインタ事象（押す・動かす・離す・キャンセル）を解釈し、操作コマンドの生成と見込み更新を行うサービス。
export class 楽曲編集ポインタ振り分け {
    private _ドラッグ中: ドラッグ中情報 | null = null

    public constructor(
        private readonly _状態: 楽曲編集状態,
        private readonly _UI状態: 楽曲編集UI状態,
        private readonly _操作: 楽曲履歴適用サービス,
        private readonly _見込み更新通知: () => void,
        private readonly _発音配線?: I楽曲発音配線,
    ) {}

    public 押された(ボタン: number, 当たり: 升目の当たりの記録): boolean {
        if (ボタン !== 0 && ボタン !== 2) return false

        if (ボタン === 0) {
            if (!this.その音は許されるか(当たり)) return false
            const 進行に従うか = !this._UI状態.進行の外モードか
            this._ドラッグ中 = { ボタン, 起点: { ...当たり }, 現在: { ...当たり }, 進行に従うか }
            this._発音配線?.升目の音を鳴らす(当たり.トラックの位置, 当たり.行の位置)
        } else {
            this._ドラッグ中 = { ボタン, 起点: { ...当たり }, 現在: { ...当たり }, 進行に従うか: true }
        }

        this._見込みを更新する()
        return true
    }

    public 動かされた(当たり: 升目の当たりの記録): boolean {
        if (this._ドラッグ中 === null) return false
        if (this._ドラッグ中.ボタン === 0 && !this.その音は許されるか(当たり)) return false

        this._ドラッグ中.現在 = { ...当たり }
        this._見込みを更新する()
        return true
    }

    public 離された(ボタン: number): void {
        if (this._ドラッグ中 === null || this._ドラッグ中.ボタン !== ボタン) return

        const ドラッグ = this._ドラッグ中
        this._ドラッグ中 = null
        this._見込みを更新する()

        const コマンド = 確定コマンドを生成する(ドラッグ.起点, ドラッグ.現在, ドラッグ.ボタン, ドラッグ.進行に従うか)
        this._操作.コマンドを実行する(コマンド)
    }

    public キャンセルされた(): void {
        if (this._ドラッグ中 === null) return
        this._ドラッグ中 = null
        this._見込みを更新する()
    }

    public その音は許されるか(当たり: 升目の当たりの記録): boolean {
        if (this._UI状態.進行の外モードか) return true
        const 楽曲 = this._状態.楽曲
        const トラック = 楽曲.トラック構成[当たり.トラックの位置]
        if (トラック === undefined) return false

        const パターン = 楽曲.パターン一覧.find((p) => p.名乗り === 当たり.パターンの名乗り)
        if (パターン === undefined) return false

        const 和音一覧 = トラックに適用される和音一覧を解決する(
            トラック,
            パターン.進行の参照,
            楽曲.独自進行一覧,
        )
        return トラックの行の音はステップで許されるか(
            トラック,
            当たり.行の位置,
            当たり.ステップ,
            和音一覧,
        )
    }

    private _見込みを更新する(): void {
        if (this._ドラッグ中 === null) {
            this._UI状態.ドラッグ見込み = null
        } else {
            this._UI状態.ドラッグ見込み = ドラッグ見込みを導出する(
                this._ドラッグ中.起点,
                this._ドラッグ中.現在,
                this._ドラッグ中.ボタン,
                this._ドラッグ中.進行に従うか,
            )
        }
        this._見込み更新通知()
    }
}
