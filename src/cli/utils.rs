use clap_generate::Shell;
use env_logger::fmt::Color;
use env_logger::Builder;
use log::{Level, LevelFilter};
use std::{env, path::PathBuf};

#[cfg(feature = "ring")]
pub mod ring {
    #[derive(Debug, Clone, Copy)]
    pub struct Ring<T, const S: usize> {
        buf: [Option<T>; S],
        start: usize,
        len: usize,
    }

    impl<T, const S: usize> Ring<T, S> {
        pub fn new() -> Self {
            Self {
                buf: array_init::array_init(|_| None),
                start: 0,
                len: 0,
            }
        }

        pub fn space(&self) -> usize {
            S - self.len
        }

        pub fn len(&self) -> usize {
            self.len
        }

        pub fn is_empty(&self) -> bool {
            self.len == 0
        }

        pub fn is_full(&self) -> bool {
            self.len == S
        }

        pub fn push(&mut self, value: T) -> Option<T> {
            if self.space() == 0 {
                let val = self.buf[self.start].replace(value);
                self.start += 1;
                val
            } else {
                let val = self.buf[(self.start + self.len) % S].replace(value);
                self.len += 1;
                val
            }
        }

        pub fn pop(&mut self) -> Option<T> {
            if self.is_empty() {
                None
            } else {
                let val = self.buf[self.start].take();
                self.start = if self.start == S - 1 {
                    0
                } else {
                    self.start + 1
                };
                self.len -= 1;
                val
            }
        }

        pub fn get(&self, index: usize) -> Option<&T> {
            if index >= S {
                None
            } else {
                self.buf[(self.start + index) % S].as_ref()
            }
        }

        pub fn iter(&self) -> RingIter<T, S> {
            RingIter::new(self)
        }

        pub fn resize<const SIZE: usize>(self) -> Ring<T, SIZE> {
            let mut new = Ring::default();
            for item in self {
                new.push(item);
            }
            new
        }
    }

    impl<T, const S: usize> Default for Ring<T, S> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T, const S: usize> Iterator for Ring<T, S> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            self.pop()
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.len, Some(self.len))
        }
    }

    pub struct RingIter<'a, T, const S: usize> {
        ring: &'a Ring<T, S>,
        index: usize,
    }

    impl<'a, T, const S: usize> RingIter<'a, T, S> {
        pub fn new(ring: &'a Ring<T, S>) -> Self {
            Self { ring, index: 0 }
        }
    }

    impl<'a, T, const S: usize> Iterator for RingIter<'a, T, S> {
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            let val = self.ring.get(self.index);
            self.index += 1;
            val
        }
    }

    #[test]
    fn test_ring() {
        let mut ring: Ring<usize, 3> = Ring::new();

        // Ring [None, None, None]
        //       ^^                         len = 0
        //  start & end

        assert_eq!(ring.push(1), None);
        assert_eq!(ring.len(), 1);
        assert!(!ring.is_full());

        // Ring [Some(1), None, None]
        //       ^ ------ ^                 len = 1
        //     start     end

        assert_eq!(ring.push(2), None);
        assert_eq!(ring.len(), 2);
        assert!(!ring.is_full());

        // Ring [Some(1), Some(2), None]
        //       ^ --------------- ^        len = 2
        //     start              end

        assert_eq!(ring.push(3), None);
        assert_eq!(ring.len(), 3);
        assert!(ring.is_full());

        // Ring [Some(1), Some(2), Some(3)]
        //       ^^                         len = 3
        //  start & end

        assert_eq!(ring.push(4), Some(1));
        assert_eq!(ring.len(), 3);
        assert!(ring.is_full());

        // Ring [Some(4), Some(2), Some(3)]
        //       ^^                         len = 3
        //  start & end
        //
        // Some(1) got removed and replaced with Some(4)

        assert_eq!(ring.pop(), Some(2));
        assert_eq!(ring.len(), 2);

        // Ring [None, Some(2), Some(3)]
        //             ^ ------ ^           len = 2
        //           start     end

        assert_eq!(ring.pop(), Some(3));
        assert_eq!(ring.len(), 1);

        // Ring [Some(1), None, None]
        //    -- ^              ^ -----     len = 1
        //      end           start

        assert_eq!(ring.pop(), Some(4));
        assert_eq!(ring.len(), 0);
        assert!(!ring.is_full());
        assert!(ring.is_empty());

        // Ring [None, None, None]
        //       ^^                         len = 0
        //  start & end
    }

    #[test]
    fn test_ring_iter() {
        let mut ring: Ring<usize, 5> = Ring::new();

        let items = [1, 2, 3, 4, 5, 6, 7, 8, 9];

        for item in items {
            ring.push(item);
        }

        assert_eq!(ring.get(0), Some(&5));
        assert_eq!(ring.get(1), Some(&6));
        assert_eq!(ring.get(2), Some(&7));
        assert_eq!(ring.get(6), None);

        assert_eq!(vec![&5, &6, &7, &8, &9], ring.iter().collect::<Vec<_>>());

        assert_eq!(vec![5, 6, 7, 8, 9], ring.collect::<Vec<_>>());
    }
}

#[cfg(feature = "ring")]
pub use ring::*;

pub fn detect_shell() -> Option<Shell> {
    match env::var("SHELL") {
        Ok(shell) => PathBuf::from(shell)
            .file_name()
            .and_then(|name| name.to_str())
            .and_then(|name| name.parse().ok()),
        Err(_) => None,
    }
}

pub fn init_logger(level: Option<LevelFilter>) {
    let mut builder = Builder::new();

    if let Some(lf) = level {
        builder.filter_level(lf);
    } else if let Ok(s) = ::std::env::var("CLASHCTL_LOG") {
        builder.parse_filters(&s);
    } else {
        builder.filter_level(LevelFilter::Info);
    }

    builder.format(|f, record| {
        use std::io::Write;
        let mut style = f.style();

        let level = match record.level() {
            Level::Trace => style.set_color(Color::Magenta).value("Trace"),
            Level::Debug => style.set_color(Color::Blue).value("Debug"),
            Level::Info => style.set_color(Color::Green).value(" Info"),
            Level::Warn => style.set_color(Color::Yellow).value(" Warn"),
            Level::Error => style.set_color(Color::Red).value("Error"),
        };

        writeln!(f, " {} > {}", level, record.args(),)
    });

    builder.init()
}
