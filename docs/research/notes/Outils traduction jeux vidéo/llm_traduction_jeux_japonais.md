# LLM et traduction automatique pour jeux japonais (VN, RPG) — état 2026

Notes de recherche du 2026-09-25. Les résumés de pages ont été produits via WebFetch (modèle de synthèse) : les chiffres sont repris tels que rapportés et les points douteux sont signalés.

## 1. Modèles spécialisés et benchmarks : qui est en tête ?

### Takeaway
Le seul benchmark public dédié aux VN (VNTL, JA→EN) place encore les modèles cloud en tête (GPT-4o, Claude 3 Opus/3.5 Sonnet, DeepSeek V3, autour de 0,74-0,75). Les meilleurs modèles locaux (Qwen 2.5 72B/32B, VNTL Gemma2 27B, Shisa V2 12B) restent 3 à 5 points derrière. Mais ce classement est vieillissant et saturé : il ne contient pas les modèles frontière de 2025-2026, et sa métrique (similarité d'embeddings) sépare mal les bonnes traductions entre elles. Côté JA→ZH, l'écosystème repose sur SakuraLLM et GalTransl-7B/14B.

### Cited Findings
**VNTL leaderboard (JA→EN, VN)**
- Le classement VNTL note les LLM sur 256 lignes de VN (128 de *Senren\*Banka*, 128 de *Mashiro-iro Symphony*). Il vise l'exactitude sémantique plutôt que la conformité à la structure de la référence. — [HF lmg-anon/vntl-leaderboard](https://huggingface.co/datasets/lmg-anon/vntl-leaderboard)
- Le classement original de lmg-anon n'est plus mis à jour depuis janvier 2025. Une reprise tierce (« vntl-leaderboard-2026 », utilisateur yukobayashi500) compte 91 entrées : les 87 d'origine plus 4 nouveaux modèles, testés le 7 septembre 2026. — [HF yukobayashi500/vntl-leaderboard-2026](https://huggingface.co/datasets/yukobayashi500/vntl-leaderboard-2026)
- Méthode de la reprise 2026 : similarité cosinus (all-mpnet-base-v2), chrF et BLEU. Un score ≤ 0,5 par échantillon compte comme 0. IC à 95 %. Température 0 (décodage greedy), 150 tokens maximum, arrêt sur `\n`, donc une ligne à la fois. — [HF vntl-leaderboard-2026](https://huggingface.co/datasets/yukobayashi500/vntl-leaderboard-2026)
- Top du classement (2026) : GPT-4o mai 2024 0,7516 · GPT-4o août 2024 0,7497 · Claude 3 Opus 0,7459 · Claude 3.5 Sonnet (juin) 0,7440 · DeepSeek V3 0,7424 · Claude 3.5 Sonnet (oct.) 0,7280 · Nemotron 340B 0,7279 · GPT-4o mini 0,7223 · Grok 2 0,7160. — [HF vntl-leaderboard-2026](https://huggingface.co/datasets/yukobayashi500/vntl-leaderboard-2026)
- Modèles locaux (quantifiés) : Qwen 2.5 72B Q5_K_M 0,7079 (#12) · VNTL Gemma2 27B 0,7067 (#13) · Qwen 2.5 32B 0,7066 (#14) · Shisa V2 Mistral Nemo 12B 0,6985 (#17) · VNTL Llama3 8B v2 Q8 0,6952 (#19) · Sugoi 14B Ultra Q4_K_M 0,6827 (#31) · Gemma3 12B 0,6759 (#37) · Qwen3 8B 0,6192 (#70). — [HF vntl-leaderboard-2026](https://huggingface.co/datasets/yukobayashi500/vntl-leaderboard-2026)
- Selon le mainteneur de la reprise, Sugoi 14B Ultra fait moins bien que VNTL Llama3 8B v2, pourtant plus ancien de 20 mois, peut-être à cause de la quantification (Q4 contre Q8). — [HF vntl-leaderboard-2026](https://huggingface.co/datasets/yukobayashi500/vntl-leaderboard-2026)
- Le README du dépôt vntl-benchmark est incomplet (« still in development »). Configurations YAML, jeux de données Senren et Mashiro. — [GitHub lmg-anon/vntl-benchmark](https://github.com/lmg-anon/vntl-benchmark)
- lmg-anon publie des fine-tunes dédiés aux VN, par exemple vntl-llama3-8b-v2-hf. — [HF lmg-anon/vntl-llama3-8b-v2-hf](https://huggingface.co/lmg-anon/vntl-llama3-8b-v2-hf)

**Sugoi (MingShiba)**
- Sugoi 14B Ultra est basé sur Qwen 2.5 14B Instruct (licence Apache 2.0). Le modèle annonce un BLEU de 21,38 contre 13,67 pour son prédécesseur quantifié, et un bon suivi d'instructions sur le texte chargé de crochets typique de RPG Maker. — [HF sugoitoolkit/Sugoi-14B-Ultra-HF](https://huggingface.co/sugoitoolkit/Sugoi-14B-Ultra-HF) ; [blog Sugoi](https://blog.sugoitoolkit.com/sugoi-llm-14b-ultra/)
- Consignes d'usage : un system prompt de « professional localizer » est obligatoire (il autorise argot et vocabulaire NSFW). Un contexte trop long « empoisonne » la sortie : garder environ 10 lignes. — [HF Sugoi-14B-Ultra](https://huggingface.co/sugoitoolkit/Sugoi-14B-Ultra-HF)
- Distribution via Patreon (versions 14B/32B) et Hugging Face (GGUF). — [Patreon MingShiba](https://www.patreon.com/mingshiba/posts/sugoi-llm-14b-135007523) ; [HF GGUF](https://huggingface.co/sugoitoolkit/Sugoi-14B-Ultra-GGUF)
- Sugoi LLM se connecte à LunaTranslator. — [blog Sugoi « connecting »](https://blog.sugoitoolkit.com/connecting/)

**SakuraLLM (JA→ZH)**
- Modèle JA→ZH adapté aux light novels et galges. Versions v1.0 : Sakura-1.5B/7B/14B-Qwen2.5-v1.0 (7B : 8-10 Go de VRAM ; 14B : 11-16 Go). Versions héritées en 32B, plus des variantes GalTransl. — [GitHub SakuraLLM](https://github.com/SakuraLLM/SakuraLLM)
- Le prompt injecte un glossaire au format `src->dst #info`. Réglages recommandés : température 0,1, top-p 0,3, frequency penalty 0,1-0,2 en cas de dégradation. — [GitHub SakuraLLM](https://github.com/SakuraLLM/SakuraLLM)
- Licence CC BY-NC-SA 4.0 : usage commercial interdit. Les traductions doivent être signalées clairement comme MTL et indiquer la version du modèle. — [GitHub SakuraLLM](https://github.com/SakuraLLM/SakuraLLM)
- Limites reconnues par le projet : pronoms (genre, nombre, pragmatique) et compréhension du contexte. — [GitHub SakuraLLM](https://github.com/SakuraLLM/SakuraLLM)
- Écosystème intégré : GalTransl, AiNiee (RPG), SakuraTranslator (Unity), RPGMaker_LLaMA_Translator, manga-image-translator, BallonsTranslator. — [GitHub SakuraLLM](https://github.com/SakuraLLM/SakuraLLM)
- Sakura-GalTransl 7B est optimisé pour les VN : il préserve les retours à la ligne, les caractères de contrôle et les ruby. — [HF SakuraLLM/GalTransl-v1](https://huggingface.co/SakuraLLM/GalTransl-v1)

**Shisa (Shisa.AI)**
- Shisa V2 : modèles bilingues JA/EN de 7B à 70B (et un 405B), poids ouverts, présentés comme utilisables commercialement. Annoncé comme état de l'art sur les benchmarks japonais à chaque taille. — [blog Shisa V2](https://blog.shisa.ai/posts/shisa-v2/) ; [HF shisa-v2-llama3.1-405b](https://huggingface.co/shisa-ai/shisa-v2-llama3.1-405b)
- Shisa V2.1 : modèles plus petits. — [blog Shisa V2.1](https://blog.shisa.ai/posts/shisa-v2.1/)
- JP-TL-Bench (Lin et Lensenmayer, Shisa.AI, arXiv, 1er janvier 2026) : comparaisons LLM par paires sans référence contre un jeu d'ancres versionné, agrégation Bradley-Terry, score « LT » de 0 à 10. Constat central : en JA↔EN, la vraie question est souvent « laquelle de ces deux bonnes traductions est la meilleure ? » plutôt que « est-ce acceptable ? ». — [arXiv 2601.00223](https://arxiv.org/abs/2601.00223)
- JP-TL-Bench utilise Gemini-2.5-flash comme juge par défaut (température 0). Top 5 JA→EN (score LT / taux de victoire) : Gemini-3-flash-preview 9,98 / 97,4 % · Qwen3-235B-A22B 9,93 / 93,2 % · Gemini-2.5-pro 9,94 / 95,7 % · GPT-OSS-120B 9,92 / 92,6 % · DeepSeek-V3.1-Terminus 9,89 / 90,9 %. — [arXiv 2601.00223 (HTML)](https://arxiv.org/html/2601.00223)
- Top 5 EN→JA : Gemini-3-flash-preview 9,99 · Gemini-3-pro-preview 9,99 · Gemini-2.5-pro 9,97 · Gemini-2.5-flash 9,96 · DeepSeek-V3.1-Terminus 9,94. — [arXiv 2601.00223 (HTML)](https://arxiv.org/html/2601.00223)
- Limites déclarées par les auteurs : les scores dépendent du juge et du prompt, et un autre juge peut produire un autre ordre. Gemini-2.5-flash est à la fois juge et ancre, d'où un biais d'auto-préférence possible, même si des vérifications humaines ponctuelles confirment la tête Gemini. — [arXiv 2601.00223 (HTML)](https://arxiv.org/html/2601.00223)
- *Réserve : texte général JA↔EN, pas spécifiquement du VN ou du jeu. Les tableaux ont été extraits par résumé automatique ; les positions de GPT-5 et de Claude 4.x n'ont pas été extraites.*

### Inferences
- Le VNTL, à une ligne par requête, ne mesure ni l'usage du contexte, ni le respect des placeholders, ni la validité d'un format structuré. Son classement dit donc peu de choses sur les performances en pipeline de jeu.
- Tous les scores entre 0,70 et 0,75 sont proches. Avec une métrique d'embeddings et 256 échantillons, les écarts de 1 à 2 points sont probablement dans le bruit (l'IC à 95 % existe mais sa valeur n'a pas été extraite).
- Les modèles frontière de 2025-2026 (GPT-5, Claude 4.x, Gemini 2.5/3, DeepSeek R1/V3.x) sont absents des classements VN publics trouvés. Pour choisir un modèle aujourd'hui, les tests de la communauté ou des évaluations maison restent nécessaires.
- Sur le plan des licences, SakuraLLM (NC, marquage MTL obligatoire) est plus contraignant que Sugoi 14B Ultra (Apache 2.0) ou Shisa V2 (permissif).

### Gaps
- Le Sugoi Translator historique (Fairseq JA→EN hors ligne) n'a pas de source directe dans ces notes.
- Scores de GPT-5, Claude 4.x et Gemini sur un benchmark spécifique VN ou jeu : aucune source trouvée. Seul JP-TL-Bench (texte général) couvre Gemini 3, Qwen3-235B, GPT-OSS et DeepSeek V3.1, jugés par un LLM.
- Position de GPT-5 et de Claude 4.x dans JP-TL-Bench : non extraite.
- Aucun benchmark public JA→ZH équivalent pour Sakura et GalTransl n'a été trouvé.
- DeepSeek, Gemini et Llama n'apparaissent que via le VNTL ou comme backends compatibles OpenAI des outils.

## 2. Techniques de pipeline

### Takeaway
Les outils matures convergent vers la même recette. On masque les codes de contrôle, on envoie des lots bornés par la scène ou l'événement avec les lignes précédentes et le locuteur, et on n'injecte que les entrées de glossaire présentes dans le lot. La sortie est structurée (JSON, TSV ou lignes numérotées). Des contrôles automatiques (japonais résiduel, retours à la ligne, ratio de longueur, répétitions) déclenchent une nouvelle tentative. Enfin, un cache verrouillable, un retour à la ligne automatique et une relecture complètent le tout.

### Cited Findings
- **Masquage des codes de contrôle** : rpgmaker-translator remplace les codes RPG Maker (`\N[n]`…) par des placeholders `«CODE»` avant l'appel au LLM, puis les restaure. — [GitHub MoriTranslates/rpgmaker-translator](https://github.com/MoriTranslates/rpgmaker-translator)
- **Lots bornés par l'événement** : « Batches never cross event boundaries ». Un événement de 25 lignes avec batch=10 part en 10/10/5. — [GitHub rpgmaker-translator](https://github.com/MoriTranslates/rpgmaker-translator)
- **Contexte glissant** : les 10 dernières traductions de l'événement courant sont envoyées comme paires user/assistant pour la cohérence des pronoms et du ton, puis remises à zéro à chaque frontière d'événement. — [GitHub rpgmaker-translator](https://github.com/MoriTranslates/rpgmaker-translator)
- **Glossaire à deux couches** : un glossaire général d'environ 100 entrées préréglées, plus un glossaire projet construit automatiquement à partir des traductions de la base de données. Seuls les termes présents dans la requête sont injectés (« smart injection »). — [GitHub rpgmaker-translator](https://github.com/MoriTranslates/rpgmaker-translator)
- **Locuteur et genre** : le genre est attribué à chaque acteur et les en-têtes de dialogue sont associés aux personnages, pour des pronoms corrects (« pronoun-aware »). — [GitHub rpgmaker-translator](https://github.com/MoriTranslates/rpgmaker-translator)
- **Retour à la ligne** : un plugin JS coupe le texte au rendu d'après les mesures en pixels, y compris dans Window_Help (descriptions d'objets et de compétences). — [GitHub rpgmaker-translator](https://github.com/MoriTranslates/rpgmaker-translator)
- **Retry** : en cas de japonais résiduel, nouvelle tentative avec un prompt « plus fort ». Modèle local recommandé : « Qwen 3.5:9b » via Ollama. Export en ZIP de patch. — [GitHub rpgmaker-translator](https://github.com/MoriTranslates/rpgmaker-translator). *Réserve : le nom du modèle vient d'un résumé automatique et n'a pas été vérifié.*
- **DazedMTLTool (miroir GitHub, version ancienne)** : traite les JSON MV depuis `/files` vers `/translated`. Le style se règle dans un `prompt.txt` (exemples few-shot, au prix d'un coût accru). Les traductions partielles sont sauvegardées pour reprendre sans repayer. L'outil est peu testé selon son auteur. — [GitHub sh2288/DazedMTLTool](https://github.com/sh2288/DazedMTLTool)
- **Détection de problèmes (GalTransl)** : japonais résiduel (残留日文), fréquence de mot trop élevée (plus de 20 répétitions), retour à la ligne perdu (丢失换行) ou ajouté (多加换行), traduction plus de 1,3 fois plus longue que le japonais (比日文长). — [GitHub xd2333/GalTransl](https://github.com/xd2333/GalTransl)
- **Dictionnaires (GalTransl)** : dictionnaire pré-traduction, post-traduction et « GPT dictionary » injecté dans le prompt. Les dictionnaires conditionnels évitent les remplacements abusifs. S'y ajoute un dictionnaire de noms de personnages. — [GitHub xd2333/GalTransl](https://github.com/xd2333/GalTransl)
- **Cache et reprise (GalTransl)** : cache en temps réel et reprise au point d'arrêt. Modèles de prompts ForGal (JSON/TSV) et ForNovel. — [GitHub xd2333/GalTransl](https://github.com/xd2333/GalTransl)
- Un fork, « GalTransl Suite » (GPL-3.0, basé sur l'amont v7.4.0), ajoute un cache à verrouillage manuel, un éditeur bilingue, une chaîne complète (détection du moteur → extraction → LLM → réinjection → patch), une file de tâches et des retries sur sortie malformée. — [GitHub JunjieLin98/GalTransl](https://github.com/JunjieLin98/GalTransl)
- **Contexte et glossaire (LunaTranslator)** : variables de prompt `{contextOriginal[N]}`, `{contextTranslation[N]}` et `{contextBoth[N]}` pour les N lignes précédentes. `{DictWithPrompt[...]}` n'injecte que les noms propres présents dans la ligne et disparaît s'il n'y en a aucun. — [docs LunaTranslator, LLM](https://docs.lunatranslator.org/en/guochandamoxing.html) ; [docs optimisations](https://docs.lunatranslator.org/en/transoptimi.html)
- **Glossaire en un clic (LinguaGacha)** : génération d'un glossaire pour la cohérence des noms propres, remplacement de texte, relecture automatique optionnelle. « Style and code retention » présenté comme un point fort. WOLF, RenPy, RPGMaker et Kirikiri pris en charge « sans traitement manuel » pour la plupart, ainsi que les exports Translator++ et MTool. — [GitHub neavo/LinguaGacha README_EN](https://github.com/neavo/LinguaGacha/blob/main/README_EN.md) ; [wiki Best Practice RPG Maker](https://github.com/neavo/LinguaGacha/wiki/BestPracticeForRPGMakerEN) ; [wiki Replacement](https://github.com/neavo/LinguaGacha/wiki/ReplacementEN)
- **Glossaire dans le prompt (Sakura)** : format `src->dst #info` placé avant le texte. — [GitHub SakuraLLM](https://github.com/SakuraLLM/SakuraLLM)
- **Contexte court (Sugoi)** : environ 10 lignes recommandées, sans quoi la qualité se dégrade. — [HF Sugoi-14B-Ultra](https://huggingface.co/sugoitoolkit/Sugoi-14B-Ultra-HF)
- **Pipeline agentique (étude de cas 2025)** : traduction d'un VN de plus de 30 000 lignes (*12Riven*) avec des agents Claude Code. Rétro-ingénierie du format, extraction, lots avec contexte, glossaire géré automatiquement, relecture en plusieurs passes. Les noms des locuteurs ont été retrouvés dans DATA.BIN. Coût estimé à environ 1 100 AUD en tokens, absorbé par l'abonnement Max. L'auteur ne lit pas le japonais et ne peut pas juger la qualité. — [ryandavis.io](https://ryandavis.io/how-not-to-translate-a-videogame-2025-ver/)

### Inferences
- La combinaison « lots bornés par l'événement + contexte réinitialisé + injection sélective du glossaire + locuteur et genre + validation avec retry » ressort comme l'état de l'art pratique pour RPG Maker.
- Les heuristiques QA de GalTransl (ratio de longueur 1,3, répétitions, retours à la ligne, japonais résiduel) sont bon marché et directement transposables à d'autres moteurs.
- Petits modèles locaux (Sugoi 14B) : peu de contexte. Modèles cloud et agents : beaucoup de contexte et plusieurs passes. La taille de fenêtre utile dépend donc du modèle.

### Gaps
- DazedMTLTool : dépôt gitgud inaccessible (HTTP 403) et miroir GitHub non lu. Son format exact (lignes numérotées ou JSON, gestion des écarts de nombre de lignes, estimation de coût, retour à la ligne) n'est donc pas vérifié.
- Traitement post-traduction des honorifiques (-san/-chan conservés ou non) : aucune documentation d'outil trouvée.
- Déduplication et cache inter-projets : seul le cache GalTransl est documenté.

## 3. Outils open source de traduction LLM pour jeux : différences

### Takeaway
L'écosystème se divise en deux. Côté chinois, GalTransl, AiNiee, LinguaGacha, SakuraLLM et LunaTranslator sont les plus avancés en glossaires, QA et intégration de modèles locaux. Côté anglophone, DazedMTLTool, Translator++ et rpgmaker-translator sont plus centrés sur RPG Maker et GPT. LunaTranslator traduit en temps réel par hooking, les autres produisent des patchs hors ligne.

### Cited Findings
- **GalTransl** : solution pour galges (GPT-4, Claude, DeepSeek, Sakura). Modèles locaux GalTransl-7B-v3.5 et 14B-v3 (6 Go de VRAM ou plus). D'après la page lue, la v7 (avril 2026) apporte une GUI de bureau **Tauri + React** avec gestion multi-projets. — [GitHub xd2333/GalTransl](https://github.com/xd2333/GalTransl). *Réserve : cette information vient d'un résumé automatique et pourrait concerner le fork JunjieLin98 ; à vérifier.*
- **LinguaGacha** (neavo) : v0.123.1 datée du 23 septembre 2026. 16 langues. APIs compatibles OpenAI, Google, Anthropic et SakuraLLM. Formats SRT, ASS, TXT, EPUB, PDF, Markdown, exports T++ et MTool. — [GitHub neavo/LinguaGacha](https://github.com/neavo/LinguaGacha) ; [README_EN](https://github.com/neavo/LinguaGacha/blob/main/README_EN.md). *Réserve : le résumé mentionne des noms de modèles pris en charge (« GPT 6 Sol / Luna », « Claude Opus 5.5 ») non vérifiés.*
- **AiNiee** : outil de traduction par lots d'RPG via ChatGPT, qui s'appuie sur mtool ou Translator++. Cité par SakuraLLM comme intégration pour les RPG. — [GitHub SakuraLLM (liste d'intégrations)](https://github.com/SakuraLLM/SakuraLLM) ; [LinguaGacha](https://github.com/neavo/LinguaGacha)
- **DazedMTLTool** : script qui traduit les JSON de RPG Maker MV via l'API ChatGPT « with context ». Il s'est étendu (GUI, Ren'Py, Wolf, Tyrano, Kirikiri), licence MIT. — [miroir GitHub sh2288/DazedMTLTool](https://github.com/sh2288/DazedMTLTool) ; [Context7](https://context7.com/git_gitgud_io/dazedanon_dazedmtltool) ; [gitgud](https://gitgud.io/DazedAnon/DazedMTLTool)
- **Translator++** a intégré DazedMTLTool comme add-on dans sa version 6.5.31 (1er juin 2024), avec un éditeur de parseurs personnalisés. — [Dreamsavior](https://dreamsavior.net/translator-ver-6-5-31-dazedmtltool-and-custom-parser-editor/)
- **rpgmaker-translator** (MoriTranslates) : MV/MZ, LLM local via Ollama, moteur cloud porté de DazedMTLTool (MIT). — [GitHub](https://github.com/MoriTranslates/rpgmaker-translator)
- Autres outils RPG Maker : RPGMTL (MizaGBF) et rpgmaker-mv-translator (davide97l). — [RPGMTL](https://github.com/MizaGBF/RPGMTL) ; [rpgmaker-mv-translator](https://github.com/davide97l/rpgmaker-mv-translator)
- **LunaTranslator** : traduction en temps réel (hook ou OCR), plusieurs moteurs (LLM, services en ligne, hors ligne), TTS VOICEVOX. Le délai des LLM rend la lecture en temps réel « brutale ». — [Sakimichi, nov. 2025](https://sakimichi.wordpress.com/2025/11/22/translating-visual-novels-in-2026/) ; [docs LunaTranslator](https://docs.lunatranslator.org/en/guochandamoxing.html)
- **Manga** : BallonsTranslator et manga-image-translator s'appuient sur Sakura. Une demande d'ajout de Sugoi LLM existe dans manga-image-translator. — [GitHub SakuraLLM](https://github.com/SakuraLLM/SakuraLLM) ; [issue #979](https://github.com/zyddnys/manga-image-translator/issues/979)

### Inferences
- GalTransl v7 (Tauri + React, multi-projets, dictionnaires, QA) est fonctionnellement le concurrent le plus proche de Hoshi2Star. Il vise toutefois les galges et le chinois, pas les RPG MV/MZ ni Wolf vers l'anglais ou le français.
- Les différences entre outils portent surtout sur la gestion des moteurs (extraction et réinjection), la QA automatique et la finesse du glossaire, plus que sur le LLM lui-même, car tous acceptent un endpoint compatible OpenAI.

### Gaps
- SLR Translator (mode LLM) : non recherché, aucune source.
- Détails techniques propres à AiNiee (dépôt non lu).
- Dates de dernière version de DazedMTLTool et de rpgmaker-translator : non trouvées.

## 4. Recherche académique et modes d'échec

### Takeaway
Pour le japonais, la recherche identifie des difficultés structurelles : sujets omis (zero pronouns), keigo et registre, honorifiques, jeux de mots. Les métriques automatiques sont jugées peu fiables pour le texte créatif. Aucune tâche WMT littéraire JA→EN centrée sur le jeu n'a été trouvée.

### Cited Findings
- Le japonais omet souvent sujet et objet (pro-drop). Le traducteur doit les déduire du discours, et le keigo encode la politesse dans la morphologie verbale. — [arXiv 2601.00223 JP-TL-Bench](https://arxiv.org/pdf/2601.00223)
- Le japonais préfère l'ellipsis pour ne pas nommer l'interlocuteur, ce qui pose un problème de formalité et de sensibilité culturelle. — [arXiv 2509.11921](https://arxiv.org/pdf/2509.11921)
- Un cadre d'évaluation littéraire en deux étapes distingue quatre types d'erreurs, dont les erreurs de contexte et celles liées aux honorifiques. — [arXiv 2412.01340](https://arxiv.org/html/2412.01340v2)
- LiTransProQA : métrique LLM sans référence, fondée sur des questions de traducteurs professionnels (figures littéraires, culture, voix de l'auteur). — [arXiv 2505.05423](https://arxiv.org/html/2505.05423)
- Évaluation humaine contre LLM de la traduction littéraire. — [arXiv 2410.18697](https://arxiv.org/html/2410.18697v1)
- Traduction multi-agents de textes littéraires très longs. — [arXiv 2405.11804](https://arxiv.org/pdf/2405.11804)
- Étude de la relecture par LLM sur des documents littéraires. — [arXiv 2605.13368](https://arxiv.org/pdf/2605.13368)
- HalOmi : benchmark annoté d'hallucinations et d'omissions en MT, cité comme ressource. — [liste de lectures hallucination](https://github.com/HillZhang1999/llm-hallucination-survey)
- WMT24 Literary : ZH→EN/DE/RU uniquement, 10 soumissions de 5 équipes, pas de japonais. — [WMT24 literary task](https://www2.statmt.org/wmt24/literary-translation-task.html) ; [Findings arXiv 2412.11732](https://arxiv.org/pdf/2412.11732)
- Visual novel *KillerTrait* (Copet et de Faria Pires) : 28 étudiants de master (9 post-édition DeepL, 10 post-édition ChatGPT, 9 traduction humaine). La post-édition ChatGPT est plus fidèle que celle de DeepL. DeepL met par défaut le masculin, ChatGPT propose les deux genres. Aucun moteur ne réussit les jeux de mots sans contexte visuel. Les métriques automatiques (HTER) sont jugées peu fiables pour le créatif. — [Espaces Linguistiques, Unilim](https://www.unilim.fr/espaces-linguistiques/995). *Réserve : d'après le contexte (français neutre), la paire est probablement EN→FR et non JA ; c'est un VN, mais pas du japonais.*
- SakuraLLM reconnaît aussi des erreurs de pronoms (genre, nombre) et de contexte. — [GitHub SakuraLLM](https://github.com/SakuraLLM/SakuraLLM)
- Constaté en pratique : japonais résiduel, lignes fusionnées ou perdues (retours à la ligne), répétitions en boucle, sorties trop longues. Ce sont les cibles des contrôles GalTransl. — [GitHub xd2333/GalTransl](https://github.com/xd2333/GalTransl)

### Inferences
- Pronoms et genre sont le mode d'échec n°1 en JA→EN/FR. D'où l'importance des fiches personnages (genre, registre) et du contexte du locuteur, déjà mise en œuvre par rpgmaker-translator.
- Pour le français, s'ajoutent l'accord en genre et le choix tu/vous, absents du VNTL (JA→EN).

### Gaps
- Aucun résultat WMT25 sur une tâche littéraire JA→EN ou JA→FR trouvé.
- Aucun papier 2024-2026 mesurant spécifiquement les lignes perdues ou fusionnées en traduction par lots de jeux.

## 5. Réception de la communauté, éthique et coût

### Takeaway
Le stigmate « MTL » reste fort : 68 % des sondés d'ANN rejettent totalement l'IA en traduction. Les éditeurs explorent pourtant la post-édition (MTPE). Les fans reconnaissent que les LLM dépassent l'ancienne MT, tout en regrettant la latence en temps réel. Certaines licences (Sakura) exigent un étiquetage MTL.

### Cited Findings
- Sondage ANN : 68 % de 550 répondants sont « totally unaccepting » de l'IA en traduction manga/anime. Leurs craintes : contresens (90 %), perte des références culturelles (83 %), pertes d'emplois (79 %). — [Anime News Network, 7 novembre 2025](https://www.animenewsnetwork.com/feature/2025-11-07/does-the-light-novel-translation-process-need-ai/.230680)
- Orange Inc. utilise l'IA comme premier jet relu par des humains. J-Novel Club ne l'a pas adoptée. Un initié affirme que « tout le monde » explore la MTPE. TO Books s'est associé à l'outil Corona EX. — [ANN](https://www.animenewsnetwork.com/feature/2025-11-07/does-the-light-novel-translation-process-need-ai/.230680)
- Incidents : sous-titres allemands de Crunchyroll apparemment copiés de ChatGPT. Messages de Claude laissés dans la VF d'*Honkai: Star Rail* (Hoyoverse). — [ANN](https://www.animenewsnetwork.com/feature/2025-11-07/does-the-light-novel-translation-process-need-ai/.230680)
- Côté fans : les LLM font « frequently better » que la MT en ligne classique. La communauté VN vit une « quiet revolution ». Les patchs entièrement IA partagés en sont l'usage idéal, plus que la lecture en direct. — [Sakimichi, nov. 2025](https://sakimichi.wordpress.com/2025/11/22/translating-visual-novels-in-2026/)
- Débat sur l'acceptabilité de la lecture en MTL dans les VN : [Toolify (agrégateur, faible fiabilité)](https://www.toolify.ai/ai-news/the-great-debate-machine-translation-vs-official-translation-in-visual-novels-714170)
- Licence Sakura : usage non commercial, étiquetage MTL obligatoire, version du modèle à indiquer. — [GitHub SakuraLLM](https://github.com/SakuraLLM/SakuraLLM)
- Coût : environ 1 100 AUD de tokens pour un VN de plus de 30 000 lignes via Claude (absorbé par un abonnement). — [ryandavis.io](https://ryandavis.io/how-not-to-translate-a-videogame-2025-ver/)
- Local : Sakura 7B tient dans 8 à 10 Go de VRAM, GalTransl-7B dans 6 Go ou plus. — [SakuraLLM](https://github.com/SakuraLLM/SakuraLLM) ; [GalTransl](https://github.com/xd2333/GalTransl)
- Le cas *12Riven* sert d'avertissement : on ne peut pas valider une traduction dans une langue qu'on ne lit pas. — [ryandavis.io](https://ryandavis.io/how-not-to-translate-a-videogame-2025-ver/)

### Inferences
- Pour un outil comme Hoshi2Star, l'étiquetage MTL, la traçabilité du modèle utilisé et une relecture humaine assistée (TM, QA) répondent directement aux critiques recensées.
- Choix du modèle : un local de 7 à 14B coûte zéro par token mais plafonne environ 3 à 5 points sous le cloud sur le VNTL. Côté cloud, un seul cas documenté : environ 1 100 AUD pour plus de 30 000 lignes en multi-passes agentiques avec Claude. Les coûts d'une passe unique ou d'un modèle bon marché ne sont pas sourcés.

### Gaps
- Threads F95zone et Reddit (r/visualnovels, r/LocalLLaMA) non trouvés via la recherche : pas de citation directe.
- Aspects juridiques (droit d'auteur des patchs de fans, position des éditeurs japonais face à la MTL) : aucune source fiable trouvée.
- Aucune comparaison chiffrée du coût API par million de caractères pour un jeu type.
