// #[derive(Debug)]
#![allow(unused)]
extern crate rand;
#[macro_use] extern crate prettytable;

use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::any::type_name;
use chrono::{Duration, Utc};
use prettytable::{Table, Row, Cell};

mod format;

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
            end_date: end_date.to_string()
        }
    }
}

struct User {
    name: String,
    cuzler: Vec<Cuz>
}
impl User {
    fn add(name: &str, cuzler: Vec<Cuz>) -> User {
        User {
            name: name.to_string(),
            cuzler: cuzler
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 0 {
        let file = &args[1];
        read_user_list(file);
    }
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn write_table(list: Vec<User>) -> std::io::Result<()> {
    let dates : Vec<_> = (1 ..=30).map(|x| { 
        let start_date = Utc::now() + Duration::weeks(x as i64);
        let end_date = start_date + Duration::days(6);

        let (s_year, mut s_month, s_day) = 
            (start_date.format("%Y"), start_date.format("%B"), start_date.format("%d"));
        let (e_year, mut e_month, e_day) = 
            (end_date.format("%Y"), end_date.format("%B"), end_date.format("%d"));

            let formatted = format::make(
                (s_year).to_string(), (s_month).to_string(), (s_day).to_string(),
                (e_year).to_string(), (e_month).to_string(), (e_day).to_string(),
            );
        formatted
    }).collect();
    
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
    let first_year = &dates[0][first_len - 4 .. first_len]; 
    let last_len = dates[29].len();
    let last_year = &dates[29][last_len - 4 .. last_len];
    let name = format!("hatim_listesi-[{}-{}].txt", first_year, last_year);

    let out = File::create(name)?;
    table.to_csv(out)?;

    Ok(())
}

fn read_user_list(file: &str) -> std::io::Result<()> {
    use rand::thread_rng;
    use rand::seq::SliceRandom;

    let mut open = File::open(file).expect("Hata! Dosya bulunamadı!");
    let reader = BufReader::new(open);
    let mut list = vec![];
    for line in reader.lines() {
        list.push(line?)
    }
    list.shuffle(&mut thread_rng());

    let mut start = 1;
    let mut hatim_list : Vec<User> = vec![];
    for (_i, user) in list.iter().enumerate() {
        // println!("{}. {}", start, user);
        let mut row = User::add(user, generate_weeks(1, (_i + 1) as i32));
        hatim_list.push(row);
        // generate_weeks(1, start);
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
        let mut temp : Vec<Cuz> = vec![];
        let mut week = 1;
        for (_i, counter) in (start..appends + 1).enumerate() {
            let start_date = Utc::now() + Duration::weeks(week);
            let end_date = start_date + Duration::days(6);
            let s_date = start_date.format("%Y-%m-%d").to_string();
            let e_date = end_date.format("%Y-%m-%d").to_string();
            
            let cuz = Cuz::add(counter, &s_date, &e_date);

            temp.push(cuz);
            
            // DEBUG
            // println!(
            //     "Cüz: {}, {} - {}",
            //     counter,
            //     start_date.format("%Y-%m-%d"),
            //     end_date.format("%Y-%m-%d")
            // );
            week += 1;
        }
        temp
    }

    let mut list : Vec<Cuz> = vec![];
    let mut kalan : Vec<Cuz> = vec![];

    list = generator(start, max);

    if balance > 0 {
        // FIXME: Balance'a kalan günler için tarihler önceki listeden devam etmeli!
        kalan = generator(1, balance - 1);
    }
    if(kalan.len() > 0) {
        list.extend(kalan.iter().cloned());
    }
    list
}