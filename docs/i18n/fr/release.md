<!-- fennara-i18n: locale=fr source=docs/release.md sha256=60b8cc51e0fcde9b4e18eadc230aaf1d8cc4fad2fe70cbf5190ab9123bac0073 -->
<a id="release-process"></a>
# Processus de publication

<!-- fennara-doc-nav:start -->
[English](../../release.md) · [简体中文](../zh-CN/release.md) · [Español](../es/release.md) · [Português do Brasil](../pt-BR/release.md) · [日本語](../ja/release.md) · [한국어](../ko/release.md) · [Русский](../ru/release.md) · **Français** · [Deutsch](../de/release.md) · [Türkçe](../tr/release.md)

> ℹ️ Traduction rédigée par une IA à partir de la source anglaise. La relecture par des locuteurs natifs est la bienvenue. [Source anglaise](../../release.md)
<!-- fennara-doc-nav:end -->

Les versions sont publiées manuellement. Ne publiez rien depuis les processus de pull request.

> [!IMPORTANT]
> Exécutez les publications depuis `main`, gardez `VERSION` identique à l'entrée
> du processus et décidez explicitement si la version exige une version minimale
> plus élevée de la CLI.

<a id="release-at-a-glance"></a>
## Vue d'ensemble d'une publication

| Étape | Résultat |
| --- | --- |
| Préparer et fusionner la modification de version | Les sources de version du dépôt concordent |
| Exécuter Package Preview | Des artefacts ayant la forme d'une version sont compilés sans être publiés |
| Inspecter la prévisualisation | Les archives, le manifeste, les hachages et la disposition CEF sous Linux sont vérifiés |
| Exécuter Release depuis `main` | L'étiquette et la GitHub Release sont publiées |
| Tester rapidement l'installation et la mise à jour | Le parcours utilisateur public est vérifié |

<a id="versioning"></a>
## Gestion des versions

`VERSION` est la source de référence.

Les outils de publication acceptent des valeurs SemVer. Les versions stables
utilisent `X.Y.Z`. Les candidats de prépublication utilisent une préversion
isolée par pull request, comme `1.2.3-pr.101.2`, où `pr-101` est le canal de
prépublication et `2` le numéro du candidat sur ce canal.

Pour incrémenter la version du dépôt :

```bash
node scripts/set-version.mjs X.Y.Z
```

Le script met à jour :

- `VERSION`
- `godot_demo/addons/fennara/VERSION`
- les constantes de version du plugin
- la version des paquets de l'espace de travail Rust sous `local/`
- `local/Cargo.lock`

L'addon contient aussi `addons/fennara/release.json`. L'identité stable est
écrite automatiquement par la commande normale ci-dessus. Un espace de travail
de compilation de prépublication utilise les entrées d'identité explicites :

```bash
node scripts/set-version.mjs 1.2.3-pr.101.2 \
  --track staging \
  --channel pr-101 \
  --source-commit <full-commit-sha>
```

La version de prépublication, le canal, le commit source et l'étiquette exacte
de publication doivent concorder. Un addon de préversion dépourvu de cette
identité est refusé. Les addons stables existants antérieurs à `release.json`
continuent d'utiliser la piste stable par défaut.

Vérifiez la synchronisation des versions :

```bash
node scripts/check-version.mjs
```

<a id="1-prepare-the-release-commit"></a>
## 1. Préparer le commit de publication

1. Exécutez le script de version.
2. Relisez le diff.
3. Exécutez les vérifications locales adaptées à la surface modifiée.
4. Fusionnez la PR de préparation de la version dans `main`.

Vérifications courantes :

```bash
node scripts/check-version.mjs
cd local
cargo test --locked
```

Pour les modifications de la GDExtension, compilez également l'addon localement lorsque c'est possible :

```bash
cd fennara-cpp
scons platform=windows target=editor
```

<a id="2-run-package-preview"></a>
## 2. Exécuter Package Preview

Utilisez ce processus avant la publication lorsque les paquets ont changé ou si vous souhaitez effectuer un essai à blanc.

GitHub :

```text
Actions > Package Preview > Run workflow
```

Le processus compile les paquets Windows, Linux et macOS et téléverse des
artefacts temporaires. Il ne crée ni étiquette, ni GitHub Release, ni `latest`.

Package Preview reproduit suffisamment fidèlement les parties non publiantes
de Release pour tester la création des paquets avant la fusion :

