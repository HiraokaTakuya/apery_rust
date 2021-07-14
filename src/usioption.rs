#[cfg(feature = "kppt")]
use crate::evaluate::kppt::*;
use crate::search::*;
use crate::thread::*;
use crate::tt::*;

#[derive(Clone)]
enum UsiOptionValue {
    #[allow(dead_code)]
    String {
        default: String,
        current: String,
    },
    #[allow(dead_code)]
    Filename {
        default: String,
        current: String,
    },
    Spin {
        default: i64,
        current: i64,
        min: i64,
        max: i64,
    },
    Check {
        default: bool,
        current: bool,
    },
    Button,
}

impl UsiOptionValue {
    #[allow(dead_code)]
    fn string(default: &str) -> UsiOptionValue {
        UsiOptionValue::String {
            default: default.to_string(),
            current: default.to_string(),
        }
    }
    #[allow(dead_code)]
    fn filename(default: &str) -> UsiOptionValue {
        UsiOptionValue::Filename {
            default: default.to_string(),
            current: default.to_string(),
        }
    }
    fn spin(default: i64, min: i64, max: i64) -> UsiOptionValue {
        UsiOptionValue::Spin {
            default,
            current: default,
            min,
            max,
        }
    }
    fn check(default: bool) -> UsiOptionValue {
        UsiOptionValue::Check {
            default,
            current: default,
        }
    }
}

#[derive(Clone)]
pub struct UsiOptions {
    v: std::collections::HashMap<&'static str, UsiOptionValue>,
}

impl UsiOptions {
    pub const BOOK_ENABLE: &'static str = "Book_Enable";
    pub const BOOK_FILE: &'static str = "Book_File";
    pub const BYOYOMI_MARGIN: &'static str = "Byoyomi_Margin";
    const CLEAR_HASH: &'static str = "Clear_Hash";
    pub const EVAL_DIR: &'static str = "Eval_Dir";
    #[cfg(feature = "kppt")]
    pub const EVAL_HASH: &'static str = "Eval_Hash";
    pub const MINIMUM_THINKING_TIME: &'static str = "Minimum_Thinking_Time";
    pub const MULTI_PV: &'static str = "MultiPV";
    pub const SLOW_MOVER: &'static str = "Slow_Mover";
    pub const THREADS: &'static str = "Threads";
    pub const TIME_MARGIN: &'static str = "Time_Margin";
    pub const USI_HASH: &'static str = "USI_Hash";
    pub const USI_PONDER: &'static str = "USI_Ponder";

