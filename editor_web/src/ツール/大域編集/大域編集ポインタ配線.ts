import type { 大域編集画面部品 } from './画面/index.ts'
import type { 大域編集状態 } from './大域編集状態.ts'
import type { 大域編集操作サービス } from './大域編集操作サービス.ts'
import type { 大域編集同期サービス } from './大域編集同期サービス.ts'
import { 大域編集筆致ハンドラ } from './大域編集筆致ハンドラ.ts'
import { 大域編集クリックハンドラ } from './大域編集クリックハンドラ.ts'
import type { ワールド編集状態 } from '../チャンク編集/編集モデル/index.ts'

// 大域三次元ビューのポインタ操作およびレイキャスト入力を配線する。
export function 大域ポインタとキー入力を配線する(
    部品: 大域編集画面部品,
    状態: 大域編集状態,
    操作: 大域編集操作サービス,
    同期: 大域編集同期サービス,
    モデル: ワールド編集状態,
): () => void {
    let 左ボタン押下中 = false
    let 右ボタン押下中 = false
    let 直前X = 0
    let 直前Y = 0

    const 筆致 = new 大域編集筆致ハンドラ(モデル, 状態, 部品, 同期)
    const クリック = new 大域編集クリックハンドラ(状態, 部品, 操作, 同期)
    const ビュー = 部品.三次元ビュー

    ビュー.レイキャスト.配線する({
        移動時: ({ 最前面当たり, 原初事象 }) => {
            const dx = 原初事象.clientX - 直前X
            const dy = 原初事象.clientY - 直前Y
            直前X = 原初事象.clientX
            直前Y = 原初事象.clientY

            if (右ボタン押下中 || (左ボタン押下中 && 状態.モード === '大域カメラ')) {
                if (原初事象.buttons === 2 && 状態.モード === '大域カメラ') {
                    ビュー.カメラ制御.移動する(dx, dy)
                } else {
                    ビュー.カメラ制御.回転する(dx, dy)
                }
                return
            }

            if (最前面当たり !== null && 最前面当たり.部品 === ビュー.地形) {
                筆致.移動時(最前面当たり.交差点, 左ボタン押下中, 原初事象.shiftKey)
            } else {
                ビュー.ブラシリング.可視性を設定する(false)
            }
        },
        押し時: ({ 原初事象 }) => {
            直前X = 原初事象.clientX
            直前Y = 原初事象.clientY
            if (原初事象.button === 0) 左ボタン押下中 = true
            if (原初事象.button === 2) 右ボタン押下中 = true
            筆致.押し時(原初事象.button)
        },
        離し時: ({ 原初事象 }) => {
            if (原初事象.button === 0) 左ボタン押下中 = false
            if (原初事象.button === 2) 右ボタン押下中 = false
            筆致.離し時(原初事象.button)
        },
        クリック時: ({ 最前面当たり, 当たり一覧 }) => {
            クリック.クリック処理(最前面当たり, 当たり一覧)
        },
    })

    ビュー.レイキャスト.キャンバスの購読を開始する({
        キャンバス: ビュー.キャンバス要素.dom.element,
        カメラ: ビュー.カメラ,
        対象部品一覧: [ビュー.地形, ビュー.道路ノード],
    })

    const ホイール処理 = (e: WheelEvent): void => {
        ビュー.カメラ制御.拡大縮小する(e.deltaY)
    }
    const コンテキストメニュー抑止 = (e: MouseEvent): void => {
        e.preventDefault()
    }
    ビュー.キャンバス要素.addTypedEventListener('wheel', ホイール処理, { passive: true })
    ビュー.キャンバス要素.addTypedEventListener('contextmenu', コンテキストメニュー抑止)

    const キーボード処理 = (e: KeyboardEvent): void => {
        if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'z') {
            e.preventDefault()
            操作.直前の操作を取り消す()
        }
    }
    window.addEventListener('keydown', キーボード処理)

    return () => {
        ビュー.レイキャスト.キャンバスの購読を解除する()
        ビュー.キャンバス要素.removeTypedEventListener('wheel', ホイール処理)
        ビュー.キャンバス要素.removeTypedEventListener('contextmenu', コンテキストメニュー抑止)
        window.removeEventListener('keydown', キーボード処理)
    }
}
