# Outils CAT / TMS professionnels pour la localisation de jeux vidéo (état 2026)

Notes de recherche, collectées le 2026-09-25. Toutes les affirmations sont sourcées. Les chiffres marketing de fournisseurs (« -70 % de coût », « 5x plus rapide ») sont rapportés comme tels, non vérifiés. Les prix viennent souvent d'agrégateurs (Capterra, costbench, locize) : ils sont indicatifs et à revérifier sur la page officielle.

Niveau de vérification : pages effectivement lues (WebFetch) = locize blog (prix Phrase/Lokalise), Gridly continuous-game-localization, memoQ pricing, Nimdzi 100 2026, Keywords Supercharged #05, Weblate hosting. Tout le reste provient de résumés de résultats de recherche : l'URL citée est le résultat le plus probable mais la page n'a pas été ouverte ; ces lignes sont marquées « (résumé de recherche) » quand l'attribution est particulièrement incertaine.

## Q1 — Gestion des string IDs, du contexte (captures, personnage, longueur max), des placeholders/balises et de la QA

### Takeaway
Les TMS orientés « software/game » (Crowdin, Gridly, XTM, LocDirect) sont organisés en clés/string IDs avec métadonnées (contexte, longueur max, captures) et intègrent ces métadonnées dans les prompts LLM ; les CAT de bureau (memoQ, Trados, OmegaT) sont orientés document/segment avec validation de balises, mais pas de notion native de « personnage » ou de capture de jeu.