    pub fn new() -> UsiOptions {
        let mut options = std::collections::HashMap::new();

        // The following are all options.
        options.insert(Self::BOOK_ENABLE, UsiOptionValue::check(false));
        options.insert(Self::BOOK_FILE, UsiOptionValue::filename("book/20191216/book.json"));
        options.insert(Self::BYOYOMI_MARGIN, UsiOptionValue::spin(500, 0, i64::max_value()));
        options.insert(Self::CLEAR_HASH, UsiOptionValue::Button);
        options.insert(Self::EVAL_DIR, UsiOptionValue::string("eval/20190617"));
        #[cfg(feature = "kppt")]
        options.insert(Self::EVAL_HASH, UsiOptionValue::spin(256, 1, 1024 * 1024));
        options.insert(Self::MINIMUM_THINKING_TIME, UsiOptionValue::spin(20, 0, 5000));
        options.insert(Self::MULTI_PV, UsiOptionValue::spin(1, 1, 500));
        options.insert(Self::SLOW_MOVER, UsiOptionValue::spin(100, 10, 1000));
        options.insert(Self::THREADS, UsiOptionValue::spin(1, 1, 8192));
        options.insert(Self::TIME_MARGIN, UsiOptionValue::spin(500, 0, i64::max_value()));
        options.insert(Self::USI_HASH, UsiOptionValue::spin(256, 1, 1024 * 1024));
        options.insert(Self::USI_PONDER, UsiOptionValue::check(true));

        UsiOptions { v: options }
    }
    pub fn push_button(&self, key: &str, tt: &mut TranspositionTable) {
        match self.v.get(key) {
            None => {
                println!("Error: illegal option name: {}", key);
            }
            Some(UsiOptionValue::Button) => match key {
                Self::CLEAR_HASH => {
                    tt.clear();
                }
                _ => unreachable!(),
            },
            _ => {
                println!(r#"Error: The option "{}" isn't button type"#, key);
            }
        }
    }
    pub fn set(
        &mut self,
        key: &str,
        value: &str,
        thread_pool: &mut ThreadPool,
        tt: &mut TranspositionTable,
        #[cfg(feature = "kppt")] ehash: &mut EvalHash,
        breadcrumbs: &mut Breadcrumbs,
        reductions: &mut Reductions,
        is_ready: &mut bool,
    ) {
        match self.v.get_mut(key) {
            None => {
                println!("Error: illegal option name: {}", key);
            }
            Some(UsiOptionValue::String { current, .. }) => {
                *current = value.to_string();
                if key == Self::EVAL_DIR {
                    *is_ready = false;
                }
            }
            Some(UsiOptionValue::Filename { current, .. }) => {
                *current = value.to_string();
                if key == Self::BOOK_FILE {
                    *is_ready = false;
                }
            }
            Some(UsiOptionValue::Spin { current, min, max, .. }) => match value.parse::<i64>() {
                Ok(n) => {
                    let n = std::cmp::min(n, *max);
                    let n = std::cmp::max(n, *min);
                    *current = n;
                    match key {
                        #[cfg(feature = "kppt")]
                        Self::EVAL_HASH => ehash.resize(n as usize, thread_pool),
                        Self::THREADS => thread_pool.set(
                            n as usize,
                            tt,
                            #[cfg(feature = "kppt")]
                            ehash,
                            breadcrumbs,
                            reductions,
                        ),
                        Self::USI_HASH => tt.resize(n as usize, thread_pool),
                        _ => {}
                    }
                }
                Err(err) => {
                    println!("{:?}", err);
                }
            },
            Some(UsiOptionValue::Check { current, .. }) => match value {
                "true" => *current = true,
                "false" => *current = false,
                _ => println!("Error: illegal option value: {}", value),
            },
            Some(UsiOptionValue::Button) => println!(r#"Error: The option "{}" is button type. You can't set value to it."#, key),
        }
    }
    pub fn to_usi_string(&self) -> String {
        let mut s = self
            .v
            .iter()
            .map(|(key, opt)| match opt {
                UsiOptionValue::String { default, .. } => {
                    format!("option name {} type string default {}", key, default)
                }
                UsiOptionValue::Filename { default, .. } => {
                    format!("option name {} type filename default {}", key, default)
                }
                UsiOptionValue::Spin { default, min, max, .. } => {
                    format!("option name {} type spin default {} min {} max {}", key, default, min, max)
                }
                UsiOptionValue::Check { default, .. } => {
                    format!("option name {} type check default {}", key, default)
                }
                UsiOptionValue::Button => format!("option name {} type button", key),
            })
            .collect::<Vec<_>>();
        s.sort_unstable();
        s.join("\n") // The last line has no "\n".
    }
    pub fn get_i64(&self, key: &str) -> i64 {
        match self.v.get(key) {
            Some(UsiOptionValue::Spin { current, .. }) => *current,
            _ => panic!("Error: illegal option name: {}", key),
        }
    }
    #[allow(dead_code)]
    pub fn get_string(&self, key: &str) -> String {
        match self.v.get(key) {
            Some(UsiOptionValue::String { current, .. }) => current.clone(),
            _ => panic!("Error: illegal option name: {}", key),
        }
    }
    #[allow(dead_code)]
    pub fn get_filename(&self, key: &str) -> String {
        match self.v.get(key) {
            Some(UsiOptionValue::Filename { current, .. }) => current.clone(),
            _ => panic!("Error: illegal option name: {}", key),
        }
    }
    pub fn get_bool(&self, key: &str) -> bool {
        match self.v.get(key) {
            Some(UsiOptionValue::Check { current, .. }) => *current,
            _ => panic!("Error: illegal option name: {}", key),
        }
    }
}
