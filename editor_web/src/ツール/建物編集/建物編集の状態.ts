import type { 建物の格子 } from '../../生成/編集資源契約.ts'
import { 建物編集の操作履歴, type 升目への筆, type 面への筆 } from './操作コマンド/index.ts'

// 建物1件を編集しているあいだの状態。操作履歴と、いま選んでいる筆と階を持つ。
// 画面から分けるのは、同じ状態を画面の作り直しをまたいで保つためである。
export class 建物編集の状態 {
    private _履歴: 建物編集の操作履歴
    private _選んだ升目への筆: 升目への筆 = '升目を置く'
    private _選んだ面への筆: 面への筆 = '平壁'
    private _選んだ階 = 0

    public constructor(格子: 建物の格子) {
        this._履歴 = new 建物編集の操作履歴(格子)
    }

    public get 履歴(): 建物編集の操作履歴 {
        return this._履歴
    }

    // 保存済みの正本を読み直したときに呼ぶ。取り消しの履歴は捨てる。読み直しは編集操作ではなく、
    // 「いま編集している内容を捨てて正本へ戻す」ことだからである。
    public 読み直した格子で作り直す(格子: 建物の格子): void {
        this._履歴 = new 建物編集の操作履歴(格子)
        this._選んだ階 = 0
    }

    public get 選んだ升目への筆(): 升目への筆 {
        return this._選んだ升目への筆
    }

    public get 選んだ面への筆(): 面への筆 {
        return this._選んだ面への筆
    }

    public get 選んだ階(): number {
        return this._選んだ階
    }

    public 升目への筆を選ぶ(筆: 升目への筆): void {
        this._選んだ升目への筆 = 筆
    }

    public 面への筆を選ぶ(筆: 面への筆): void {
        this._選んだ面への筆 = 筆
    }

    public 階を選ぶ(階: number): void {
        this._選んだ階 = Math.max(0, Math.trunc(階))
    }

    // いま格子が持つ升目のうち最も高い階。階の切替が出す選択肢の上限を決める。
    public 最上階(): number {
        return this._履歴.モデル.升目を昇順に並べる().reduce((最大, 宣言) => Math.max(最大, 宣言.座標.階), 0)
    }
}
