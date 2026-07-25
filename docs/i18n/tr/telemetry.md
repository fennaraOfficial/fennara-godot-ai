<!-- fennara-i18n: locale=tr source=docs/telemetry.md sha256=925414507b4bfef9d6b7f207125bc0df953c8392e168f3ae20be78cf79c58d6a -->
<a id="anonymous-telemetry"></a>
# Anonim Telemetri

<!-- fennara-doc-nav:start -->
[Belgeler](README.md)

🌐 **Diller**

[English](../../telemetry.md) · [简体中文](../zh-CN/telemetry.md) · [Español](../es/telemetry.md) · [Português do Brasil](../pt-BR/telemetry.md) · [日本語](../ja/telemetry.md) · [한국어](../ko/telemetry.md) · [Русский](../ru/telemetry.md) · [Français](../fr/telemetry.md) · [Deutsch](../de/telemetry.md) · **Türkçe**

> ℹ️ Bu çeviri İngilizce kaynak temel alınarak yapay zeka tarafından yazılmıştır. Ana dil konuşurlarının incelemesi memnuniyetle karşılanır. [İngilizce kaynak](../../telemetry.md)
<!-- fennara-doc-nav:end -->

Fennara, UTC günü başına en fazla bir küçük anonim etkinlik olayı gönderir. Olay yalnızca uyumlu bir Godot editörü yerel daemon'a bağlandıktan sonra gönderilir. Bakım yapanların etkin kurulumları, desteklenen platform kullanımını ve sürüm benimsenmesini ölçmesine yardımcı olur.

Telemetri varsayılan olarak etkindir. Devre dışı bırakmak için **Chat Settings > Chat > Anonymous telemetry** bölümünü açın. Başsız ve otomatik ortamlar şunlardan birini ayarlayabilir:

```text
FENNARA_DISABLE_TELEMETRY=true
DO_NOT_TRACK=1
```

Ortam değişkeni kaydedilmiş kullanıcı arayüzü tercihinden önceliklidir. Telemetriyi kapatmak gelecekteki olayları durdurur ve yerel telemetri kimliğiyle son gönderim durumunu siler. Yeniden açılması, Godot bir sonraki kez bağlandığında yeni bir rastgele kimlik oluşturur.

<a id="event-contents"></a>
## Olay İçeriği

`fennara_active_installation` olayı yalnızca şunları içerir:

| Alan | Amaç |
| --- | --- |
| `schema_version` | Küçük telemetri veri yükü sözleşmesinin sürümü |
| `event` | Sabit olay adı |
| `installation_id` | Yerel olarak oluşturulan, donanım veya hesaplardan türetilmeyen rastgele UUID |
| `fennara_version` | Çalışan daemon sürümü |
| `godot_version` | `4.6.3` gibi sayısal Godot sürümü |
| `platform` | `windows`, `macos` veya `linux` |
| `architecture` | `x86_64` veya `aarch64` |

Fennara proje adlarını, proje yollarını, hesap bilgilerini, istemleri, sohbet mesajlarını, sağlayıcı anahtarlarını, model adlarını, araç adlarını, araç bağımsız değişkenlerini, araç sonuçlarını, günlükleri, ekran görüntülerini, sahne içeriklerini, dosya adlarını veya hata metnini göndermez.

<a id="storage-and-transport"></a>
## Depolama ve Aktarım

Daemon rastgele kimliğini ve son başarılı UTC gününü paylaşımlı Fennara uygulama verileri dizini altında depolar:

```text
Fennara/
  telemetry/
    state.json
```

Daemon olayı HTTPS üzerinden `https://fennara.io/api/telemetry` adresine gönderir. Alıcı tam bir alan izin listesini doğrular ve olayı PostHog'a iletmeden önce ham kurulum UUID'sini sunucu taraflı HMAC ile değiştirir. Bu olayda PostHog kişi profilleri ve IP coğrafi konumlandırması devre dışıdır.

Vercel alıcısı HTTPS isteğini işlerken zorunlu olarak normal ağ meta verilerini görür. Bu meta veriler PostHog olay veri yüküne kopyalanmaz.

<a id="delivery-behavior"></a>
## Teslim Davranışı

Telemetri, Godot araç çağrısı yollarının dışında çalışır:

- Sınırlı bir kuyruk etkinlik sinyallerini beklemeden kabul eder.
- Tek bir arka plan çalışanı, tek bir HTTP istemcisini yeniden kullanır.
- İsteklerin kısa bir zaman aşımı vardır.
- Dolu kuyruk, dosya sistemi sorunu, ağ hatası veya sunucu reddi sessizce kabul edilir ve hiçbir Fennara aracını başarısız kılmaz.
- UTC günü yalnızca sunucu bir olayı kabul ettikten sonra kaydedilir, böylece sonraki bir Godot bağlantısı başarısız teslimi yeniden deneyebilir.
- Kapanış kısa süre bekler, ardından daemon'u geciktirmek yerine telemetri çalışanını iptal eder.

Bir kurulum, kalıcılaştırılmış bir rastgele UUID'dir. Fennara'yı iki bilgisayarda kullanmak iki kurulum olarak sayılır. Fennara uygulama verilerini temizlemek veya telemetriyi devre dışı bırakıp sonra yeniden etkinleştirmek yeni bir kimlik oluşturur.

Aylık etkin kurulumlar, takvim ayı içinde en az bir `fennara_active_installation` olayı gönderen farklı anonim kurulum kimlikleri olarak sayılır.
