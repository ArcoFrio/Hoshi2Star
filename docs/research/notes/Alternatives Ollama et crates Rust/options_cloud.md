# Options cloud / en ligne pour la traduction LLM quand le PC est trop faible (état au 2026-09-25)

Toutes les données de prix ont été relevées le **2026-09-25** sauf mention contraire. Les prix sont en USD par million de tokens (MTok), entrée / sortie. Les prix changent souvent : revérifier avant de coder des valeurs par défaut.

---

## 1. Prix par MTok des modèles les moins chers adaptés à la traduction JP→EN/FR, et coût estimé pour un RPG

### Takeaway
En sept. 2026, traduire un RPG entier (environ 1,5 à 4,5 M tokens facturés) coûte **moins de 1 $** avec les modèles "flash" chinois ou ouverts (DeepSeek V4 Flash, GLM-5.3-Flash, Qwen3.8-Flash, gpt-5-nano, Gemini 2.5 Flash-Lite), **1 à 10 $** avec les modèles "mini" de milieu de gamme (gpt-5-mini, Gemini Flash, Haiku 4.5, Mistral Large 3), et **7 à 30 $** avec les modèles de premier plan (Sonnet 5, Kimi K3). Le Batch API divise la facture par deux (−50 %).

### Cited Findings

**OpenAI (prix standard / Batch ; relevés le 2026-09-25)** — [OpenAI API pricing](https://developers.openai.com/api/docs/pricing)
- gpt-5-nano : 0,05 $ entrée / 0,005 $ entrée en cache / 0,40 $ sortie ; Batch 0,025 / 0,0025 / 0,20.
- gpt-5-mini : 0,25 / 0,025 / 2,00 ; Batch 0,125 / 0,0125 / 1,00.
- gpt-5.4-nano : 0,20 / 0,02 / 1,25 ; gpt-5.4-mini : 0,75 / 0,075 / 4,50.
- gpt-5 / gpt-5.1 : 1,25 / 0,125 / 10,00 ; gpt-5.4 : 2,50 / 0,25 / 15,00 ; gpt-5.5 : 5,00 / 0,50 / 30,00.
- Le tier "Flex" facture le même prix que Batch pour la plupart des modèles, par ex. gpt-5-mini Flex 0,125 / 1,00. Exception relevée sur la page : gpt-5.1 Flex affiche 2,50 / 20,00, ce qui ressemble à une anomalie de la page.

**Google Gemini (standard / Batch / cache ; relevés le 2026-09-25)** — [Gemini API pricing](https://ai.google.dev/gemini-api/docs/pricing)
- Gemini 2.5 Flash-Lite : 0,10 / 0,40 ; Batch 0,05 / 0,20 ; cache 0,01.
- Gemini 2.5 Flash et Gemini 3.5 Flash-Lite : 0,30 / 2,50 ; Batch 0,15 / 1,25 ; cache 0,03.
- Gemini 3.7 Flash et 3.8 Flash : 0,75 / 3,75 ; Batch 0,375 / 1,875. Ce prix vaut **jusqu'au 31 déc. 2026** : la page annonce une hausse des prix des nouveaux modèles Gemini 3 au 1er janv. 2027.
- Gemini 3.5 Flash : 1,50 / 9,00 ; Gemini 2.5 Pro : 1,25 / 10,00 (≤200k) ; Gemini 3.1 Pro Preview : 2,00 / 12,00 (payant uniquement).

**Anthropic Claude (relevés le 2026-09-25)** — [Claude pricing](https://platform.claude.com/docs/en/about-claude/pricing)
- Haiku 4.5 : 1 $ / 5 $ ; lecture de cache 0,10 $ ; Batch 0,50 / 2,50.
- Sonnet 5 : 2 $ / 10 $. Le prix d'introduction est devenu le prix standard, et la hausse prévue à 3 / 15 $ le 1er sept. 2026 n'aura pas lieu. Batch 1 / 5.
- Sonnet 4.6 : 3 / 15 $.
- Les modèles Claude 4.7 et suivants utilisent un nouveau tokenizer qui produit environ 30 % de tokens en plus pour le même texte.

**DeepSeek, API directe (relevés le 2026-09-25)** — [DeepSeek pricing](https://api-docs.deepseek.com/quick_start/pricing)
- deepseek-flash (V4.1-Flash), en heures pleines : entrée en cache 0,006 $, entrée hors cache 0,30 $, sortie 1,20 $.
- deepseek-v4-pro : 0,044 / 1,32 / 3,96.
- **Heures creuses à −50 %.** Les heures pleines sont du lundi au vendredi, 01:00–04:00 et 06:00–10:00 UTC. Contexte de 1M tokens, sortie JSON prise en charge.

**Mistral (relevés le 2026-09-25)** — [Mistral API pricing](https://mistral.ai/pricing/api)
- Mistral Small 4 : 0,15 / 0,60 ; Mistral Large 3 : 0,50 / 1,50 ; Mistral Medium 3.5 : 1,50 / 7,50.
- Ministral 3 : 3B à 0,10 / 0,10 ; 8B à 0,15 / 0,15 ; 14B à 0,20 / 0,20.
- Le Batch réduit de 50 %, et le cache d'entrée réduit jusqu'à 90 % — [Mistral pricing](https://mistral.ai/pricing)

**OpenRouter (API publique `/api/v1/models`, relevée le 2026-09-25 à 11:43 UTC)** — [OpenRouter models API](https://openrouter.ai/api/v1/models)
- deepseek/deepseek-v4-flash-0731 : 0,03 / 0,32 ; deepseek-v4-flash : 0,049 / 0,098 ; deepseek-v4.1-flash : 0,15 / 0,60 (variante `:batch` 0,112 / 0,336).
- z-ai/glm-5.3-flash : 0,045 / 0,14 ; glm-5.2 : 0,65 / 2,04 ; glm-5.3 : 1,40 / 4,40.
- qwen/qwen3.8-flash : 0,15 / 0,47 ; qwen3.7-flash : 0,03 / 0,13 (sans structured outputs) ; qwen3.7-plus : 0,32 / 1,28 ; qwen3.8-27b : 0,42 / 3,00 ; qwen3-235b-a22b-2507 : 0,0875 / 0,35.
- moonshotai/kimi-k2.5 : 0,45 / 2,25 ; kimi-k3 : 3 / 15.
- x-ai/grok-4.3 : 1,25 / 2,50 ; grok-4.7 : 1,60 / 4,80.
- xiaomi/mimo-v2.6-flash : 0,14 / 0,28.
- google/gemma-4-31b-it : 0,09 / 0,34 ; meta-llama/llama-3.3-70b-instruct : 0,10 / 0,32 ; mistral-small-2603 : 0,15 / 0,60.
- openai/gpt-oss-120b : 0,15 / 0,60 (`:batch` 0,03 / 0,136).

**DeepInfra (relevés le 2026-09-25)** — [DeepInfra pricing](https://deepinfra.com/pricing)
- DeepSeek-V4-Flash-0731 : 0,06 / 0,18 ; Qwen3.5-35B-A3B : 0,14 / 1,00 ; Qwen3.6-27B : 0,32 / 3,20.
- Gemma-4-31B-it : 0,13 / 0,38 ; Llama-3.3-70B-Turbo : 0,10 / 0,32 ; Kimi-K3 : 2,85 / 14,25 ; Mistral-Nemo : 0,019 / 0,03.

### Calcul du coût pour un RPG type (inférence à partir des prix ci-dessus)
Hypothèses, non sourcées et à ajuster avec les vraies mesures de Hoshi2Star :
- De 30k à 100k lignes, environ 20 à 30 tokens JP par ligne. Cela donne ~0,6 à 3 M tokens source.
- Le prompt système, le glossaire et le contexte sont envoyés par lots de lignes, ce qui gonfle l'entrée.
- **Scénario Bas** : 1 M tokens en entrée + 0,5 M en sortie.
- **Scénario Haut** : 3 M tokens en entrée + 1,5 M en sortie.
- Formule : coût = entrée(M) × prix_entrée + sortie(M) × prix_sortie.

| Modèle (source du prix) | Prix entrée/sortie | Bas (1M/0,5M) | Haut (3M/1,5M) | Haut en Batch (−50 %) |
|---|---|---|---|---|
| GLM-5.3-Flash (OpenRouter) | 0,045 / 0,14 | 0,045+0,07 = **0,12 $** | 0,135+0,21 = **0,35 $** | – |
| DeepSeek V4-Flash-0731 (DeepInfra) | 0,06 / 0,18 | 0,06+0,09 = **0,15 $** | 0,18+0,27 = **0,45 $** | – |
| gpt-5-nano (OpenAI) | 0,05 / 0,40 | 0,05+0,20 = **0,25 $** | 0,15+0,60 = **0,75 $** | 0,38 $ |
| Gemini 2.5 Flash-Lite | 0,10 / 0,40 | 0,10+0,20 = **0,30 $** | 0,30+0,60 = **0,90 $** | 0,45 $ |
| Qwen3.8-Flash (OpenRouter) | 0,15 / 0,47 | 0,15+0,235 = **0,39 $** | 0,45+0,705 = **1,16 $** | – |
| Mistral Small 4 | 0,15 / 0,60 | 0,15+0,30 = **0,45 $** | 0,45+0,90 = **1,35 $** | 0,68 $ |
| DeepSeek flash direct (heures pleines) | 0,30 / 1,20 | 0,30+0,60 = **0,90 $** | 0,90+1,80 = **2,70 $** | heures creuses : 1,35 $ |
| gpt-5-mini | 0,25 / 2,00 | 0,25+1,00 = **1,25 $** | 0,75+3,00 = **3,75 $** | 1,88 $ |
| Mistral Large 3 | 0,50 / 1,50 | 0,50+0,75 = **1,25 $** | 1,50+2,25 = **3,75 $** | 1,88 $ |
| Gemini 2.5 Flash | 0,30 / 2,50 | 0,30+1,25 = **1,55 $** | 0,90+3,75 = **4,65 $** | 2,33 $ |
| Gemini 3.8 Flash (prix 2026) | 0,75 / 3,75 | 0,75+1,875 = **2,63 $** | 2,25+5,625 = **7,88 $** | 3,94 $ |
| Grok 4.3 (OpenRouter) | 1,25 / 2,50 | 1,25+1,25 = **2,50 $** | 3,75+3,75 = **7,50 $** | – |
| gpt-5.4-mini | 0,75 / 4,50 | 0,75+2,25 = **3,00 $** | 2,25+6,75 = **9,00 $** | 4,50 $ |
| DeepSeek v4-pro direct (heures pleines) | 1,32 / 3,96 | 1,32+1,98 = **3,30 $** | 3,96+5,94 = **9,90 $** | heures creuses : 4,95 $ |
| Claude Haiku 4.5 | 1 / 5 | 1+2,5 = **3,50 $** | 3+7,5 = **10,50 $** | 5,25 $ |
| Claude Sonnet 5 | 2 / 10 | 2+5 = **7,00 $** | 6+15 = **21,00 $** | 10,50 $ |
| Kimi K3 (OpenRouter) | 3 / 15 | 3+7,5 = **10,50 $** | 9+22,5 = **31,50 $** | – |

### Inferences
- **Mode "réflexion" / reasoning** : les modèles qui raisonnent (gpt-5*, Gemini 3.x, variantes *thinking*) facturent aussi les tokens de raisonnement comme de la sortie. Cela peut multiplier le coût de sortie. Pour de la traduction ligne à ligne, il faut régler l'effort de raisonnement au minimum ou choisir des variantes sans raisonnement. Ce point est déduit des pratiques connues des API et n'a pas été vérifié par page officielle dans cette session.
- **Tokenizer** : le japonais consomme en général plus de tokens par caractère que l'anglais. Pour Claude 4.7 et suivants, il faut aussi compter environ +30 % à cause du nouveau tokenizer ([Claude pricing](https://platform.claude.com/docs/en/about-claude/pricing)). Le calcul du tableau est donc un ordre de grandeur, à recalibrer avec les compteurs réels du pipeline.
- **Prompt caching** : si le prompt système et le glossaire sont identiques d'une requête à l'autre, le cache réduit l'entrée d'environ 90 %. Tarifs du cache : DeepSeek 0,006 $, OpenAI 10 % du prix d'entrée, Gemini environ 10 %, Claude 0,1× avec une écriture à 1,25×. Pour un RPG, l'entrée dominante est ce préfixe répété, donc le gain est fort.
- Même le scénario haut avec un modèle "mini" reste sous les **10 $**. Le coût n'est pas le facteur bloquant ; les vrais critères sont la qualité JP, les refus NSFW et les limites de débit.

### Gaps
- Aucune source ne donne la qualité comparée JP→EN/FR de ces modèles en 2026 (benchmark de traduction). Le choix "bon en japonais" n'est donc pas étayé ici.
- La page officielle openai.com/api/pricing a répondu 403. La page developers.openai.com a été utilisée à la place.
- Tarifs first-party non relevés : Alibaba Cloud Model Studio (Qwen), Moonshot, Zhipu (bigmodel.cn / z.ai), xAI. Seuls les prix via OpenRouter ont été relevés.
- Prix non relevés : Together AI, Fireworks, Groq (payant), SambaNova, Novita, Hyperbolic, Nebius, Chutes, Hugging Face Inference Providers.

---

## 2. Endpoints compatibles OpenAI et sorties structurées (JSON schema)

### Takeaway
Les agrégateurs (OpenRouter, DeepInfra, Groq, Cerebras) et DeepSeek exposent tous une API de type OpenAI Chat Completions. Un seul client "OpenAI-compatible" + `base_url` configurable dans Hoshi2Star couvre donc la majorité des options. La prise en charge du JSON schema strict varie **selon le modèle** : il faut la vérifier par modèle.

### Cited Findings
- L'API OpenRouter publie pour chaque modèle un champ `supported_parameters`. `structured_outputs` y est présent pour la plupart des modèles récents (DeepSeek V4, GLM-5.x, Qwen3.8-Flash, Kimi, Grok, Gemma 4). Il est **absent** pour : qwen3.7-flash, glm-4.5 / 4.5-air, les variantes `:free` de Gemma 4, et mistral-small-3.1 — [OpenRouter models API](https://openrouter.ai/api/v1/models)
- DeepSeek prend en charge la sortie JSON sur deepseek-flash et deepseek-v4-pro. Limites de concurrence : 2500 pour flash, 500 pour pro — [DeepSeek pricing](https://api-docs.deepseek.com/quick_start/pricing)

### Inferences
- Stratégie robuste : demander un JSON schema quand `supported_parameters` le permet. Sinon, prévoir un repli "JSON mode" et une validation et réparation côté Rust. Le pipeline adaptatif retry/split existant sert déjà à ça.

### Gaps
- Non vérifié dans cette session : la couche de compatibilité OpenAI SDK d'Anthropic, l'endpoint OpenAI-compatible de Gemini, et le endpoint OpenAI-compatible de Mistral, avec leur prise en charge exacte de `response_format: json_schema`. Ces endpoints existent d'après les connaissances antérieures, mais sans preuve relevée ici.

---

## 3. Offres gratuites et leurs limites (dont usage des données pour l'entraînement)

### Takeaway
Les offres gratuites suffisent pour tester ou pour de petits jeux, mais pas pour un RPG complet d'une traite. OpenRouter `:free` se limite à 50 requêtes/jour sans achat, Groq à 200k tokens/jour, et Cerebras donne 1M tokens/jour mais seulement 5 $ de crédits d'essai. Point clé pour les utilisateurs français : **dans l'EEE, en Suisse et au Royaume-Uni, le gratuit de Gemini est traité comme le payant côté données**, donc pas d'usage pour améliorer les produits.

### Cited Findings
- **Gemini API gratuit** : les free tiers existent pour 2.5 Flash, Flash-Lite, 2.5 Pro et 3.x Flash. Le Pro 3.1 Preview est payant uniquement. Les données du gratuit servent à améliorer les produits de Google — [Gemini pricing](https://ai.google.dev/gemini-api/docs/pricing)
- Les conditions de Gemini précisent que des relecteurs humains peuvent lire les entrées et sorties des services gratuits. Mais *« If you're in the European Economic Area, Switzerland, or the United Kingdom, the terms under 'How Google uses Your Data' in 'Paid Services' apply to all Services »* — [Gemini API Additional Terms](https://ai.google.dev/gemini-api/terms)
- La page officielle ne publie plus de chiffres RPM/TPM/RPD pour le gratuit. Elle renvoie au tableau de bord AI Studio et précise que ces limites ne sont pas garanties — [Gemini rate limits](https://ai.google.dev/gemini-api/docs/rate-limits)
- **OpenRouter `:free`** : 20 requêtes/min. 50 requêtes/jour si l'utilisateur a acheté moins de 10 $ de crédits au total, 1000 requêtes/jour à partir de 10 $ achetés — [OpenRouter limits](https://openrouter.ai/docs/api-reference/limits)
- Modèles `:free` sur OpenRouter au 2026-09-25 : qwen/qwen3.8-27b (avec structured outputs), google/gemma-4-31b-it, gemma-4-26b-a4b-it, z-ai/glm-5.2 (contexte 32k), nvidia/nemotron-3-super-120b et nemotron-3-ultra-550b, thinkingmachines/inkling, entre autres — [OpenRouter models API](https://openrouter.ai/api/v1/models)
- **Groq, plan gratuit** : openai/gpt-oss-120b, gpt-oss-20b et qwen/qwen3.8-27b sont limités à 30 RPM, 1000 RPD, 8K TPM et 200K TPD. Les limites s'appliquent à l'organisation — [Groq rate limits](https://console.groq.com/docs/rate-limits)
- **Cerebras, essai gratuit** : gpt-oss-120b et qwen-3.8-27b sont limités à 5 RPM, 30K TPM hors cache, 90K TPM au total et 1M TPD. Les crédits de 5 $ expirent après 30 jours, puis il faut payer — [Cerebras rate limits](https://inference-docs.cerebras.ai/support/rate-limits)
- **GitHub Models** : sa situation est contradictoire. Une source signale un retrait le 2026-07-30 — [agentdeals issue #1672](https://github.com/robhunter/agentdeals/issues/1672). D'autres pages de 2026 citent encore des limites : 50 req/jour pour les modèles "high", 150 pour les "mini", 8K tokens en entrée et 4K en sortie par requête — [GetAITools](https://getaitools.dev/service/github-models). Statut à vérifier sur la doc officielle.
- **Anthropic** : pas de free tier API, seulement « a small amount of free credits » à l'inscription — [Claude pricing](https://platform.claude.com/docs/en/about-claude/pricing)

### Inferences
- Calcul pour un RPG en gratuit avec 50 lignes par requête : 100k lignes demandent environ 2000 requêtes. Il faudrait donc 40 jours avec OpenRouter sans achat, ou 2 jours avec 10 $ achetés.
- Avec Groq à 200K TPD, 3M tokens demandent environ 15 jours.
- Le free tier convient donc surtout comme "essai" dans l'app, pas comme moteur principal.

### Gaps
- Offre gratuite "Experiment" de Mistral : ni limites ni politique d'entraînement trouvées sur les pages de prix consultées.
- Chiffres RPM/RPD du gratuit Gemini par modèle : non publiés sur la doc, uniquement visibles dans AI Studio.
- Politique d'entraînement des providers derrière les modèles `:free` d'OpenRouter : non documentée dans la page consultée.

---

## 4. GPU louées / serverless pour auto-héberger un modèle ouvert

### Takeaway
Une RTX 4090 (24 Go) se loue de 0,14 à 0,42 $/h (Vast.ai, marketplace) ou 0,34 $/h (RunPod Community), soit de quoi faire tourner un modèle de 14 à 32B quantifié avec vLLM ou llama.cpp en serveur OpenAI-compatible. Colab et Kaggle offrent environ 15 à 30 h de GPU T4 par semaine gratuitement. Pour un utilisateur final, c'est plus complexe que payer une API à moins de 1 $ par jeu. Ce n'est pertinent que pour la confidentialité ou le contenu refusé par les API.

### Cited Findings
- **RunPod (page mise à jour le 2026-09-13), Pods Community / Secure** : RTX 4090 à 0,34 / 0,74 $/h ; RTX 5090 à 0,69 / 0,99 ; A40 à 0,35 / 0,49 ; L40S à 0,79 / 1,09 ; A100 80GB à 1,19–1,39 / 1,59 ; H100 à 1,99–3,49 / 2,89–3,49 — [RunPod pricing](https://www.runpod.io/pricing)
- **RunPod Serverless** (facturé à la seconde) : RTX 4090 à 1,10 $/h, L40S à 1,75 $/h, A100 80GB à 2,72 $/h — [RunPod pricing](https://www.runpod.io/pricing)
- **Vast.ai** : RTX 4090 à partir de 0,14 $/h selon la page officielle — [Vast.ai RTX 4090](https://vast.ai/pricing/gpu/RTX-4090). Une source tierce indique une médiane on-demand de 0,42 $/h au 2 sept. 2026 et des spots dès 0,11 $/h — [comparateurs agrégés via recherche](https://gpus.io/en/gpus/rtx4090). Une autre donne 0,29 à 0,59 $/h en avril 2026 — [SynpixCloud](https://www.synpixcloud.com/blog/vast-ai-vs-runpod-rtx-4090-pricing)
- **Google Colab gratuit** : T4 de 16 Go, limites hebdomadaires dynamiques d'environ 15 à 30 h, sessions jusqu'à ~12 h, sans quota fixe garanti. **Kaggle** : quota visible d'environ 30 h GPU par semaine (P100 16 Go ou 2×T4), sessions de 12 h — [Thunder Compute](https://www.thundercompute.com/blog/colab-alternatives-for-cheap-deep-learning-in-2025) ; [AIMultiple](https://aimultiple.com/free-cloud-gpu). Ce sont des sources secondaires.

### Inferences
- **Exposer un endpoint** : vLLM (`vllm serve <model>`) et `llama-server` de llama.cpp exposent tous deux une route `/v1/chat/completions` compatible OpenAI. Hoshi2Star n'aurait qu'à pointer son `base_url` vers l'URL proxy du pod (RunPod fournit un proxy HTTPS par port). Ce sont des connaissances antérieures, sans page vérifiée dans cette session.
- **Coût** : une 4090 à 0,34 $/h avec un débit batché de quelques centaines de tokens/s produirait 1,5 M tokens de sortie en environ 1 à 2 h, soit **moins de 1 $** plus le temps de téléchargement du modèle. C'est comparable aux API "flash" les moins chères, mais avec beaucoup plus de friction. Le débit est une estimation non mesurée.
- **Démarrage à froid** : pour le serverless, le chargement d'un modèle de plusieurs dizaines de Go ajoute des dizaines de secondes à quelques minutes. C'est acceptable en traitement par lots, gênant en usage interactif. Non chiffré par source ici.
- **Colab/Kaggle** : ils interdisent en pratique les usages "serveur" de longue durée. Leurs conditions n'ont pas été vérifiées ici, donc c'est à éviter comme recommandation officielle aux utilisateurs.

### Gaps
- Tarifs non relevés : Modal, Lambda, TensorDock, Salad.
- Temps de cold start chiffrés (RunPod FlashBoot, Modal) : non trouvés.

---

## 5. Batch API et prompt caching

### Takeaway
OpenAI, Anthropic, Gemini et Mistral offrent **−50 %** en Batch. Le cache d'entrée coûte de 1 à 10 % du prix normal selon le fournisseur. Les deux se cumulent chez Anthropic. Pour des milliers de lignes non interactives, Batch + cache peut ramener le coût d'un RPG à quelques dizaines de centimes, y compris sur un modèle "mini".

### Cited Findings
- **OpenAI Batch** : −50 % sur l'entrée, l'entrée en cache et la sortie (par ex. gpt-5-mini à 0,125 / 1,00). Le tier Flex a le même prix réduit pour la plupart des modèles — [OpenAI pricing](https://developers.openai.com/api/docs/pricing)
- **Anthropic Message Batches** : −50 % sur l'entrée et la sortie (Haiku 4.5 à 0,50 / 2,50 ; Sonnet 5 à 1 / 5). Le cache coûte 1,25× en écriture sur 5 min, 2× en écriture sur 1 h, et 0,1× en lecture. *« Batch API and prompt caching discounts can be combined »* — [Claude pricing](https://platform.claude.com/docs/en/about-claude/pricing)
- **Gemini Batch** : −50 % (2.5 Flash-Lite à 0,05 / 0,20). Le cache de Gemini 2.5 Flash-Lite coûte 0,01 $/MTok — [Gemini pricing](https://ai.google.dev/gemini-api/docs/pricing)
- **Mistral** : Batch −50 %, entrée en cache jusqu'à −90 % — [Mistral pricing](https://mistral.ai/pricing)
- **DeepSeek** : pas de Batch documenté. En revanche, l'entrée en cache coûte 0,006 $ (heures pleines, flash) contre 0,30 $ hors cache, et les heures creuses sont à −50 % — [DeepSeek pricing](https://api-docs.deepseek.com/quick_start/pricing)
- **OpenRouter** propose des variantes `:batch` pour certains modèles (deepseek-v4.1-flash:batch, gpt-oss-120b:batch, glm-5.3:batch, grok-4.3:batch, kimi-k3:batch) — [OpenRouter models API](https://openrouter.ai/api/v1/models)

### Inferences
- Pour Hoshi2Star, il faut garder un **préfixe stable** (prompt système, puis glossaire, puis règles) placé **avant** les lignes variables. Sinon le cache ne s'active pas.
- Le Batch est asynchrone (délai jusqu'à 24 h chez OpenAI et Anthropic d'après les connaissances antérieures, non revérifié ici). Il faut donc un mode "traduction différée" distinct du mode interactif.

### Gaps
- Délais de traitement Batch garantis et limites de taille par lot : non revérifiés dans cette session.

---

## 6. Confidentialité, rétention des données et contenu adulte (NSFW)

### Takeaway
Pour les jeux adultes, les modèles ouverts ou chinois servis par des agrégateurs sont les options les plus tolérantes en pratique, par exemple DeepSeek V4 Flash via OpenRouter, noté « no provider-level moderation ». OpenAI a suspendu son "adult mode". La politique exacte d'Anthropic et de Google sur le texte sexuel n'a pas été vérifiée ici. Côté données, OpenRouter ne stocke pas les prompts par défaut, mais chaque provider en aval a sa propre politique.

### Cited Findings
- **OpenRouter** ne stocke ni prompts ni réponses par défaut (opt-in). Il garde les métadonnées (tokens, latence). L'utilisateur peut accepter l'usage de ses données contre 1 % de remise. Il documente quels providers peuvent entraîner sur les données et permet de restreindre les providers — [OpenRouter data collection](https://openrouter.ai/docs/guides/privacy/data-collection)
- Sur OpenRouter, chaque provider a sa propre politique de rétention et d'entraînement, visible par modèle et filtrable. DeepSeek V4 Flash 0423 représente 18,4 % du trafic roleplay, *« with 1.0M context and no provider-level moderation »* — [Arcanum RPGs, blog tiers](https://arcanumrpgs.com/blog/openrouter-roleplay/). C'est une source secondaire.
- Les modèles les plus utilisés pour le roleplay sur OpenRouter (semaine du 14 sept. 2026) sont DeepSeek V4 Flash, Xiaomi MiMo, GLM et Gemini Flash — [même source](https://arcanumrpgs.com/blog/openrouter-roleplay/) ; [OpenRouter collection roleplay](https://openrouter.ai/collections/roleplay)
- **OpenAI** : l'"adult mode" de ChatGPT a été mis en pause sans date au 26 mars 2026 (d'après le Financial Times, cité par des sources secondaires). Le contenu sexuel impliquant des mineurs reste interdit même fictif — [JustAINews](https://justainews.com/companies/openai/adult-mode-in-chatgpt-explained-nsfw-erotica-porn-policy/) ; [Built In](https://builtin.com/articles/chatgpt-ai-erotica-rollout). Sur le forum développeurs, des retours indiquent que « la politique dit oui, le modèle dit non », c'est-à-dire des refus malgré la politique — [OpenAI Community](https://community.openai.com/t/openai-the-policy-says-yes-the-model-says-no-stop-sterilizing-art-implement-age-verified-adult-mode/1361089)
- Même les providers permissifs peuvent changer leurs CGU ou retirer des modèles sans préavis : c'est un risque — [Adent.io, Medium](https://adentio.medium.com/llm-options-for-nsfw-ai-platforms-when-openai-terms-block-your-use-case-d819dc9cf771)
- **Gemini gratuit** : en dehors de l'EEE, de la Suisse et du Royaume-Uni, des relecteurs humains peuvent lire les entrées et sorties. Pas de règle explicite sur le contenu sexuel dans les Additional Terms, qui renvoient à la Prohibited Use Policy — [Gemini API terms](https://ai.google.dev/gemini-api/terms)

### Inferences
- Pour les jeux adultes, recommander aux utilisateurs des modèles ouverts ou chinois via un agrégateur, ou l'auto-hébergement, plutôt qu'OpenAI, Anthropic ou Google. Prévoir aussi dans Hoshi2Star une détection des refus ("I can't help with that") pour déclencher un repli vers un autre provider.
- **Contenu impliquant des mineurs, même fictif** : c'est interdit partout chez les fournisseurs majeurs. Un jeu JP adulte avec des personnages d'apparence jeune risque des refus ou un bannissement du compte, quel que soit le provider occidental.

### Gaps
- CGU exactes d'Anthropic (Usage Policy) et de Google (Prohibited Use Policy) sur le texte sexuel entre adultes fictifs : non consultées.
- Durées de rétention par défaut d'OpenAI (abus) et de DeepSeek (serveurs en Chine) : non relevées.
- Politique de modération de DeepInfra, Together, Fireworks et Groq : non relevée.

---

## 7. Fiabilité, limites de débit, disponibilité UE/France

### Takeaway
Les limites de débit des offres gratuites sont basses et non garanties. Les offres payantes (DeepSeek avec 2500 requêtes concurrentes en flash, les agrégateurs) conviennent à un traitement de masse. Pour l'UE, le point positif vérifié est la protection des données du gratuit Gemini dans l'EEE. Mistral est l'option européenne native.

### Cited Findings
- **Concurrence DeepSeek** : 2500 pour flash, 500 pour pro — [DeepSeek pricing](https://api-docs.deepseek.com/quick_start/pricing)
- **Gemini** : *« Specified rate limits are not guaranteed and actual capacity may vary »* — [Gemini rate limits](https://ai.google.dev/gemini-api/docs/rate-limits)
- **Anthropic** : l'API est globale par défaut. `inference_geo: "us"` coûte 1,1× ; Bedrock et Vertex proposent des endpoints régionaux avec un surcoût de 10 % — [Claude pricing](https://platform.claude.com/docs/en/about-claude/pricing)
- **Groq et Cerebras** : limites gratuites détaillées en section 3 — [Groq](https://console.groq.com/docs/rate-limits) ; [Cerebras](https://inference-docs.cerebras.ai/support/rate-limits)

### Inferences
- Pour un utilisateur français sensible au RGPD : Mistral (entreprise française) ou Gemini (le gratuit est protégé dans l'EEE). Pour le moins cher : DeepSeek ou GLM via un agrégateur, en acceptant un transit de données hors UE.

### Gaps
- Disponibilité régionale de DeepSeek, Moonshot et Zhipu depuis la France, et régions d'hébergement de Mistral : non vérifiées.
- Données de disponibilité et de SLA mesurées (uptime) par provider : non trouvées.
