# Outils de traduction runtime / hook / overlay pour jeux (état au 25/09/2026)

Notes de recherche. Les métadonnées GitHub (dernier push, dernière release, étoiles, licence, statut archivé) ont été relevées via l'API GitHub le 25/09/2026 (`api.github.com/repos/<owner>/<repo>` et `/releases/latest`) ; elles sont citées avec l'URL du dépôt correspondant.

## Q1. Mécanisme : comment le texte est capturé et remplacé (in-place vs overlay), moteurs couverts, état de maintenance

### Takeaway
Quatre familles : (1) patch IL / hooks Harmony dans le process Unity (XUnity.AutoTranslator) avec remplacement **in-place** ; (2) injection DLL + hooks de fonctions de rendu texte (Textractor, LunaTranslator/LunaHook, Sugoi Hook) avec affichage **en fenêtre séparée/overlay**, l'« embedded translation » in-place restant minoritaire et fragile ; (3) OCR d'écran (Translumo, Game2Text, OCR de LunaTranslator) totalement agnostique au moteur mais overlay uniquement ; (4) outils « hybrides » par moteur (MTool, Translator3000 pour Ren'Py) qui chargent/interceptent les données du jeu pour traduire à la volée. VNTextPatch, KirikiriTools et GARbro sont des outils **statiques** (extraction/réinjection/archives) souvent utilisés en complément.

### Cited Findings

**XUnity.AutoTranslator (bbepis) — Unity**
- Plugin Unity installable via BepInEx (recommandé, variantes IL2CPP), MelonLoader (Mono et IL2CPP), IPA, UnityInjector ou ReiPatcher (autonome) ; embarque Mono.Cecil, Harmony, MonoMod — [README XUnity](https://github.com/bbepis/XUnity.AutoTranslator)
- Hooke les frameworks texte UGUI, NGUI, TextMeshPro (activés par défaut), TextMesh et IMGUI (désactivés par défaut), FairyGUI, moteur VN Utage — [README XUnity](https://github.com/bbepis/XUnity.AutoTranslator)
- Remplacement in-place du texte des composants ; option `ForceMonoModHooks` (détours runtime au lieu de Harmony) et `TextGetterCompatibilityMode` pour empêcher le jeu de relire le texte traduit — [README XUnity](https://github.com/bbepis/XUnity.AutoTranslator)
- Support IL2CPP déclaré incomplet : hooking réduit (refresh manuel), pas de TextGetterCompatibilityMode, pas d'IMGUI en IL2CPP, « many features remain unproven » — [README XUnity](https://github.com/bbepis/XUnity.AutoTranslator)
- Traduction de textures optionnelle (dump + rechargement d'images, identification par hash SHA1) — [README XUnity](https://github.com/bbepis/XUnity.AutoTranslator)
- Maintenance active : v5.6.2 publiée le 10/09/2026 ; dernier push 20/09/2026 ; ~3,4k étoiles ; MIT — [API GitHub / dépôt](https://github.com/bbepis/XUnity.AutoTranslator)
- Historique récent : v5.5.0 (27/10/2025, balise `XUAIGNORETREE`, Yandex API v2), v5.5.2 (06/03/2026, auth DeepL par header), v5.6 (10/04/2026, support UIElements, fix Papago), v5.6.1 (19/04/2026, fixes IL2CPP), v5.6.2 (police système dans OverrideFontTextMeshPro) — [Releases XUnity](https://github.com/bbepis/XUnity.AutoTranslator/releases)
- Chargeurs sous-jacents : BepInEx v5.4.23.5 (08/02/2026, LGPL-2.1, push 20/09/2026) ; MelonLoader v0.7.3 (14/05/2026, Apache-2.0, compat Il2Cpp et Mono) — [BepInEx](https://github.com/BepInEx/BepInEx), [MelonLoader](https://github.com/LavaGang/MelonLoader)

**LunaTranslator (HIllya51) — VN / jeux PC et émulateurs**
- Extraction principalement par HOOK (« compatible with almost all popular and niche visual novels »), plus OCR intégré (modèle maison + moteurs OCR en ligne/hors ligne), et presse-papiers — [README LunaTranslator](https://github.com/HIllya51/LunaTranslator), [Docs](https://docs.lunatranslator.org/en/)
- Hook d'émulateurs NS/PSP/PSV/PS2 — [Docs LunaTranslator](https://docs.lunatranslator.org/en/)
- « Embedded translation » : met le jeu en pause avant affichage, traduit, réécrit le texte en mémoire puis reprend ; seulement pour certains jeux ; « embedding may cause the game to crash » ; marche mieux sur moteurs Unicode (KiriKiri, Ren'Py, TyranoScript, RPG Maker MV) — [Docs embedtranslate](https://docs.lunatranslator.org/en/embedtranslate.html)
- Extras : TTS en/hors ligne, reconnaissance vocale Windows, segmentation japonaise, furigana, AnkiConnect, Yomitan — [README LunaTranslator](https://github.com/HIllya51/LunaTranslator)
- Très actif : v10.17.1.11 publiée le 23/09/2026 ; ~13,4k étoiles ; GPL-3.0 — [API GitHub / dépôt](https://github.com/HIllya51/LunaTranslator)

**Textractor (Artikash) et forks**
- L'hôte injecte `texthook` dans le process cible, communication par 2 pipes ; interception des fonctions de sortie texte (TextOut, GetGlyphOutline…) — [README Textractor](https://github.com/Artikash/Textractor)
- Accepte les codes `/H` (compatibles avec la plupart des codes AGTH), recherche automatique de hooks, auto-hook de nombreux moteurs « including some not supported by VNR » ; Windows 7+, Wine — [README Textractor](https://github.com/Artikash/Textractor)
- Dépôt original en sommeil : dernière release v5.2.0 du 31/01/2022, dernier push 15/03/2024 (non archivé) ; ~2,7k étoiles ; GPL-3.0 — [API GitHub / dépôt](https://github.com/Artikash/Textractor)
- Fork maintenu Chenx221/Textractor : push 23/09/2026, release « dev » du 16/01/2025, ~118 étoiles — [Chenx221/Textractor](https://github.com/Chenx221/Textractor)

**Sugoi Toolkit (MingShiba)**
- Traducteur hors ligne JA→EN basé sur un réseau (« CNN deep learning ») entraîné sur des VN et de la littérature japonaise ; modèle hors ligne inclus dans le toolkit — [Résultats Patreon/Internet Archive agrégés via recherche](https://archive.org/details/sugoi-translator-toolkit)
- V16 (Patreon le 24/01/2026, public le 15/02/2026) : traduction hors ligne multi-langues (chinois, coréen et « over 10 languages »), « Sugoi Hook » intégré pour la traduction VN temps réel, amélioration de la détection de texte pour les mangas — [Blog Sugoi V16](https://blog.sugoitoolkit.com/sugoi-toolkit-v16/)

**MTool**
- Supporte RPG Maker 2000/2003/XP/VX/VX Ace/MV/MZ (+ MKXP-Z, RGU), Wolf RPG, SMILE GAME BUILDER, RPG Developer Bakin, TyranoBuilder, Kirikiri 2/Z, Ren'Py, Visual Novel Maker, ChoiceScript, SRPG Studio, Pixel Game Maker MV ; « automatic game type recognition », glisser-déposer — [mtool.app](https://mtool.app/?lang=en)
- Fonctions de modification : valeurs/attributs du jeu, accélération, « skip repetitive grinding » — [mtool.app](https://mtool.app/?lang=en), [madinfinite](https://www.madinfinite.com/news/mtool-automatic-translation-for-rpgmaker-games/)
- Plateformes : Windows 7+ et Android 11+ — [mtool.app](https://mtool.app/?lang=en)
- Des fichiers de traduction MTool peuvent être repris dans Translator++ (tutoriel vidéo) — [YouTube](https://www.youtube.com/watch?v=IHDC6A_aEM8)

**OCR d'écran : Translumo, Game2Text**
- Translumo : traducteur d'écran temps réel pour jeux, sous-titres incrustés, toute app ; moteurs OCR Windows OCR (recommandé), Tesseract 5.2 et EasyOCR (legacy), combinables avec un modèle ML qui note chaque résultat OCR et choisit le meilleur ; langues EN/RU/JA/ZH-Hans/KO — [README Translumo](https://github.com/ramjke/Translumo/blob/master/README.md)
- Translumo actif : v.1.1.0 publiée le 22/09/2026 ; ~5,8k étoiles ; Apache-2.0 — [API GitHub / dépôt](https://github.com/ramjke/Translumo)
- Game2Text : OCR (Tesseract, OCR Space), « script-assisted OCR » (appariement avec des scripts de jeux), hook via Textractor, presse-papiers ; orienté apprentissage (Anki, Yomichan) ; Windows 10, macOS — [README Game2Text](https://github.com/mathewthe2/Game2Text)
- Game2Text : dernière release 0.4.9 d'avril 2021, dernier push 19/09/2023 — [API GitHub / dépôt](https://github.com/mathewthe2/Game2Text) ; la page fetchée le qualifiait d'« active », ce que contredisent ces dates.

**Ren'Py runtime**
- Translator3000 : traduction « en temps réel » dans les jeux Ren'Py 8.0.3+ ; support des autres versions dans la version payante (Patreon/Boosty) — [Translator3000](https://github.com/NyashniyVladya/Translator3000)
- Ren'Py n'a pas de bouton de traduction automatique intégré ; le système officiel repose sur des fichiers `tl/` — [Docs Ren'Py Translation](https://www.renpy.org/doc/html/translation.html)
- La plupart des autres outils Ren'Py sont statiques (extraction → MT → génération de fichiers de traduction / mod) : RenPy-Translator, renpy_auto_translate, RenPy_automatic_translations, renpy translator v2.4.1 — [huchukato/RenPy-Translator](https://github.com/huchukato/RenPy-Translator), [ricardol96/renpy_auto_translate](https://github.com/ricardol96/renpy_auto_translate), [Lemma Soft](https://lemmasoft.renai.us/forums/viewtopic.php?t=67823)

**Unreal Engine**
- Aucun équivalent établi de XUnity pour UE (runtime) n'a été trouvé ; les approches joueur sont statiques : extraction des `.locres` depuis `.pak` / IoStore (`.utoc/.ucas`) puis pak de surcharge `_P` dans `Content/Paks` (RuneTranslate, Translator++) — [RuneTranslate](https://runetranslate.com/engines/unreal), [Dreamsavior](https://dreamsavior.net/how-to-translate-unreal-engine-games-with-translator)
- Côté développeur : plugins UE « AI Localization Automator » (OpenAI, Claude, DeepSeek, Gemini, Ollama via Localization Dashboard) et « Runtime Text Translation AI » (DeepL) — [Georgy Dev](https://solutions.georgy.dev/ai-localization-automator), [Fab](https://www.fab.com/listings/cfb84fcf-4a92-4ecb-b72c-24717e542b2c)

**Outils statiques associés (VN)**
- VNTextPatch : extraction/réinjection pour 20+ formats (Ren'Py, Kirikiri, RealLive…) vers xlsx, JSON ou Google Sheets ; génère `names.xml` des noms de personnages ; word wrap pour polices monospace et proportionnelles — [VNTranslationTools](https://github.com/arcusmaximus/VNTranslationTools)
- VNTextProxy : DLL proxy qui charge une police custom, repositionne les caractères pour polices proportionnelles, étend la page de code SJIS / tunnelise des caractères non-SJIS — [VNTranslationTools](https://github.com/arcusmaximus/VNTranslationTools)
- VNTranslationTools archivé (dernière release v0.0.41, 20/02/2023) — [API GitHub / dépôt](https://github.com/arcusmaximus/VNTranslationTools)
- KirikiriTools : KirikiriDescrambler (scripts brouillés `FE FE 00 FF FE`), KirikiriUnencryptedArchive (`version.dll` qui fait accepter des `.xp3` non chiffrés, hash de table = 0), Xp3Pack — [KirikiriTools](https://github.com/arcusmaximus/KirikiriTools) ; archivé, release 1.7 du 12/04/2023 — [API GitHub](https://github.com/arcusmaximus/KirikiriTools)
- GARbro : navigateur/extracteur de ressources VN (.NET 4.7.2+), extraction F4 avec conversion images/audio, archives chiffrées non extractibles sans clé/identification du jeu — [crskycode/GARbro](https://github.com/crskycode/GARbro) ; original morkt dernière release v1.5.44 (31/03/2019), push 08/07/2024 ; fork crskycode actif (GARbro-Mod-1.0.2.2 du 19/07/2026, push 19/09/2026) — [morkt/GARbro](https://github.com/morkt/GARbro), [crskycode/GARbro](https://github.com/crskycode/GARbro)

### Inferences
- Le remplacement in-place fiable n'existe que là où l'outil vit dans le runtime du moteur (Unity via Harmony ; Ren'Py via script Python). Les hookers de DLL génériques traitent surtout l'affichage à côté du jeu ; leur mode embarqué dépend du moteur, de l'encodage et de la police.
- Unreal est un angle mort du runtime MT côté joueur : le travail passe par des outils statiques sur `.locres`.
- Pour un outil statique comme Hoshi2Star, MTool est le concurrent le plus direct par couverture moteurs (RPG Maker + Wolf + Bakin + Kirikiri…), tandis que XUnity/LunaTranslator visent des moteurs ou usages différents.

### Gaps
- Mécanisme interne exact de MTool (hook runtime vs lecture de fichiers vs chargement modifié) : la page officielle ne le détaille pas ; un tutoriel vidéo évoque un mode « FullAuto Translate » pour Wolf ([YouTube](https://www.youtube.com/watch?v=YCRtDgH1GyE)) mais non vérifié.
- Mécanisme de Translator3000 (hook de `say`/texte Ren'Py) non documenté sur la page consultée.
- Existence d'un mod runtime UE4SS de traduction automatique : non trouvé.
- LunaHook comme dépôt séparé : non confirmé (requête API sans résultat).

## Q2. Gestion des répétitions/texte parasite, codes de contrôle, balises rich text, retour à la ligne, polices

### Takeaway
XUnity est l'outil le plus outillé (anti-spam multi-niveaux, pré/post-processeurs regex, gestion rich text, redimensionnement UI, override de polices). Les hookers de VN gèrent répétitions/garbage via filtres configurables mais leur documentation détaillée n'a pas pu être consultée ; les limites d'affichage embarqué (encodage, police, largeur de ligne) sont reconnues explicitement par LunaTranslator.

### Cited Findings
- XUnity : délai d'1 s avant mise en file pour vérifier si le texte change encore ; détection du texte « défilant » ; détection de spam frame par frame (seuil 90 frames) ; vérification de la présence de symboles de la langue source — [README XUnity](https://github.com/bbepis/XUnity.AutoTranslator)
- XUnity : `_Substitutions.txt` (remplacements de noms/termes avant traduction), `_Preprocessors.txt`, `_Postprocessors.txt`, regex `r:` et « splitter regex » `sr:` pour traduire un texte par morceaux — [README XUnity](https://github.com/bbepis/XUnity.AutoTranslator)
- XUnity : gestion du rich text avec modes de persistance configurables ; `IgnoreWhitespaceInDialogue`, `IgnoreWhitespaceInNGUI`, `ForceSplitTextAfterCharacters` ; `MaxCharactersPerTranslation` 200 par défaut — [README XUnity](https://github.com/bbepis/XUnity.AutoTranslator)
- XUnity : `OverrideFont`, `FallbackFontTextMeshPro`, `OverrideFontTextMeshPro`, `EnableUIResizing`/`ForceUIResizing`, fichier `resizer.txt` de règles de taille de police par composant — [README XUnity](https://github.com/bbepis/XUnity.AutoTranslator)
- XUnity : balise `XUAIGNORETREE` (v5.5.0) pour exclure une branche de GameObjects — [Releases XUnity](https://github.com/bbepis/XUnity.AutoTranslator/releases)
- LunaTranslator embarqué : charabia dû au jeu de caractères et aux polices ; nombre de caractères par ligne limité ; traduction lente = saccades ; réglages : police, conversion de caractères (simplifié→traditionnel/kanji japonais), limite de caractères par ligne, taille de police — [Docs embedtranslate](https://docs.lunatranslator.org/en/embedtranslate.html)
- VNTextPatch : word wrap monospace et proportionnel car « most visual novel engines do not support automatic word wrapping » ; VNTextProxy pour polices proportionnelles et caractères hors SJIS — [VNTranslationTools](https://github.com/arcusmaximus/VNTranslationTools)
- Translumo : combinaison de plusieurs OCR avec sélection par modèle ML pour limiter les erreurs de reconnaissance ; Tesseract jugé lent et fautif, EasyOCR lourd (GPU) — [README Translumo](https://github.com/ramjke/Translumo/blob/master/README.md)

### Inferences
- Les problèmes que Hoshi2Star traite en statique (placeholders `\V[n]`, wrap, police) sont les mêmes que ceux des outils runtime ; XUnity les résout par pré/post-processeurs regex, ce qui est proche d'une tokenisation de placeholders.
- L'OCR ajoute une couche d'erreurs (reconnaissance) absente des approches hook/statique.

### Gaps
- Liste et comportement des extensions Textractor (« Remove Repeated Characters/Phrases », Regex Filter, Replacer, Thread Linker, extensions de traduction) : la page wiki consultée n'était qu'un index ; non vérifié.
- Filtres de pré-traitement de texte de LunaTranslator (dédoublonnage, filtres regex) : page de doc non consultée.
- Gestion des codes de contrôle RPG Maker / Wolf par MTool : non documentée publiquement.

## Q3. Cache / mémoire de traduction, fichiers manuels, partage entre utilisateurs

### Takeaway
XUnity dispose du système le plus explicite (fichiers texte générés + surcharges manuelles, prioritisables, zippables, et une consigne de redistribuer le cache pour épargner les endpoints). Translator3000 met en cache ; LunaTranslator cache non documenté ici. Le partage communautaire de caches XUnity est explicitement encouragé par les règles de redistribution.

### Cited Findings
- XUnity : `_AutoGeneratedTranslations.txt` stocke les traductions automatiques ; fichiers de traduction manuels avec directives de portée (`#set level`, `#set exe`, `#set required-resolution`) et possibilité de les empaqueter en ZIP ; hotkey ALT+R recharge les fichiers ; ALT+T bascule original/traduit — [README XUnity](https://github.com/bbepis/XUnity.AutoTranslator)
- XUnity : pour la redistribution, fournir `_AutoGeneratedTranslations.txt` « with maximum translations » pour minimiser l'usage des endpoints en ligne ; ne pas redistribuer avec des endpoints propriétaires non par défaut ; `MaxCharactersPerTranslation` ≤ 400 pour les redistributeurs — [README XUnity](https://github.com/bbepis/XUnity.AutoTranslator)
- XUnity : API `TryTranslate()` (cache seul), `TranslateAsync()`, `ITranslationRegistry`, `ITranslateEndpoint` pour endpoints custom — [README XUnity](https://github.com/bbepis/XUnity.AutoTranslator)
- XUnity : dictionnaire statique interne (~2 000 phrases JA-EN, `UseStaticTranslations`) et dédoublonnage des requêtes identiques concurrentes — [README XUnity](https://github.com/bbepis/XUnity.AutoTranslator)
- Translator3000 : « When reading the same phrases again, there will be no freezes, because translations are cached » — [Translator3000](https://github.com/NyashniyVladya/Translator3000)
- LunaTranslator : glossaire « Proper Noun Translation » injecté dans les prompts LLM — [Docs LLM LunaTranslator](https://docs.lunatranslator.org/en/guochandamoxing.html)
- MTool → Translator++ : les fichiers de traduction MTool peuvent être importés dans Translator++ — [YouTube](https://www.youtube.com/watch?v=IHDC6A_aEM8)

### Inferences
- Le cache XUnity est une mémoire de traduction « exact match » par jeu, en texte brut, éditable à la main et partageable : c'est le mode de diffusion de fait des « patchs MT » Unity. Il n'y a ni fuzzy match ni TM inter-projets, contrairement à Hoshi2Star.

### Gaps
- Format et partage de cache de LunaTranslator (historique de traduction, base locale) : non vérifié.
- Existence d'une base partagée en ligne de traductions MTool : non mentionnée sur la page officielle ; non vérifié.

## Q4. Backends MT/LLM, limitation de débit, batching, contexte

### Takeaway
En 2026, les backends classiques (Google non authentifié, DeepL, Bing, Papago, Baidu, Yandex) restent la base ; les LLM arrivent soit nativement (LunaTranslator : très large liste de fournisseurs + SakuraLLM, historique de contexte, glossaire), soit via plugins tiers (XUnity + OpenAI/Ollama/LM Studio). XUnity bride volontairement les requêtes (séquentiel, plafonds). Des modèles spécialisés hors ligne (Sugoi, SakuraLLM) servent de backend local.

### Cited Findings
- XUnity intégré : Google (non auth, instable), Google authentifié, DeepL (libre et API), Bing et Azure, Papago, Baidu, Yandex, Watson, LEC Power Translator 15, LingoCloud, endpoint HTTP custom ; avertissement : les traducteurs non authentifiés peuvent casser à tout moment — [README XUnity](https://github.com/bbepis/XUnity.AutoTranslator)
- XUnity anti-spam : plafond de 8 000 requêtes par session, pas de requêtes concurrentes, arrêt si >4 000 en file, arrêt après 5 réponses vides consécutives, arrêt si file saturée >60 s ; `EnableBatching` pour les endpoints qui le supportent — [README XUnity](https://github.com/bbepis/XUnity.AutoTranslator)
- XUnity ne contient pas de traducteur LLM natif dans les releases récentes (jusqu'à v5.6.2) — [Releases XUnity](https://github.com/bbepis/XUnity.AutoTranslator/releases) ; le README renvoie à des implémentations tierces pour Sugoi, OpenAI/ChatGPT, Ollama, Pollinations.ai — [README XUnity](https://github.com/bbepis/XUnity.AutoTranslator)
- Plugin tiers XUnity.AutoTranslate.LlmTranslators : endpoints OpenAI, Ollama, LM Studio ; prompts, glossaire, concurrence supérieure à l'endpoint Custom ; config YAML — [joshfreitas1984/XUnity.AutoTranslate.LlmTranslators](https://github.com/joshfreitas1984/XUnity.AutoTranslate.LlmTranslators) ; fork à fichier YAML unique — [gickass/XUnity.LLMEndpoint](https://github.com/gickass/XUnity.LLMEndpoint) ; autres : proxy OpenAI-compatible [ReturnToFirst/XUnity.AutoTranslator-openai](https://github.com/ReturnToFirst/XUnity.AutoTranslator-openai), [NothingNullNull/XUnity.AutoLLMTranslator](https://github.com/NothingNullNull/XUnity.AutoLLMTranslator)
- LunaTranslator LLM : OpenAI, Gemini, Nvidia, Claude, Cohere, x.ai, Groq, OpenRouter, Mistral, Azure, Cerebras ; DeepSeek, Xiaomi MiMo, Alibaba Bailian, Volcano Engine, Moonshot, Zhipu, iFlytek Spark, Hunyuan, Qianfan, MiniMax ; modèles hors ligne Hy-MT2 (Tencent) et SakuraLLM — [Docs LLM LunaTranslator](https://docs.lunatranslator.org/en/guochandamoxing.html)
- LunaTranslator : contexte = nombre configurable de messages source/traduits précédents (désactivable) ; prompts avec variables `{sentence}`, `{srclang}`, `{tgtlang}` ; température, max tokens, top_p, reasoning effort ; suggestion d'agrégateur new-api pour plusieurs clés — [Docs LLM LunaTranslator](https://docs.lunatranslator.org/en/guochandamoxing.html)
- Translumo : DeepL (recommandé), Google, Yandex, Papago — [README Translumo](https://github.com/ramjke/Translumo/blob/master/README.md)
- Game2Text : DeepL, Papago, Google — [README Game2Text](https://github.com/mathewthe2/Game2Text)
- Sugoi : modèle hors ligne JA→EN ; V16 hors ligne multi-langues — [Blog Sugoi V16](https://blog.sugoitoolkit.com/sugoi-toolkit-v16/), [Internet Archive](https://archive.org/details/sugoi-translator-toolkit)
- MTool : « multiple professional translation engines » et « complete offline support », sans noms de backends — [mtool.app](https://mtool.app/?lang=en)
- SakuraLLM : LLM JA→ZH spécialisé light novels/Galgame ; GPL-3.0 ; ~4,8k étoiles ; dernière release v1.1.0 (10/05/2024), dernier push 23/07/2026 — [SakuraLLM](https://github.com/SakuraLLM/SakuraLLM)
- GalTransl : solution de traduction automatisée de VN supportant GPT-4/Claude/DeepSeek/Sakura ; v8.0.0 publiée le 18/09/2026 ; GPL-3.0 ; ~2,3k étoiles (statique, via extraction de scripts) — [GalTransl](https://github.com/GalTransl/GalTransl)

### Inferences
- Le runtime traduit ligne par ligne au fil de l'affichage : le contexte LLM se limite à l'historique récent (LunaTranslator) et le batching est opportuniste (XUnity). Un pipeline statique peut envoyer des lots plus larges avec glossaire et contexte de scène, avantage qualité/coût.
- SakuraLLM et GalTransl ciblent surtout JA→ZH ; l'écosystème LLM VN est centré sur la communauté chinoise.

### Gaps
- Chiffres de latence/coût comparés runtime vs batch : pas de source trouvée.
- Backends exacts de MTool (DeepL ? LLM ?) : non documentés publiquement.
- Détails de Sugoi en 2026 (LLM ou toujours modèle Fairseq/CNN) : non vérifiés au-delà du blog V16.

## Q5. Avantages/inconvénients vs extraction statique (qualité, édition, distribution, légal) ; modèle économique, communauté, limites

### Takeaway
Le runtime gagne sur l'immédiateté et la couverture « sans connaissance du format » ; il perd sur la qualité (phrase isolée, pas de relecture), l'édition, la stabilité (crashs, IL2CPP, endpoints gratuits qui cassent) et la finition d'affichage. Côté économie : open source + dons (XUnity MIT, LunaTranslator GPL + Patreon/Afdian, Translumo Apache), freemium par abonnement (MTool), accès anticipé Patreon (Sugoi, Translator3000 payant pour anciennes versions de Ren'Py).

### Cited Findings
- XUnity avertit que les traducteurs non authentifiés « may break at any time » et impose des règles de redistribution anti-spam — [README XUnity](https://github.com/bbepis/XUnity.AutoTranslator)
- LunaTranslator : le mode embarqué peut faire crasher le jeu, provoquer du charabia et des saccades si la traduction est lente — [Docs embedtranslate](https://docs.lunatranslator.org/en/embedtranslate.html)
- Ren'Py : la MT est « highly frowned upon » pour les sorties officielles à cause de résultats médiocres (propos rapportés par le résumé de recherche sur le sujet) — [Docs Ren'Py Translation](https://www.renpy.org/doc/html/translation.html) (non vérifié mot pour mot sur la page)
- Patch statique non destructif pour UE : pak `_P` chargé par-dessus les données d'origine sans modifier l'installation — [Dreamsavior](https://dreamsavior.net/how-to-translate-unreal-engine-games-with-translator) ; idem Kirikiri via `version.dll` + `.xp3` non chiffré — [KirikiriTools](https://github.com/arcusmaximus/KirikiriTools)
- LunaTranslator : financement par dons Patreon et Afdian, pas de palier payant — [README LunaTranslator](https://github.com/HIllya51/LunaTranslator)
- Sugoi : MingShiba travaille à plein temps sur Sugoi ; Patreon à partir de 6 $/mois ; versions d'abord pour les abonnés, public ~3 semaines après (V16 : 24/01 → 15/02/2026) — [Patreon MingShiba](https://www.patreon.com/mingshiba), [Blog Sugoi V16](https://blog.sugoitoolkit.com/sugoi-toolkit-v16/)
- MTool : « download first to try it out, then subscribe if you like it » (freemium par abonnement), Windows + Android — [mtool.app](https://mtool.app/?lang=en)
- Translator3000 : version GitHub pour Ren'Py 8.0.3+, autres versions dans la version payante Patreon/Boosty — [Translator3000](https://github.com/NyashniyVladya/Translator3000)
- Popularité (étoiles GitHub au 25/09/2026) : LunaTranslator ~13,4k, BepInEx ~8,7k, Translumo ~5,8k, SakuraLLM ~4,8k, MelonLoader ~4,2k, XUnity ~3,4k, GARbro morkt ~3,3k, Textractor ~2,7k, GalTransl ~2,3k — [API GitHub, dépôts cités ci-dessus](https://github.com/HIllya51/LunaTranslator)
- Outils statiques clés archivés : VNTranslationTools (02/2023) et KirikiriTools (04/2023) ; Textractor original sans release depuis 01/2022 — [VNTranslationTools](https://github.com/arcusmaximus/VNTranslationTools), [KirikiriTools](https://github.com/arcusmaximus/KirikiriTools), [Textractor](https://github.com/Artikash/Textractor)

### Inferences
- Avantages runtime : zéro extraction, fonctionne sur jeux chiffrés/obscurs, résultat immédiat, adapté aux joueurs. Inconvénients : qualité ligne à ligne, pas de QA ni relecture structurée, dépendance à un endpoint pendant le jeu, instabilité, affichage approximatif.
- Avantages statique : relecture, glossaire, TM, QA, patch distribuable sans dépendance réseau ; inconvénients : parseur par moteur, chiffrement, effort initial.
- Juridique : les deux approches restent des modifications non autorisées d'œuvres protégées ; la distribution d'un cache XUnity ou d'un pak de surcharge contient du texte traduit dérivé, comme un patch statique. Aucune source juridique n'a été consultée : à traiter comme hypothèse.
- Positionnement Hoshi2Star : l'espace « statique + LLM + TM + QA » pour RPG Maker/Wolf est occupé côté joueur par MTool (freemium, fermé) et Translator++, côté VN chinois par GalTransl ; les outils runtime sont complémentaires plutôt que concurrents directs.

### Gaps
- Analyses juridiques (DMCA/droit d'auteur UE, contournement de chiffrement pour Kirikiri/XP3) : non recherchées.
- Discussions communautaires F95zone/Reddit/Fuwanovel sur la qualité MT runtime vs patchs humains : non consultées (limite de budget de recherche).
- Tarifs exacts MTool et nombre d'abonnés Patreon Sugoi/LunaTranslator : non trouvés.
