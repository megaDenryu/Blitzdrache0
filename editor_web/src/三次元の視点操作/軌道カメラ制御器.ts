import { Vector3 } from 'three'
import type { 透視カメラ } from 'SengenThree'
import type { 位置3次元 } from '../生成/編集資源契約.ts'

// 軌道カメラの姿勢・注視点・距離を保持し、透視カメラへ反映する制御器。
// 三次元を持つエディターの視点はどれもこの形であるため、道具ごとに持たずこの共通の置き場が1つ持つ。
// 参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断13」
export class 軌道カメラ制御器 {
    private readonly _注視点: Vector3 = new Vector3(0, 0, 0)
    private readonly _カメラ位置キャッシュ: Vector3 = new Vector3(0, 0, 0)
    private _水平角: number = 0
    private _垂直角: number = Math.PI / 4
    private _距離: number
    private readonly _最小距離: number
    private readonly _最大距離: number

    public constructor(
        private readonly _カメラ: 透視カメラ,
        初期距離: number = 180,
        最小距離: number = 10,
        最大距離: number = 1000,
    ) {
        this._距離 = 初期距離
        this._最小距離 = 最小距離
        this._最大距離 = 最大距離
        this.更新する()
    }

    public 回転する(デルタX: number, デルタY: number): void {
        this._水平角 -= デルタX * 0.005
        this._垂直角 = Math.max(0.05, Math.min(Math.PI / 2 - 0.02, this._垂直角 - デルタY * 0.005))
        this.更新する()
    }

    public 移動する(デルタX: number, デルタY: number): void {
        const 前方 = new Vector3(
            -Math.sin(this._水平角),
            0,
            -Math.cos(this._水平角),
        ).normalize()
        const 右方 = new Vector3(
            Math.cos(this._水平角),
            0,
            -Math.sin(this._水平角),
        ).normalize()

        const 係数 = this._距離 * 0.002
        this._注視点.addScaledVector(右方, -デルタX * 係数)
        this._注視点.addScaledVector(前方, デルタY * 係数)
        this.更新する()
    }

    public 昇降する(デルタ: number): void {
        const 係数 = this._距離 * 0.002
        this._注視点.y += デルタ * 係数
        this.更新する()
    }

    public 拡大縮小する(デルタY: number): void {
        const 変化量 = デルタY * 0.1
        this._距離 = Math.max(this._最小距離, Math.min(this._最大距離, this._距離 + 変化量))
        this.更新する()
    }

    public 更新する(): void {
        const sinPhi = Math.sin(this._垂直角)
        const cosPhi = Math.cos(this._垂直角)
        const sinTheta = Math.sin(this._水平角)
        const cosTheta = Math.cos(this._水平角)

        const x = this._注視点.x + this._距離 * sinPhi * sinTheta
        const y = this._注視点.y + this._距離 * cosPhi
        const z = this._注視点.z + this._距離 * sinPhi * cosTheta

        this._カメラ位置キャッシュ.set(x, y, z)
        this._カメラ.位置を設定する(x, y, z)
        this._カメラ.注視点を設定する(this._注視点.x, this._注視点.y, this._注視点.z)
    }

    // 注視点の高さだけを直接差し替える。地形追従カメラ制御器が地形の高さへ追従させるために使う
    // (水平角・垂直角・距離は変えないため、カメラは注視点と同じ量だけ上下する)。
    public 注視点の高さを設定する(高さ: number): void {
        this._注視点.y = 高さ
        this.更新する()
    }

    // 注視点を名指した地点へ移す。編集の対象ぜんたいが画面へ収まる置き場を呼び出し側が決めるときに使う。
    public 注視点を設定する(x: number, y: number, z: number): void {
        this._注視点.set(x, y, z)
        this.更新する()
    }

    // 注視点からの距離を名指した値へ移す。最小と最大の間へ収めるのは拡大縮小と同じ規律である。
    public 距離を設定する(距離: number): void {
        this._距離 = Math.max(this._最小距離, Math.min(this._最大距離, 距離))
        this.更新する()
    }

    public 注視点を取得する(): 位置3次元 {
        return { x: this._注視点.x, y: this._注視点.y, z: this._注視点.z }
    }

    public カメラ位置を取得する(): 位置3次元 {
        return { x: this._カメラ位置キャッシュ.x, y: this._カメラ位置キャッシュ.y, z: this._カメラ位置キャッシュ.z }
    }

    public 距離を取得する(): number {
        return this._距離
    }
}