- synchronise l'interface de chat sans compilation et les sources des auxiliaires d'exécution dans la charge utile de l'addon
- compile le fichier ZIP d'exécution CEF sous Linux
- écrit le manifeste généré de l'environnement CEF sous Linux
- fournit ce manifeste généré aux compilations des paquets de plateformes
- assemble l'archive de l'addon pour toutes les plateformes
- renomme les paquets locaux et de l'addon selon les noms de ressources gérés par le manifeste
- valide la ressource d'exécution CEF sous Linux selon le manifeste généré
- écrit `fennara-release-manifest-v<version>.json`
- téléverse un artefact `fennara-package-preview-release-assets` contenant les
  fichiers ZIP et le manifeste ayant la forme de la version publiée

Les artefacts de prévisualisation sont utiles pour vérifier le contenu des
fichiers ZIP et la forme du manifeste avant la publication. Ce sont des artefacts
Actions, pas des ressources publiques d'une version.

<a id="3-run-release"></a>
## 3. Exécuter Release

Exécutez le processus de publication manuel depuis `main` :

```text
Actions > Release > Run workflow
```

Entrées :

```text
version: X.Y.Z
promote_latest: true
```

L'entrée `version` doit correspondre à `VERSION`.

Le processus publie :

- `v<version>`
- marque `v<version>` comme GitHub Latest lorsque `promote_latest` vaut true

Le processus Release prépare l'environnement CEF Linux avant la création des
paquets de plateformes. Il télécharge le SDK minimal officiel CEF 139 Linux
épinglé, assemble le fichier ZIP distinct
`fennara-webview-cef-linux-x64-<cef-version>.zip`, retire les symboles des
binaires ELF préparés, écrit un manifeste généré et activé
`local/webview-runtimes/linux-cef.json`, puis fournit ce manifeste aux paquets
de la CLI. Le job de publication vérifie ensuite que les ressources publiées
comprennent le fichier ZIP CEF exact nommé par le manifeste généré et que son
SHA-256 correspond. Il écrit aussi `fennara-release-manifest-v<version>.json`,
valide chaque ressource et chaque hachage référencés, puis téléverse ce manifeste
avec la version publiée.

Les processus de pull request ne publient aucune version. Package Preview crée
des artefacts de test ayant la forme d'une version, notamment le manifeste et
la charge utile CEF sous Linux, afin que les responsables puissent tester les
paquets avant la fusion. Package Preview n'est pas le canal de publication
destiné aux utilisateurs.

<a id="release-assets"></a>
## Ressources de la version

Chaque version doit contenir les paquets de CLI et d'environnement local de chaque plateforme et un paquet d'addon partagé pour toutes les plateformes.

| Cible | Ressources |
| --- | --- |
| Windows x86_64 | `fennara-cli-windows-x86_64-v<version>.zip`<br>`fennara-release-local-windows-x86_64-v<version>.zip` |
| Linux x86_64 | `fennara-cli-linux-x86_64-v<version>.zip`<br>`fennara-release-local-linux-x86_64-v<version>.zip`<br>`fennara-webview-cef-linux-x64-<cef-version>.zip` |
| macOS arm64 | `fennara-cli-macos-arm64-v<version>.zip`<br>`fennara-release-local-macos-arm64-v<version>.zip` |
| Toutes les plateformes | `fennara-release-addon-v<version>.zip`<br>`fennara-addon-latest.zip`<br>`fennara-release-manifest-v<version>.json` |

Rôles des paquets :

| Motif | Rôle |
| --- | --- |
| `fennara-cli-*` | Charge utile du script d'installation qui contient uniquement la CLI `fennara` pour une plateforme |
| `fennara-release-local-*` | Lanceurs MCP et daemon, ainsi que leurs binaires d'exécution versionnés, pour une plateforme |
| `fennara-release-addon-v*` | Addon versionné pour toutes les plateformes, résolu par le manifeste de version |
| `fennara-addon-latest.zip` | Alias au nom stable de l'addon pour toutes les plateformes, destiné à la documentation et aux téléchargements manuels |
| `fennara-webview-cef-linux-x64-*` | Environnement CEF partagé réservé à Linux, installé une seule fois dans les données d'application de Fennara |
| `fennara-release-manifest-v*` | Plan d'installation et de mise à jour qui contient les noms des ressources, les valeurs SHA-256, les primitives d'installation et les environnements partagés |

