import type { 編集コマンド, 建物外形定義 } from '../../生成/編集資源契約.ts'
import { 建物定義IDを生成する, type 建物定義ID } from '../../境界/建物定義ID.ts'
import type { ワールド編集状態 } from './編集モデル/index.ts'
import { 編集コマンドを適用する, 差し戻しを適用する } from './操作コマンド/index.ts'
import type { チャンク編集状態 } from './チャンク編集状態.ts'
import type { チャンク編集同期サービス } from './チャンク編集同期サービス.ts'
import type { 編集モード } from './画面/index.ts'
import { 見下ろし図で編集するモードか } from './画面/パネル/モード切替/モード定義.ts'

// 操作コマンドの適用・取り消しおよびドメイン操作のディスパッチを担当するサービス。
export class チャンク編集操作サービス {
    private readonly _建物定義: Map<建物定義ID, 建物外形定義> = new Map()
    public constructor(
        private readonly _モデル: ワールド編集状態,
        private readonly _UI状態: チャンク編集状態,
        private readonly _同期: チャンク編集同期サービス,
    ) {}

    public コマンドを実行する(コマンド: 編集コマンド): void {
        const 差し戻し = 編集コマンドを適用する(this._モデル, コマンド)
        this._UI状態.取り消し断片を積む(差し戻し)
        this._同期.全体を同期する()
    }

    // 編集のモードを移し、モードで見え方が変わるもの(道路点マーカー・モードのボタン・棚の案内)を
    // 一度に合わせる。棚から道具を取ったときにも呼び、選んだ結果が必ず画面へ返るようにする。
    // 見下ろし図で編集するモードを三次元の表示中に選んだら表示面も見下ろし図へ移す。逆向きは移さない(判断13)。
    public モードを移す(モード: 編集モード): void {
        this._UI状態.モード = モード
        if (見下ろし図で編集するモードか(モード)) this._UI状態.表示面 = '見下ろし図'
        this._同期.UIを同期する()
    }

    public 直前の操作を取り消す(): void {
        const 差し戻し = this._UI状態.取り消し断片を取り出す()
        if (差し戻し !== undefined) {
            差し戻しを適用する(this._モデル, 差し戻し)
            this._同期.全体を同期する()
        }
    }

    public 建物定義一覧を更新する(定義一覧: ReadonlyArray<建物外形定義>): void {
        this._建物定義.clear()
        for (const 定義 of 定義一覧) this._建物定義.set(建物定義IDを生成する(定義.識別子), 定義)
    }

    public 建物生成(建物定義ID: 建物定義ID): void {
        const 定義 = this._建物定義.get(建物定義ID)
        if (定義 === undefined || 定義.用途 !== '家屋') throw new Error(`配置可能な家屋定義ではない: ${建物定義ID}`)
        const チャンク = this._モデル.チャンクを取得する(this._UI状態.対象チャンク座標)
        const 識別子 = `建物_${建物定義ID}_${Date.now()}`
        const x = (Math.random() - 0.5) * 40
        const z = (Math.random() - 0.5) * 40
        const y = チャンク.高さ場.標本高さを取得する(x, z)
        const x半径 = Math.max(Math.abs(定義.外接箱.最小[0]), Math.abs(定義.外接箱.最大[0]))
        const z半径 = Math.max(Math.abs(定義.外接箱.最小[2]), Math.abs(定義.外接箱.最大[2]))
        const 基礎半径 = Math.hypot(x半径, z半径)

        this.コマンドを実行する({
            種類: '建物を配置する',
            値: {
                チャンク座標: { ...this._UI状態.対象チャンク座標 },
                建物: {
                    識別子,
                    建物定義ID,
                    位置: { x, y, z },
                    向きラジアン: 0,
                    基礎半径メートル: 基礎半径,
                    なじみ半径メートル: 基礎半径 * 2,
                },
            },
        })
        this._UI状態.選択中建物識別子 = 識別子
        this._同期.建物を同期する()
        this._同期.UIを同期する()
    }

    public 基礎を平坦化する(): void {
        if (this._UI状態.選択中建物識別子 === null) return
        this.コマンドを実行する({
            種類: '建物基礎を平坦化する',
            値: {
                チャンク座標: { ...this._UI状態.対象チャンク座標 },
                建物識別子: this._UI状態.選択中建物識別子,
            },
        })
    }

    public 建物を地面へ接地させる(): void {
        if (this._UI状態.選択中建物識別子 === null) return
        const チャンク = this._モデル.チャンクを取得する(this._UI状態.対象チャンク座標)
        const 建物 = チャンク.建物.取得する(this._UI状態.選択中建物識別子)
        const 地面高さ = チャンク.高さ場.標本高さを取得する(建物.位置.x, 建物.位置.z)

        this.コマンドを実行する({
            種類: '建物を移動する',
            値: {
                チャンク座標: { ...this._UI状態.対象チャンク座標 },
                識別子: this._UI状態.選択中建物識別子,
                新しい位置: { x: 建物.位置.x, y: 地面高さ, z: 建物.位置.z },
                新しい向きラジアン: 建物.向きラジアン,
            },
        })
    }
}
