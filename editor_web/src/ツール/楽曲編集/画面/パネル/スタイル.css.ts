import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../境界/index.ts'

// 右サイドバーへ収める設定の並び。中央のトラックとは別に、この枠の中だけが縦にスクロールする(設計正本の判断14)。
export const 楽曲インスペクター枠 = style({
    width: '100%',
    height: '100%',
    overflowY: 'auto',
    boxSizing: 'border-box',
    padding: '16px',
    backgroundColor: エディターCSS変数('パネル背景'),
    color: エディターCSS変数('テキスト主'),
    display: 'flex',
    flexDirection: 'column',
    gap: '14px',
})
