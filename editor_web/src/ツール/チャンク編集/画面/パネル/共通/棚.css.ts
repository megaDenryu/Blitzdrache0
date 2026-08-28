import { style, globalStyle } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../境界/index.ts'

// 下パネルへ収める「これから使うものの棚」。横長の区画であるため、選ぶものを横へ並べて
// 一度に見渡せるようにする(設計正本の判断14)。
export const 棚の枠 = style({
    width: '100%',
    height: '100%',
    boxSizing: 'border-box',
    padding: '10px 14px',
    overflowY: 'auto',
    display: 'flex',
    flexDirection: 'column',
    gap: '10px',
    backgroundColor: エディターCSS変数('パネル背景'),
    color: エディターCSS変数('テキスト主'),
})

// 使うものの組は種類が増えるほど横へ伸びるため、この列だけが折り返して広がる。
export const 棚の列 = style({
    display: 'flex',
    alignItems: 'flex-start',
    gap: '12px',
    flexWrap: 'wrap',
})

// 組の幅を揃えるのは、幅が中身の量で変わると、モードを変えるたびに棚の並びが動いて
// 同じ道具を同じ位置で探せなくなるためである。
globalStyle(`${棚の列} > *`, {
    width: '280px',
    flexShrink: 0,
})

// 棚に並ぶ1組の外枠。造成の筆・地表の材質のパネルが持つカードと同じ姿にして、
// 棚の中の組が同じ見え方で並ぶようにする。
export const 棚のカード = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '10px',
    padding: '12px',
    boxSizing: 'border-box',
    backgroundColor: エディターCSS変数('カード背景'),
    backdropFilter: エディターCSS変数('ガラス背景ぼかし'),
    borderRadius: '8px',
    border: `1px solid ${エディターCSS変数('カード枠線')}`,
    boxShadow: エディターCSS変数('カード影'),
})

export const 棚のカードの見出し = style({
    fontSize: '11px',
    fontWeight: 600,
    color: エディターCSS変数('テキスト副'),
})

// いまのモードで何ができるかの案内。棚の一番上へ置き、道具を選ぶ前に読める位置にする。
export const 棚の案内文 = style({
    fontSize: '11px',
    lineHeight: '1.4',
    color: エディターCSS変数('テキスト薄'),
    textWrap: 'pretty',
})
