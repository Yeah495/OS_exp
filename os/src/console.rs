use core::fmt::{self, Write};

use crate::sys_write;

//标准输出
struct Stdout;

//实现格式化输出接口
impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();
        let mut written = 0;

        //循环写入
        while written < bytes.len() {
            let n = sys_write(1, &bytes[written..]);

            if n <= 0 {
                return Err(fmt::Error);
            }

            written += n as usize;
        }

        Ok(())
    }
}

//格式化打印函数
pub fn print(args: fmt::Arguments<'_>) {
    Stdout.write_fmt(args).unwrap();
}

//实现print!宏
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::console::print(format_args!($($arg)*))
    };
}

//实现println!宏
#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($fmt:literal $(, $($arg:tt)+)?) => {
        $crate::print!(
            concat!($fmt, "\n") $(, $($arg)+)?
        )
    };
}
