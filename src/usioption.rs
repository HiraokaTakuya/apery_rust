use crate::evaluate::*;
use crate::thread::*;
use crate::tt::*;

#[derive(Clone)]
enum UsiOptionValue {
    StringOption {
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
    fn string(default: &str, current: &str) -> UsiOptionValue {
        UsiOptionValue::StringOption {
            default: default.to_string(),
            current: current.to_string(),
        }
    }
    fn spin(default: i64, current: i64, min: i64, max: i64) -> UsiOptionValue {
        UsiOptionValue::Spin {
            default,
            current,
            min,
            max,
        }
    }
    fn check(default: bool, current: bool) -> UsiOptionValue {
        UsiOptionValue::Check { default, current }
    }

    fn string_default(default: &str) -> UsiOptionValue {
        Self::string(default, default)
    }
    fn spin_default(default: i64, min: i64, max: i64) -> UsiOptionValue {
        Self::spin(default, default, min, max)
    }
    fn check_default(default: bool) -> UsiOptionValue {
        Self::check(default, default)
    }
}

#[derive(Clone)]
pub struct UsiOptions {
    v: std::collections::HashMap<&'static str, UsiOptionValue>,
}

impl UsiOptions {
    pub fn new() -> UsiOptions {
        let mut options = std::collections::HashMap::new();

        // The following are all options.
        options.insert(
            "Byoyomi_Margin",
            UsiOptionValue::spin_default(500, 0, i64::max_value()),
        );
        options.insert("Clear_Hash", UsiOptionValue::Button);
        options.insert("Eval_Dir", UsiOptionValue::string_default("eval/20190617"));
        options.insert(
            "Eval_Hash",
            UsiOptionValue::spin_default(256, 1, 1024 * 1024),
        );
        options.insert(
            "Minimum_Thinking_Time",
            UsiOptionValue::spin_default(20, 0, 5000),
        );
        options.insert("MultiPV", UsiOptionValue::spin_default(1, 1, 500));
        options.insert("Slow_Mover", UsiOptionValue::spin_default(84, 10, 1000));
        options.insert("Threads", UsiOptionValue::spin_default(1, 1, 8192));
        options.insert(
            "Time_Margin",
            UsiOptionValue::spin_default(500, 0, i64::max_value()),
        );
        options.insert(
            "USI_Hash",
            UsiOptionValue::spin_default(256, 1, 1024 * 1024),
        );
        options.insert("USI_Ponder", UsiOptionValue::check_default(true));

        UsiOptions { v: options }
    }
    pub fn push_button(&self, key: &str, tt: &mut TranspositionTable) {
        match self.v.get(key) {
            None => {
                println!("Error: illegal option name: {}", key);
            }
            Some(UsiOptionValue::Button) => match key {
                "Clear_Hash" => {
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
        ehash: &mut EvalHash,
    ) {
        match self.v.get_mut(key) {
            None => {
                println!("Error: illegal option name: {}", key);
            }
            Some(UsiOptionValue::StringOption { current, .. }) => {
                *current = value.to_string();
            }
            Some(UsiOptionValue::Spin {
                current, min, max, ..
            }) => match value.parse::<i64>() {
                Ok(n) => {
                    let n = std::cmp::min(n, *max);
                    let n = std::cmp::max(n, *min);
                    *current = n;
                    match key {
                        "Eval_Hash" => {
                            ehash.resize(n as usize, thread_pool);
                        }
                        "Threads" => {
                            thread_pool.set(n as usize, tt, ehash);
                        }
                        "USI_Hash" => {
                            tt.resize(n as usize, thread_pool);
                        }
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
            Some(UsiOptionValue::Button) => println!(
                r#"Error: The option "{}" is button type. You can't set value to it."#,
                key
            ),
        }
    }
    pub fn to_usi_string(&self) -> String {
        let mut s = self
            .v
            .iter()
            .map(|(key, opt)| match opt {
                UsiOptionValue::StringOption { default, .. } => {
                    format!("option name {} type string default {}", key, default)
                }
                UsiOptionValue::Spin {
                    default, min, max, ..
                } => format!(
                    "option name {} type spin default {} min {} max {}",
                    key, default, min, max
                ),
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
    pub fn get_string(&self, key: &str) -> String {
        match self.v.get(key) {
            Some(UsiOptionValue::StringOption { current, .. }) => current.clone(),
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