### Cited Findings
**Crowdin**
- Les prompts Crowdin AI exposent des variables comme `%strings%` qui incluent l'identifiant de la chaîne (clé), le contexte, le texte, la longueur maximale et les traductions dans d'autres langues ; `%translationUnits%` / `%contextStrings%` servent à la QA par IA — [Crowdin Docs – Crowdin AI](https://support.crowdin.com/crowdin-ai/)
- L'« Agentic AI » de Crowdin lit les fichiers source, TM, glossaires, guides de style et captures d'écran avant de traduire ; billet dédié au contrôle du contexte, du ton et des longueurs — [Crowdin Blog, 2025-07-24](https://crowdin.com/blog/2025/07/24/crowdin-agentic-ai-how-to-master-context-tone-and-text-lengths)
- Longueur max définissable par chaîne (Sources > Strings) ou via le tagging de captures ; un check QA signale les traductions dépassant la limite — [Crowdin Docs – Screenshots](https://support.crowdin.com/screenshots/); [Crowdin QA settings](https://support.crowdin.com/enterprise/project-settings/qa-checks/)
- (résumé de recherche, peut venir du blog Crowdin) QA Crowdin : omissions, dépassement de longueur UI, fautes, grammaire, erreurs de balises/placeholders — [Crowdin QA settings](https://support.crowdin.com/enterprise/project-settings/qa-checks/)
- « Context harvesting » : outils IA/visuels pour fournir du contexte aux traducteurs — [Crowdin – Context harvesting](https://crowdin.com/features/context-harvesting)

**Gridly**
- Base de type tableur servant de « source unique de vérité » pour textes, dialogues, descriptions, audio et métadonnées ; stockage de captures, QA automatisée, catégorisation d'issues LQA avec commentaires — [Gridly – Continuous game localization](https://www.gridly.com/continuous-game-localization/)
- Le plugin Unity pousse texte ET captures d'écran vers Gridly ; le plugin Unreal supporte chaînes inline et StringTables — [Gridly – Unity](https://www.gridly.com/integrations/unity/); [Gridly – Unreal](https://www.gridly.com/integrations/unreal-engine/)

**XTM**
- Prévisualisation du texte sur modèles 3D, menus et scènes ; l'IA identifie les segments « à risque » pour orienter les testeurs humains — [XTM – Game localisation](https://xtm.ai/solutions/game-localisation)

**LocDirect (LocalizeDirect)**
- CMS de localisation conçu « par des développeurs de jeux pour des développeurs de jeux », permettant de localiser en parallèle du développement et reliant développeurs, producteurs, traducteurs, relecteurs, loc managers et LQA — [LocalizeDirect – Game localization](https://www.localizedirect.com/use-cases/game-localization); [LocalizeDirect – Qu'est-ce qu'un système de loc de jeu](https://www.localizedirect.com/posts/game-localization-system-cms)
- Article LocalizeDirect sur les fonctions attendues d'un système de loc (référence utile pour la checklist) — [LocalizeDirect, partie 2](https://www.localizedirect.com/posts/localization-system-features-and-functions)

**OmegaT (gratuit)**
- Validation de balises après traduction pour éviter les erreurs de tags ; 30+ formats (DOCX, XLSX, IDML, XLIFF, PO…) — [omegat.org](http://omegat.org/); [Wikipedia – OmegaT](https://en.wikipedia.org/wiki/OmegaT)

**Weblate**
- (résumé de recherche) Page features : contrôles qualité intégrés (placeholders, etc.), intégration étroite au contrôle de version — [Weblate features](https://weblate.org/en/features/); [GitHub WeblateOrg/weblate](https://github.com/WeblateOrg/weblate)

### Inferences
- Pour un projet de type RPG Maker/Wolf (fichiers extraits, pas de pipeline Unity/Unreal), les connecteurs moteurs des TMS ne s'appliquent pas : il faut exporter vers un format intermédiaire (XLIFF/PO/CSV/JSON clé-valeur). Les codes RPG Maker (`\V[n]`, `\C[n]`) devraient être déclarés comme placeholders personnalisés (regex) dans le TMS/CAT, sinon la QA de balises ne les verra pas.
- Le modèle « prompt enrichi par clé + contexte + longueur max + captures » de Crowdin est le patron de référence du marché pour la traduction LLM de chaînes de jeu.

### Gaps
- Pas de source consultée détaillant la notion de « locuteur/personnage » comme champ natif (Gridly et LocDirect le permettent probablement via colonnes/métadonnées, non vérifié).
- Pas de doc vérifiée pour les placeholders personnalisés (regex) dans memoQ (« regex tagger ») ou Trados — connu mais non sourcé ici.
- Keywords Studios KantanAI, Inlingo, Pootle : aucune doc technique consultée.

## Q2 — TM (exact/fuzzy, context match/ICE), termbases, MT (DeepL, Google, LLM + glossaire), QA/LQA par IA

### Takeaway
Les CAT historiques ont des niveaux de correspondance normalisés (memoQ 101 % = context match, 102 % = double context ; Trados « Context Match » / « PerfectMatch ») qui pilotent aussi la facturation ; tous les acteurs ont ajouté en 2024–2026 une couche LLM « adaptative » alimentée par TM + glossaire (memoQ AGT, Trados Generative Translation/Copilot, Crowdin AI, Gridly GenAI, Alocai ModelWiz).

### Cited Findings
**Niveaux de correspondance**
- memoQ : 101 % = context match (segment identique + segment précédent identique) ; 102 % = double context match (segments précédent et suivant identiques) — [memoQ Docs – Double context matches](https://docs.memoq.com/current/en/Concepts/concepts-double-context-matches.html); [memoQ Docs – Match rates](https://docs.memoq.com/current/en/Concepts/concepts-match-rates-from-translation-m.html)
- Équivalence : le 101 % memoQ ≈ « Context Match » Trados, le 102 % ≈ « PerfectMatch » Trados ; ICE (In-Context Exact) désigne le même concept — [Coruzant – Context vs Perfect Match](https://coruzant.com/tech/context-match/); [RWS Community – 102 %](https://community.rws.com/product-groups/trados-portfolio/trados-studio/f/studio/28132/match-percentage-102)
- Ces niveaux structurent les grilles tarifaires (remises par tranche de fuzzy) — [TranslateCluj – pricing grids](https://translatecluj.com/2024/01/16/understanding-translation-pricing-grids/)
- OmegaT : seuil de fuzzy configurable, insertion auto du meilleur fuzzy, glossaires, « leveraging » dans les projets mis à jour — [SourceForge OmegaT news](https://sourceforge.net/p/omegat/news/); [omegat.org](http://omegat.org/)

**MT / LLM**
- memoQ AGT (Adaptive Generative Translation) : combine LLM (Microsoft Azure OpenAI), MT et TM ; adaptation de domaine instantanée à partir des TM, termbases et documents alignés, sans réentraînement — [memoQ – AGT](https://www.memoq.com/product/memoq-agt/); [memoQ Docs – Translate with AGT](https://docs.memoq.com/current/en/Workspace/pre-translate-with-agt.html); [memoQ blog](https://blog.memoq.com/transforming-translation-with-memoq-agt)
- Trados : add-on « AI Essentials » (Generative Translation + Smart Review) pour Team/Accelerate/Enterprise ; Trados Copilot permet d'affiner, appliquer la terminologie, ajuster le ton, raccourcir ; possibilité de connecter son LLM (Anthropic, DeepSeek, OpenAI) — [Trados – AI Essentials FAQ](https://www.trados.com/product/discover/AI-translation/FAQ/what-is-included-in-the-AI-Essentials-add-on/); [Trados pricing](https://www.trados.com/pricing/)
- OmegaT : connecteurs MT intégrés Google Translate, DeepL, IBM Watson ; plugins communautaires DeepL Free API et Microsoft Translator ; Yandex.Cloud — [OmegaT wiki – Plugins](https://sourceforge.net/p/omegat/wiki/Plugins/); [SourceForge OmegaT news](https://sourceforge.net/p/omegat/news/)
- Gridly : MT Google et Amazon, IA générative pour traduction/relecture/résumé ; partenaires OpenAI, DeepL, memoQ, Phrase — [Gridly – Continuous game localization](https://www.gridly.com/continuous-game-localization/)
- Crowdin : « AI Pipeline » combinant plusieurs étapes IA (traduction, QA) — [Crowdin Blog – AI Pipeline](https://crowdin.com/blog/crowdin-power-up-ai-pipeline)
- Alocai (lié à Altagram) : moteur ModelWiz = NMT + GenAI ; NMT custom revendiqué entraîné sur 250 M+ termes de jeu traduits professionnellement ; workflow humain-dans-la-boucle ; revendications marketing « jusqu'à -70 % de coûts », « 5x plus rapide », « -50 % time-to-market » — [Alocai](https://www.alocai.com/); [Alocai – Platform](https://www.alocai.com/our-platform); [Alocai – GenAI](https://www.alocai.com/generative-ai-integration); [Altagram – Alocai](https://altagram.com/solution/alocai/)
- Keywords Studios utilise sa technologie propriétaire KantanAI, présentée comme IA développée pour la localisation de jeux — [Keywords – Game translation](https://www.keywordsstudios.com/en/services/globalize/localization/translation/)
- LocalizeDirect annonce une « localisation IA full-pipeline avec validation par linguiste natif à chaque étape » — [LocalizeDirect](https://www.localizedirect.com/)

### Inferences
- Le « context match » à la memoQ (segment précédent/suivant identique) est directement transposable aux dialogues RPG séquentiels, où une même réplique courte (« はい », « ……») doit se traduire différemment selon la ligne précédente : une TM uniquement exacte + fuzzy sans contexte est insuffisante.
- Le modèle dominant 2025–2026 est « TM + glossaire injectés dans le prompt LLM » (AGT, Crowdin, Trados BYO-LLM), pas le fine-tuning.

### Gaps
- Aucune doc consultée sur les fonctionnalités MT/LLM de Phrase (Phrase Language AI / QPS), Smartcat, Localazy, POEditor, Lokalise en détail.
- Pas de tarif public trouvé pour memoQ AGT ni Trados AI Essentials.
- Qualité mesurée JA→EN des LLM dans ces outils : aucune donnée indépendante trouvée.

## Q3 — Localisation continue : mises à jour de la source, diff, versioning, API et plugins moteurs

### Takeaway
Gridly et XTM offrent des connecteurs Unity/Unreal et Git ; Weblate est bâti autour du contrôle de version ; les CAT de bureau gèrent les mises à jour par « leveraging » de TM dans une nouvelle version du fichier. Aucun connecteur RPG Maker / Wolf RPG n'a été trouvé.

### Cited Findings
- Gridly : plugin Unity gratuit (GitHub), plugin Unreal gratuit (Marketplace), nécessitent un compte Gridly ; API ; synchronisation automatique du statut quand le contenu change — [Gridly – Unity](https://www.gridly.com/integrations/unity/); [Gridly – Unreal](https://www.gridly.com/integrations/unreal-engine/); [Unity Asset Store – Gridly](https://assetstore.unity.com/packages/tools/localization/gridly-204301); [Gridly – Continuous](https://www.gridly.com/continuous-game-localization/)
- XTM : connecteurs GitHub, GitLab, Unity, Unreal ; synchronisation automatique et push des chaînes traduites dans les builds — [XTM – Game localisation](https://xtm.ai/solutions/game-localisation)
- Weblate : « intégration étroite au contrôle de version » (Git) — [GitHub WeblateOrg/weblate](https://github.com/WeblateOrg/weblate)
- LocDirect : localisation menée en parallèle du développement pour un sim-ship — [LocalizeDirect – Game localization](https://www.localizedirect.com/use-cases/game-localization)
- OmegaT : réutilisation de la TM lors de la mise à jour d'un projet — [omegat.org](http://omegat.org/)
- Nimdzi prévoit en 2026 une vague de solutions « agentiques » poussant les TMS vers des plateformes de données unifiées / hubs IA (prévision, pas un fait) — [Nimdzi 100 2026](https://www.nimdzi.com/nimdzi-100-2026/)

### Inferences
- Pour un outil comme Hoshi2Star, l'équivalent de la « localisation continue » est la ré-extraction d'une nouvelle version du jeu + réapplication de la TM par string ID/position : c'est exactement ce que les TMS font via leurs connecteurs.

### Gaps
- Pas de doc vérifiée sur les connecteurs Godot (Crowdin et Lokalise en ont probablement, non vérifié).
- Branching/versioning Crowdin et Phrase non documentés ici.

## Q4 — Tarification 2025–2026 (siège, mot, gratuit, open source)

### Takeaway
Tendance nette 2025–2026 : disparition des offres d'entrée self-serve (Phrase Starter, free plan Lokalise) et passage à une facturation au volume traité avec IA incluse ; les options réellement accessibles à un fan-traducteur restent OmegaT (gratuit), Weblate (libre/auto-hébergé ou Libre gratuit), les free tiers Crowdin/POEditor, et la licence freelance Trados/memoQ Translator Pro.

### Cited Findings
- **Crowdin** : Free (sans limite de durée, fonctions limitées), Pro 59 $/mois, Team 179 $/mois, Team+ 450 $/mois, Business/Enterprise sur devis ; facturation par mots hébergés et sièges ; option Open Source — [costbench – Crowdin](https://costbench.com/software/localization/crowdin/); [Capterra – Crowdin pricing](https://www.capterra.com/p/163509/Crowdin/pricing/). Note : un autre agrégateur cite « Paid starts at $50/mo » — [costbench free-plan](https://costbench.com/software/localization/crowdin/free-plan/) (divergence non résolue).
- **Lokalise** (restructuration nov. 2025) : Explorer 144 $/mois, Growth 375 $/mois (lancé à 499 $), Advanced 999 $/mois (démo requise), Enterprise sur devis ; facturation passée des sièges/clés aux « processed words » ; free-forever supprimé (essai 14 j) ; traduction IA incluse dans tous les plans (auparavant add-on ~0,05 $/mot) — [Locize blog, 2026](https://www.locize.com/blog/phrase-lokalise-price-changes-2026); [Lokalise pricing](https://lokalise.com/pricing/). Source concurrente (Locize) : à croiser avec la page officielle.
- **Phrase** : 2024 Starter 135 $/mois, Team 1 045 $, Business 4 395 $ ; juillet 2026 : Freelancer 27 $/mois (1 siège), Developer/Designer 525 $/mois, Team 1 245 $/mois annuel (vente), Business/Enterprise sur devis ; Starter supprimé entre août et oct. 2025 — [Locize blog, 2026](https://www.locize.com/blog/phrase-lokalise-price-changes-2026)
- **Trados Studio** : plan Freelance dès 54 $/mois ; ~408 $/an cité (résumé de recherche, non vérifié) ; abonnements mensuels ou annuels ; essai 14 j ; AI Essentials = add-on payant sur Team/Accelerate/Enterprise — [Trados pricing](https://www.trados.com/pricing/); [Capterra – Trados](https://www.capterra.com/p/151746/Trados-Studio/pricing/); [Trados – freelance subscription](https://www.trados.com/spotlight/trados-studio-subscription-benefits/)
- **memoQ TMS** : Starter dès 2 750 €/an (3 025 $/an), 1 licence PM + 5 linguistes ; Essential/Professional/Enterprise sur devis ; « pas de tarification au volume » ; AGT disponible en add-on (prix non public) — [memoQ pricing](https://www.memoq.com/pricing/); [memoQ AGT](https://www.memoq.com/product/memoq-agt/)
- **XTM** : aucun plan fixe, tarification entièrement sur mesure selon produits, connecteurs et IA — [XTM pricing](https://xtm.ai/pricing/platform)
- **Gridly** : plugins gratuits, compte gratuit ou premium, essai 30 j des plans Pro — [Gridly – Unity](https://www.gridly.com/integrations/unity/); [Gridly FAQ](https://help.gridly.com/frequently-asked-questions/360013320117-Frequently-asked-questions-about-Gridly-and-beyond)
- (résumé de recherche) **POEditor** : plan gratuit 1 000 chaînes, projets illimités, API, 5 contributeurs ; Start 17 $/mois pour 3 000 chaînes — [Alconost – comparatif 2026](https://alconost.com/en/blog/localization-platforms-comparison)
- (résumé de recherche) **Smartcat** : dès 1 200 $/an forfaitaire, pas de plan gratuit selon l'agrégateur — [Capterra – Smartcat](https://www.capterra.com/p/155255/smartCAT/pricing/); [Alconost](https://alconost.com/en/blog/localization-platforms-comparison)
- (résumé de recherche) **Localazy** : dès 39 $/mois, free tier et essai — [Alconost](https://alconost.com/en/blog/localization-platforms-comparison)
- **Weblate** (page vérifiée) : cloud facturé en « hosted strings » (chaînes source × (langues + 1)) : 10k 47 €/mois, 40k 70 €, 160k 114 €, 640k 193 €, 2,5M 342 €, 10M 616 € ; -20 % en annuel, essai 14 j, projets/traducteurs illimités ; plan Libre gratuit pour projets open source publics = équivalent 160k ; auto-hébergement gratuit (support payant 53–106 €/mois optionnel) — [Weblate hosting](https://weblate.org/en/hosting/)
- **OmegaT** : gratuit et open source, multiplateforme — [omegat.org](http://omegat.org/)

### Inferences
- Pour de la fan-traduction de RPG japonais (projets non commerciaux, souvent illégaux au sens du droit d'auteur, donc non « open source » au sens des plans Libre/Open Source), les plans gratuits OSS de Crowdin/Weblate sont probablement inéligibles ; seuls OmegaT, Weblate auto-hébergé et les free tiers à quota restent viables.
- La facturation au mot traité (Lokalise) pénalise les gros volumes de dialogue de RPG (souvent des centaines de milliers de caractères).

### Gaps
- Pas de prix vérifiés pour memoQ Translator Pro (licence individuelle), Localazy détaillé, Smartcat 2026 officiel.
- Conditions exactes d'éligibilité des plans open source Crowdin/Weblate non vérifiées.

## Q5 — Tendances : part MT/LLM, MTPE, controverses

### Takeaway
Le MTPE est désormais standard chez les LSP (82 % en proposent) ; le jeu vidéo est l'un des secteurs les plus touchés par l'érosion des prix due à l'IA ; les grands LSP (Keywords) défendent l'humain-dans-la-boucle ; les joueurs réagissent négativement aux traductions perçues comme automatiques. Pas de chiffre fiable trouvé sur la part MT/LLM spécifique au jeu vidéo.

### Cited Findings
- Services fournis par les LSP (Nimdzi 100 édition 2026, page vérifiée) : traduction/localisation 94,6 %, MTPE 81,1 %, sous-titrage 69,6 %. Note : un résumé de recherche donnait 96,2 / 82,4 / 65,6 %, probablement une autre édition — les chiffres vérifiés sont ceux-ci — [Nimdzi 100 2026](https://www.nimdzi.com/nimdzi-100-2026/)
- Nimdzi 2026 (vérifié) : les jeux vidéo ont « glissé dans le classement », fortement touchés par l'amélioration rapide de la localisation IA ; seuls 11,0 % des sociétés de langue déclarent un fort focus techno — [Nimdzi 100 2026](https://www.nimdzi.com/nimdzi-100-2026/)
- Keywords Studios, newsletter « AI's Hard Truth » (vérifiée) : cite l'Interactive Media Agreement de juillet 2025 (ratifié à 95,04 %, consentement vocal/IA) comme jalon ; recommande de « garder des humains à chaque point de décision » — [Keywords Studios – Supercharged #05](https://www.keywordsstudios.com/en/about-us/news-events/supercharged-newsletter/supercharged-edition-05/)
- (résumé de recherche, page source non identifiée) Les échecs de traduction (noms de personnages offensants, dialogues perçus comme « machine ») remontent sur les réseaux ; érosion des prix de l'outsourcing gaming depuis ChatGPT (2022). Ces phrases ne figurent PAS dans la newsletter Keywords #05 ; source probable parmi [Allcorrect – 10 companies 2025](https://allcorrectgames.com/insights/10-most-experienced-localization-companies-in-2025/) ou d'autres pages Keywords, non confirmé.
- Quand le public remarque une traduction machine/IA, la réaction peut forcer des patches voire une retraduction complète ; recommandation de « tiering » : IA pour patch notes / langues secondaires, humain pour dialogues et narration — [Gridly – AI translation guide](https://www.gridly.com/blog/ai-translation-game-localization/)
- Les éditeurs ne se demandent plus si utiliser l'IA mais jusqu'où, avec des workflows « AI-first » — [Game Cookies](https://gamecookies.news/game-localization-is-not-dying-it-is-being-rebuilt-around-ai/)
- IGDA : article d'orientation sur MT/IA pour traduire un jeu — [IGDA](https://igda.org/news-archive/should-i-translate-my-game-using-machine-translation-or-ai/)

### Inferences
- Le marché pro converge vers « LLM + TM + glossaire + LQA humaine ciblée par l'IA » ; pour un fan-traducteur solo, l'équivalent réaliste est LLM + TM/glossaire locaux + QA automatique des placeholders, la relecture humaine étant le goulot.

### Gaps
- Aucune statistique indépendante (Slator, Nimdzi, CSA) trouvée sur le pourcentage de mots de jeu traduits par MT/LLM en 2025–2026.
- Startups « AI-native » 2024–2026 hors Alocai (Ludo.ai, etc.) : non couvertes, pas de source solide consultée. Ludo.ai est un outil d'idéation de jeux, pas de localisation (à vérifier).
- Inlingo, Pootle (projet largement inactif selon mémoire, non vérifié), Localize/« Lingo » SaaS : non couverts.
- Talks GDC sur la loc IA : non consultés.
- Un résumé de recherche affirmait que Metaphor: ReFantazio a été critiqué pour des « traductions IA » : non vérifié, probablement faux, à ne pas reprendre.
- Aucun cas documenté et vérifié de retraduction forcée d'un jeu suite à une MT détectée n'a été consulté (l'affirmation générale vient du guide Gridly).
