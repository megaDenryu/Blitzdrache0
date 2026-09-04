import type { 等高線, 粗マスの塗り, 見下ろし図の下書き } from '../../../生成/編集資源契約.ts'

// 1チャンクの見下ろし図の下書き(等高線一覧と粗マスの塗り)を保持する。高さ場と材質重みの正本ではなく、
// 正本を生成するための入力であり、チャンク構造のJSONへ併存して保存する。
// 参照: `_doc/設計/見下ろし図による地形編集.md`「判断1」「判断4」。
export class 見下ろし図の下書きの管理 {
    private 等高線一覧: Array<等高線>
    private 粗マスの一辺の升目数: number
    private readonly 粗マスの塗りマップ: Map<string, 粗マスの塗り>

    public constructor(初期下書き: 見下ろし図の下書き) {
        this.等高線一覧 = 初期下書き.等高線一覧.map(等高線を複製する)
        this.粗マスの一辺の升目数 = 初期下書き.粗マスの一辺の升目数
        this.粗マスの塗りマップ = new Map<string, 粗マスの塗り>()
        this.粗マスを塗る(初期下書き.粗マスの一辺の升目数, 初期下書き.粗マスの塗り一覧)
    }

    public 等高線一覧を取得する(): Array<等高線> {
        return this.等高線一覧.map(等高線を複製する)
    }

    public 粗マスの一辺の升目数を取得する(): number {
        return this.粗マスの一辺の升目数
    }

    // 列・行の昇順で返し、同じ塗りの集まりが同じ並びになるようにする(保存物の差分と決定性の試験のため)。
    public 粗マスの塗り一覧を取得する(): Array<粗マスの塗り> {
        return Array.from(this.粗マスの塗りマップ.values())
            .map((塗り) => ({ ...塗り }))
            .sort((a, b) => (a.行 - b.行) || (a.列 - b.列))
    }

    public 等高線を追加する(線: 等高線): number {
        this.等高線一覧.push(等高線を複製する(線))
        return this.等高線一覧.length - 1
    }

    public 等高線を添字へ挿入する(添字: number, 線: 等高線): void {
        if (添字 < 0 || 添字 > this.等高線一覧.length) {
            throw new Error(`等高線の挿入先の添字が範囲外: 添字=${添字}, 本数=${this.等高線一覧.length}`)
        }
        this.等高線一覧.splice(添字, 0, 等高線を複製する(線))
    }

    public 等高線を変更する(添字: number, 線: 等高線): 等高線 {
        const 変更前 = this.等高線を取り出す(添字)
        this.等高線一覧[添字] = 等高線を複製する(線)
        return 変更前
    }

    public 等高線を削除する(添字: number): 等高線 {
        const 削除前 = this.等高線を取り出す(添字)
        this.等高線一覧.splice(添字, 1)
        return 削除前
    }

    public 等高線一覧を置き換える(一覧: ReadonlyArray<等高線>): void {
        this.等高線一覧 = 一覧.map(等高線を複製する)
    }

    // 一辺が今と違えば塗りの意味が変わるため、塗りを空にしてから置く。同じ(列,行)は上書きし、
    // 高さも層もnullの項目はその粗マスの塗りを消す。
    public 粗マスを塗る(一辺の升目数: number, 塗り一覧: ReadonlyArray<粗マスの塗り>): void {
        if (!Number.isInteger(一辺の升目数) || 一辺の升目数 < 1) {
            throw new Error(`粗マスの一辺の升目数は正の整数でなければならない: ${一辺の升目数}`)
        }
        if (一辺の升目数 !== this.粗マスの一辺の升目数) {
            this.粗マスの塗りマップ.clear()
            this.粗マスの一辺の升目数 = 一辺の升目数
        }
        for (const 塗り of 塗り一覧) {
            const キー = 粗マスのキー(塗り.列, 塗り.行)
            if (塗り.高さメートル === null && 塗り.層 === null) {
                this.粗マスの塗りマップ.delete(キー)
            } else {
                this.粗マスの塗りマップ.set(キー, { ...塗り })
            }
        }
    }

    public 粗マスの塗り全体を置き換える(一辺の升目数: number, 塗り一覧: ReadonlyArray<粗マスの塗り>): void {
        this.粗マスの塗りマップ.clear()
        this.粗マスの一辺の升目数 = 一辺の升目数
        this.粗マスを塗る(一辺の升目数, 塗り一覧)
    }

    public 契約の形で取り出す(): 見下ろし図の下書き {
        return {
            等高線一覧: this.等高線一覧を取得する(),
            粗マスの一辺の升目数: this.粗マスの一辺の升目数,
            粗マスの塗り一覧: this.粗マスの塗り一覧を取得する(),
        }
    }

    private 等高線を取り出す(添字: number): 等高線 {
        const 線 = this.等高線一覧[添字]
        if (線 === undefined) {
            throw new Error(`等高線の添字が範囲外: 添字=${添字}, 本数=${this.等高線一覧.length}`)
        }
        return 等高線を複製する(線)
    }
}

function 等高線を複製する(線: 等高線): 等高線 {
    return { 高さメートル: 線.高さメートル, 頂点列: 線.頂点列.map((p) => ({ x: p.x, z: p.z })), 閉じている: 線.閉じている }
}

function 粗マスのキー(列: number, 行: number): string {
    return `${列},${行}`
}
