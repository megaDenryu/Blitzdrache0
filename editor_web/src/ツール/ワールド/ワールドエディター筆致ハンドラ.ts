import type { 位置3次元 } from '../../生成/編集資源契約.ts'
import type { ワールド編集状態 } from './編集モデル/index.ts'
import type { ワールド画面部品 } from './画面/index.ts'
import type { ワールドエディター状態 } from './ワールドエディター状態.ts'
import type { ワールドエディター同期サービス } from './ワールドエディター同期サービス.ts'

// 地形造成および地表ペイントの筆致ドラッグストロークを管理する。
export class ワールドエディター筆致ハンドラ {
    private _筆致通過点列: 位置3次元[] = []
    private _直前高さ退避: Float32Array | null = null
    private _直前材質退避: Uint8Array | null = null

    public constructor(
        private readonly _モデル: ワールド編集状態,
        private readonly _状態: ワールドエディター状態,
        private readonly _部品: ワールド画面部品,
        private readonly _同期: ワールドエディター同期サービス,
    ) {}

    public 押し時(ボタン: number): void {
        if (ボタン === 0) {
            this._筆致通過点列 = []
            const チャンク = this._モデル.チャンクを取得する(this._状態.対象チャンク座標)
            if (this._状態.モード === '造成') this._直前高さ退避 = new Float32Array(チャンク.高さ場.格子データ)
            if (this._状態.モード === '地表ペイント') this._直前材質退避 = new Uint8Array(チャンク.地表材質.材質データ)
        }
    }

    public 移動時(交差点: 位置3次元, 左ボタン押下: boolean, shift押下: boolean): void {
        const ビュー = this._部品.三次元ビュー
        const 半径 = this._状態.モード === '造成' ? this._状態.造成半径 : this._状態.ペイント半径
        ビュー.ブラシリング.地点へ配置する(交差点.x, 交差点.y, 交差点.z).半径を設定する(半径).可視性を設定する(true)

        if (左ボタン押下) {
            this._筆致通過点列.push({ x: 交差点.x, y: 交差点.y, z: 交差点.z })
            const チャンク = this._モデル.チャンクを取得する(this._状態.対象チャンク座標)
            if (this._状態.モード === '造成') {
                チャンク.高さ場.造成筆致を適用する({
                    種別: this._状態.造成筆致種別,
                    通過点列: [{ x: 交差点.x, y: 交差点.y, z: 交差点.z }],
                    半径メートル: this._状態.造成半径,
                    強さ: this._状態.造成強さ * (shift押下 ? -1 : 1),
                })
                this._同期.地形を同期する()
                this._同期.道路を同期する()
            } else if (this._状態.モード === '地表ペイント') {
                チャンク.地表材質.材質筆致を適用する({
                    層: this._状態.材質層,
                    通過点列: [{ x: 交差点.x, y: 交差点.y, z: 交差点.z }],
                    半径メートル: this._状態.ペイント半径,
                    流量: this._状態.ペイント流量,
                })
                this._同期.地形を同期する()
            }
        }
    }

    public 離し時(ボタン: number): void {
        if (ボタン === 0) {
            if (this._状態.モード === '造成' && this._直前高さ退避 !== null && this._筆致通過点列.length > 0) {
                this._状態.取り消し断片を積む({
                    種類: '造成筆致',
                    対象チャンク: { ...this._状態.対象チャンク座標 },
                    変更前格子データ: this._直前高さ退避,
                })
                this._同期.散布を同期する()
            } else if (this._状態.モード === '地表ペイント' && this._直前材質退避 !== null && this._筆致通過点列.length > 0) {
                this._状態.取り消し断片を積む({
                    種類: '材質の筆致',
                    対象チャンク: { ...this._状態.対象チャンク座標 },
                    変更前材質データ: this._直前材質退避,
                })
            }
            this._直前高さ退避 = null
            this._直前材質退避 = null
        }
    }
}
