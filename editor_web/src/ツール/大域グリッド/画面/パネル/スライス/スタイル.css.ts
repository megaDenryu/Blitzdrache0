import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../境界/index.ts'

export const パネル = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '8px',
    padding: '12px',
    backgroundColor: エディターCSS変数('カード背景'),
    backdropFilter: エディターCSS変数('ガラス背景ぼかし'),
    borderRadius: '8px',
    border: `1px solid ${エディターCSS変数('カード枠線')}`,
})

export const 見出し行 = style({
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    fontSize: '11px',
    fontWeight: 600,
    color: エディターCSS変数('テキスト副'),
})

export const バッジ = style({
    fontSize: '10px',
    fontFamily: 'monospace',
    color: エディターCSS変数('バッジ文字'),
    backgroundColor: エディターCSS変数('バッジ背景'),
    border: `1px solid ${エディターCSS変数('バッジ枠線')}`,
    borderRadius: '4px',
    padding: '1px 6px',
})

export const 説明リスト = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '4px',
    fontSize: '11px',
    lineHeight: '1.4',
    color: エディターCSS変数('テキスト薄'),
})