La GDExtension de l'addon macOS n'est actuellement pas notariée par Apple. Les
téléchargements dans un navigateur et l'extraction manuelle dans Finder peuvent
propager les métadonnées de quarantaine et déclencher la notification de
vérification de macOS. La documentation d'installation destinée aux utilisateurs
doit recommander `fennara install` sous macOS, expliquer la limite du fichier ZIP
manuel et demander aux utilisateurs concernés de supprimer l'addon copié
manuellement avant de le réinstaller par la CLI. La validation d'une version ne
considère pas que la simple création du fichier ZIP constitue une signature ou
une notarisation macOS.

Le préfixe `fennara-release-local-*` empêche les anciennes CLI de contourner
silencieusement le parcours des paquets géré par le manifeste.

<a id="release-manifest"></a>
## Manifeste de version

Depuis la version 0.3.0, `fennara install` et `fennara update` préfèrent le
manifeste lorsqu'une version en publie un. Le manifeste enregistre :

- `schema_version`
- `version`
- `minimum_cli_version`
- les primitives d'installation prises en charge
- les ressources de CLI et d'environnement local de chaque plateforme avec leurs hachages SHA-256
- la ressource partagée de l'addon avec son SHA-256
- les ressources d'exécution partagées propres aux plateformes, actuellement CEF sous Linux

`scripts/release-policy.mjs` est la source de référence pour
`minimum_cli_version`. Le générateur de manifeste sélectionne la politique après
la validation de l'identité de la version. Stable, Package Preview et Staging ne
peuvent donc pas choisir des valeurs indépendantes. Les changements ordinaires
de disposition des paquets ou de noms de ressources doivent être gérés par les
données du manifeste, et non en modifiant la CLI externe. Augmentez la politique
lorsqu'une version exige une transmission plus récente du programme de mise à
jour, un nouveau schéma de manifeste, une primitive d'installation, un comportement
de mise à jour automatique ou une autre capacité de la CLI qu'une ancienne CLI
publiée ne peut pas exécuter en toute sécurité.

Lorsque la CLI est trop ancienne, `fennara update` doit utiliser l'entrée
`assets.cli` propre à la plateforme dans le manifeste pour mettre d'abord à jour
la CLI installée, puis reprendre la mise à jour du paquet avec `--no-self-update`.
Si la mise à jour automatique n'est pas disponible pour cette version ou cet
emplacement, la commande doit échouer avant d'installer les paquets et indiquer
clairement de réexécuter `install.sh` ou `install.ps1`.

L'identité facultative ajoutée au schéma 1 du manifeste ne nécessite pas
d'augmenter la version minimale de la CLI. Les anciens clients du schéma 1
ignorent les champs inconnus, tandis que les clients qui comprennent les
prépublications valident l'identité lorsqu'elle est présente. Une future version
qui dépend d'une activation propre au canal ou d'une transmission au programme
de mise à jour doit réexaminer la version minimale de la CLI avant sa publication.

<a id="staging-identity-and-discovery-contract"></a>
## Contrat d'identité et de découverte des prépublications

Les canaux de prépublication sont isolés par pull request :

| Valeur | Exemple pour la PR 101 |
| --- | --- |
| Canal | `pr-101` |
| Version du candidat | `1.2.3-pr.101.2` |
| Version exacte | `v1.2.3-pr.101.2` |
| Référence du canal | `fennara-staging/pr-101` |
| Fichier de pointeur | `fennara-staging-channel-pr-101.json` |

La référence Git propre au canal contient uniquement un petit fichier de pointeur
vers une version numérotée exacte. Les binaires publiés ne résident jamais sous
la référence mobile du canal. La CLI peut résoudre ce pointeur avec la demande
interne de version `channel:pr-101`, puis continue en utilisant uniquement la
version exacte.

Les PR 101 et 125 utilisent donc des étiquettes et des ressources de pointeur
différentes. La mise à jour d'un canal ne peut pas rediriger les testeurs de
l'autre canal. La publication d'un canal ne modifie jamais la désignation stable
GitHub Latest ni le canal d'une autre pull request.

<a id="staging-candidate-workflow"></a>
## Processus des candidats de prépublication

Le processus manuel **Staging Release** compile un candidat à partir de la tête
actuelle d'une pull request ouverte. Exécutez-le depuis `main` et fournissez :

