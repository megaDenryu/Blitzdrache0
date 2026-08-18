import { style } from '@vanilla-extract/css'

export const パネル = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '10px',
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

export const 件数ラベル = style({
    fontFamily: 'monospace',
    color: '#22d3ee',
})

export const 生成ボタングリッド = style({
    display: 'grid',
    gridTemplateColumns: 'repeat(3, 1fr)',
    gap: '4px',
})

export const 生成ボタン = style({
    padding: '6px 2px',
    fontSize: '10px',
    fontWeight: 500,
    borderRadius: '4px',
    border: '1px solid #334155',
    backgroundColor: '#1e293b',
    color: '#cbd5e1',
    cursor: 'pointer',
    transition: 'all 0.15s ease',
    ':hover': { backgroundColor: '#334155' },
})

export const アクション区画 = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '6px',
    paddingTop: '8px',
    borderTop: '1px solid #1e293b',
})

export const 平坦化ボタン = style({
    padding: '8px',
    fontSize: '11px',
    fontWeight: 600,
    borderRadius: '4px',
    border: 'none',
    backgroundColor: '#059669',
    color: '#ffffff',
    cursor: 'pointer',
    transition: 'all 0.15s ease',
    ':hover': { backgroundColor: '#10b981' },
})

export const 行ボタン群 = style({
    display: 'flex',
    gap: '8px',
})

export const 接地ボタン = style({
    flex: 1,
    padding: '4px 8px',
    fontSize: '11px',
    borderRadius: '4px',
    border: '1px solid #334155',
    backgroundColor: '#1e293b',
    color: '#cbd5e1',
    cursor: 'pointer',
    ':hover': { backgroundColor: '#334155' },
})

export const 削除ボタン = style({
    flex: 1,
    padding: '4px 8px',
    fontSize: '11px',
    borderRadius: '4px',
    border: '1px solid #9f1239',
    backgroundColor: '#4c0519',
    color: '#fda4af',
    cursor: 'pointer',
    ':disabled': { opacity: 0.3, cursor: 'not-allowed' },
})
