import type { 造成筆致種別 } from '../../生成/編集資源契約.ts'
import type { 道路点の在り処 } from '../チャンク編集/編集モデル/index.ts'
import type { 差し戻し断片 } from '../チャンク編集/操作コマンド/index.ts'
import type { 大域編集モード } from './画面/パネル/モード切替/大域モード定義.ts'

// 大域編集ツールセッション中のUI選択・操作パラメータ・取り消し履歴を保持する。
export class 大域編集状態 {
    public モード: 大域編集モード = '選択'
    public 造成筆致種別: 造成筆致種別 = '盛る'
    public 造成半径: number = 80.0
    public 造成強さ: number = 1.2
    // 描き足す先の道。nullは「次に地形をクリックしたら新しい道を1本始める」ことを表す。
    public アクティブな道路の添字: number | null = 0
    public 選択中の道路点: 道路点の在り処 | null = null
    public つかんでいる道路点: 道路点の在り処 | null = null
    public static readonly 履歴スタック上限: number = 50
    public readonly 取り消し履歴スタック: 差し戻し断片[] = []

    public 取り消し断片を積む(断片: 差し戻し断片): void {
        this.取り消し履歴スタック.push(断片)
        if (this.取り消し履歴スタック.length > 大域編集状態.履歴スタック上限) {
            this.取り消し履歴スタック.shift()
        }
    }

    public 取り消し断片を取り出す(): 差し戻し断片 | undefined {
        return this.取り消し履歴スタック.pop()
    }
}
