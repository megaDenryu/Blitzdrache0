import type { 演奏の範囲 } from './編集モデル/index.ts'
import { 見本の曲のコマンド列を組み立てる } from './編集モデル/index.ts'
import type { 楽曲編集状態 } from './編集モデル/index.ts'
import type { 楽曲履歴適用サービス } from './操作コマンド/楽曲履歴適用サービス.ts'
import type { I演奏の操作帯配線, 演奏サービス, 楽曲編集画面 } from './画面/index.ts'

// 演奏の操作帯の各操作を、演奏サービスと操作コマンドへ振り分ける。
// 押した結果が画面に出ることを揃えるため、コマンドを積む操作は必ず表示の同期を通る経路にしてある。
export class 演奏の操作帯ハンドラ implements I演奏の操作帯配線 {
    public constructor(
        private readonly _画面: 楽曲編集画面,
        private readonly _状態: 楽曲編集状態,
        private readonly _操作: 楽曲履歴適用サービス,
        private readonly _演奏: 演奏サービス,
    ) {}

    public on再生と停止(): void {
        this._演奏.再生と停止を切り替える()
    }

    public on先頭へ戻す(): void {
        this._演奏.先頭へ戻す()
    }

    public on演奏の範囲変更(範囲: 演奏の範囲): void {
        this._演奏.演奏の範囲を変える(範囲)
        this.演奏の様子を画面へ映す()
    }

    public on拍毎分変更(新しい拍毎分: number): void {
        this._操作.コマンドを実行する({ 種類: '拍毎分を変える', 値: { 新しい拍毎分 } })
        this._演奏.楽曲の設定の変更を反映する()
    }

    public on全消去(): void {
        const パターンの名乗り = this._状態.選択中パターンの名乗り
        if (パターンの名乗り === null) return
        this._操作.コマンドを実行する({ 種類: 'パターンの打点を全部消す', 値: { パターンの名乗り } })
    }

    public on見本の曲(): void {
        const パターンの名乗り = this._状態.選択中パターンの名乗り
        if (パターンの名乗り === null) return
        this._操作.コマンド列を1つの操作として実行する(
            見本の曲のコマンド列を組み立てる(this._状態.楽曲を取得する(), パターンの名乗り),
        )
        this._演奏.楽曲の設定の変更を反映する()
    }

    public 演奏の様子を画面へ映す(): void {
        this._画面.再生位置を示す(
            this._演奏.現在の再生位置(),
            this._演奏.再生中か,
            this._演奏.演奏の範囲,
        )
    }
}
