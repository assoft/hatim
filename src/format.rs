use std::collections::HashMap;

pub fn make(s_year: String, s_month: String, s_day: String, e_year: String, e_month: String, e_day: String) -> String {
    let format;
    let s_month_tr = get_month(&s_month.to_string());
    let e_month_tr = get_month(&e_month.to_string());
    if s_month == e_month {
        if s_year == e_year {
            format = format!("{}-{} {} {}", s_day, e_day, s_month_tr, s_year) // 10-12 Haziran 2020
        } else {
            format = format!("{} {} {} - {} {} {}", s_day, s_month_tr, s_year, e_day, e_month_tr, e_year) // 30 Aralık 2020 - 06 Ocak 2021
        }
    } else {
        if s_year == e_year {
            format = format!("{} {} - {} {} {}", s_day, s_month_tr, e_day, e_month_tr, s_year) // 28 Haziran - 03 Ağustos 2020
        } else {
            format = format!("{} {} {} - {} {} {}", s_day, s_month_tr, s_year, e_day, e_month_tr, e_year) // 30 Aralık 2020 - 06 Ocak 2021
        }
    }
    format
}

pub fn get_month (_month: &str) -> &'static str {
    let mut months = HashMap::new();
    months.insert("January", "Ocak");
    months.insert("February", "Şubat");
    months.insert("March", "Mart");
    months.insert("April", "Nisan");
    months.insert("May", "Mayıs");
    months.insert("June", "Haziran");
    months.insert("July", "Temmuz");
    months.insert("August", "Ağustos");
    months.insert("September", "Eylül");
    months.insert("October", "Ekim");
    months.insert("November", "Kasım");
    months.insert("December", "Aralık");
    let month = match &months.get(_month) {
        Some(val) => val,
        None => "",
    };
    month
}