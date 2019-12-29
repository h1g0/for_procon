// =====================================================================
// macros
// =====================================================================

/// stdin macro from https://techracho.bpsinc.jp/jhonda/2019_08_05/78537
macro_rules! stdin {
    () => {{
        use std::io::Read;
        #[allow(unused_mut)]
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).unwrap();
        s
    }};
}

/// input macro from https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let mut s = {
            use std::io::Read;
            #[allow(unused_mut)]
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        #[allow(unused_mut)]
        let mut $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}
/// yet another input macro from https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e
#[allow(unused_macros)]
macro_rules! inputln {
    ($t:ty) => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
        }
    };
    ($($t:ty),*) => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                $(iter.next().unwrap().parse::<$t>().unwrap(),)*
            )
        }
    };
    ($t:ty; $n:expr) => {
        (0..$n).map(|_|
            get!($t)
        ).collect::<Vec<_>>()
    };
    ($($t:ty),*; $n:expr) => {
        (0..$n).map(|_|
            get!($($t),*)
        ).collect::<Vec<_>>()
    };
    ($t:ty ;;) => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.split_whitespace()
                .map(|t| t.parse::<$t>().unwrap())
                .collect::<Vec<_>>()
        }
    };
    ($t:ty ;; $n:expr) => {
        (0..$n).map(|_| get!($t ;;)).collect::<Vec<_>>()
    };
}
/// test macro from https://techracho.bpsinc.jp/jhonda/2019_08_05/78537
macro_rules! test {
    ($($input:expr => $output:expr),* $(,)*) => {
        #[test]
        fn answer_test() {
            $(
                assert_eq!(answer($input), $output);
            )*
        }
    };
}
// =====================================================================
// test cases
// =====================================================================

test!{
    /// write inputs for test cases here.
    /// e.g.) 
    /// ```
    /// "1 2 3"=>"4",
    /// r"4 5 6
    ///   7 8 9"=>"10",
    /// ```
    r"
    "=>"",
}

// =====================================================================
// functions
// =====================================================================

/// answer fn for the problem
fn answer(input: &str)->String{
    input!{
        source = input,
        n: usize,
    }
    let result = n;
    return result.to_string();
}

/// the entry point
fn main() {
    println!("{}",answer(&stdin!()));
}
