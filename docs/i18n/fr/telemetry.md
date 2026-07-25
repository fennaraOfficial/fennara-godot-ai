<!-- fennara-i18n: locale=fr source=docs/telemetry.md sha256=925414507b4bfef9d6b7f207125bc0df953c8392e168f3ae20be78cf79c58d6a -->
<a id="anonymous-telemetry"></a>
# Télémétrie anonyme

<!-- fennara-doc-nav:start -->
[English](../../telemetry.md) · [简体中文](../zh-CN/telemetry.md) · [Español](../es/telemetry.md) · [Português do Brasil](../pt-BR/telemetry.md) · [日本語](../ja/telemetry.md) · [한국어](../ko/telemetry.md) · [Русский](../ru/telemetry.md) · **Français** · [Deutsch](../de/telemetry.md) · [Türkçe](../tr/telemetry.md)

> ℹ️ Traduction rédigée par une IA à partir de la source anglaise. La relecture par des locuteurs natifs est la bienvenue. [Source anglaise](../../telemetry.md)
<!-- fennara-doc-nav:end -->

Fennara envoie au maximum un petit événement d'activité anonyme par jour UTC.
L'événement est envoyé uniquement après la connexion d'un éditeur Godot compatible
au daemon local. Il aide les responsables à mesurer les installations actives,
l'utilisation des plateformes prises en charge et l'adoption des versions.

La télémétrie est activée par défaut. Ouvrez **Chat Settings > Chat > Anonymous
telemetry** pour la désactiver. Les environnements sans interface graphique et
automatisés peuvent définir l'une des variables suivantes :

```text
FENNARA_DISABLE_TELEMETRY=true
DO_NOT_TRACK=1
```

Une variable d'environnement est prioritaire sur la préférence enregistrée dans
l'interface. La désactivation de la télémétrie arrête les événements futurs et
supprime l'identité de télémétrie locale ainsi que l'état du dernier envoi. Sa
réactivation crée une nouvelle identité aléatoire lors de la prochaine connexion
de Godot.

<a id="event-contents"></a>
## Contenu de l'événement

L'événement `fennara_active_installation` contient uniquement :

| Champ | Objectif |
| --- | --- |
| `schema_version` | Version du petit contrat de charge utile de télémétrie |
| `event` | Nom d'événement fixe |
| `installation_id` | UUID aléatoire généré localement, sans dérivation à partir du matériel ou des comptes |
| `fennara_version` | Version du daemon en cours d'exécution |
| `godot_version` | Version numérique de Godot, comme `4.6.3` |
| `platform` | `windows`, `macos` ou `linux` |
| `architecture` | `x86_64` ou `aarch64` |

Fennara n'envoie pas les noms ou chemins des projets, les informations de compte,
les prompts, les messages de chat, les clés de fournisseur, les noms de modèles,
les noms, arguments ou résultats des outils, les journaux, les captures d'écran,
le contenu des scènes, les noms de fichiers ou le texte des erreurs.

<a id="storage-and-transport"></a>
## Stockage et transport

Le daemon conserve son identité aléatoire et le dernier jour UTC réussi dans le
répertoire partagé des données d'application de Fennara :

```text
Fennara/
  telemetry/
    state.json
```

Le daemon envoie l'événement par HTTPS à
`https://fennara.io/api/telemetry`. Le destinataire valide une liste exacte
de champs autorisés et remplace l'UUID d'installation brut par un HMAC côté
serveur avant de transmettre l'événement à PostHog. Les profils de personnes
et la géolocalisation par IP de PostHog sont désactivés pour cet événement.

Le destinataire Vercel observe nécessairement les métadonnées réseau normales
pendant le traitement de la requête HTTPS. Ces métadonnées ne sont pas copiées
dans la charge utile de l'événement PostHog.

<a id="delivery-behavior"></a>
## Comportement de l'envoi

La télémétrie s'exécute en dehors des parcours d'appel d'outil Godot :

- Une file bornée accepte les signaux d'activité sans attendre.
- Un seul worker en arrière-plan réutilise un client HTTP unique.
- Les requêtes ont un délai d'expiration court.
- Une file pleine, un problème de système de fichiers, une panne réseau ou un
  refus du serveur est toléré silencieusement et ne fait jamais échouer un outil Fennara.
- Le jour UTC n'est enregistré qu'après l'acceptation d'un événement par le
  serveur. Une connexion Godot ultérieure peut donc réessayer un envoi qui a échoué.
- À l'arrêt, le daemon attend brièvement puis annule le worker de télémétrie au
  lieu de se laisser retarder par celui-ci.

Une installation correspond à un UUID aléatoire conservé. L'utilisation de
Fennara sur deux ordinateurs compte comme deux installations. Effacer les
données d'application de Fennara, ou désactiver puis réactiver la télémétrie,
crée une nouvelle identité.

Les installations actives mensuelles sont comptées comme les identités anonymes
d'installation distinctes ayant envoyé au moins un événement
`fennara_active_installation` au cours du mois civil.
