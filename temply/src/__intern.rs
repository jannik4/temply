pub mod indent {
    use std::fmt::{self};

    pub struct Indenter<'a, T: ?Sized> {
        inner: &'a mut T,
        indentation: &'static str,
        needs_indent: bool,
    }

    impl<'a, T: ?Sized> Indenter<'a, T> {
        pub fn new(f: &'a mut T, indentation: &'static str) -> Self {
            Self {
                inner: f,
                indentation,
                needs_indent: false,
            }
        }
    }

    impl<T> fmt::Write for Indenter<'_, T>
    where
        T: fmt::Write + ?Sized,
    {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            for (idx, line) in s.split('\n').enumerate() {
                if idx > 0 {
                    self.inner.write_char('\n')?;
                    self.needs_indent = true;
                }

                // Skip empty lines
                if line.is_empty() {
                    continue;
                }

                if self.needs_indent {
                    write!(self.inner, "{}", self.indentation)?;
                    self.needs_indent = false;
                }

                write!(self.inner, "{}", line)?;
            }

            Ok(())
        }
    }
}
