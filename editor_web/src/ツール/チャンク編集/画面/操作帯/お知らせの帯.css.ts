import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../境界/index.ts'

// 操作帯のすぐ下に出す文言の行。三次元と一緒にスクロールして消えないよう、固定の行と同じく縮まない行にする。
export const お知らせの帯の枠 = style({
    flexShrink: 0,
    padding: '4px 12px',
    fontSize: '11px',
    lineHeight: '1.4',
    borderBottom: `1px solid ${エディターCSS変数('境界線')}`,
    backgroundColor: エディターCSS変数('パネル背景'),
    color: エディターCSS変数('テキスト副'),
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
})
