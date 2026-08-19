import { style } from '@vanilla-extract/css'

// VSL右サイドバースロットに収まるインスペクターパネルのスタイル定義。
export const インスペクター枠 = style({
    width: '100%',
    height: '100%',
    overflowY: 'auto',
    backgroundColor: '#0f172a',
    padding: '16px',
    boxSizing: 'border-box',
    display: 'flex',
    flexDirection: 'column',
    gap: '14px',
    color: '#e2e8f0',
})

export const ヘッダー = style({
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
    borderBottom: '1px solid #1e293b',
    paddingBottom: '10px',
})

export const タイトル = style({
    fontSize: '13px',
    fontWeight: 700,
    color: '#22d3ee',
    letterSpacing: '0.05em',
})

export const サブタイトル = style({
    fontSize: '10px',
    fontFamily: 'monospace',
    color: '#94a3b8',
})

export const バッジ = style({
    padding: '2px 8px',
    borderRadius: '4px',
    fontSize: '10px',
    fontFamily: 'monospace',
    backgroundColor: '#083344',
    color: '#22d3ee',
    border: '1px solid #155e75',
})
