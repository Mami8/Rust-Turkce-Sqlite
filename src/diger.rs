use std::{
    fmt::{self, format},
    str::FromStr,
};

use rusqlite::Connection;

// ENUM TANIMLAMALARI

pub enum TabloTürü {
    UygulamaYolu,
}
pub enum İşlemTürü {
    TabloOluşturma,
    Giriş,
}
pub fn sql_sorgusu_oluştur(
    tablo_ismi: &str,
    tablo_türü: TabloTürü,
    işlem_türü: İşlemTürü,
    bilgiler: Vec<String>,
) -> String {
    match işlem_türü {
        İşlemTürü::TabloOluşturma => match tablo_türü {
            TabloTürü::UygulamaYolu => fmt::format(format_args!(
                "CREATE TABLE IF NOT EXISTS {} 
                (
                    id INTEGER PRIMARY KEY NOT NULL,
            isim varchar(255),
            yol varchar(255)
        )",
                tablo_ismi
            )),
        },
        İşlemTürü::Giriş => match tablo_türü {
            TabloTürü::UygulamaYolu => {
                let mut sorgu = format(format_args!(
                    "INSERT INTO {} (isim, yol) VALUES (\"{}\",\"{}\")",
                    tablo_ismi, bilgiler[0], bilgiler[1]
                ));
                return sorgu;
            }
        },
    }
}

// STRUCT TANIMLAMALARI

#[derive(Debug)]
pub struct ExeData {
    isim: String,
    yol: String,
}
impl ExeData {
    pub fn new(isim: &str, yol: &str) -> ExeData {
        ExeData {
            isim: isim.to_string(),
            yol: yol.to_string(),
        }
    }
}

impl DatabasePushable for ExeData {
    fn database_push(
        &self,
        bağlantı: &Connection,
        tablo_ismi: &str,
    ) -> Result<(), rusqlite::Error> {
        sql_sorgusu_oluştur(
            tablo_ismi,
            TabloTürü::UygulamaYolu,
            İşlemTürü::Giriş,
            vec![self.isim.clone(), self.yol.clone()],
        );
        Ok(())
    }
}

// TRAIT TANIMLAMALARI

pub trait DatabasePushable {
    fn database_push(
        &self, bağlantı: &Connection, tablo_ismi: &str
    ) -> Result<(), rusqlite::Error>;
}
