import type { CanvasC } from 'sengen-ui'

// ホイールで寄り引きし、右クリックの既定のメニューを止めるだけの配線。チャンク編集ビューと
// 大域編集ビューで中身が同じであるため、両方のポインタ配線がこの1つを呼ぶ。
// 返す関数を呼ぶと購読を解除する。

// 触る三次元ビューの部分。キャンバスは事象の購読先、カメラ制御は寄り引きの相手である。
export interface ホイールと右クリックの配線先 {
    readonly キャンバス要素: CanvasC
    readonly カメラ制御: { 拡大縮小する(ホイールの移動量: number): void }
}

export function ホイールと右クリックを配線する(ビュー: ホイールと右クリックの配線先): () => void {
    const ホイール処理 = (e: WheelEvent): void => { ビュー.カメラ制御.拡大縮小する(e.deltaY) }
    const コンテキストメニュー抑止 = (e: MouseEvent): void => { e.preventDefault() }
    ビュー.キャンバス要素.addTypedEventListener('wheel', ホイール処理, { passive: true })
    ビュー.キャンバス要素.addTypedEventListener('contextmenu', コンテキストメニュー抑止)
    return () => {
        ビュー.キャンバス要素.removeTypedEventListener('wheel', ホイール処理)
        ビュー.キャンバス要素.removeTypedEventListener('contextmenu', コンテキストメニュー抑止)
    }
}
