import type { 楽曲編集コマンド } from '../../../生成/編集資源契約.ts'
import type { 楽曲編集状態 } from '../編集モデル/index.ts'
import type { 差し戻し断片 } from './差し戻し断片.ts'
import { 楽曲編集コマンドを適用する } from './編集コマンドの適用.ts'
import { 差し戻しを適用する } from './差し戻しの適用.ts'

// 楽曲編集コマンドを適用し、取り消し履歴スタックを管理する操作サービス。
export class 楽曲履歴適用サービス {
    public static readonly 履歴スタック上限: number = 50
    private readonly _取り消し履歴スタック: 差し戻し断片[] = []

    public constructor(
        private readonly _状態: 楽曲編集状態,
        private readonly _同期通知: () => void = () => {},
    ) {}

    public コマンドを実行する(コマンド: 楽曲編集コマンド): void {
        const 差し戻し = 楽曲編集コマンドを適用する(this._状態, コマンド)
        this._取り消し履歴スタック.push(差し戻し)
        if (this._取り消し履歴スタック.length > 楽曲履歴適用サービス.履歴スタック上限) {
            this._取り消し履歴スタック.shift()
        }
        this._同期通知()
    }

    public 直前の操作を取り消す(): boolean {
        const 差し戻し = this._取り消し履歴スタック.pop()
        if (差し戻し === undefined) {
            return false
        }
        差し戻しを適用する(this._状態, 差し戻し)
        this._同期通知()
        return true
    }

    public get 履歴件数(): number {
        return this._取り消し履歴スタック.length
    }
}