| Entrée | Signification |
| --- | --- |
| `pull_request` | Pull request ouverte à compiler |
| `base_version` | Version stable prévue sous la forme `X.Y.Z` |
| `candidate` | Numéro de candidat croissant pour cette pull request |
| `source_commit` | SHA complet facultatif qui doit toujours être la tête de la pull request |
| `publish` | Désactivé pour la validation avec artefacts uniquement, activé pour publier le candidat |

Le processus fige le SHA de la tête de la pull request avant toute compilation
de plateforme. Les jobs Windows, Linux et macOS extraient ce commit précis avec
des autorisations en lecture seule, sans identifiants Git persistants, sans
identifiants de publication et sans possibilité d'enregistrer les caches partagés
des dépendances. Ils peuvent restaurer les caches SCons, godot-cpp et Cargo
compatibles écrits par les processus fiables de la branche par défaut. La
prépublication utilise l'action de cache uniquement pour la restauration. Le
code candidat peut donc consommer les résultats fiables d'une compilation, mais
ne peut ni remplacer ni empoisonner les caches des exécutions suivantes. Il peut
produire des artefacts de compilation, mais ne peut pas publier de GitHub Release.

Les scripts fiables du dépôt valident ensuite l'identité du candidat, l'inventaire
exact de l'archive, le contenu de l'addon, la disposition des paquets de plateformes,
le manifeste de version et chaque valeur SHA-256. La publication reste désactivée
tant que `publish` n'a pas été sélectionné explicitement.

Lorsque la publication est activée, le job final fiable :

1. Revalide les artefacts du candidat comme données.
2. Crée un brouillon, téléverse chaque ressource et le publie comme préversion
   exacte `v<exact-version>` sans modifier GitHub Latest.
3. Télécharge les ressources publiées et compare leurs noms et leurs hachages.
4. Refuse toute modification en arrière ou contradictoire du canal.
5. Met à jour en dernier la petite référence de pointeur
   `fennara-staging/pr-<number>` par une écriture conditionnelle dans l'API GitHub Contents.
6. Télécharge le pointeur actif et vérifie son contenu exact.

Les exécutions d'une même pull request sont sérialisées. Les différentes pull
requests utilisent des groupes de concurrence, des étiquettes de publication
et des références de pointeur distincts. La nouvelle tentative du même candidat
vérifie la version exacte existante au lieu d'y mélanger des fichiers. Le
processus ne crée jamais, ne téléverse jamais vers et ne promeut jamais la
version stable GitHub Latest.

La publication stable n'utilise ni étiquette ni version littérale `latest`. Le
processus Release crée la version exacte `v<version>` sous forme de brouillon,
vérifie les ressources téléversées octet par octet, la publie comme version
modifiable et marque cette version précise comme GitHub Latest lorsque
`promote_latest` vaut true. Les programmes d'installation et la découverte
stable de la CLI résolvent le point d'accès Latest Release de l'API GitHub.

Les versions stables et de prépublication sont modifiables tant que l'immuabilité
des versions du dépôt est désactivée. Les deux processus vérifient les métadonnées
de la version et les octets des ressources téléchargées avant de terminer la
publication ou d'avancer un canal. La publication des ressources utilise le
`GITHUB_TOKEN` limité au job, avec un accès en écriture au contenu.

La politique de publication exige actuellement la CLI `0.4.1` pour les
manifestes stables et la CLI `0.3.8` pour les manifestes de prépublication.
La découverte stable ne résout plus l'ancienne étiquette `latest`. La version
stable `0.4.1` exige la validation corrigée de la mise à jour, la vérification
préalable du changement de version, la gestion du journal d'opération sous
Windows et la réparation du marqueur d'exécution CEF sous Linux. Un candidat comme `0.4.1-pr.123.1` est inférieur à la version stable
`0.4.1` selon SemVer. Sa version minimale doit donc rester inférieure à la
version du candidat afin que la première installation puisse installer la CLI
candidate. Ne modifiez pas l'une de ces versions minimales en vous fondant
uniquement sur la compatibilité du schéma de manifeste.

Le fichier ZIP de l'addon partagé contient tous les binaires GDExtension compilés
et référencés par `godot_demo/addons/fennara/fennara.gdextension`. Godot charge
la bibliothèque qui correspond au système d'exploitation de l'utilisateur et
ignore les autres.

