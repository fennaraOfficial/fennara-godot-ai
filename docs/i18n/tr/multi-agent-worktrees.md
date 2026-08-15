<!-- fennara-i18n: locale=tr source=docs/multi-agent-worktrees.md sha256=037571da5f2479b968f6b74e8994d0272042d4a26616c50af5f62f3d2a9bbcf9 -->
<a id="multiple-agents-and-godot-worktrees"></a>
# Birden Fazla Ajan ve Godot Worktree'leri

<!-- fennara-doc-nav:start -->
[English](../../multi-agent-worktrees.md) · [简体中文](../zh-CN/multi-agent-worktrees.md) · [Español](../es/multi-agent-worktrees.md) · [Português do Brasil](../pt-BR/multi-agent-worktrees.md) · [日本語](../ja/multi-agent-worktrees.md) · [한국어](../ko/multi-agent-worktrees.md) · [Русский](../ru/multi-agent-worktrees.md) · [Français](../fr/multi-agent-worktrees.md) · [Deutsch](../de/multi-agent-worktrees.md) · **Türkçe**

> ℹ️ Bu çeviri İngilizce kaynak temel alınarak yapay zeka tarafından yazılmıştır. Ana dil konuşurlarının incelemesi memnuniyetle karşılanır. [İngilizce kaynak](../../multi-agent-worktrees.md)
<!-- fennara-doc-nav:end -->

Bir ajanın hedef seçiminin diğerini yeniden yönlendirmesine izin vermeden aynı
makinede ayrı depolar veya worktree'ler üzerinde birden fazla kodlama ajanı
çalıştırın. Her proje kendi Fennara MCP işlemine ve bağlantısına sahip olur; tüm
projeler kullanıcı başına paylaşılan aynı daemon'ı kullanır.

```text
agent A -> MCP process A -- Project Binding A --\
                                                  shared daemon -> Godot editor A
agent B -> MCP process B -- Project Binding B --/              -> Godot editor B
```

Düzenleme, inceleme, doğrulama ve ekran görüntüsü çağrıları eşzamanlı
çalışabilir. Daemon tarafından yönetilen oyun çalıştırmaları, makine genelindeki
tek bir Çalışma Zamanı Yuvası üzerinden sıralı olarak yürütülür.

<a id="one-mcp-connection-per-project"></a>
## Proje Başına Bir MCP Bağlantısı

Bir MCP işlemi başladığında kararlı bir Proje Kökü seçer. Bu MCP Proje Bağlaması,
`project.godot` içeren dizinin kanonik dosya sistemi kimliğidir; proje adı veya
Godot işlem kimliği değildir.

Her depo veya worktree için ayrı bir MCP işlemi ve bağlantısı kullanın. Bir
bağlantı yalnızca bütün ajanlar bilinçli olarak aynı proje üzerinde çalıştığında
birden fazla ajana hizmet edebilir. Fennara araçları çağrı başına bir proje
seçicisi sunmaz; dolayısıyla model yanlışlıkla bir işlemi başka projeye
geçiremez.

Her proje ayrıca Fennara'nın etkin olduğu bağlı bir Godot editörüne ihtiyaç
duyar. Bir editör kapanıp yeni bir işlem kimliğiyle yeniden bağlanırsa mevcut MCP
işlemi aynı Proje Kökü yeniden bağlandığında yönlendirmeye devam eder.

<a id="how-a-process-chooses-its-project"></a>
## Bir İşlem Projesini Nasıl Seçer

MCP çalışma zamanı başlangıç çalışma dizinini yakalar ve bağlamasını şu sırayla
bir kez seçer:

1. `--project-path <path>` veya `--project-path=<path>`.
2. `FENNARA_PROJECT_PATH`.
3. Başlangıç dizininin `project.godot` içeren en yakın üst öğesi.
4. Otomatik keşif bir Godot projesi bulamazsa eski bağlı olmayan uyumluluk modu.

Komut satırı ve ortam yolları açık beyanlardır. Boş, erişilemeyen, eksik, dizin
olmayan, Godot projesi olmayan veya desteklenmeyen bir yol MCP sunucusunun
başlamasını engeller; başka bir projeye hiçbir zaman geri dönmez. Göreli yollar,
yakalanan başlangıç dizininden çözümlenir. MCP ana bilgisayarının başlatma dizini
belirsiz olduğunda mutlak yol tercih edin.

