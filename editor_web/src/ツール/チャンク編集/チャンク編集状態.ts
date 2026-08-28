import type { 造成筆致種別, 地表材質層, チャンク座標 } from '../../生成/編集資源契約.ts'
import type { 道路点の在り処 } from './編集モデル/index.ts'
import type { 差し戻し断片 } from './操作コマンド/index.ts'
import type { 編集モード } from './画面/index.ts'
import { 初期の編集モード } from './画面/パネル/モード切替/モード定義.ts'
import { 地表の材質の筆の初期値, 造成の筆の初期値 } from './画面/下パネル/筆の初期値.ts'

// エディターセッション中のUI選択・操作パラメータ・取り消し履歴スタックを保持する状態。
export class チャンク編集状態 {
    public モード: 編集モード = 初期の編集モード
    public 造成筆致種別: 造成筆致種別 = 造成の筆の初期値.種別
    public 造成半径: number = 造成の筆の初期値.半径メートル
    public 造成強さ: number = 造成の筆の初期値.強さ
    public 材質層: 地表材質層 = 地表の材質の筆の初期値.層
    public ペイント半径: number = 地表の材質の筆の初期値.半径メートル
    public ペイント流量: number = 地表の材質の筆の初期値.流量
    // 描き足す先の道。nullは「次に地形をクリックしたら新しい道を1本始める」ことを表す。
    public アクティブな道路の添字: number | null = 0
    public 選択中の道路点: 道路点の在り処 | null = null
    public つかんでいる道路点: 道路点の在り処 | null = null
    public 選択中建物識別子: string | null = null
    public readonly 対象チャンク座標: チャンク座標
    public static readonly 履歴スタック上限: number = 50
    public readonly 取り消し履歴スタック: 差し戻し断片[] = []

    public constructor(対象チャンク座標: チャンク座標 = { x: 0, z: 0 }) {
        this.対象チャンク座標 = { ...対象チャンク座標 }
    }

    public 取り消し断片を積む(断片: 差し戻し断片): void {
        this.取り消し履歴スタック.push(断片)
        if (this.取り消し履歴スタック.length > チャンク編集状態.履歴スタック上限) {
            this.取り消し履歴スタック.shift()
        }
    }

    public 取り消し断片を取り出す(): 差し戻し断片 | undefined {
        return this.取り消し履歴スタック.pop()
    }
}
