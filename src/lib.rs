use std::{
    io::stdin,
    str::FromStr,
    sync::{Mutex, MutexGuard},
};

type Buffer = Vec<String>;

/// 从 [`Buffer`] 中解析值
///
/// [`Buffer`] 是一个 [`Vec<String>`] 的可变借用
///
/// 你也可以为你的类型实现 [`FromStr`]，
/// 所有实现 [`FromStr`] 的类型都会自动实现 [`FromBuf`]
pub trait FromBuf: Sized {
    type Err;

    /// 将输入流解析为此类型的值
    ///
    /// # Examples
    /// ```
    /// let mut buf = vec!["123", "abc"];
    /// let n = i32::from_buf(&mut buf).unwrap();
    ///
    /// assert_eq!(123, n);
    /// assert_eq!(vec!["abc"], buf);
    /// ```
    fn from_buf(b: &mut Buffer) -> Result<Self, Self::Err>;
}

impl<T: FromStr> FromBuf for T {
    type Err = T::Err;

    fn from_buf(b: &mut Buffer) -> Result<Self, Self::Err> {
        T::from_str(b.remove(0).as_str())
    }
}

/// 输入流
pub struct InputStream {
    buf_mutex: Mutex<Buffer>,
}

impl InputStream {
    const fn new() -> Self {
        Self {
            buf_mutex: Mutex::new(Buffer::new()),
        }
    }

    fn get_buf(&self) -> MutexGuard<Vec<String>> {
        let mut buf = self.buf_mutex.lock().unwrap();
        if buf.is_empty() {
            let mut temp_str = String::new();
            stdin().read_line(&mut temp_str).unwrap();
            buf.extend(
                temp_str
                    .trim()
                    .split_ascii_whitespace()
                    .map(|s| String::from_str(s).unwrap()),
            );
        }
        buf
    }

    /// 从输入流中解析一个类型为 [`T`] 的值
    ///
    /// [`T`] 需要实现 [`FromBuf`] 和 [`Default`]
    pub fn read<T: FromBuf + Default>(&self) -> T {
        T::from_buf(&mut self.get_buf()).unwrap_or_default()
    }

    /// 读取流中的下一个字符串
    pub fn read_str(&self) -> String {
        self.get_buf().remove(0)
    }

    /// 读取流中的下一个字符
    pub fn read_char(&self) -> char {
        let mut buf = self.get_buf();
        let c = buf[0].remove(0);
        if buf[0].is_empty() {
            buf.remove(0);
        }
        c
    }
}

/// 全局输入流
///
/// 查看 [`InputStream`]
#[allow(non_upper_case_globals)]
pub static rin: InputStream = InputStream::new();

/// 声明并从控制台输入变量
///
/// 使用 [`rinput::rin.read()`][read] 输入
///
/// [read]: InputStream
///
/// # Panics
/// 如果 [`io::stdin`] 读入失败
///
/// [`io::stdin`]: std::io::stdin
///
/// # Examples
///
/// ```
/// use rinput::input;
///
/// input!(n: u8);
/// input!(a: i32, mut c: char, s: String);
/// ```
#[macro_export]
macro_rules! input {
    (mut $name:ident : $type:ty) => {
        let mut $name: $type = rinput::rin.read();
    };

    ($name:ident : $type:ty) => {
        let $name: $type = rinput::rin.read();
    };

    (mut $name:ident : $type:ty, $($tail:tt)+) => {
        input!(mut $name:$type);
        input!($($tail)+);
    };

    ($name:ident : $type:ty, $($tail:tt)+) => {
        input!($name:$type);
        input!($($tail)+);
    };
}
