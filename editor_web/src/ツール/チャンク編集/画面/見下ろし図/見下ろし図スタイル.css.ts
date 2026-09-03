import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../境界/index.ts'

// 見下ろし図は三次元ビューと同じ枠を占め、片方だけが表示される(設計正本の判断6)。
export const コンテナ = style({
    position: 'absolute',
    inset: 0,
    width: '100%',
    height: '100%',
    overflow: 'hidden',
    backgroundColor: エディターCSS変数('ビューポート背景'),
})

// 中ドラッグのパンとホイールのズームを受けるため、ブラウザ既定のタッチ操作と中クリックの自動スクロールを切る。
export const キャンバス = style({
    display: 'block',
    width: '100%',
    height: '100%',
    outline: 'none',
    touchAction: 'none',
    cursor: 'crosshair',
})
