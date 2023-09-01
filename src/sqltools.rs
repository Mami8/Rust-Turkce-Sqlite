use super::diger::{TabloTürü, İşlemTürü};
use crate::diger::sql_sorgusu_oluştur;
use rusqlite::{params, Connection, Result};
use std::fmt::format;

pub fn veritabanı_aç(veritabanı_yolu: &str, geçici: bool) -> Connection {
    if geçici {
        Connection::open_in_memory().unwrap()
    } else {
        match Connection::open(veritabanı_yolu) {
            Ok(conn) => conn,
            Err(e) => panic!("{veritabanı_yolu} oluşturulamadı: {e}"),
        }
    }
}
pub fn tablo_oluştur(
    tablo_ismi: &str,
    tablo_türü: TabloTürü,
    bağlantı: &Connection,
) -> Result<usize> {
    let sorgu = match tablo_türü {
        TabloTürü::UygulamaYolu => {
            sql_sorgusu_oluştur(tablo_ismi, tablo_türü, İşlemTürü::TabloOluşturma, vec![])
        }
        _ => panic!("Belirtilen tablo türü için uygun match kolu yok."),
    };
    bağlantı.execute(&sorgu, ())
}

pub fn tablo_isimleri_al(baglanti: &Connection) -> Result<Vec<String>, rusqlite::Error> {
    // SQLite veritabanında mevcut olan tablo isimlerini almak için bir SQL sorgusu hazırlayın
    let sql = "SELECT * FROM sqlite_master WHERE type='table'";

    // SQL sorgusunu çalıştırın ve sonuçları bir vektörde toplayın
    let mut statement = baglanti.prepare(sql)?;
    let mut tablo_isimleri = Vec::new();

    let mut rows = statement.query(())?;
    while let Some(row) = rows.next()? {
        let tablo_adi: String = row.get(1)?;
        tablo_isimleri.push(tablo_adi);
    }

    Ok(tablo_isimleri)
}

pub fn bilgi_ekle(
    tablo_ismi: &str,
    bağlantı: &Connection,
    bilgiler: Vec<String>,
) -> Result<(), rusqlite::Error> {
    let sql = format(format_args!("INSERT INTO {} VALUES (", tablo_ismi));
    for veri in bilgiler {
        println!("{:?}", veri);
    }

    Result::Ok(())
}
