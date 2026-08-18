import type { 位置3次元 } from '../../生成/編集資源契約.ts'
import type { ワールド編集状態 } from '../ワールド/編集モデル/index.ts'
import type { 大域グリッド画面部品 } from './画面/index.ts'
import type { 大域グリッド状態 } from './大域グリッド状態.ts'
import type { 大域グリッド同期サービス } from './大域グリッド同期サービス.ts'

// 大域地形造成の筆致ドラッグストロークを管理する。
export class 大域グリッド筆致ハンドラ {
    private _筆致通過点列: 位置3次元[] = []
    private _直前高さ退避: Float32Array | null = null

    public constructor(
        private readonly _モデル: ワールド編集状態,
        private readonly _状態: 大域グリッド状態,
        private readonly _部品: 大域グリッド画面部品,
        private readonly _同期: 大域グリッド同期サービス,
    ) {}

    public 押し時(ボタン: number): void {
        if (ボタン === 0 && this._状態.モード === '大域造成') {
            this._筆致通過点列 = []
            this._直前高さ退避 = new Float32Array(this._モデル.大域高さ場.格子データ)
        }
    }

    public 移動時(交差点: 位置3次元, 左ボタン押下: boolean, shift押下: boolean): void {
        const ビュー = this._部品.三次元ビュー
        if (this._状態.モード === '大域造成') {
            ビュー.ブラシリング
                .地点へ配置する(交差点.x, 交差点.y, 交差点.z)
                .半径を設定する(this._状態.造成半径)
                .可視性を設定する(true)
        } else {
            ビュー.ブラシリング.可視性を設定する(false)
        }

        if (左ボタン押下 && this._状態.モード === '大域造成') {
            this._筆致通過点列.push({ x: 交差点.x, y: 交差点.y, z: 交差点.z })
            this._モデル.大域高さ場.造成筆致を適用する({
                種別: this._状態.造成筆致種別,
                通過点列: [{ x: 交差点.x, y: 交差点.y, z: 交差点.z }],
                半径メートル: this._状態.造成半径,
                強さ: this._状態.造成強さ * (shift押下 ? -1 : 1),
            })
            this._同期.地形を同期する()
            this._同期.道路を同期する()
        }
    }

    public 離し時(ボタン: number): void {
        if (ボタン === 0 && this._状態.モード === '大域造成') {
            if (this._直前高さ退避 !== null && this._筆致通過点列.length > 0) {
                this._状態.取り消し断片を積む({
                    種類: '造成筆致',
                    対象チャンク: null,
                    変更前格子データ: this._直前高さ退避,
                })
            }
            this._直前高さ退避 = null
        }
    }
}
