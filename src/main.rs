// #[derive(Debug)]
#![allow(unused)]
extern crate rand;
extern crate regex;

#[macro_use]
extern crate prettytable;

use chrono::{Duration, Utc};
use prettytable::{Cell, Row, Table};
use std::any::type_name;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;

mod format;

static mut FORMAT_EXCEL: bool = false;

#[derive(Clone, Debug)]
struct Cuz {
    cuz: i32,
    start_date: String,
    end_date: String,
}
impl Cuz {
    fn add(cuz: i32, start_date: &str, end_date: &str) -> Cuz {
        Cuz {
            cuz: cuz,
            start_date: start_date.to_string(),
            end_date: end_date.to_string(),
        }
    }
}

struct User {
    name: String,
    cuzler: Vec<Cuz>,
}
impl User {
    fn add(name: &str, cuzler: Vec<Cuz>) -> User {
        User {
            name: name.to_string(),
            cuzler: cuzler,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    unsafe {
        if args.len() > 0 {
            let file = &args[1];
            let format_arg = &args[2];
            if format_arg == "excel" {
                FORMAT_EXCEL = true
            } else {
                FORMAT_EXCEL = false
            }
            read_user_list(file);
        }
        match FORMAT_EXCEL {
            true => println!("Output format: EXCEL"),
            false => println!("Output format: CSV"),
        }
    }
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn write_table(list: Vec<User>) -> std::io::Result<()> {
    let dates: Vec<_> = (1..=30)
        .map(|x| {
            let start_date = Utc::now() + Duration::weeks(x as i64);
            let end_date = start_date + Duration::days(6);

            let (s_year, mut s_month, s_day) = (
                start_date.format("%Y"),
                start_date.format("%B"),
                start_date.format("%d"),
            );
            let (e_year, mut e_month, e_day) = (
                end_date.format("%Y"),
                end_date.format("%B"),
                end_date.format("%d"),
            );

            let formatted = format::make(
                (s_year).to_string(),
                (s_month).to_string(),
                (s_day).to_string(),
                (e_year).to_string(),
                (e_month).to_string(),
                (e_day).to_string(),
            );
            formatted
        })
        .collect();
    // Create the table
    let mut table = Table::new();

    // Add header row:
    let mut header = vec![];
    header.push(Cell::new("Sıra"));
    header.push(Cell::new("Adı Soyadı"));
    for date in dates.iter() {
        header.push(Cell::new(&date.to_string()));
    }
    table.add_row(Row::new(header));

    for (_i, user) in list.iter().enumerate() {
        let mut row = vec![];
        row.push(Cell::new(&(_i + 1).to_string()));
        row.push(Cell::new(&user.name));
        for cuz in user.cuzler.iter() {
            row.push(Cell::new(&cuz.cuz.to_string()))
        }
        table.add_row(Row::new(row));
    }

    // Print the table to stdout
    // table.printstd();

    // Write table
    let first_len = dates[0].len();
    let first_year = &dates[0][first_len - 4..first_len];
    let last_len = dates[29].len();
    let last_year = &dates[29][last_len - 4..last_len];
    let timestamp = Utc::now().timestamp();
    println!("{}", timestamp);
    let name = format!(
        "hatim_listesi-[{}-{}]-{}.txt",
        first_year, last_year, timestamp
    );

    let out = File::create(&name)?;
    table.to_csv(out)?;

    unsafe {
        if FORMAT_EXCEL == true {
            set_format(&name);
        }
    }
    Ok(())
}

fn read_user_list(file: &str) -> std::io::Result<()> {
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    println!("UYARI: Argüman olarak verdiğiniz listedeki sıralama geçersiz sayılır ve her defasında karışıltırılır!");
    let mut open = File::open(file).expect("Hata! Dosya bulunamadı!");
    let reader = BufReader::new(open);
    let mut list = vec![];
    let mut over_size = false;
    for (_i, line) in reader.lines().enumerate() {
        if _i < 30 {
            list.push(line?)
        } else {
            over_size = true
        }
    }
    if over_size {
        println!(
            "İsim listesinde 30'dan fazla kayıt mevcut. 30'dan sonrası listeye dahil edilmemiştir."
        );
        println!("30 kişiden fazla listeniz varsa bunların listesini ikinci bir parti olarak alabilirsiniz");
        println!("Örneğin: liste1.txt (30 kişi), liste2.txt (10 Kişi)");
    }
    list.shuffle(&mut thread_rng());

    let mut start = 1;
    let mut hatim_list: Vec<User> = vec![];
    for (_i, user) in list.iter().enumerate() {
        let mut row = User::add(user, generate_weeks(1, (_i + 1) as i32));
        hatim_list.push(row);
        start += 1
    }
    write_table(hatim_list);
    Ok(())
}

fn generate_weeks(week: i32, start: i32) -> std::vec::Vec<Cuz> {
    // maximum cüz
    let max: i32 = 30;
    // ilk parti alacağı cüzler
    let next: i32 = max - start;
    // sonraki parti alacağı cüzler
    let balance: i32 = if (max - next) > 0 { max - next } else { 0 };

    fn generator(start: i32, appends: i32) -> std::vec::Vec<Cuz> {
        let mut temp: Vec<Cuz> = vec![];
        let mut week = 1;
        for (_i, counter) in (start..appends + 1).enumerate() {
            let start_date = Utc::now() + Duration::weeks(week);
            let end_date = start_date + Duration::days(6);
            let s_date = start_date.format("%Y-%m-%d").to_string();
            let e_date = end_date.format("%Y-%m-%d").to_string();
            let cuz = Cuz::add(counter, &s_date, &e_date);

            temp.push(cuz);
            week += 1;
        }
        temp
    }

    let mut list: Vec<Cuz> = vec![];
    let mut kalan: Vec<Cuz> = vec![];

    list = generator(start, max);

    if balance > 0 {
        kalan = generator(1, balance - 1);
    }
    if (kalan.len() > 0) {
        list.extend(kalan.iter().cloned());
    }
    list
}

fn set_format(file_name: &str) -> std::io::Result<()> {
    use regex::Regex;

    let mut open = File::open(&file_name).expect("Hata! Dosya bulunamadı!");
    let mut contents = String::new();
    open.read_to_string(&mut contents)
        .expect("Hata! Dosya okunamadı!");
    let re = Regex::new(",").unwrap();
    let replaced = re.replace_all(&contents, ";");
    fs::write(&file_name, String::from(replaced))?;
    Ok(())
}
