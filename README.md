<div align="center">

<img src="./app-icon.png" alt="BetterYTMusic Logo" width="96" height="96" />

# BetterYTMusic

**Hafif, hızlı ve kişiselleştirilebilir masaüstü YouTube Music istemcisi**  
*Tauri 2 · Rust (`rodio` + `symphonia`) · Svelte 5 · Innertube API · Windows & macOS*

[![Version](https://img.shields.io/badge/version-v0.33.0-ff4e45?style=for-the-badge)](https://github.com/ardacsgn00/betterytmusic)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS-3a86ff?style=for-the-badge)](https://github.com/ardacsgn00/betterytmusic)
[![Built with Tauri](https://img.shields.io/badge/Tauri_2-Rust_%2B_Svelte_5-06d6a0?style=for-the-badge)](https://tauri.app)
[![Developer](https://img.shields.io/badge/Geli%C5%9Ftirici-Arda_Co%C5%9Fgun-8338ec?style=for-the-badge)](https://github.com/ardacsgn00)

</div>

---

## ✨ Öne Çıkan Özellikler

- **🦀 Doğrudan Rust Ses Motoru (`rodio` + `symphonia`):** Tarayıcı sekmesi veya ağır Electron katmanı olmadan, düşük RAM ve CPU kullanımıyla yerel AAC/M4A ses çözme ve paket düzeyinde milisaniyelik ileri-geri sarma (seek).
  - **Sıradaki Parça Ön Yüklemesi (Zero-Latency Prefetch):** Çalma sırasında sıradaki şarkıyı arka planda bellek içi MRU önbelleğine alarak şarkı geçişlerini anlık hale getirir.
  - **Ses Seviyesi Dengeleme (Loudness Normalization):** YouTube Music ses yüksekliği (`loudnessDb`) verisini kullanarak parçalar arasındaki ani ses farklarını dengeler (Ayarlardan açılıp kapatılabilir).
  - **Kaldığın Yerden Devam Et (Session Resume):** Kuyruğu, aktif parçayı, milisaniyelik konumu ve çalma ayarlarını yerel olarak saklar; sonraki açılışta aynı noktadan devam etmeni sağlar.
- **🖥️ Tam Ekran Dinleme Modu:** `F` veya `F11` kısayolu ya da oynatıcı çubuğundaki tam ekran butonuyla açılan, büyük kapak görseli, senkronize şarkı sözleri / sıradaki görünümü ve dokunsal kontroller sunan odaklanmış tam ekran deneyimi.
- **🎵 YouTube Music & Google Hesabı Entegrasyonu:**
  - Google hesabınla güvenli giriş yapıp kişisel **Süpermix'im**, **Arşivden Şarkılar**, kişisel çalma listelerin ve **Beğenilen Şarkılar** kitaplığına anında erişim.
  - **Şarkı & Sanatçı Radyosu:** Herhangi bir şarkı veya sanatçıdan tek tıkla benzer müzik akışı başlatma.
  - **Sonsuz Otomatik Çalma (Autoplay):** Sıradaki parçalar bittiğinde YouTube Music üzerinden proaktif kuyruk beslemesiyle kesintisiz müzik dinleme.
  - **Ruh Hali & Tür Filtreleri (Moods & Genres):** Ana sayfada odaklanma, enerji, rahatlama, parti vb. tür ve ruh haline özel çalma listelerini filtreleme ve keşfetme.
  - **Sanatçı Sayfası & Diskografi:** Sanatçı biyografisi, popüler şarkılar, albümler, single'lar ve benzer sanatçılar arasında gezinme.
  - Otomatik **Radyo / Kuyruk** oluşturma ve **Karışık Çal (Shuffle)** / **Tekrar (Repeat)** modları.
- **🎤 Senkronize Şarkı Sözleri (Lyrics):**
  - Saniyeye duyarlı (`LRC`) otomatik kayan şarkı sözleri ve herhangi bir söz satırına tıklayarak şarkıyı doğrudan o saniyeye sarabilme.
  - Geniş ekranlarda **Kapak | Şarkı Sözleri | Sıradaki** şeklinde 3 sütunlu yan yana görünüm desteği.
- **🖥️ Çapraz Platform Masaüstü & Discord Entegrasyonu:**
  - **Discord Zengin Varlık (Rich Presence):** Discord profilinde anlık dinleme durumu; şarkı adı, sanatçı, albüm kapağı, canlı akan süre çubuğu ve arkadaşlarının şarkıyı tek tıkla dinleyebileceği "YouTube Music'te Aç" butonu.
  - **Windows Entegrasyonu:** Görev çubuğunda (`ITaskbarList3`) Spotify benzeri **Önceki / Oynat-Duraklat / Sonraki** küçük resim butonları ve Windows ses kaplaması (SMTC).
  - **macOS Entegrasyonu:** Apple Silicon (M-serisi) ve Intel Mac desteği, yerel macOS `MPNowPlayingInfoCenter` medya tuşları, macOS `LaunchAgents` açılış servisi ve yerel WebKit motoruyla ultra düşük bellek tüketimi (~60-80 MB RAM).
  - **Sistem Tepsisi (Tray) & Otomatik Başlatma:** Kapatıldığında arka planda çalma ve işletim sistemi başlangıcında otomatik başlatma.
- **🎨 Dokunsal (Tactile) Arayüz & Geniş Kişiselleştirme:**
  - **12 Tema Modu:** Koyu, Gece Siyahı (OLED), Kuzey Mavisi, Çam Ormanı, Velvet Erik, Kahve Kavurma, Akustik Şarap, Buzul Nord, Açık, Parşömen Sepya, Buzlu Mint ve Sistem.
  - **10 Vurgu Rengi + Özel Renk Seçici**, **5 Yazı Tipi Ailesi**, metin ölçeği, liste yoğunluğu, köşe yuvarlaklığı ve ayarlanabilir **3B Dokunsal Alt Kenar (`0px` – `6px`)**.
  - Sol üstteki uygulama başlığını istediğin metinle değiştirebilme ve uygulama içi **GitHub Güncelleme Denetleyicisi**.

---

## 🚀 Hızlı Kullanım & İndirme

### Windows (Tek Dosya `.exe`)
Herhangi bir kurulum gerektirmeden doğrudan depodaki **[`BetterYTMusic.exe`](./BetterYTMusic.exe)** dosyasını indirip çalıştırabilirsin. Tüm ses motoru ve arayüz tek `.exe` (~9 MB) içerisine gömülüdür.

### macOS (Apple Silicon & Intel DMG)
macOS sürümleri GitHub Actions üzerinde otomatik olarak hem **Apple Silicon (M1/M2/M3/M4)** hem de **Intel Mac'ler (x86_64)** için ayrı ayrı `.dmg` paketleri olarak derlenmektedir.

1. **İndirme:**
   - [GitHub Actions - Build macOS DMG](https://github.com/ardacsgn00/BetterYTMusic/actions/workflows/build-mac.yml) sayfasına gidin.
   - En son başarılı çalışan iş akışına tıklayıp sayfanın altındaki **Artifacts** bölümünden cihazınıza uygun paketi indirin:
     - **Intel Mac'ler için:** `BetterYTMusic-macOS-x86_64`
     - **Apple Silicon için:** `BetterYTMusic-macOS-aarch64`
   - *Not: GitHub Artifacts dosyalarını indirebilmek için GitHub hesabınıza giriş yapmış olmanız gerekir. İndirdiğiniz zip arşivini açtığınızda içinden `.dmg` kurulum dosyası çıkacaktır.*
2. **macOS İlk Açılış İzni:**
   - Apple Geliştirici sertifikası bulunmayan açık kaynak uygulamalarda olduğu gibi, ilk açılışta *"Geliştirici doğrulanamadı"* uyarısı alırsanız: uygulamaya **Sağ Tık (veya Control + Sol Tık) -> Aç** diyerek tek seferlik onay verebilirsiniz.

---

## 🛠️ Kaynak Koddan Derleme

### Gereksinimler
- **Node.js** 20+
- **Rust** 1.77+ (`stable-x86_64-pc-windows-msvc`)
- **Visual Studio C++ Build Tools** & Windows 10/11 SDK

### Adımlar

```bash
# 1. Depoyu klonla
git clone https://github.com/ardacsgn00/betterytmusic.git
cd betterytmusic

# 2. Bağımlılıkları yükle
npm install

# 3. Geliştirme modunda çalıştır
npm run tauri:dev

# 4. Üretim (Release) sürümünü derle
npm run tauri:build
```

---

## ⌨️ Klavye Kısayolları

| Kısayol | İşlev |
| :--- | :--- |
| `Space` | Oynat / Duraklat |
| `←` / `→` | 5 saniye geri / ileri sar |
| `Ctrl + ←` / `Ctrl + →` | Önceki / Sonraki şarkı |
| `Medya Tuşları` | Arka planda bile küresel Oynat / Duraklat / İleri / Geri |

---

## 👤 Geliştirici

- **Arda Coşgun** — [@ardacsgn00](https://github.com/ardacsgn00)
- **Proje Adresi:** [https://github.com/ardacsgn00/betterytmusic](https://github.com/ardacsgn00/betterytmusic)

---

## ⚠️ Legal Disclaimer

**BetterYTMusic** is an **unofficial, open-source third-party desktop client**. 

- **No Affiliation:** This project is independent and is **not** affiliated with, authorized, maintained, sponsored, or endorsed by YouTube, Google LLC, or any of their affiliates or subsidiaries.
- **Trademarks:** All product names, logos, brands, trademarks, and registered trademarks mentioned or shown within this application belong to their respective owners. Their use does not imply any affiliation with or endorsement by them.
- **No Media Hosting / DRM Bypass:** BetterYTMusic does **not** host, store, re-upload, or distribute any audio or video content, nor does it attempt to bypass any Digital Rights Management (DRM) protections. All media streams are accessed dynamically directly from YouTube servers for online playback only.
- **Downloading & Ripping:** This application does **not** provide any functionality for downloading, converting, or saving offline copies of copyrighted media content.
- **Terms of Service:** Users are solely responsible for ensuring that their use of this software complies with YouTube's Terms of Service and local copyright laws in their respective jurisdictions.
