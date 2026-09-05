import { カードの操作をコマンドへ写す } from './操作コマンド/index.ts'
import type { 楽曲履歴適用サービス } from './操作コマンド/楽曲履歴適用サービス.ts'
import type { 楽曲編集UI状態 } from './楽曲編集UI状態.ts'
import { カード位置は同じか, 曲構成をカードの列へ展開する, type カード位置 } from './編集モデル/index.ts'
import type { 楽曲編集状態 } from './編集モデル/index.ts'
import type { Iタイムライン配線, カード操作の種類 } from './画面/タイムライン/index.ts'

// タイムラインのカード選択と6つの操作を、楽曲編集状態とコマンドへ振り分ける。
// 押せないはずの操作が呼ばれたら、画面側の押せない状態が破れている配線の誤りとして失敗させる。
export class タイムラインハンドラ implements Iタイムライン配線 {
    public constructor(
        private readonly _状態: 楽曲編集状態,
        private readonly _UI状態: 楽曲編集UI状態,
        private readonly _操作: 楽曲履歴適用サービス,
        private readonly _表示を再構築する: () => void,
    ) {}

    public onカード選択(位置: カード位置): void {
        const カード = this._カードを探す(位置)
        if (カード === null) return
        this._状態.選択中パターンの名乗り = カード.パターンの名乗り
        this._UI状態.選択中のカード = 位置
        this._表示を再構築する()
    }

    public onカード操作(位置: カード位置, 種類: カード操作の種類): void {
        const 写し = カードの操作をコマンドへ写す(this._状態.楽曲.曲構成, { 種類, 位置 })
        if (写し.種類 === 'できない') {
            throw new Error(`配線の誤り: 押せないはずのカード操作(${種類})が実行されました: ${写し.理由}`)
        }
        this._操作.コマンドを実行する(写し.コマンド)
        this._UI状態.選択中のカードが失われていたら外す(this._状態.楽曲.曲構成)
        this._表示を再構築する()
    }

    public on末尾へ追加(): void {
        const パターンの名乗り = this._状態.選択中パターンの名乗り
        if (パターンの名乗り === null) return
        const 写し = カードの操作をコマンドへ写す(this._状態.楽曲.曲構成, { 種類: '末尾へ追加', パターンの名乗り })
        if (写し.種類 === 'できない') {
            throw new Error(`配線の誤り: 押せないはずの末尾へ追加が実行されました: ${写し.理由}`)
        }
        this._操作.コマンドを実行する(写し.コマンド)
        this._表示を再構築する()
    }

    private _カードを探す(位置: カード位置): { readonly パターンの名乗り: string } | null {
        const カード列 = 曲構成をカードの列へ展開する(this._状態.楽曲.曲構成)
        return カード列.find((カード) => カード位置は同じか(カード.位置, 位置)) ?? null
    }
}
