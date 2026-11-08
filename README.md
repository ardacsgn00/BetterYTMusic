<div align="center">

<img src="./app-icon.png" alt="BetterYTMusic Logo" width="96" height="96" />

# BetterYTMusic

**Hafif, hızlı ve kişiselleştirilebilir masaüstü YouTube Music istemcisi**  
*Tauri 2 · Rust (`rodio` + `symphonia`) · Svelte 5 · Innertube API*

[![Version](https://img.shields.io/badge/version-v0.12.0-ff4e45?style=for-the-badge)](https://github.com/ardacsgn00/betterytmusic)
[![Platform](https://img.shields.io/badge/platform-Windows%2010%2F11-3a86ff?style=for-the-badge)](https://github.com/ardacsgn00/betterytmusic)
[![Built with Tauri](https://img.shields.io/badge/Tauri_2-Rust_%2B_Svelte_5-06d6a0?style=for-the-badge)](https://tauri.app)
[![Developer](https://img.shields.io/badge/Geli%C5%9Ftirici-Arda_Co%C5%9Fgun-8338ec?style=for-the-badge)](https://github.com/ardacsgn00)

</div>

---

## ✨ Öne Çıkan Özellikler

- **🦀 Doğrudan Rust Ses Motoru (`rodio` + `symphonia`):** Tarayıcı sekmesi veya ağır Electron katmanı olmadan, düşük RAM ve CPU kullanımıyla yerel AAC/M4A ses çözme ve paket düzeyinde milisaniyelik ileri-geri sarma (seek).
- **🎵 YouTube Music & Google Hesabı Entegrasyonu:**
  - Google hesabınla güvenli giriş yapıp kişisel **Süpermix'im**, **Arşivden Şarkılar**, kişisel çalma listelerin ve **Beğenilen Şarkılar** kitaplığına anında erişim.
  - Şarkı veya liste seçildiğinde otomatik **Radyo / Kuyruk** oluşturma ve **Karışık Çal (Shuffle)** / **Tekrar (Repeat)** modları.
- **🎤 Senkronize Şarkı Sözleri (Lyrics):**
  - Saniyeye duyarlı (`LRC`) otomatik kayan şarkı sözleri ve herhangi bir söz satırına tıklayarak şarkıyı doğrudan o saniyeye sarabilme.
  - Geniş ekranlarda **Kapak | Şarkı Sözleri | Sıradaki** şeklinde 3 sütunlu yan yana görünüm desteği.
- **🖥️ Tam Masaüstü & Windows Entegrasyonu:**
  - **Görev Çubuğu Önizleme Kontrolleri (`ITaskbarList3`):** Görev çubuğunda uygulama simgesinin üzerine gelindiğinde Spotify benzeri **Önceki / Oynat-Duraklat / Sonraki** düğmeleri.
  - **Sistem Medya Kontrolleri (SMTC):** Klavyedeki medya tuşları (`Play/Pause`, `Next`, `Prev`) ve Windows ses kaplaması ile tam uyum.
  - **Sistem Tepsisi (System Tray) & Otomatik Başlatma:** Pencere kapatıldığında sistem tepsisinde arka planda çalmaya devam etme, Windows açılışında otomatik çalışma ve küçültülmüş başlatma seçenekleri.
- **🎨 Dokunsal (Tactile) Arayüz & Geniş Kişiselleştirme:**
  - **5 Tema Modu:** Koyu, Gece Siyahı (OLED), Kuzey Mavisi, Açık ve Sistem.
  - **10 Vurgu Rengi + Özel Renk Seçici**, **5 Yazı Tipi Ailesi**, metin ölçeği, liste yoğunluğu, köşe yuvarlaklığı ve ayarlanabilir **3B Dokunsal Alt Kenar (`0px` – `6px`)**.
  - Sol üstteki uygulama başlığını istediğin metinle değiştirebilme ve uygulama içi **GitHub Güncelleme Denetleyicisi**.

---

## 🚀 Hızlı Kullanım (Tek Dosya `.exe`)

Herhangi bir Rust veya Node.js kurulumuna gerek kalmadan doğrudan depodaki **[`BetterYTMusic.exe`](./BetterYTMusic.exe)** dosyasını indirip çalıştırabilirsin. Tüm ses motoru ve arayüz tek `.exe` (~8 MB) içerisine gömülüdür.

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
