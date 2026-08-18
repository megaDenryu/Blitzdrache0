import { style } from '@vanilla-extract/css'

export const パネル = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '8px',
    padding: '12px',
    backgroundColor: 'rgba(2, 6, 23, 0.6)',
    borderRadius: '8px',
    border: '1px solid rgba(30, 41, 59, 0.8)',
})

export const 見出し行 = style({
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    fontSize: '11px',
    fontWeight: 600,
    color: '#cbd5e1',
})

export const バッジ = style({
    fontSize: '10px',
    fontFamily: 'monospace',
    color: '#34d399',
    backgroundColor: 'rgba(6, 78, 59, 0.5)',
    border: '1px solid #059669',
    borderRadius: '4px',
    padding: '1px 6px',
})

export const 説明リスト = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '4px',
    fontSize: '11px',
    lineHeight: '1.4',
    color: '#94a3b8',
})
