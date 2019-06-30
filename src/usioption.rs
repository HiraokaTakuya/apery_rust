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
    pub fn string(default: &str) -> UsiOptionValue {
        UsiOptionValue::StringOption {
            default: default.to_string(),
            current: default.to_string(),
        }
    }
    pub fn spin(default: i64, min: i64, max: i64) -> UsiOptionValue {
        UsiOptionValue::Spin {
            default,
            current: default,
            min,
            max,
        }
    }
    pub fn check(default: bool) -> UsiOptionValue {
        UsiOptionValue::Check {
            default,
            current: default,
        }
    }
}

#[derive(Clone)]
pub struct UsiOptions {
    v: std::collections::HashMap<String, UsiOptionValue>,
}

impl UsiOptions {
    pub fn new() -> UsiOptions {
        let mut options = std::collections::HashMap::new();

        // The following are all options.
        options.insert(
            "Byoyomi_Margin".to_string(),
            UsiOptionValue::spin(500, 0, i64::max_value()),
        );
        options.insert("Clear_Hash".to_string(), UsiOptionValue::Button);
        options.insert(
            "Eval_Dir".to_string(),
            UsiOptionValue::string("eval/20190617"),
        );
        options.insert(
            "Eval_Hash".to_string(),
            UsiOptionValue::spin(256, 1, 1024 * 1024),
        );
        options.insert(
            "Minimum_Thinking_Time".to_string(),
            UsiOptionValue::spin(20, 0, 5000),
        );
        options.insert("MultiPV".to_string(), UsiOptionValue::spin(1, 1, 500));
        options.insert("Slow_Mover".to_string(), UsiOptionValue::spin(84, 10, 1000));
        options.insert("Threads".to_string(), UsiOptionValue::spin(1, 1, 8192));
        options.insert(
            "Time_Margin".to_string(),
            UsiOptionValue::spin(500, 0, i64::max_value()),
        );
        options.insert(
            "USI_Hash".to_string(),
            UsiOptionValue::spin(256, 1, 1024 * 1024),
        );
        options.insert("USI_Ponder".to_string(), UsiOptionValue::check(true));

        UsiOptions { v: options }
    }
    pub fn push_button(&self, key: &str, tt: &mut TranspositionTable) {
        if self.v.get(key).is_none() {
            println!("Error: illegal option name: {}", key);
            return;
        }
        match &self.v[key] {
            UsiOptionValue::Button => match key {
                "Clear_Hash" => {
                    tt.clear();
                }
                _ => unreachable!(),
            },
            _ => {
                println!(r#"Error: The option "{}" isn't button type"#, key);
                return;
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
        if self.v.get(key).is_none() {
            println!("Error: illegal option name: {}", key);
            return;
        }
        match self.v[key] {
            UsiOptionValue::StringOption { .. } => {
                self.v
                    .insert(key.to_string(), UsiOptionValue::string(value));
            }
            UsiOptionValue::Spin { min, max, .. } => match value.parse::<i64>() {
                Ok(n) => {
                    let n = std::cmp::min(n, max);
                    let n = std::cmp::max(n, min);
                    self.v
                        .insert(key.to_string(), UsiOptionValue::spin(n, min, max));
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
                    return;
                }
            },
            UsiOptionValue::Check { .. } => {
                // "true" or "false" is ok. You can only use lowercase.
                match value.parse::<bool>() {
                    Ok(b) => {
                        self.v.insert(key.to_string(), UsiOptionValue::check(b));
                    }
                    Err(err) => {
                        println!("{:?}", err);
                        return;
                    }
                }
            }
            UsiOptionValue::Button => {}
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
        match &self.v[key] {
            UsiOptionValue::Spin { current, .. } => *current,
            _ => panic!("Error: illegal option name: {}", key),
        }
    }
    pub fn get_string(&self, key: &str) -> String {
        match &self.v[key] {
            UsiOptionValue::StringOption { current, .. } => current.clone(),
            _ => panic!("Error: illegal option name: {}", key),
        }
    }
    pub fn get_bool(&self, key: &str) -> bool {
        match &self.v[key] {
            UsiOptionValue::Check { current, .. } => *current,
            _ => panic!("Error: illegal option name: {}", key),
        }
    }
}