Fennara, ana bilgisayara özgü çalışma alanı değişkenlerini örtük olarak
kullanmaz. Bir MCP ana bilgisayarı kendi çalışma alanı değerini
`--project-path` veya `FENNARA_PROJECT_PATH` içine eşleyebilir.

<a id="configure-a-project-bound-connection"></a>
## Projeye Bağlı Bir Bağlantı Yapılandırma

`fennara mcp-setup` genel ve projeden bağımsız kalır. Bunu bir projenin içinde
çalıştırmak gelecekteki her MCP işlemini o projeye bağlamaz. Kararlı başlatıcı
yolunu koruyun, ardından bağlama eklemek için MCP ana bilgisayarının proje veya
çalışma alanı yapılandırmasını kullanın.

JSON biçimli yapılandırma için:

```json
{
  "mcpServers": {
    "fennara": {
      "command": "/absolute/path/to/fennara-mcp",
      "args": ["--project-path", "/absolute/path/to/worktree-a"],
      "env": {}
    }
  }
}
```

Ya da ortamı kullanın:

```json
{
  "mcpServers": {
    "fennara": {
      "command": "/absolute/path/to/fennara-mcp",
      "args": [],
      "env": {
        "FENNARA_PROJECT_PATH": "/absolute/path/to/worktree-a"
      }
    }
  }
}
```

Codex biçimli TOML için:

```toml
[mcp_servers.fennara]
command = "/absolute/path/to/fennara-mcp"
args = ["--project-path", "/absolute/path/to/worktree-a"]
startup_timeout_sec = 30
tool_timeout_sec = 300
```

Sonraki ajanı kendi proje/çalışma alanı yapılandırmasında
`/absolute/path/to/worktree-b` ile yapılandırın. Bir ana bilgisayar her proje
dizininden güvenilir biçimde ayrı bir MCP işlemi başlatıyorsa üst öğe keşfi açık
bir yol olmadan da aynı bağlamayı sağlayabilir.

<a id="mcp-host-boundaries"></a>
## MCP Ana Bilgisayarı Sınırları

Projeye özel yapılandırma ve başlangıç dizini davranışı ana bilgisayara göre
değişir:

- VS Code tek klasörlü çalışma alanları, ana bilgisayarın belgelenmiş alt işlem
  çalışma dizinine güvenebilir; yine de açık bir proje bağlaması en anlaşılır
  yapılandırmadır.
- Claude Code, Gemini CLI, Antigravity, Cline, Cursor, OpenCode, Kiro ve Codex
  proje/çalışma alanı yapılandırmasını kullanabilir. Yalıtımın garanti edilmesi
  gerektiğinde açık bir bağlama veya belgelenmiş bir proje başlangıç dizini
  kullanın.
- Claude Desktop ile eski Windsurf/Cascade yapılandırması geneldir. Varsayılan
  Fennara girdileri eski bağlı olmayan modda kalır ve otomatik projeye özel
  yalıtım sağlayamaz. İleri düzey kullanıcılar farklı açık yollarla ayrı
  adlandırılmış genel girdiler oluşturabilir, ancak doğru girdiyi seçmelidir.

Otomatik projeye özel yapılandırma oluşturma ve yeni Windsurf/Devin Local desteği
bu iş akışının dışındadır.

<a id="start-and-verify-the-editors"></a>
## Editörleri Başlatma ve Doğrulama

Her worktree, Fennara'nın etkin olduğu kendi Godot editörüne ihtiyaç duyar.
Grafik arayüzü olmayan (headless) Godot editörleri Fennara daemon'ını paylaşırken
ayrı Godot LSP bağlantı noktaları kullanabilir:

```bash
godot --editor --headless --path /absolute/path/to/worktree-a --lsp-port 6006
godot --editor --headless --path /absolute/path/to/worktree-b --lsp-port 6007
```

LSP bağlantı noktaları Godot'ya aittir. Fennara normal loopback adresinde
paylaşılan tek bir daemon kullanmaya devam eder.

Eşzamanlı çalışmadan önce her ajandan `fennara_status` çalıştırın. Şunları
bildirdiğini doğrulayın:

- yönlendirme modu `bound`
- beklenen bağlama kaynağı ve kanonik Proje Kökü
- bağlı editör durumu `connected`
- o editörün dosya sistemi hazırlığı

