use serde::{Deserialize, Serialize};
use std::{fs, io, path::{Path, PathBuf}};

/// Формат полей совпадает со старым data.json от Python-версии.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Item {
    pub name: String,
    pub price: f64,
    pub sale_comm: u32,
    pub withdraw_comm: u32,
    pub purchase: f64,
    pub premium: f64,
    pub on_card: f64,
    pub profit: f64,
}

fn round2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

/// (на карту, профит)
pub fn calc(price: f64, purchase: f64, premium: f64, sale: u32, withdraw: u32) -> (f64, f64) {
    let on_card = price * (1.0 - sale as f64 / 100.0) * (1.0 - withdraw as f64 / 100.0);
    (round2(on_card), round2(on_card - purchase - premium))
}

/// Число из поля ввода: пусто = 0, запятая = точка, отрицательные/NaN нельзя.
pub fn parse_num(s: &str) -> Option<f64> {
    let s = s.trim().replace(',', ".");
    if s.is_empty() {
        return Some(0.0);
    }
    s.parse::<f64>().ok().filter(|v| v.is_finite() && *v >= 0.0)
}

pub fn totals(items: &[Item]) -> (f64, f64) {
    let rev = items.iter().map(|i| i.price * (1.0 - i.sale_comm as f64 / 100.0)).sum();
    (rev, items.iter().map(|i| i.profit).sum())
}

/// Рядом с exe лежит data.json (портативный режим / старая версия) — берём его,
/// иначе %APPDATA%\PlayerokUniversity (в Program Files писать нельзя).
pub fn data_path() -> PathBuf {
    let exe_dir = std::env::current_exe().ok().and_then(|p| p.parent().map(Path::to_path_buf));
    if let Some(p) = exe_dir.as_ref().map(|d| d.join("data.json")).filter(|p| p.exists()) {
        return p;
    }
    let base = std::env::var_os("APPDATA")
        .or_else(|| std::env::var_os("XDG_DATA_HOME"))
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share")))
        .or(exe_dir)
        .unwrap_or_default();
    base.join("PlayerokUniversity").join("data.json")
}

pub fn load(path: &Path) -> Vec<Item> {
    let Ok(text) = fs::read_to_string(path) else { return vec![] };
    match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(_) => {
            // битый файл не затираем молча — откладываем копию
            let _ = fs::copy(path, path.with_extension("json.bak"));
            vec![]
        }
    }
}

/// Запись через временный файл, чтобы сбой посреди записи не убил базу.
pub fn save(path: &Path, items: &[Item]) -> io::Result<()> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, serde_json::to_string_pretty(items)?)?;
    fs::rename(tmp, path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_matches_python() {
        // 1000 * 0.9 * 0.94 = 846; профит 846 - 500 - 100 = 246
        assert_eq!(calc(1000.0, 500.0, 100.0, 10, 6), (846.0, 246.0));
        assert_eq!(calc(1000.0, 0.0, 0.0, 20, 10), (720.0, 720.0));
    }

    #[test]
    fn parse() {
        assert_eq!(parse_num(" 1,5 "), Some(1.5));
        assert_eq!(parse_num(""), Some(0.0));
        assert_eq!(parse_num("abc"), None);
        assert_eq!(parse_num("-3"), None);
        assert_eq!(parse_num("inf"), None);
    }

    #[test]
    fn reads_old_python_json() {
        let j = r#"[{"name":"Акк","price":1000.0,"sale_comm":10,"withdraw_comm":6,"purchase":500.0,"premium":100.0,"on_card":846.0,"profit":246.0}]"#;
        let v: Vec<Item> = serde_json::from_str(j).unwrap();
        assert_eq!(totals(&v), (900.0, 246.0));
    }
}
