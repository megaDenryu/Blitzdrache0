import type { 建物の格子 } from '../../../生成/編集資源契約.ts'
import { 建物の格子の編集モデル } from '../編集モデル/index.ts'
import { 建物編集コマンド } from './建物編集コマンド.ts'

// 編集モデルと、そこへ積んだコマンドの列を保持し、操作と取り消しとやり直しをメソッドで公開する型。
// 取り消しの実体はコマンドが持つ写しからのモデルの作り直しであるため、モデルの差し替えをこの型が受け持つ。
export class 建物編集の操作履歴 {
    private _モデル: 建物の格子の編集モデル
    private readonly _積んだコマンド: 建物編集コマンド[] = []
    private _取り消した数 = 0

    public constructor(格子: 建物の格子) {
        this._モデル = 建物の格子の編集モデル.契約の値から復元する(格子)
    }

    public get モデル(): 建物の格子の編集モデル {
        return this._モデル
    }

    public get 取り消せるか(): boolean {
        return this._積んだコマンド.length > this._取り消した数
    }

    public get やり直せるか(): boolean {
        return this._取り消した数 > 0
    }

    // 1回の操作を積む。何も変わらなかった操作は履歴へ積まない(取り消しが空振りするため)。
    public 操作を1つ積む(名前: string, 操作する: (モデル: 建物の格子の編集モデル) => void): boolean {
        const コマンド = 建物編集コマンド.モデルへ適用して記録する(名前, this._モデル, 操作する)
        if (コマンド === undefined) return false
        this._積んだコマンド.length = this._積んだコマンド.length - this._取り消した数
        this._取り消した数 = 0
        this._積んだコマンド.push(コマンド)
        return true
    }

    public 取り消す(): boolean {
        const コマンド = this._積んだコマンド[this._積んだコマンド.length - this._取り消した数 - 1]
        if (コマンド === undefined) return false
        this._取り消した数 += 1
        this._モデル = コマンド.取り消した後のモデル()
        return true
    }

    public やり直す(): boolean {
        const コマンド = this._積んだコマンド[this._積んだコマンド.length - this._取り消した数]
        if (コマンド === undefined) return false
        this._取り消した数 -= 1
        this._モデル = コマンド.やり直した後のモデル()
        return true
    }
}
