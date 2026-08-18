import type { ワールド編集状態 } from './編集モデル/index.ts'
import type { ワールド画面部品 } from './画面/index.ts'
import type { ワールドエディター状態 } from './ワールドエディター状態.ts'
import type { ワールドエディター操作サービス } from './ワールドエディター操作サービス.ts'
import type { ワールドエディター同期サービス } from './ワールドエディター同期サービス.ts'
import { ワールドエディター筆致ハンドラ } from './ワールドエディター筆致ハンドラ.ts'
import { ワールドエディタークリックハンドラ } from './ワールドエディタークリックハンドラ.ts'

// 三次元ビューのポインタ操作およびレイキャスト入力を配線する。
export function ポインタとキー入力を配線する(
    部品: ワールド画面部品,
    状態: ワールドエディター状態,
    操作: ワールドエディター操作サービス,
    同期: ワールドエディター同期サービス,
    モデル: ワールド編集状態,
): () => void {
    let 左ボタン押下中 = false
    let 右ボタン押下中 = false
    let 直前X = 0
    let 直前Y = 0

    const 筆致 = new ワールドエディター筆致ハンドラ(モデル, 状態, 部品, 同期)
    const クリック = new ワールドエディタークリックハンドラ(モデル, 状態, 部品, 操作, 同期)
    const ビュー = 部品.三次元ビュー

    ビュー.レイキャスト.配線する({
        移動時: ({ 最前面当たり, 原初事象 }) => {
            const dx = 原初事象.clientX - 直前X
            const dy = 原初事象.clientY - 直前Y
            直前X = 原初事象.clientX
            直前Y = 原初事象.clientY

            if (右ボタン押下中 || (左ボタン押下中 && 状態.モード === 'カメラ')) {
                if (原初事象.buttons === 2 && 状態.モード === 'カメラ') {
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
        対象部品一覧: [ビュー.地形, ビュー.道路ノード, ビュー.建物],
    })

    const キャンバスDOM = ビュー.キャンバス要素.dom.element
    const ホイール処理 = (e: WheelEvent): void => { ビュー.カメラ制御.拡大縮小する(e.deltaY) }
    const コンテキストメニュー抑止 = (e: Event): void => { e.preventDefault() }
    キャンバスDOM.addEventListener('wheel', ホイール処理, { passive: true })
    キャンバスDOM.addEventListener('contextmenu', コンテキストメニュー抑止)

    const キーボード処理 = (e: KeyboardEvent): void => {
        if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'z') {
            e.preventDefault()
            操作.取り消す()
        }
    }
    window.addEventListener('keydown', キーボード処理)

    return () => {
        キャンバスDOM.removeEventListener('wheel', ホイール処理)
        キャンバスDOM.removeEventListener('contextmenu', コンテキストメニュー抑止)
        window.removeEventListener('keydown', キーボード処理)
    }
}
