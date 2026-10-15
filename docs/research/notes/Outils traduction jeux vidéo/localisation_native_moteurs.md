# Systèmes de localisation natifs des moteurs de jeu et points d'accroche des traducteurs (état 2026-09)

Notes de recherche (Claude, 2026-09-25). Environ 20 appels outils ; plusieurs pages officielles lues via un résumeur automatique. Les affirmations marquées « à vérifier » viennent d'un résumé non confirmé par une seconde source.

## Q1 — Pipeline par moteur : identifiants/clés, extraction, réinjection, détection des mises à jour

### Takeaway
Les moteurs occidentaux (Ren'Py, Unity, Unreal, Godot) ont des pipelines natifs avec des clés stables et une détection des changements (hash du dialogue, gather/diff, merge PO). Les moteurs japonais ont rattrapé ce retard récemment : un outil officiel Excel pour Wolf v3 (2023), un DLC payant pour Bakin (2025), une fonction de traduction dans TyranoScript V600+ et un localization.dat pour SRPG Studio. RPG Maker MZ, Kirikiri et PGMMV reposent sur des plugins ou des outils de fans.

### Cited Findings

**Ren'Py 8**
- Les unités de traduction sont générées automatiquement à partir des blocs de dialogue. Chaque identifiant est formé du label précédent et du contenu des instructions. En cas de collision, un numéro de série est ajouté (exemple : `start_636ae3f5`) — [Ren'Py docs, Translation](https://www.renpy.org/doc/html/translation.html)
- Code source : chaque instruction `Say` du bloc est passée dans un MD5 via `md5.update((code + "\r\n").encode("utf-8"))`. Seuls les 8 premiers caractères hex sont conservés (`md5.hexdigest()[:8]`). L'identifiant vaut `label.replace(".", "_") + "_" + digest`, ou le digest seul s'il n'y a pas de label. Une collision reçoit le suffixe `_{i}`. Un identifiant `alternate` existe, et un `id` explicite remplace l'identifiant calculé — [renpy/translation/__init__.py](https://github.com/renpy/renpy/blob/master/renpy/translation/__init__.py)
- Conséquence : toute modification du texte source d'une réplique change le hash, donc l'ancienne traduction n'est plus rattachée. La clause `id` permet de garder la traduction malgré une édition du dialogue — [Ren'Py docs](https://www.renpy.org/doc/html/translation.html) ; [source](https://github.com/renpy/renpy/blob/master/renpy/translation/__init__.py)
- Organisation : `game/tl/<language>/`, avec un fichier de traduction par script et un `common.rpy` pour les chaînes communes à tous les jeux Ren'Py — [Ren'Py docs](https://www.renpy.org/doc/html/translation.html)
- Chaînes d'interface : blocs `translate <language> strings:` avec paires `old`/`new`. La substitution est un pour un, partout où la chaîne apparaît. Les chaînes marquées par `_()` sont traduisibles. `__()` et `_p()` servent aussi au marquage — [Ren'Py docs](https://www.renpy.org/doc/html/translation.html)
- Un bloc de dialogue traduit peut découper une longue réplique, supprimer une instruction via `pass`, exécuter du Python conditionnel et interpoler avec le drapeau `!t`. `translate <lang> python:` et `translate <lang> style:` changent les polices et les styles par langue — [Ren'Py docs](https://www.renpy.org/doc/html/translation.html)
- Changement de langue : action `Language(language)` et `renpy.change_language()` — [Ren'Py docs](https://www.renpy.org/doc/html/translation.html)

**Unity 6 (package com.unity.localization) et I2 Localization**
- Une String Table Collection est un asset éditeur qui regroupe plusieurs String Tables (une par locale) partageant la même Shared Table Data. La Shared Table Data stocke les noms de clés, leurs Key Id et les métadonnées partagées. Les métadonnées peuvent aussi être propres à une locale — [Unity Localization 1.5, String Tables](https://docs.unity3d.com/Packages/com.unity.localization@1.5/manual/StringTables.html)
- Le package gère l'import et l'export XLIFF, CSV et Google Sheets, les Smart Strings et la pseudo-localisation — [Unity Localization 1.5, index](https://docs.unity3d.com/Packages/com.unity.localization@1.5/manual/index.html) ; [manuel Unity 6000.2](https://docs.unity3d.com/6000.2/Documentation/Manual/com.unity.localization.html)
- L'export CSV propose une variante « CSV (With Comments) » qui exporte les métadonnées Comment dans des colonnes à part. Les CSV Extensions se configurent par collection — [Unity Localization 0.9, CSV](https://docs.unity3d.com/Packages/com.unity.localization@0.9/manual/CSV.html)
- I2 Localization (asset tiers) repose sur des « terms », importe depuis Google Spreadsheets ou CSV et peut télécharger automatiquement les changements d'une feuille liée, même après la sortie du jeu. Des « Secondary terms » changent polices et atlas par langue, et le RTL est corrigé automatiquement — [LinkedIn, Irajdoost](https://www.linkedin.com/pulse/game-localization-unity-leveraging-i2-plugin-iman-irajdoost) ; [June Jiang](https://chaoyuej.middcreate.net/translation/dive-into-the-world-of-game-localization-in-unity-with-i2/) (sources secondaires)

**Unreal Engine 5**
- Le type de base est `FText`, écrit à la source via `LOCTEXT`/`NSLOCTEXT` et identifié par un couple namespace + key. Les String Tables offrent l'alternative « author once, reference » — [Epic, Localization Overview](https://dev.epicgames.com/documentation/en-us/unreal-engine/localization-overview-for-unreal-engine)
- Pipeline du Localization Dashboard : Gather (`.manifest`) → Export (`.po`) → Import (`.archive`) → Compile (`.locres` binaire). Chemin : `Content/Localization/{Target}/{Culture}/{Target}.locres` — [Epic](https://dev.epicgames.com/documentation/en-us/unreal-engine/localization-overview-for-unreal-engine)
- Détection des changements : le gather compare le texte source. Si le texte natif change, les traductions deviennent « stale » (elles restent compilables selon un réglage) — [Epic](https://dev.epicgames.com/documentation/en-us/unreal-engine/localization-overview-for-unreal-engine) (résumé automatique, à vérifier)
- Outils de fans : UnrealLocres gère toutes les versions de locres jusqu'à la v3. Commandes : `export <locres> [-f csv|pot]`, `import <locres> <translation> [-f csv|pot]` et `merge`. Règle : ne **jamais** modifier la colonne key. L'outil reprend TextKey, CRC et CityHash du code UE4 — [UnrealLocres README](https://github.com/akintos/UnrealLocres/blob/master/README.md)
- Autres outils : pylocres (locres↔csv/po) — [GitHub](https://github.com/stas96111/pylocres) ; LocresStudio (GUI, TSV) — [GitHub](https://github.com/AcTePuKc/LocresStudio) ; LocRes-Builder — [GitHub](https://github.com/matyalatte/LocRes-Builder) ; ue-localization-tools — [GitHub](https://github.com/efonte/ue-localization-tools)

**Godot 4**
- `TranslationServer` gère les langues à l'exécution. `tr()` convertit une clé (ex. `MAIN_SCREEN_GREETING1`) ou une chaîne source. `tr(msg, context)` sert à la désambiguïsation, `tr_n(singular, plural, n[, ctx])` au pluriel. Mettre `Auto Translate > Mode = Disabled` empêche de traduire un nœud (ex. nom du joueur) — [Godot docs, Internationalizing games](https://docs.godotengine.org/en/stable/tutorials/i18n/internationalizing_games.html)
- Le fichier POT est généré dans l'éditeur (Project Settings > Localization > Template Generation). L'extraction couvre `tr`, `tr_n`, `atr`, `atr_n`, les propriétés `text`, `placeholder_text` et `tooltip_text`, ainsi que `add_tab`, `add_item` et les filtres FileDialog. L'argument doit être une chaîne constante. La mise à jour passe par `msgmerge --update`. Les entrées « fuzzy » sont ignorées par Godot tant qu'elles ne sont pas corrigées. Les `.mo` compilés (`msgfmt`) chargent plus vite sur les gros projets — [Godot docs, gettext](https://docs.godotengine.org/en/stable/tutorials/i18n/localization_using_gettext.html)

**TyranoScript / TyranoBuilder**
- La fonction de traduction native (TyranoScript V600+) extrait automatiquement le texte traduisible. Les langues sont identifiées par un code (`en`, `ch`…). La traduction se saisit dans un tableau de l'éditeur, avec export CSV possible pour Excel. Le changement de langue se fait par `[lang_set name="en"]` à tout moment, avec un seul build. Une fonction « Tag Registration » déclare les macros et paramètres personnalisés à extraire. Pour persister la langue : `sf.selected_lang`, `navigator.language` et `TYRANO.kag.lang` dans `[if]` — [tyranoscript.com, Translate](https://tyranoscript.com/usage/advance/translate)
- Outils de fans : filetranslate_TB — [GitHub](https://github.com/UserUnknownFactor/filetranslate_TB) ; Translator++ — [Patreon Dreamsavior](https://www.patreon.com/posts/translator-ver-3-53789142) ; VNTranslator — [docs](https://docs.vntranslator.com/user-guide/tyrano-games) ; RuneTranslate (`data/scenario/*.ks`, `lang.js`, `Config.tjs`) — [runetranslate.com](https://runetranslate.com/engines/tyranobuilder)

**Kirikiri / KAG (krkr2/krkrz)**
- Pas d'outil de localisation natif documenté trouvé. La chaîne des fans est la suivante. GARbro extrait les archives `.xp3`. KrkrExtract déchiffre certaines versions, pas toutes. FreeMote extrait et recompile les `.psb` (E-mote, images). KrkrzSceneManager édite les `.ks.scn` (scènes compilées PSB) et les textes de menu — [Gist LukeZGD, Nekopara](https://gist.github.com/LukeZGD/64b0ecfb7eaccdfbb327ddfea4621723) ; [GARbro issue #153](https://github.com/morkt/GARbro/issues/153)
- KagEditor : GUI de traduction des `.ks` — [GitHub begerard/KagEditor](https://github.com/begerard/KagEditor). Guide de l'écosystème krkrz — [pantsudev](https://github.com/pantsudev/krkrz_ecosystem_guide). Tutoriel Translator++ KAG — [Dreamsavior](https://dreamsavior.net/translating-kag-kirikiri-visual-novels/)
- RuneTranslate annonce un tokenizer KAG pour les répliques, les tags ruby, les noms, les boutons et les tags d'édition, ainsi qu'un hook « AfterInit2 » (source commerciale) — [runetranslate.com](https://runetranslate.com/engines/kirikiri)

**RPG Developer Bakin**
- Le Localization Toolkit est un DLC Steam officiel à 34,99 USD, sorti le 28 août 2025. Il permet de préparer les langues dans l'éditeur, de tester en jeu avec la localisation appliquée et de détecter la langue d'affichage de Windows. Le changement manuel passe par Settings > Change Language et impose un redémarrage. Il échange aussi les images d'UI, les panneaux et l'audio, s'applique aux projets existants et permet une édition en masse filtrée par personnage ou événement — [rpgbakin.com, Localization Toolkit](https://rpgbakin.com/en/products/localization-toolkit) ; [Steam](https://store.steampowered.com/app/3920860/RPG_Developer_Bakin_Localization_Toolkit/)
- La table de chaînes s'exporte en xlsx puis se réimporte — [recherche résumant rpgbakin.com / wiki](https://rpgbakin.com/pukiwiki_en/?Localization+Toolkit=) (page du wiki non lue directement, à vérifier)
- Côté fans : BakinTranslate — [GitHub HNIdesu](https://github.com/HNIdesu/BakinTranslate) ; Translator++ — [Dreamsavior](https://dreamsavior.net/docs/rpg-developer-bakin/how-to-translate-rpg-developer-bakin/) ; guide Steam — [Steam guide](https://steamcommunity.com/sharedfiles/filedetails/?id=3557698878)

**SRPG Studio**
- Les textes se trouvent à deux endroits : le fichier `.srpgs`/`project.dat` et un `localization.dat`. Translator++ gère entièrement le `localization.dat`, jusqu'à la version 1.317 de SRPG Studio — [Dreamsavior, Translator++ 6.8.28](https://dreamsavior.net/translator-ver-6-8-28-srpg-studio-support/) ; [Patreon 8.3.25](https://www.patreon.com/posts/translator-ver-8-154383386)
- SRPG-ToolBox dépaquette et repaquette `project.dat`, puis extrait le texte en JSON selon une structure inspirée du format de localisation de SRPG Studio — [GitHub Sinflower/SRPG-ToolBox](https://github.com/Sinflower/SRPG-ToolBox)

**RPG Maker MZ (plugins)**
- DKTools Localization (MZ, v5.5.8, requiert DKTools ≥ 11.2.4) : nombre de langues illimité, audio et images par langue, fichiers JSON ou CSV — [dk-plugins.ru](https://dk-plugins.ru/mz/system/localization/) (page vide au fetch, infos issues de l'index de recherche) ; [fil RPG Maker Forums](https://forums.rpgmakerweb.com/threads/localization-translate-your-projects-into-several-languages.95225/)
- ODW Multi-Language System (MIT, gratuit ; la v2 requiert ODW Core) : JSON par dossier de langue, référencés par `${text code}` dans les textes d'événement et de base de données. Les caractères de contrôle doivent être doublement échappés (`\\C[0]`) et les clés peuvent s'imbriquer. Le plugin couvre la base de données, les commandes d'événement et les textes codés en dur du moteur. Il ajoute une police par langue, une scène de choix au démarrage et une option dans le menu — [itch.io ODW](https://opendigitalworld.itch.io/rmmz-plugin-multi-language-system)
- Outils d'export CSV « one-click » pour MV/MZ — [RPG Maker Forums](https://forums.rpgmakerweb.com/threads/one-click-import-export-multi-language-text-tool.181319/)

**WOLF RPG Editor v3**
- La v3.00 est sortie le 22/11/2022. Le passage interne à Unicode réduit les problèmes multilingues — [GameMakers](https://gamemakers.jp/article/2022_11_22_24582/) ; [窓の杜](https://forest.watch.impress.co.jp/docs/news/1458766.html)
- L'outil officiel payant « ウディタ翻訳サポートツール » est sorti en novembre 2023 (BOOTH) — [Game*Spark](https://www.gamespark.jp/article/2023/11/10/135839.html) ; [BOOTH](https://booth.pm/ja/items/5151747)
- Il requiert Wolf ≥ 3.00 et génère `WOLF_Translation_Text.xlsx`. Colonnes : Code (emplacement), Flag, Original, Type/Info, notes, et jusqu'à 10 colonnes de traduction. Il produit des copies complètes du jeu traduit (dossiers Translated1…). La fonction « mettre à jour l'Excel depuis les données actuelles » conserve les traductions et marque les changements `<NEW>`/`<MOVE>`. Il gère les drapeaux `<FILENAME>` et « demi-chasse uniquement », mais pas le texte intégré aux images — [Manuel officiel](https://silversecond.com/WOLF_Translation_tool/Manual.html) (lu via un résumé automatique ; la mention « max 100 mises à jour par cycle » est douteuse, à vérifier)
- Outils de fans historiques : wolftrans (Wolf v2) — [GitHub](https://github.com/elizagamedev/wolftrans)

**Pixel Game Maker MV**
- L'éditeur permet d'ajouter des onglets de langue sur les textes et de changer de langue à l'exécution — [Steam discussions](https://steamcommunity.com/app/837510/discussions/0/1845819570024702660/) ; [vidéo « Editor Tidbits - Localization »](https://www.youtube.com/watch?v=PjVQNW4KcZ8)
- pgm-translator extrait et réimporte les chaînes de `project.json` — [GitHub MillenniumEarl/pgm-translator](https://github.com/MillenniumEarl/pgm-translator)

### Inferences
- Pour Hoshi2Star, les moteurs dont les clés sont stables (Unreal namespace+key, Unity Key Id, code de position Wolf, `${code}` ODW) permettent une réinjection déterministe. Ren'Py est le seul où la clé dérive du contenu : une édition du jeu source rompt l'alignement si on ne mémorise pas l'ancien texte, d'où l'intérêt d'une TM sur le texte source.
- L'outil officiel Wolf et le toolkit Bakin produisent tous deux du xlsx à colonnes fixes. Un import/export xlsx dans ces formats serait un point d'interopérabilité direct.

### Gaps
- Format exact de l'export Bakin (colonnes, clés) : la page du wiki n'a pas été lue.
- Format de stockage exact de la traduction TyranoScript V600 (JSON interne ?) : non documenté sur la page lue.
- La notion d'« ID » dans le code Wolf et son format précis n'ont pas été vérifiés au-delà du résumé.
- Aucune source primaire trouvée sur un système de localisation natif dans Kirikiri/krkrz.
- Contenu détaillé de la page DKTools : fetch vide.

## Q2 — Syntaxe des placeholders et du texte enrichi à préserver

### Takeaway
Chaque moteur a sa propre syntaxe : `[var]`/`{b}` pour Ren'Py, `{0}`/`{Name}` + ICU pour Unreal, SmartFormat pour Unity, `String.format` et contexte pour Godot, tags `[..]` pour KAG/Tyrano, `\C[n]` pour RPG Maker et Wolf. Tous les outils imposent de conserver ces codes à l'identique.

### Cited Findings
- Ren'Py : interpolation de variables avec le drapeau `!t` pour traduire la valeur interpolée — [Ren'Py docs](https://www.renpy.org/doc/html/translation.html). Les text tags `{b}`, `{i}`, `{color}`, `{w}`… sont documentés dans [Ren'Py Text](https://www.renpy.org/doc/html/text.html) (page non relue ici)
- Unreal : arguments `{0}` et `{Name}`, pluriel et genre par règles ICU, balisage de texte enrichi — [Epic](https://dev.epicgames.com/documentation/en-us/unreal-engine/localization-overview-for-unreal-engine)
- Unity : les Smart Strings (SmartFormat) gèrent pluriel, genre, listes et conditions, par exemple `{count:plural:an Apple|{} apples}` — [Unity String Tables](https://docs.unity3d.com/Packages/com.unity.localization@1.5/manual/StringTables.html)
- Godot : `String.format()` à placeholders nommés permet au traducteur de réordonner les éléments — [Godot i18n](https://docs.godotengine.org/en/stable/tutorials/i18n/internationalizing_games.html)
- I2 : les paramètres (score, noms) s'insèrent via des callbacks qui respectent l'ordre des mots de la langue — [LinkedIn](https://www.linkedin.com/pulse/game-localization-unity-leveraging-i2-plugin-iman-irajdoost)
- RPG Maker MZ / ODW : codes `\C[n]` à doubler dans le JSON (`\\C[0]`) — [ODW](https://opendigitalworld.itch.io/rmmz-plugin-multi-language-system)
- Wolf : les codes de contrôle comme `\c[...]` et les noms de fichiers (`<FILENAME>`) doivent être préservés — [Manuel Wolf](https://silversecond.com/WOLF_Translation_tool/Manual.html)
- TyranoScript : les paramètres de macros personnalisées doivent être déclarés via Tag Registration pour être extraits — [tyranoscript.com](https://tyranoscript.com/usage/advance/translate)
- UnrealLocres : la colonne key ne doit jamais être modifiée — [README](https://github.com/akintos/UnrealLocres/blob/master/README.md)

### Inferences
- Le tokenizer de placeholders d'Hoshi2Star (ADR-002) devrait avoir un profil par moteur : `{…}` (Unity, Unreal, Godot, Ren'Py), `[…]` (Ren'Py, KAG, Tyrano), `\X[n]` (RM, Wolf).
- Ren'Py présente un conflit : `[var]` est une interpolation alors que `[...]` est un tag dans KAG et Tyrano.

### Gaps
- Aucune vérification directe du traitement de `[r]`, `[l]`, `[p]` et `[ruby]` par la fonction de traduction Tyrano (non documenté sur la page lue).

## Q3 — Formats d'export et interopérabilité avec les outils de TAO

### Takeaway
PO/POT est le format pivot côté occidental (Unreal, Godot, UnrealLocres, pylocres). Unity ajoute XLIFF, CSV et Google Sheets. Les moteurs japonais passent par Excel/xlsx (Wolf officiel, Bakin, CSV Tyrano), et Ren'Py utilise ses propres fichiers `.rpy`.

### Cited Findings
- Unreal : export et import `.po` depuis le Dashboard — [Epic](https://dev.epicgames.com/documentation/en-us/unreal-engine/localization-overview-for-unreal-engine)
- Godot : POT, PO et MO, avec `msgmerge` pour les mises à jour ; le CSV est aussi supporté — [Godot gettext](https://docs.godotengine.org/en/stable/tutorials/i18n/localization_using_gettext.html)
- Unity : XLIFF, CSV (avec ou sans commentaires) et Google Sheets — [Unity docs](https://docs.unity3d.com/Packages/com.unity.localization@1.5/manual/index.html) ; [CSV](https://docs.unity3d.com/Packages/com.unity.localization@0.9/manual/CSV.html)
- Locres de fans : CSV et POT (UnrealLocres), CSV et PO (pylocres), TSV (LocresStudio) — [UnrealLocres](https://github.com/akintos/UnrealLocres/blob/master/README.md) ; [pylocres](https://github.com/stas96111/pylocres) ; [LocresStudio](https://github.com/AcTePuKc/LocresStudio)
- Wolf : xlsx officiel, CSV/TXT en option — [Manuel](https://silversecond.com/WOLF_Translation_tool/Manual.html)
- Bakin : xlsx — [Steam/rpgbakin](https://rpgbakin.com/en/products/localization-toolkit) (à vérifier)
- Tyrano : CSV pour Excel — [tyranoscript.com](https://tyranoscript.com/usage/advance/translate)
- Translator++ prend en charge les fichiers MO (v8.3.25) — [Patreon](https://www.patreon.com/posts/translator-ver-8-154383386)

### Inferences
- Un import/export PO et XLIFF dans Hoshi2Star couvrirait Unity, Unreal et Godot. Un import/export xlsx à colonnes configurables couvrirait Wolf, Bakin et Tyrano.

### Gaps
- Aucune source primaire lue sur un export PO ou XLIFF natif dans Ren'Py. Ren'Py propose un « extract dialogue » (dialogue.tab), mais ce n'est pas vérifié ici.

## Q4 — Polices, longueur du texte, retour à la ligne (JA→EN/FR)

### Takeaway
Les moteurs modernes proposent des polices par langue (Ren'Py `translate style`, I2 Secondary terms, ODW, Godot fallback) et des outils de test d'expansion (pseudo-localisation Unity et Godot). Les moteurs japonais n'offrent guère de gestion automatique : Wolf ne modifie pas les polices, et les images contenant du texte restent à traiter à la main.

### Cited Findings
- Godot : les polices par défaut ne couvrent que Latin-1. Il faut prévoir Noto ou un système de fallback pour le CJK. La pseudo-localisation se règle dans les Project Settings, le rendu se prévisualise via View > Preview Translation et le BiDi est automatique — [Godot i18n](https://docs.godotengine.org/en/stable/tutorials/i18n/internationalizing_games.html)
- Unity : le package calcule le jeu de caractères de chaque table et l'exporte pour générer les atlas de police TextMeshPro — [Unity String Tables](https://docs.unity3d.com/Packages/com.unity.localization@1.5/manual/StringTables.html)
- Unreal : les jeux de données de culture (EFIGS, CJK…) sont sélectionnables pour réduire la taille du build — [Epic](https://dev.epicgames.com/documentation/en-us/unreal-engine/localization-overview-for-unreal-engine)
- Ren'Py : blocs `translate <lang> style/python` pour changer de police ; une traduction peut découper une longue réplique en plusieurs — [Ren'Py docs](https://www.renpy.org/doc/html/translation.html)
- I2 : polices et atlas par langue, RTL avec retour à la ligne — [LinkedIn](https://www.linkedin.com/pulse/game-localization-unity-leveraging-i2-plugin-iman-irajdoost)
- RPG Maker MZ / ODW : police spécifique par langue — [ODW](https://opendigitalworld.itch.io/rmmz-plugin-multi-language-system)
- Wolf : le support des polices varie selon la langue et l'outil ne touche pas aux polices. Les éléments marqués demi-chasse uniquement et le texte dans les images restent manuels — [Manuel Wolf](https://silversecond.com/WOLF_Translation_tool/Manual.html)
- Bakin : les images d'UI et les panneaux sont remplaçables par langue — [rpgbakin.com](https://rpgbakin.com/en/products/localization-toolkit)

### Inferences
- En JA→EN/FR, le texte s'allonge et les moteurs RM, Wolf et KAG coupent souvent à largeur fixe. Un contrôle QA de longueur ou de largeur par moteur (déjà prévu côté Hoshi2Star) reste nécessaire. Aucune source lue ne chiffre le taux d'expansion.

### Gaps
- Pas de source trouvée sur le retour à la ligne automatique natif de Wolf, KAG ou Tyrano pour l'anglais.

## Q5 — Moteurs sans support natif : pratiques des traducteurs de fans

### Takeaway
Le schéma est toujours le même : dépaqueter ou déchiffrer, extraire vers un format tabulaire (CSV/JSON/PO), traduire (souvent avec Translator++ ou un outil maison), réinjecter, puis repaqueter ou livrer un patch à charge prioritaire.

### Cited Findings
- Kirikiri : GARbro, KrkrExtract, FreeMote et KrkrzSceneManager — [Gist](https://gist.github.com/LukeZGD/64b0ecfb7eaccdfbb327ddfea4621723) ; KagEditor — [GitHub](https://github.com/begerard/KagEditor)
- Unreal (jeux publiés) : extraction du `.locres` puis CSV/PO et réimport via UnrealLocres ou pylocres — [UnrealLocres](https://github.com/akintos/UnrealLocres/blob/master/README.md)
- SRPG Studio : SRPG-ToolBox (unpack, JSON, repack) — [GitHub](https://github.com/Sinflower/SRPG-ToolBox) ; fil FEU — [feuniverse](https://feuniverse.us/t/srpg-studio-translation/2020)
- PGMMV : pgm-translator (`project.json`) — [GitHub](https://github.com/MillenniumEarl/pgm-translator)
- Bakin avant le DLC : BakinTranslate — [GitHub](https://github.com/HNIdesu/BakinTranslate)
- Wolf avant l'outil officiel : wolftrans — [GitHub](https://github.com/elizagamedev/wolftrans)
- Outil multi-moteurs : Translator++ (RPG Maker, Tyrano, KAG, SRPG Studio, Bakin) — [Dreamsavior KAG](https://dreamsavior.net/translating-kag-kirikiri-visual-novels/) ; [SRPG](https://dreamsavior.net/translator-ver-6-8-28-srpg-studio-support/). RuneTranslate (commercial, 2026) : Kirikiri, Tyrano, Wolf, Bakin — [runetranslate.com](https://runetranslate.com/)

### Inferences
- Translator++ et RuneTranslate sont les concurrents directs d'Hoshi2Star sur le périmètre des moteurs japonais.

### Gaps
- Mécanismes de chargement de patchs (`_P.pak` pour UE, `patch.xp3` pour krkr) : non confirmés par une source lue dans cette session.
