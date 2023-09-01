#![allow(unused, dead_code)]

use std::fs::File;
use std::io::{BufWriter, ErrorKind, Write};

mod diger;
mod sqltools;
use diger::{DatabasePushable, ExeData, TabloTürü, İşlemTürü};
use sqltools::*;

#[cfg(test)]
mod tests {
    use std::fmt::write;

    use super::*;

    #[test]
    fn structs_oluşturma() {
        let data1: ExeData = ExeData::new("güzel oyun", "D:/oyunlar/güzel_oyun/güzel_oyun.exe");
    }
    #[test]
    fn tablo_oluşturma() {
        let bağ = veritabanı_aç("deneme.db", true);
        match tablo_oluştur("deneme_tablo", TabloTürü::UygulamaYolu, &bağ) {
            Ok(val) => println!("Tablo oluşturma {val} koduyla başarılı oldu"),
            Err(e) => panic!("Tablo oluşturma başarısız!\n{e}"),
        };
    }
    #[test]
    fn database_push() {
        let bağlantı = veritabanı_aç("geçici.db", true);
        tablo_oluştur("deneme_tablosu", TabloTürü::UygulamaYolu, &bağlantı);
        ExeData::database_push(&ExeData::new("A", "A/A.exe"), &bağlantı, "deneme_tablosu");
    }
    #[test]
    fn tablo_isimleri_alma() {
        let bağ = veritabanı_aç("deneme.db", true);
        tablo_oluştur("deneme_tablo1", TabloTürü::UygulamaYolu, &bağ);
        tablo_oluştur("deneme_tablo2", TabloTürü::UygulamaYolu, &bağ);
        tablo_oluştur("deneme_tablo3", TabloTürü::UygulamaYolu, &bağ);
        tablo_oluştur("deneme_tablo4", TabloTürü::UygulamaYolu, &bağ);

        let isimler: Vec<String> = tablo_isimleri_al(&bağ).unwrap();

        assert_eq!(isimler.len(), 4);
        println!("\n{:?}\n", isimler);
    }
}
