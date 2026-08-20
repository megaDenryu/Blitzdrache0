// 道路点の在り処とは、道路一覧の何本目の道路の、何番目の制御点かを表す値のことである。
// 1つの道路一覧に道路が何本でも入るようになったため、点を1つ指すには道路の添字と
// 制御点の添字の2つが要る。
export interface 道路点の在り処 {
    readonly 道路添字: number
    readonly 制御点添字: number
}

export function 同じ道路点を指すか(片方: 道路点の在り処 | null, 相手: 道路点の在り処 | null): boolean {
    if (片方 === null || 相手 === null) return 片方 === 相手
    return 片方.道路添字 === 相手.道路添字 && 片方.制御点添字 === 相手.制御点添字
}
