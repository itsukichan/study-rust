pub fn main() {
    // 整数型
    // 符号付き(i8, i16, i32, i64, i128, isize)
    let _x: i32 = 30;
    //符号なし(u8, u16, u32, u64, u128, usize)
    let _y: u32 = 30;

    // 浮動小数点型(f32, f64)
    let _z: f32 = 3.0;

    // 文字型(char): Unicodeスカラ値 (4バイト)
    let _c: char = '🐣';

    // 論理型(bool)
    let _b: bool = true;

    // 配列型([T; N])
    let _a: [i32; 3] = [1, 2, 3];

    // スライス型(&[T]) (配列の参照)
    let _s: &[i32] = &[1, 2, 3];

    // タプル型(T1, T2, T3, ...) (最大12要素)
    let _t: (i32, bool) = (1, true);

    // 文字列型
    // &str: スライス型(&[u8])の参照
    let _st1: &str = "Hello, world!";
    // String: ヒープに確保された可変長文字列
    let _st2: String = "Hello, world!".to_string();

    // オプション型
    // Option<T>: 値が存在するかしないかを表す型
    let _o1: Option<i32> = Some(1); // 値が存在する
    let _o2: Option<i32> = None; // 値が存在しない

    // Result型
    // Result<T, E>: 値が存在するか、エラーが発生したかを表す型
    let _r1: Result<i32, &str> = Ok(1); // 値が存在する
    let _r2: Result<i32, &str> = Err("error"); // エラーが発生した

    // ポインタ型 (参照型)
    // &T: 参照型(&[T])の参照
    let _p1: &i32 = &1;
    // &mut T: 参照型(&mut [T])の参照
    let _p2: &mut i32 = &mut 1;
    // *const T: 定数ポインタ型
    let _p3: *const i32 = &1 as *const i32;
    // *mut T: 可変ポインタ型
    let _p4: *mut i32 = &mut 1 as *mut i32;
    // Box<T>: ヒープに確保された値へのポインタ
    let _p5: Box<i32> = Box::new(1);
    // Rc<T>: 参照カウント型
    // let _p6: Rc<i32> = Rc::new(1);
    // Arc<T>: 参照カウント型 (スレッドセーフ)
    // let _p7: Arc<i32> = Arc::new(1);

    // 関数型
    // fn(T1, T2, T3, ...) -> T (関数ポインタ型)
    fn _f(x: i32) -> i32 {
        x // 関数の終わりにセミコロンがないと、関数の戻り値となる
    }

    // クロージャ型
    // |T1, T2, T3, ...| -> T (クロージャ型)
    let _c1 = |x: i32| x;

    // ユーザー定義型
    // struct S { ... } (構造体型): フィールドを持つ
    // enum E { ... } (列挙型): フィールドを持たない
    // union U { ... } (共用体型): フィールドを持つ
    // trait T { ... } (トレイト型): メソッドを持つ
    // impl T { ... } (実装型): メソッドを持つ
}