Otomatik keşif bir proje bulamazsa durum `legacy_unbound` ve bir eşzamanlılık
uyarısı bildirir. Bu uyumluluk modunda önce panelden seçilen MCP Hedefi, ardından
bağlı tek editör kullanılır. Yalıtılmış eşzamanlı çalışma için bağlı olmayan bir
bağlantı kullanmayın.

<a id="missing-and-duplicate-editors"></a>
## Eksik ve Yinelenen Editörler

Geçerli bir Proje Bağlaması, editörü yokken canlı kalır. Araç çağrıları o Proje
Kökü yeniden bağlanana kadar yeniden denenebilir `bound_project_not_connected`
sonucunu döndürür; panel hedefine hiçbir zaman geri dönmez.

Aynı Proje Köküne çözümlenen iki editör `ambiguous_project_binding` sonucunu
üretir. Yinelenen editörü kapatın veya ona ayrı bir worktree verin. Fennara;
işlem kimliği, bağlantı sırası, proje adı veya panel hedefine göre seçim yapmaz.

Aynı projeye giden sembolik bağlantı takma adları aynı canlı dosya sistemi
kimliğine çözümlenir. MCP başlangıcından sonra sembolik bağlantının hedefini
değiştirmek bağlamayı değiştirmez; yeniden bağlamak için o MCP işlemini yeniden
başlatın.

<a id="serialized-runtime-sessions"></a>
## Sıralı Çalışma Zamanı Oturumları

Tüm projeler daemon tarafından yönetilen oyun çalıştırmaları için makine
genelinde tek bir Çalışma Zamanı Yuvasını paylaşır. Başka bir proje bir oturumu
başlatıyor veya çalıştırıyorsa `runtime_session.start`, `availability: "busy"`,
`slot_acquired: false` ve önerilen bir `retry_after_ms` ile başarılı bir `busy`
etki alanı sonucu döndürür. Sahibi, oturum kimliğini, işlem kimliğini, sahneyi,
günlükleri, kuyruk konumunu veya beklenen süreyi açığa çıkarmaz.

FIFO kuyruğu yoktur. Önerilen yeniden deneme gecikmesine yakın bir süreyi jitter
ile sorgulayın ve her `runtime_session.start` çağrısını nihai atomik talep olarak
ele alın. Ön kontrolden sonra başka bir ajan yarışı kazanabileceğinden boş durum
yalnızca tavsiye niteliğindedir.

Yalnızca sahip Proje Kökü kendi Çalışma Zamanı Oturumunu inceleyebilir,
yenileyebilir, betikleyebilir veya durdurabilir. Sahip durum sorgusu 120
saniyelik hareketsizlik son tarihini yeniler. Sahibe ait sınırlı bir çalışma
zamanı işlemi, etkin olduğu sürece hareketsizlik süresinin dolmasını askıya
alır ve son tarihi yalnızca nihai bir betik sonucu döndürdükten sonra yeniler;
zaman aşımı, hazırlık hatası veya iptal son tarihi yenilemez. Ajanlar bir
çalışma sürerken sahip durumunu jitter ile yaklaşık her 30 saniyede bir
sorgulamalıdır.

Varsayılan mutlak Çalışma Zamanı Kirası 900 saniyedir. `max_run_seconds` en fazla
86.400 saniye olmak üzere pozitif bir tamsayı kabul eder; örneğin bir saat sürmesi
beklenen regresyon, güvenlik payı olarak 4.500 saniye isteyebilir. Mutlak son
tarih hiçbir zaman duraklatılmaz. Doğal çıkış, açık durdurma, başlangıç hatası,
hareketsizlik veya mutlak sürenin dolması oyunu durdurur ya da toplar ve Çalışma
Zamanı Yuvasını serbest bırakır.

<a id="safe-multi-agent-checklist"></a>
## Güvenli Çok Ajanlı Çalışma Denetim Listesi

1. Her proje için ayrı bir depo veya worktree oluşturun.
2. Fennara'yı kurun ve her Proje Kökü için bir Godot editörü açın.
3. Proje başına projeye bağlı bir MCP işlemi yapılandırın.
4. Her ajandan `fennara_status` çalıştırın ve kanonik kökünü doğrulayın.
5. Düzenleme, inceleme, sınırlı sahne doğrulaması ve bağımsız ekran
   görüntülerinin eşzamanlı ilerlemesine izin verin.
6. Oyun testleri için hata olmayan `busy` sonuçlarını sorgulayıp yeniden deneyin;
   kazanan oturumu çalışırken sahip durumuyla canlı tutun.
