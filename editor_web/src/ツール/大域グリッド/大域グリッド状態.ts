import type { 造成筆致種別 } from '../../生成/編集資源契約.ts'
import type { 差し戻し断片 } from '../ワールド/操作コマンド/index.ts'
import type { 大域編集モード } from './画面/パネル/モード切替/大域モード定義.ts'

// 大域グリッドエディターセッション中のUI選択・操作パラメータ・取り消し履歴を保持する。
export class 大域グリッド状態 {
    public モード: 大域編集モード = '大域カメラ'
    public 造成筆致種別: 造成筆致種別 = '盛る'
    public 造成半径: number = 80.0
    public 造成強さ: number = 1.2
    public 道路全幅: number = 12.0
    public 道路細分割数: number = 120
    public 選択中ノード添字: number = -1
    public static readonly 履歴スタック上限: number = 50
    public readonly 取り消し履歴スタック: 差し戻し断片[] = []

    public 取り消し断片を積む(断片: 差し戻し断片): void {
        this.取り消し履歴スタック.push(断片)
        if (this.取り消し履歴スタック.length > 大域グリッド状態.履歴スタック上限) {
            this.取り消し履歴スタック.shift()
        }
    }

    public 取り消し断片を取り出す(): 差し戻し断片 | undefined {
        return this.取り消し履歴スタック.pop()
    }
}
