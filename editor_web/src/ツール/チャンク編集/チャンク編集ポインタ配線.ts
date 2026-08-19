import type { ポインタ事象情報 } from 'SengenThree'
import type { ワールド編集状態 } from './編集モデル/index.ts'
import type { チャンク編集画面部品 } from './画面/index.ts'
import { モード一覧 } from './画面/パネル/モード切替/モード定義.ts'
import type { チャンク編集状態 } from './チャンク編集状態.ts'
import type { チャンク編集操作サービス } from './チャンク編集操作サービス.ts'
import type { チャンク編集同期サービス } from './チャンク編集同期サービス.ts'
import { チャンク編集筆致ハンドラ } from './チャンク編集筆致ハンドラ.ts'
import { チャンク編集クリックハンドラ } from './チャンク編集クリックハンドラ.ts'
import { ポインタ操作の割り当てを決める } from './画面/三次元/カメラ/ポインタ操作割り当て.ts'
import { キーボード入力を配線する } from './画面/三次元/カメラ/キーボード配線.ts'

// 三次元ビューのポインタ操作およびレイキャスト入力を配線する。
// キーボード入力(取り消し・WASDQE移動・Altのモード循環)は画面/三次元/カメラ/キーボード配線.ts が持つ
// (大域編集ツールと共有する工程のため、ポインタ配線とは別の配線として分離してある)。
export function ポインタとキー入力を配線する(
    部品: チャンク編集画面部品,
    状態: チャンク編集状態,
    操作: チャンク編集操作サービス,
    同期: チャンク編集同期サービス,
    モデル: ワールド編集状態,
): () => void {
    let 左ボタン押下中 = false
    let 右ボタン押下中 = false
    let 中ボタン押下中 = false
    let 直前X = 0
    let 直前Y = 0

    const 筆致 = new チャンク編集筆致ハンドラ(モデル, 状態, 部品, 同期)
    const クリック = new チャンク編集クリックハンドラ(モデル, 状態, 部品, 操作, 同期)
    const ビュー = 部品.三次元ビュー

    const 移動時 = ({ 最前面当たり, 原初事象 }: ポインタ事象情報): void => {
        const dx = 原初事象.clientX - 直前X
        const dy = 原初事象.clientY - 直前Y
        直前X = 原初事象.clientX
        直前Y = 原初事象.clientY

        // 統一操作規則: 右・中は全モードで常にカメラ、左はそのモードの主作業。
        const アクティブボタン = 右ボタン押下中 ? 2 : 中ボタン押下中 ? 1 : 左ボタン押下中 ? 0 : null
        if (アクティブボタン !== null) {
            const 操作種別 = ポインタ操作の割り当てを決める(アクティブボタン)
            if (操作種別 === 'カメラ回転') { ビュー.カメラ制御.回転する(dx, dy); return }
            if (操作種別 === 'カメラ平行移動') { ビュー.カメラ制御.移動する(dx, dy); return }
        }

        if (最前面当たり !== null && 最前面当たり.部品 === ビュー.地形) {
            筆致.移動時(最前面当たり.交差点, 左ボタン押下中, 原初事象.shiftKey)
        } else {
            ビュー.ブラシリング.可視性を設定する(false)
        }
    }

    ビュー.レイキャスト.配線する({
        移動時,
        押し時: ({ 原初事象 }) => {
            直前X = 原初事象.clientX
            直前Y = 原初事象.clientY
            if (原初事象.button === 0) 左ボタン押下中 = true
            if (原初事象.button === 2) 右ボタン押下中 = true
            if (原初事象.button === 1) { 中ボタン押下中 = true; 原初事象.preventDefault() }
            筆致.押し時(原初事象.button)
        },
        離し時: ({ 原初事象 }) => {
            if (原初事象.button === 0) 左ボタン押下中 = false
            if (原初事象.button === 2) 右ボタン押下中 = false
            if (原初事象.button === 1) 中ボタン押下中 = false
            筆致.離し時(原初事象.button)
        },
        クリック時: ({ 最前面当たり, 当たり一覧 }) => {
            クリック.クリックを処理する(最前面当たり, 当たり一覧)
        },
    })

    ビュー.レイキャスト.キャンバスの購読を開始する({
        キャンバス: ビュー.キャンバス要素.dom.element,
        カメラ: ビュー.カメラ,
        対象部品一覧: [ビュー.地形, ビュー.道路ノード, ビュー.建物],
    })

    const ホイール処理 = (e: WheelEvent): void => { ビュー.カメラ制御.拡大縮小する(e.deltaY) }
    const コンテキストメニュー抑止 = (e: MouseEvent): void => { e.preventDefault() }
    ビュー.キャンバス要素.addTypedEventListener('wheel', ホイール処理, { passive: true })
    ビュー.キャンバス要素.addTypedEventListener('contextmenu', コンテキストメニュー抑止)

    const キーボード解除 = キーボード入力を配線する(部品, 状態, 操作, 同期, モード一覧)

    return () => {
        ビュー.レイキャスト.キャンバスの購読を解除する()
        ビュー.キャンバス要素.removeTypedEventListener('wheel', ホイール処理)
        ビュー.キャンバス要素.removeTypedEventListener('contextmenu', コンテキストメニュー抑止)
        キーボード解除()
    }
}
