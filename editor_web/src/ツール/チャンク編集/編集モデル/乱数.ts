// 乱数の種から決定的な擬似乱数列を生成する。
// 同一の種に対しては常に同一の乱数列が得られることを保証する。
export class 決定的乱数生成器 {
    private 状態: bigint

    public constructor(種: bigint) {
        // 種が0の場合は状態が0に縮退するのを防ぐため非ゼロの初期値を用いる。
        this.状態 = 種 === 0n ? 0x853c49e6748fea9bn : 種 & 0xffffffffffffffffn
    }

    // 0以上1未満の64ビット浮動小数点実数を生成する。
    public 次の実数を生成する(): number {
        // xorshift64starのアルゴリズムにより内部状態を更新する。
        let x = this.状態
        x ^= (x >> 12n) & 0xffffffffffffffffn
        x ^= (x << 25n) & 0xffffffffffffffffn
        x ^= (x >> 27n) & 0xffffffffffffffffn
        this.状態 = x & 0xffffffffffffffffn
        const 乗算値 = (this.状態 * 0x2545f4914f6cdd1dn) & 0xffffffffffffffffn
        // 上位53ビットを取り出して2^53で除算し、倍精度浮動小数点の仮数部に適合させる。
        const 精度53ビット = Number(乗算値 >> 11n)
        return 精度53ビット / 9007199254740992
    }
}
