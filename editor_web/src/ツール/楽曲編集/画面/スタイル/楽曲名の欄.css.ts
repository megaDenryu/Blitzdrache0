import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../境界/index.ts'

// 演奏の操作帯と同じ見た目の札にして、同じ行に並んだときに1つの帯として読めるようにする。
export const 楽曲名の枠 = style({
    display: 'flex',
    alignItems: 'center',
    gap: '8px',
    padding: '10px 12px',
    minWidth: '240px',
    flexShrink: 0,
    boxSizing: 'border-box',
    backgroundColor: エディターCSS変数('カード背景'),
    border: `1px solid ${エディターCSS変数('カード枠線')}`,
    borderRadius: '6px',
})

export const 楽曲名の入力 = style({
    flex: 1,
    minWidth: '120px',
    padding: '4px 8px',
    fontSize: '13px',
    fontWeight: 700,
    borderRadius: '4px',
    border: `1px solid ${エディターCSS変数('境界線')}`,
    backgroundColor: エディターCSS変数('パネル背景'),
    color: エディターCSS変数('テキスト主'),
    boxSizing: 'border-box',
})

// 保存先を決めている名乗りは変えられないため、入力欄ではなく添えの表示にする。
export const 名乗りの添え = style({
    fontFamily: 'monospace',
    fontSize: '11px',
    color: エディターCSS変数('テキスト薄'),
    whiteSpace: 'nowrap',
    flexShrink: 0,
})
