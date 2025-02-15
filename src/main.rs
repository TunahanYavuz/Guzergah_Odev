use std::f64::consts::E;
#[derive(Debug)]
struct Guzergah{
    isim:String,
    istasyon_sayisi:f64,
    mesafe:f64,
    zaman:f64,
    maliyet:f64,
    sosyal_fayda:f64,
    cevresel_etki:f64,
    altyapi:f64,
    nufus_yogunlugu:f64
}

impl Guzergah {
    fn puan(&self, agirliklar:&[f64]) -> f64{
        self.istasyon_sayisi * agirliklar[0] + self.mesafe * agirliklar[1] +
            self.zaman * agirliklar[2] + self.maliyet * agirliklar[3] +
            self.sosyal_fayda * agirliklar[4] + self.cevresel_etki * agirliklar[5] +
            self.altyapi * agirliklar[6] + self.nufus_yogunlugu * agirliklar[7]
    }
}
fn softmax(degerler:&[f64]) ->Vec<f64>{
    let ustel : Vec<f64> = degerler.iter().map(|&x| E.powf(x)).collect();
    let toplam : f64 = ustel.iter().sum();
    ustel.iter().map(|x| x/toplam).collect()
}
fn main() {
    let guzergahlar = vec![
        Guzergah{
            isim: String::from("Kampüs Yurt"),
            istasyon_sayisi: 2.0,
            mesafe: 3.9,
            zaman: 9.0,
            maliyet: 20.0,
            sosyal_fayda: 80.0,
            cevresel_etki: 10.0,
            altyapi: 30.0,
            nufus_yogunlugu: 80.0,
        },
        Guzergah{
            isim: String::from("Kampüs Merkez"),
            istasyon_sayisi: 7.0,
            mesafe: 11.0,
            zaman: 25.0,
            maliyet: 20.0,
            sosyal_fayda: 90.0,
            cevresel_etki: 20.0,
            altyapi: 25.0,
            nufus_yogunlugu: 85.0,
        },
        Guzergah{
            isim: String::from("Yurt Merkez"),
            istasyon_sayisi: 6.0,
            mesafe: 7.9,
            zaman: 16.0,
            maliyet: 20.0,
            sosyal_fayda: 80.0,
            cevresel_etki: 10.0,
            altyapi: 25.0,
            nufus_yogunlugu: 90.0,
        },
        Guzergah{
            isim: String::from("Otogar Tokiler"),
            istasyon_sayisi: 6.0,
            mesafe: 3.4,
            zaman: 15.0,
            maliyet: 20.0,
            sosyal_fayda: 80.0,
            cevresel_etki: 20.0,
            altyapi: 20.0,
            nufus_yogunlugu: 80.0,
        },
    ];

    let agirliklar = vec![0.15, -0.1, -0.15, -0.25, 0.3, -0.05, 0.2, 0.25];
    let puanlar: Vec<f64> = guzergahlar.iter().map(|x| x.puan(&agirliklar)).collect();
    let agirliklar = softmax(&puanlar);
    println!("Güzergahların toplu taşıma ağırlıkları");
    for (i, guzergah) in guzergahlar.iter().enumerate(){
        println!("{}: {:.2}%", guzergah.isim, agirliklar[i]*100f64);
    }
}
