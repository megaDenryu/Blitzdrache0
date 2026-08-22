// 道路下の泥ベイクが泥へ塗り替えた画素と、塗り替える直前にそこにあった4層の重みを覚える記録。
// 道路を動かしたときに、古い帯の画素だけを塗り替える直前の材質へ正確に戻すために持つ。
// 記録の無い画素は焼き直しで一切触られないため、帯の外の手塗り・初期生成の材質は消えない。
// 注意: この記録はエディターを開いている間だけ生きる。保存物は合成し終えた材質だけを持ち、
// どの画素が道路由来かを持たないため、読み込み直した直後の記録は空である。そのため前の
// セッションが焼いた泥は、道路を動かしても焼き直しでは消えない。

export interface 道路の泥の被覆の記録の写し {
    readonly 覆っているか: Uint8Array
    readonly 覆う前の材質: Uint8Array
}

export class 道路の泥の被覆の記録 {
    private readonly _覆っているか: Uint8Array
    private readonly _覆う前の材質: Uint8Array

    public constructor(画素数: number) {
        this._覆っているか = new Uint8Array(画素数)
        this._覆う前の材質 = new Uint8Array(画素数 * 4)
    }

    // 覚えている画素の材質を塗り替える直前の値へ戻し、記録を空にする。
    public 覆う前の材質へ戻して記録を空にする(材質データ: Uint8Array): void {
        for (let 画素番号 = 0; 画素番号 < this._覆っているか.length; 画素番号++) {
            if (this._覆っているか[画素番号] === 0) continue
            const 画素先頭 = 画素番号 * 4
            for (let 成分 = 0; 成分 < 4; 成分++) {
                材質データ[画素先頭 + 成分] = this._覆う前の材質[画素先頭 + 成分] ?? 0
            }
            this._覆っているか[画素番号] = 0
        }
    }

    // 1画素を泥で塗り替える直前に、そこにあった材質を覚える。
    public 覆う前の材質を覚える(画素番号: number, 材質データ: Uint8Array): void {
        const 画素先頭 = 画素番号 * 4
        for (let 成分 = 0; 成分 < 4; 成分++) {
            this._覆う前の材質[画素先頭 + 成分] = 材質データ[画素先頭 + 成分] ?? 0
        }
        this._覆っているか[画素番号] = 1
    }

    public 写しを取る(): 道路の泥の被覆の記録の写し {
        return {
            覆っているか: new Uint8Array(this._覆っているか),
            覆う前の材質: new Uint8Array(this._覆う前の材質),
        }
    }

    public 写しから戻す(写し: 道路の泥の被覆の記録の写し): void {
        if (写し.覆っているか.length !== this._覆っているか.length) {
            throw new Error(`戻す被覆の記録の画素数が不正: 期待=${this._覆っているか.length}, 実際=${写し.覆っているか.length}`)
        }
        this._覆っているか.set(写し.覆っているか)
        this._覆う前の材質.set(写し.覆う前の材質)
    }
}
