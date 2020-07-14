# Hatim Listesi Oluşturma Uygulaması

Bu uygulama argüman olarak aldığı isim listesine 30'ar cüz atar ve csv formatında çıktı verir.

#### CSV Formatı Nedir?

> Virgülle ayrılmış değerler dosyası, değerleri ayırmak için virgül kullanan sınırlandırılmış bir metin dosyasıdır. Dosyanın her satırı bir veri kaydıdır. Her kayıt virgülle ayrılmış bir veya daha fazla alandan oluşur. Alan ayırıcısı olarak virgül kullanılması, bu dosya biçimi için adın kaynağıdır.

Uygulama isim listesini argüman olarak alır ve her bir isim için 30 haftalık cüz dağıtımı yapar. Uygulama 30 kişi üzerinden düşünüldü... böylece her bir kişi tüm cüzleri okumuş olur.

İsim listesi 2'den az, 30'dan fazla olmamalıdır. **_Argüman olarak verdiğiniz listeden 30 isim alınır._**

Eğer listenizde 30'dan daha fazla isim varsa bunları 30 kişilik listelere ayırın ve her biri için ayrı çıktı alın.

Örnek:

```bash
./hatim liste1.txt
./hatim liste2.txt
```

Beni ziyadesiyle yorduğundan hatta zıvanadan çıkardığından dolayı [RUST'a](https://www.rust-lang.org/tr/) teşekkür ederim :)