Les charges utiles de la webview CEF sous Linux sont séparées de l'archive de
l'addon. La création des paquets génère le manifeste activé de l'environnement
et intègre ces données dans `fennara-release-manifest-v<version>.json`. La CLI
installe une seule fois la charge utile CEF correspondante dans le répertoire
des données d'application Fennara de l'utilisateur :

```text
webview/cef/linux-x64/<cef-version>/
```

Ne placez pas `libcef.so`, les exécutables auxiliaires CEF, les ressources CEF
ou les paquets de langues dans `fennara-addon-*`. Package Preview compile un
artefact CEF distinct pour les tests et écrit le même type de manifeste
d'exécution généré que Release, mais seule la publication de la version constitue
la source des ressources destinée aux utilisateurs.

Les compilations GDExtension sous Linux ont aussi besoin des sources du wrapper
du SDK CEF officiel, mais pas des fichiers d'exécution CEF dans l'addon. La CI exécute :

```bash
node scripts/prepare-linux-cef-sdk.mjs
```

et transmet le répertoire extrait à SCons sous `FENNARA_CEF_ROOT`. SCons utilise
`FENNARA_CEF_ROOT/libcef_dll/` pour compiler la petite bibliothèque d'addon
`libfennara_linux_cef_bridge.so` avec le wrapper C++ épinglé de CEF 139. La
version et le hachage du SDK téléchargé sont vérifiés, car les sources générées
du wrapper doivent correspondre à l'ABI de l'environnement CEF. Le pont est
empaqueté avec l'addon. `libcef.so`, les ressources, les paquets de langues et
`fennara_cef_helper` restent dans l'environnement CEF partagé distinct.

Les scripts de création des paquets échouent si des fichiers d'exécution CEF
sont trouvés dans l'archive de l'addon. Le nom de la ressource d'exécution doit être :

```text
fennara-webview-cef-linux-x64-<cef-version>.zip
```

Le fichier ZIP doit s'extraire avec les fichiers requis à sa racine :

```text
libcef.so
fennara_cef_helper
icudtl.dat
resources.pak
chrome_100_percent.pak
chrome_200_percent.pak
v8_context_snapshot.bin
locales/en-US.pak
```

Les fichiers d'exécution CEF facultatifs comme `chrome-sandbox`, `libEGL.so`,
`libGLESv2.so`, `libvk_swiftshader.so`, `libvulkan.so.1`,
`vk_swiftshader_icd.json`, `snapshot_blob.bin` et les autres `locales/*.pak`
doivent être inclus lorsqu'ils sont présents dans la distribution CEF sélectionnée.

Pour assembler manuellement le fichier ZIP d'exécution depuis une arborescence
binaire CEF choisie par un responsable :

```bash
node scripts/prepare-linux-cef-runtime.mjs \
  --cef-root /path/to/cef_binary_<version>_linux64_minimal \
  --version <cef-version> \
  --out-dir dist/cef-runtime
```

Sous Linux, le script compile `fennara_cef_helper` depuis
`scripts/cef/linux/fennara_cef_helper.cpp` avec les en-têtes CEF officiels de
`fennara-cpp/vendor/cef/`. Sur un autre système, compilez d'abord cet auxiliaire
sous Linux et transmettez `--helper /path/to/fennara_cef_helper`. Utilisez
`--dry-run` pour inspecter les fichiers sélectionnés avant d'écrire le fichier ZIP.

Après l'affichage du SHA-256 par le script, mettez à jour
`local/webview-runtimes/linux-cef.json` :

```json
{
  "version": "<cef-version>",
  "enabled": true,
  "archive": {
    "format": "zip",
    "name": "fennara-webview-cef-linux-x64-<cef-version>.zip",
    "url": null,
    "sha256": "<sha256>"
  }
}
```

Pour les versions normales, le processus écrit automatiquement le manifeste
d'exécution CEF Linux avec `--write-manifest`, puis
`scripts/write-release-manifest.mjs` copie les champs de l'environnement dans
`fennara-release-manifest-v<version>.json`. N'activez pas manuellement le
manifeste temporaire suivi, sauf pour déboguer volontairement un chemin manuel
de ressource d'exécution ou un ancien comportement de repli. Si les données du
manifeste généré pointent vers une ressource absente ou dont le SHA-256 ne
correspond pas, le processus Release et les commandes Linux `fennara install`
et `fennara update` échouent clairement.

