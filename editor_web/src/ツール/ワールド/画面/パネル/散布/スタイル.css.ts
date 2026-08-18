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
    fontSize: '11px',
    fontWeight: 600,
    color: '#cbd5e1',
})

export const 本数ラベル = style({
    fontFamily: 'monospace',
    color: '#34d399',
})

export const 再ベイクボタン = style({
    padding: '6px',
    fontSize: '11px',
    fontFamily: 'monospace',
    borderRadius: '4px',
    border: '1px solid #334155',
    backgroundColor: '#1e293b',
    color: '#e2e8f0',
    cursor: 'pointer',
    transition: 'all 0.15s ease',
    ':hover': {
        backgroundColor: '#334155',
    },
})
