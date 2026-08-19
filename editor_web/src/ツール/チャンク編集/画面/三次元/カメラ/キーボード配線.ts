import type { 地形追従カメラ制御器 } from './地形追従カメラ制御器.ts'
import type { 注視点マーカー表示制御器 } from './注視点マーカー表示制御器.ts'
import { キー状態から移動方向を決める } from './キー移動割り当て.ts'
import { 入力欄がフォーカス中か } from './キー入力ガード.ts'
import { モードを循環させる } from '../../パネル/モード切替/モード循環.ts'
import type { 毎フレーム処理の束 } from '../毎フレーム処理の束.ts'

// WASDQEでの移動基準速度。移動する/昇降するは距離*0.002を係数として使うため、
// この値は「1ミリ秒あたりの仮想ドラッグ量」に相当する。既定ズーム距離(180m)で
// 秒速およそ50mになるよう0.15に調整した。チャンク編集・大域編集の両方でこの値を共有する。
const キー移動の基準速度 = 0.15
const 移動キー一覧 = new Set<string>(['w', 'a', 's', 'd', 'q', 'e'])

interface キーボード配線対象部品 {
    readonly 三次元ビュー: {
        readonly 毎フレーム処理: 毎フレーム処理の束
        readonly カメラ制御: 地形追従カメラ制御器
        readonly 注視点表示制御: 注視点マーカー表示制御器
    }
}

// チャンク編集・大域編集の両方で共通のキーボード入力(取り消し・WASDQE移動・Altのモード循環)を配線する。
// Tモードを一般化し、両ツールの状態クラスが持つ`モード`フィールドをそのまま受け取る。
export function キーボード入力を配線する<Tモード>(
    部品: キーボード配線対象部品,
    状態: { モード: Tモード },
    操作: { 直前の操作を取り消す(): void },
    同期: { UIを同期する(): void },
    モード一覧: readonly Tモード[],
): () => void {
    const ビュー = 部品.三次元ビュー

    const 押下中キー集合 = new Set<string>()
    let 前回時刻: number | null = null
    const 毎フレーム処理の解除 = ビュー.毎フレーム処理.加える((時刻) => {
        if (前回時刻 === null) { 前回時刻 = 時刻; return }
        const 経過ミリ秒 = 時刻 - 前回時刻
        前回時刻 = 時刻
        if (押下中キー集合.size === 0) return

        const 方向 = キー状態から移動方向を決める(押下中キー集合)
        const 量 = キー移動の基準速度 * 経過ミリ秒
        const 移動あり = 方向.前後 !== 0 || 方向.左右 !== 0
        const 昇降あり = 方向.上下 !== 0
        if (移動あり) {
            ビュー.カメラ制御.移動する(-方向.左右 * 量, 方向.前後 * 量)
        }
        if (昇降あり) {
            ビュー.カメラ制御.昇降する(方向.上下 * 量)
        }
        if (移動あり || 昇降あり) {
            const 注視点 = ビュー.カメラ制御.注視点を取得する()
            ビュー.注視点表示制御.操作された(注視点.x, 注視点.y, 注視点.z)
        }
    })

    const キー押下処理 = (e: KeyboardEvent): void => {
        if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'z') {
            e.preventDefault()
            操作.直前の操作を取り消す()
            return
        }
        if (入力欄がフォーカス中か()) return
        const キー = e.key.toLowerCase()
        if (移動キー一覧.has(キー)) {
            押下中キー集合.add(キー)
            return
        }
        if (e.key === 'Alt' && !e.repeat) {
            e.preventDefault()
            状態.モード = モードを循環させる(モード一覧, 状態.モード, e.shiftKey ? -1 : 1)
            同期.UIを同期する()
        }
    }
    const キー解放処理 = (e: KeyboardEvent): void => {
        押下中キー集合.delete(e.key.toLowerCase())
    }
    window.addEventListener('keydown', キー押下処理)
    window.addEventListener('keyup', キー解放処理)

    return () => {
        毎フレーム処理の解除()
        window.removeEventListener('keydown', キー押下処理)
        window.removeEventListener('keyup', キー解放処理)
    }
}
