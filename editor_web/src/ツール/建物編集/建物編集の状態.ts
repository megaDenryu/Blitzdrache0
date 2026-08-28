import type { 升目の座標, 建物の格子 } from '../../生成/編集資源契約.ts'
import { 煙突の段数 } from './編集モデル/index.ts'
import { 建物編集の操作履歴, type 升目への筆, type 面への筆 } from './操作コマンド/index.ts'

// 建物1件を編集しているあいだの状態。操作履歴と、いま選んでいる筆と階と煙突の段数と升目、
// そして識別色を重ねて見せるかどうかを持つ。
// 画面から分けるのは、同じ状態を画面の作り直しをまたいで保つためである。
export class 建物編集の状態 {
    private _履歴: 建物編集の操作履歴
    private _選んだ升目への筆: 升目への筆 = '升目を置く'
    private _選んだ面への筆: 面への筆 = '平壁'
    private _選んだ階 = 0
    private _選んだ煙突の段数: 煙突の段数 = 煙突の段数.既定を作る()
    private _選んでいる升目: 升目の座標 | undefined = undefined
    private _識別色を重ねるか = false

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
        this._選んでいる升目 = undefined
    }

    public get 選んでいる升目(): 升目の座標 | undefined {
        return this._選んでいる升目
    }

    // 升目を選ぶと、その升目の階を平面図へ出す。三次元で選んだものが平面図に出ていないと、
    // 選んだ升目へ筆を当てられない。
    public 升目を選ぶ(座標: 升目の座標): void {
        this._選んでいる升目 = 座標
        this._選んだ階 = 座標.階
    }

    public get 識別色を重ねるか(): boolean {
        return this._識別色を重ねるか
    }

    public 識別色の重ねを切り替える(): void {
        this._識別色を重ねるか = !this._識別色を重ねるか
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

    public get 選んだ煙突の段数(): 煙突の段数 {
        return this._選んだ煙突の段数
    }

    public 煙突の段数を選ぶ(段数: 煙突の段数): void {
        this._選んだ煙突の段数 = 段数
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

    // いま格子が持つ升目のうち最も高い階。階の一覧が出す選択肢の上限を決める。
    public 最上階(): number {
        return this._履歴.モデル.升目を昇順に並べる().reduce((最大, 宣言) => Math.max(最大, 宣言.座標.階), 0)
    }

    // 名指した階に置かれている升目の数。階の一覧に添えて、空の階と編んだ階を見分けられるようにする。
    public その階の升目の数(階: number): number {
        return this._履歴.モデル.升目を昇順に並べる().filter((宣言) => 宣言.座標.階 === 階).length
    }
}
