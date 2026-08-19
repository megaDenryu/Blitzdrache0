import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../境界/index.ts'

// VSL右サイドバースロットに収まる大域インスペクターパネルのスタイル定義。
export const インスペクター枠 = style({
    width: '100%',
    height: '100%',
    overflowY: 'auto',
    backgroundColor: エディターCSS変数('パネル背景'),
    padding: '16px',
    boxSizing: 'border-box',
    display: 'flex',
    flexDirection: 'column',
    gap: '14px',
    color: エディターCSS変数('テキスト主'),
})

export const ヘッダー = style({
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
    borderBottom: `1px solid ${エディターCSS変数('境界線')}`,
    paddingBottom: '10px',
})

export const タイトル = style({
    fontSize: '13px',
    fontWeight: 700,
    color: エディターCSS変数('アクセント文字'),
    letterSpacing: '0.05em',
})

export const サブタイトル = style({
    fontSize: '10px',
    fontFamily: 'monospace',
    color: エディターCSS変数('テキスト薄'),
})

export const バッジ = style({
    padding: '2px 8px',
    borderRadius: '4px',
    fontSize: '10px',
    fontFamily: 'monospace',
    backgroundColor: エディターCSS変数('バッジ背景'),
    color: エディターCSS変数('バッジ文字'),
    border: `1px solid ${エディターCSS変数('バッジ枠線')}`,
})
