# Güzergah Değerlendirme Programı

Bu Rust programı, farklı toplu taşıma güzergahlarının değerlendirilmesini ve softmax fonksiyonu kullanarak ağırlıkların hesaplanmasını sağlar.

## Özellikler
- Farklı güzergahlar tanımlanabilir.
- Güzergahların istasyon sayısı, mesafe, zaman, maliyet, sosyal fayda gibi kriterlere göre puanlanması sağlanır.
- Softmax fonksiyonu ile güzergahların normalize edilmiş ağırlıkları hesaplanır.
- Sonuçlar ekrana yazdırılır.

## Kullanım

### Bağımlılıklar
Program Rust dilinde yazılmıştır ve herhangi bir harici kütüphane gerektirmez.

### Kodu Anlama
- `Guzergah` yapısı, her güzergah için değişkenleri tutar.
- `puan()` fonksiyonu, verilen ağırlıklara göre bir güzergah puanı hesaplar.
- `softmax()` fonksiyonu, güzergah puanlarını normalize ederek ağırlık yüzdelerini belirler.
- `main()` fonksiyonunda farklı güzergahlar tanımlanır, puanlar hesaplanır ve softmax fonksiyonu ile normalize edilerek ekrana yazdırılır.