La CLI doit publier atomiquement les mises à jour de l'environnement CEF Linux :
extraire et valider dans un répertoire de préparation, écrire le marqueur
d'exécution uniquement une fois tous les fichiers requis présents, puis publier
le répertoire de version et mettre à jour `current.json` par le renommage d'un
fichier temporaire. Le marqueur `fennara-cef-runtime.json` installé doit identifier
le contrat du chargeur natif avec `"runtime": "cef"`. L'installation et la mise
à jour réparent un ancien marqueur correspondant qui contient seulement
`"kind": "cef"` sans télécharger de nouveau la charge utile CEF. Les éditeurs
en cours d'exécution continuent d'utiliser l'environnement déjà chargé.

La CLI intègre les modèles d'instructions de projet générées depuis
`local/templates/`. Lorsque la création des paquets compile la CLI, ces modèles
sont compilés dans le binaire avec le reste du code de la CLI.

<a id="what-latest-means"></a>
## Signification de `latest`

Le pointeur Latest Release de GitHub sélectionne la version numérotée employée
par les parcours normaux d'installation et de mise à jour. Fennara ne crée ni
ne déplace d'étiquette littérale `latest`.

- `install.ps1` et `install.sh` récupèrent par défaut la dernière ressource de la CLI.
- `fennara update` récupère par défaut le manifeste par le point d'accès Latest
  Release de GitHub, met à jour automatiquement la CLI installée si nécessaire,
  puis résout les ressources locales, de l'addon et des environnements partagés.
- Les mises à jour intégrées à l'éditeur mettent les ressources vérifiées en
  attente avant la fermeture, vérifient de nouveau le condensat de tout l'addon
  préparé avant le remplacement, conservent l'addon précédent, les lanceurs et
  le manifeste d'exécution jusqu'à la réussite de la validation d'activation,
  et exigent la confirmation de la GDExtension rouverte avant de supprimer les données d'annulation.
- `fennara install` récupère par défaut le manifeste par le point d'accès Latest
  Release de GitHub, puis résout les ressources locales, de l'addon et des environnements partagés.
- La vérification des mises à jour du plugin Godot compare avec la dernière version publiée sur GitHub.

Utilisez `promote_latest: false` uniquement pour publier une version qui ne doit pas devenir l'installation par défaut des utilisateurs.

Les programmes d'installation et les téléchargements des versions doivent
afficher les métadonnées de la version et les étapes de téléchargement,
d'extraction, d'installation et de vérification des ressources. Les accès réseau
doivent employer des délais bornés afin qu'un blocage de GitHub ou du CDN échoue
avec un diagnostic au lieu de sembler figé. Sous Windows, `install.ps1` doit
vérifier le code de sortie de la CLI avant d'afficher la réussite. Le code
`-1073741515` (`0xC0000135`) signifie que l'exécutable de la CLI a été écrit,
mais que Windows n'a pas pu le démarrer en raison de l'absence d'une DLL requise.
Demandez à l'utilisateur d'installer Microsoft Visual C++ Redistributable
2015-2022 x64, puis de réexécuter `fennara --version`, `fennara doctor` et
`fennara install`. URL de téléchargement :
`https://aka.ms/vs/17/release/vc_redist.x64.exe`.

<a id="smoke-test-after-release"></a>
## Test rapide après la publication

Sous Windows :

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
fennara --version
fennara doctor
```

Dans un projet Godot :

```bash
cd path/to/your-godot-project
fennara install
fennara mcp-setup --claude
```

Vérifiez que le projet a reçu :

```text
AGENTS.md
addons/fennara/ai/
```

Ouvrez le projet dans Godot, puis demandez à l'application MCP :

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

Test de mise à jour :

```bash
cd path/to/your-godot-project
fennara update
fennara self-update
```

<a id="rules"></a>
## Règles

- Le processus Release s'exécute uniquement depuis `main`.
- L'entrée de version de Release doit correspondre à `VERSION`.
- Les processus de pull request peuvent compiler et téléverser des artefacts de test, mais ne doivent pas publier de versions.
- Gardez la version destinée aux utilisateurs ordinaires désignée comme GitHub Latest.
- Ne réécrivez pas les étiquettes de versions publiées, sauf si les responsables décident volontairement de remplacer une version défectueuse.
