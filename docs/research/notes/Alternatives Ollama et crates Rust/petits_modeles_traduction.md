# Petits modèles de traduction (JA→EN/FR) pour matériel modeste — état au 2026-09-25

Périmètre : modèles ≈0,4–14B, CPU seul ou ≤8 Go VRAM, 8–16 Go RAM, texte de jeu (RPG Maker MV/MZ, Wolf). Recherche web du 2026-09-25 ; chaque fait porte sa source. Les tailles GGUF et les vitesses sans source sont signalées comme **inférences**.

## Q1 — Quels petits modèles (≈1–14B) traduisent bien JA→EN / JA→FR en 2025–2026 ?

### Takeaway
Deux catégories se distinguent. D'un côté, les modèles **spécialisés en traduction** de 1,8 à 7B : la famille Tencent Hy-MT (HY-MT1.5 de déc. 2025, puis Hy-MT2 de mai 2026, sous Apache 2.0), Google TranslateGemma (janv. 2026, 4B/12B), ByteDance Seed-X-7B (mi-2025) et Hunyuan-MT-7B (sept. 2025, vainqueur du WMT25). La prise en charge de fr et ja est vérifiée pour Hunyuan-MT-7B, HY-MT1.5 et Seed-X. Pour Hy-MT2 (33 langues) et TranslateGemma (55 langues), elle est très probable mais n'a pas été vérifiée ligne à ligne. De l'autre, les modèles **ajustés JA→EN pour le média japonais** (Shisa V2 Mistral-Nemo 12B, vntl-llama3-8b, Sugoi 14B Ultra). Ce sont les meilleurs locaux ≤14B sur le banc VNTL de visual novels, mais ils ne traduisent que vers l'anglais.

### Cited Findings
**Spécialistes traduction multilingues**
- Hy-MT2 : sorti le 21 mai 2026 en 1.8B, 7B et 30B-A3B (MoE). Il couvre 33 langues. — [GitHub Tencent-Hunyuan/Hy-MT2](https://github.com/Tencent-Hunyuan/Hy-MT2) ; [AIBase](https://news.aibase.com/news/28224)
- Hy-MT2-1.8B : licence Apache 2.0. La carte affirme que « the lightweight 1.8B model also surpasses mainstream commercial APIs from providers such as Microsoft and Doubao overall » (benchmarks FLORES, rapport arXiv 2605.22064). Ses prompts documentés gèrent terminologie, style, données structurées (préservation JSON/XML), contexte et délimiteurs. Paramètres recommandés : temp 0.7, top_p 0.6, top_k 20, rep. penalty 1.05. — [HF tencent/Hy-MT2-1.8B](https://huggingface.co/tencent/Hy-MT2-1.8B)
- La quantification AngelSlim 1,25 bit ramène Hy-MT2-1.8B à « only 440 MB », avec une inférence annoncée 1,5× plus rapide. — [HF tencent/Hy-MT2-1.8B](https://huggingface.co/tencent/Hy-MT2-1.8B)
- HY-MT1.5 (1.8B et 7B) : publié le 30/12/2025, 33 langues (dont fr et ja). Le 1.8B affiche une qualité « comparable » au 7B et dépasse selon Tencent « most commercial translation APIs ». Fonctions : intervention terminologique, traduction contextuelle, traduction formatée qui préserve les balises. — [HF tencent/HY-MT1.5-1.8B-GGUF](https://huggingface.co/tencent/HY-MT1.5-1.8B-GGUF) ; [HY-MT1.5 Technical Report arXiv 2512.24092](https://arxiv.org/pdf/2512.24092)
- Hunyuan-MT-7B (sept. 2025) : 33 langues, dont fr et ja. Il a pris la 1re place dans 30 des 31 catégories WMT25 où il concourait. Il existe aussi une variante ensemble, Hunyuan-MT-Chimera-7B. — [HF tencent/Hunyuan-MT-7B](https://huggingface.co/tencent/Hunyuan-MT-7B) ; [arXiv 2509.05209](https://arxiv.org/abs/2509.05209)
- TranslateGemma : sorti le 15/01/2026 en 4B, 12B et 27B, basé sur Gemma 3, 55 langues. Le 12B dépasse Gemma 3 27B sur WMT24++ (MetricX) et le 4B rivalise avec l'ancien 12B. — [AiCybr guide](https://aicybr.com/blog/translategemma-guide) ; [Medium](https://medium.com/@meshuggah22/googles-translategemma-proves-that-for-translation-smaller-can-actually-be-smarter-2ac828078aff)
- TranslateGemma-4B-it : WMT24++ MetricX 5.32, COMET 81.6 (moyenne multilingue). **Contexte d'entrée limité à 2K tokens.** Quantifications disponibles pour llama.cpp, Ollama et LM Studio. — [HF google/translategemma-4b-it](https://huggingface.co/google/translategemma-4b-it)
- Seed-X-PPO-7B (ByteDance, juillet 2025) : 28 langues, dont fr et ja. Ses auteurs le disent « on par with or outperforming ultra-large models like Gemini-2.5, Claude-3.5, and GPT-4 » (FLORES-200, WMT-25, évaluation humaine). Ce n'est pas un modèle de chat : il faut un tag de langue cible en fin de prompt (ex. `<de>`). — [HF ByteDance-Seed/Seed-X-PPO-7B](https://huggingface.co/ByteDance-Seed/Seed-X-PPO-7B) ; [ComfyUI wiki](https://comfyui-wiki.com/en/news/2025-07-24-bytedance-seed-x-7b-open-source-model)

**Modèles généralistes récents**
- Qwen3.5 Small (0.8B, 2B, 4B, 9B) : sortis le 2 mars 2026 sous Apache 2.0. Multimodaux, 201 langues et dialectes, modes thinking et non-thinking. — [The Rundown](https://www.therundown.ai/tools/qwen3-5-small) ; [Unsloth docs](https://unsloth.ai/docs/models/qwen3.5)
- Gemma 4 : sorti le 2 avril 2026 en E2B, E4B, 26B MoE et 31B dense, sous Apache 2.0. Plus de 140 langues, dont 35+ prises en charge « out-of-the-box ». — [Codersera](https://codersera.com/blog/gemma-4-complete-guide-2026/) ; [HF google/gemma-4-E4B-it](https://huggingface.co/google/gemma-4-E4B-it)
- Gemma 4 12B : ajout plus tardif (≈juillet 2026, arXiv 2607.02770), 11,95B paramètres, Apache 2.0, conçu pour laptop. — [HF google/gemma-4-12B-it](https://huggingface.co/google/gemma-4-12B-it). Les sources ne sont pas cohérentes : le guide d'avril annonce « quatre tailles » sans le 12B.

**Spécialistes JA→EN (média japonais)**
- Shisa V2 (Shisa.AI) existe en 7B (Qwen2.5), 8B (Llama 3.1), 12B (Mistral-Nemo), 14B (unphi4), et une v2.1 en 14B. L'entraînement inclut des données synthétiques de traduction (fiction, conversations). Licences Apache 2.0 ou MIT selon le modèle de base. — [Blog Shisa V2](https://blog.shisa.ai/posts/shisa-v2/) ; [HF shisa-v2-llama3.1-8b](https://huggingface.co/shisa-ai/shisa-v2-llama3.1-8b) ; [HF shisa-v2.1-unphi4-14b](https://huggingface.co/shisa-ai/shisa-v2.1-unphi4-14b)
- Sugoi 14B Ultra : JA→EN orienté jeux et RPG. L'auteur affirme une bonne gestion du texte à crochets des projets RPG Maker. La sortie JSON et la préservation des tags de contrôle RPG Maker restent « expérimentales ». Un system prompt est obligatoire. Réglages recommandés : temp 0.1, ≈10 lignes de contexte (« Too much context will poison the translation »). — [Blog Sugoi 14B Ultra (26/07/2025)](https://blog.sugoitoolkit.com/sugoi-llm-14b-ultra/) ; [HF sugoitoolkit/Sugoi-14B-Ultra-GGUF](https://huggingface.co/sugoitoolkit/Sugoi-14B-Ultra-GGUF)
- vntl-llama3-8b-v2 et vntl-gemma2-2b (lmg-anon) : modèles ajustés pour les visual novels JA→EN (voir scores en Q3). — [VNTL leaderboard 2026](https://huggingface.co/datasets/yukobayashi500/vntl-leaderboard-2026)
- LFM2-2.6B (Liquid) : proche de Qwen3-4B sur JP-TL-Bench (voir Q3). — [jp-tl-bench](https://github.com/shisa-ai/jp-tl-bench)

**NMT classique (très léger)**
- NLLB-200-distilled-600M : c'est le modèle de traduction le plus populaire sur Hugging Face. Il est convertible en CTranslate2 (dont int8) pour le CPU. — [Picovoice 2026](https://picovoice.ai/blog/open-source-translation/) ; [HF nllb ct2-int8](https://huggingface.co/mijuanlo/nllb-200-distilled-600M-ct2-int8)
- MADLAD-400-3B-MT couvre plus de 400 langues ; OPUS-MT propose un modèle ja-en dédié. — [Picovoice](https://picovoice.ai/blog/open-source-translation/) ; [MADLAD-400 paper](https://arxiv.org/pdf/2309.04662)
- Le banc VNTL 2026 montre que les services MT phrase par phrase sont bien moins stables d'une œuvre à l'autre que les LLM (écart +0.266 contre +0.093, p = 0.00005). L'auteur l'attribue à l'absence de contexte. — [VNTL leaderboard 2026](https://huggingface.co/datasets/yukobayashi500/vntl-leaderboard-2026)

**Hors cible (signalé)**
- Cohere « North Small Translate / CAT+ » (arXiv 12/09/2026) : MoE de 218B paramètres (25B actifs), CC-BY 4.0. Malgré son nom, il est **inutilisable sur matériel modeste**. — [arXiv 2609.13916](https://arxiv.org/abs/2609.13916)

### Inferences
- Pour un pipeline multi-cibles (EN **et** FR) sur matériel faible, les meilleurs candidats sont Hy-MT2-1.8B/7B (Apache 2.0, fonctions terminologie et formatage natives) et TranslateGemma-4B. Shisa, Sugoi et VNTL ne visent que l'anglais.
- Pour le JA→FR, aucun benchmark spécifique aux jeux n'a été trouvé. Le pivot JA→EN→FR reste une option à évaluer.
- Tower+ (Unbabel), GemmaX2, Aya Expanse, Ministral, SakuraLLM, Phi-4 : la recherche n'a pas porté sur eux en profondeur (voir Gaps). Phi-4 (14B) apparaît seulement au VNTL.

### Gaps
- Aucune fiche vérifiée pour Tower+ 2B/9B, GemmaX2, Aya Expanse 8B, Ministral 3/8B, Llama 4 (pas de petite taille dense connue), SakuraLLM (surtout JA→ZH) ni pour le modèle Fairseq offline historique de Sugoi.
- Le modèle de base de Sugoi 14B Ultra n'est pas confirmé par la page consultée (probablement Qwen2.5-14B, non vérifié).
- Le rapport Hy-MT2 (arXiv 2605.22064) n'a pas été lu : pas de chiffres par paire ja/fr.

## Q2 — Mémoire par quantification et vitesse (CPU seul / petit GPU)

### Takeaway
Seul un 1.8B tient confortablement en CPU pur avec une vitesse utilisable : HY-MT1.5-1.8B pèse 1,1–1,9 Go en GGUF mesuré, et les tailles GGUF de Hy-MT2 ne sont pas relevées. Un 7–9B en Q4_K_M demande ≈5 Go (estimation) et tient dans 8 Go de VRAM. Sur CPU DDR5, une source secondaire donne 6–10 tok/s pour un 7B, à confirmer par mesure. Un 12–14B en Q4 dépasse 8 Go de VRAM : Sugoi 14B demande ≈10 Go.

### Cited Findings
- HY-MT1.5-1.8B GGUF : Q4_K_M 1,13 Go, Q6_K 1,47 Go, Q8_0 1,91 Go. Compatible llama.cpp, Ollama et LM Studio. — [HF tencent/HY-MT1.5-1.8B-GGUF](https://huggingface.co/tencent/HY-MT1.5-1.8B-GGUF)
- Hy-MT1.5-1.8B existe aussi en GGUF 2 bits (574 Mo) et 1,25 bit (440 Mo). — [HF tencent/Hy-MT1.5-1.8B-2bit-GGUF](https://huggingface.co/tencent/Hy-MT1.5-1.8B-2bit-GGUF) ; [HF AngelSlim 1.25bit](https://huggingface.co/AngelSlim/Hy-MT1.5-1.8B-1.25bit-GGUF)
- Un GGUF officiel existe pour HY-MT1.5-7B. — [HF tencent/HY-MT1.5-7B-GGUF](https://huggingface.co/tencent/HY-MT1.5-7B-GGUF)
- Piège Ollama : avec le template par défaut, HY-MT1.5-1.8B-GGUF produit une sortie corrompue (« onse »). Un TEMPLATE correctif est donné dans la discussion HF. — [HF discussion #6](https://huggingface.co/tencent/HY-MT1.5-1.8B-GGUF/discussions/6)
- Hunyuan-MT-7B ne publie officiellement que du FP8 (et INT4 via AngelSlim), sans GGUF officiel. — [HF tencent/Hunyuan-MT-7B](https://huggingface.co/tencent/Hunyuan-MT-7B) ; [GitHub Hunyuan-MT](https://github.com/Tencent-Hunyuan/Hunyuan-MT)
- Seed-X-PPO-7B a des quantifications officielles GPTQ 8 bits et AWQ 4 bits (orientées GPU). Les auteurs « recommend against using unofficial quantized versions ». — [HF Seed-X-PPO-7B](https://huggingface.co/ByteDance-Seed/Seed-X-PPO-7B)
- Sugoi 14B Ultra demande ≈10 Go de VRAM. Sur RTX 4070 Ti : ≈300 ms par ligne (1,2 s au p95). **Chiffres issus d'un extrait de recherche web, non confirmés en lisant la page.** — [HF Sugoi-14B-Ultra-GGUF](https://huggingface.co/sugoitoolkit/Sugoi-14B-Ultra-GGUF)
- CPU seul (DDR5), modèles 7B : « 6-10 tok/s », sans précision de CPU ni de threads. En GPU, un 7B en Q4_K_M (llama.cpp, contexte 2048, batch 1) tourne à 95 tok/s sur RTX 3090 et 135 tok/s sur RTX 4090. — [mustafa.net benchmarks 2026](https://mustafa.net/llm-tokens-per-second-benchmarks/) (page lue ; source secondaire)
- Un extrait de recherche, non confirmé sur page et sans source identifiable, mentionnait Llama 3.1 8B Q4_K_M sur EPYC à ≈12,5–14,2 tok/s. Ce chiffre est à ignorer faute de source.
- TranslateGemma-4B pèse ≈5B paramètres en BF16. — [HF translategemma-4b-it](https://huggingface.co/google/translategemma-4b-it)

### Inferences
- Règle de pouce GGUF, **non sourcée ici** : Q4_K_M ≈ 0,6 Go par milliard de paramètres, Q8_0 ≈ 1,06 Go/B, plus le cache KV. Cohérent avec HY-MT1.5-1.8B (1,13 Go en Q4_K_M, 1,91 Go en Q8_0). On attend donc ≈4,5–5 Go pour un 7–8B en Q4_K_M et ≈8–9 Go pour un 14B.
- Sur un laptop DDR4/DDR5 à deux canaux, la génération CPU est limitée par la bande passante mémoire. Un 7–8B en Q4 donnera de l'ordre de 6–10 tok/s d'après la source secondaire. Un 1,8B en Q4 est ≈4× plus léger, donc plus rapide. À mesurer avec `llama-bench`.
- Une ligne de dialogue RPG fait ≈20–60 tokens de sortie. Un 1,8B sur CPU peut donc traiter une ligne en environ une seconde, un 7B CPU en plusieurs secondes (inférence).

### Gaps
- Pas de mesure CPU publiée et fiable pour Hy-MT2, TranslateGemma, Qwen3.5 ou Gemma 4 sur matériel grand public. La page Markaicode trouvée ne contient que des mesures GPU (A100).
- Taille exacte des GGUF de Hy-MT2-7B et de TranslateGemma non relevée.

## Q3 — Preuves de qualité (WMT, VNTL, JP-TL-Bench, FLORES)

### Takeaway
Sur le texte de visual novel JA→EN (VNTL, sept. 2026), les meilleurs ≤14B sont les modèles ajustés au japonais : shisa-v2-mistral-nemo-12b (0,6985) et vntl-llama3-8b-v2 (0,6952). Ils devancent Sugoi-14B-Ultra, Gemma-3-12B et nettement Qwen3-8B. Côté MT générale, Hunyuan-MT-7B a dominé le WMT25. Ni Hy-MT, ni TranslateGemma, ni Seed-X ne figurent au VNTL : on n'a pas de comparaison directe sur du texte de jeu.

### Cited Findings
- VNTL 2026 : relance du classement lmg-anon, arrêté en janv. 2025, avec 4 modèles ajoutés le 07/09/2026. Il porte sur 256 lignes (128 de Senren*Banka, 128 de Mashiro-iro Symphony), évaluées par similarité cosinus, chrF et BLEU. — [HF yukobayashi500/vntl-leaderboard-2026](https://huggingface.co/datasets/yukobayashi500/vntl-leaderboard-2026)
- Scores VNTL des locaux ≤14B :

  | Modèle | Quant | Score |
  | --- | --- | --- |
  | shisa-v2-mistral-nemo-12b | Q5_K_M | 0.6985 (#17) |
  | vntl-llama3-8b-v2 | Q8_0 | 0.6952 (#19) |
  | phi-4 14B | Q4_K_M | 0.6860 |
  | Sugoi-14B-Ultra | Q4_K_M | 0.6827 (#31) |
  | gemma-3-12b-it | Q5_K_M | 0.6759 |
  | Mistral-Nemo 12B | EXL2 5bpw | 0.6738 |
  | qwen-2.5-14b | — | 0.6648 |
  | vntl-gemma2-2b | Q8_0 | 0.6572 |
  | qwen-2.5-7b | Q8_0 | 0.6518 |
  | Qwen3-8B | Q8_0 | 0.6192 (#70) |

  Pour référence, qwen-2.5-72b obtient 0.7079. — [VNTL 2026](https://huggingface.co/datasets/yukobayashi500/vntl-leaderboard-2026)
- Le banc original et sa méthode sont décrits sur le GitHub VNTL. — [GitHub lmg-anon/vntl-benchmark](https://github.com/lmg-anon/vntl-benchmark) ; [HF lmg-anon/vntl-leaderboard](https://huggingface.co/datasets/lmg-anon/vntl-leaderboard)
- JP-TL-Bench (Shisa.AI) : comparaisons A/B par paires jugées par LLM, bidirectionnelles JA↔EN. Petits modèles (taux de victoire, score LT sur 10) :

  | Modèle | WR | LT |
  | --- | --- | --- |
  | Qwen3-4B | 44,68 % | 4.11 |
  | LFM2-2.6B | 43,83 % | 3.92 |
  | Llama-3.1-8B | 38,84 % | 2.95 |
  | Phi-4-mini | 24,94 % | 0.98 |
  | Llama-3.2-3B | 19,18 % | 0.54 |
  | LFM2-350M | 8,75 % | 0.13 |
  | TinySwallow-1.5B | 2,52 % | 0.03 |

  — [GitHub shisa-ai/jp-tl-bench](https://github.com/shisa-ai/jp-tl-bench)
- WMT25 : 60 systèmes évalués, sur des jeux de test volontairement difficiles et au niveau du document. Les traductions vers le japonais ont été évaluées par annotation humaine ESA. — [Findings WMT25 General MT, ACL Anthology](https://aclanthology.org/2025.wmt-1.22/)
- Au WMT25, Hunyuan-MT-7B a pris la 1re place dans 30 des 31 catégories de langue, en catégorie contrainte (open source, données publiques). — [HF tencent/Hunyuan-MT-7B](https://huggingface.co/tencent/Hunyuan-MT-7B) ; [GitHub Hunyuan-MT](https://github.com/Tencent-Hunyuan/Hunyuan-MT)
- TranslateGemma-12B dépasse Gemma 3 27B sur WMT24++ (MetricX). TranslateGemma-4B obtient MetricX 5.32 et COMET 81.6. — [HF translategemma-4b-it](https://huggingface.co/google/translategemma-4b-it) ; [AiCybr](https://aicybr.com/blog/translategemma-guide)
- Sugoi 14B Ultra : BLEU 21.38 contre 13.67 pour la version quantifiée précédente. Ce chiffre est auto-déclaré, sans jeu de test public identifié. — [Blog Sugoi](https://blog.sugoitoolkit.com/sugoi-llm-14b-ultra/)

### Inferences
- Au VNTL, Qwen3-8B (0.619) fait moins bien que Qwen2.5-7B (0.652). Un généraliste récent n'est donc pas automatiquement meilleur en JA→EN littéraire : il faut tester Qwen3.5 et Gemma 4 avant de les recommander.
- Les chiffres Hy-MT et Seed-X sont auto-déclarés, sur texte général. Leur tenue sur du dialogue de jeu (onomatopées, registre, noms propres) reste à mesurer, idéalement sur un petit set interne de Hoshi2Star.

### Gaps
- Aucune évaluation indépendante JA→FR trouvée pour les petits modèles, ni aucun score VNTL pour Hy-MT, TranslateGemma, Seed-X, Qwen3.5 ou Gemma 4.
- Classement WMT25 détaillé par paire en→ja / cs→… non extrait (PDF non lu).

## Q4 — Placeholders, sortie structurée et modes d'échec à petite taille

### Takeaway
Sans contrainte, les petits modèles (0,6–4B) produisent un JSON invalide dans 7 à 21 % des cas. Le décodage contraint par grammaire (XGrammar, Outlines, GBNF de llama.cpp) ramène la validité structurelle à 100 %, mais pas la justesse du contenu. Hy-MT et Sugoi annoncent une gestion des balises et du formatage ; elle reste auto-déclarée ou « expérimentale ». Le masquage `⟦ph_NNN⟧` avec validation après coup (ADR-002) reste nécessaire.

### Cited Findings
- Étude sept. 2026 sur Qwen3 0.6B/4B, Llama 3.2 1B/3B et Phi-4-mini. En décodage natif (greedy), la validité du schéma va de 78,6 à 92,9 %. Avec XGrammar ou Outlines, elle atteint 100 % pour tous. — [arXiv 2609.23742](https://arxiv.org/html/2609.23742)
- Même étude, justesse du contenu natif → XGrammar :

  | Modèle | Natif | XGrammar |
  | --- | --- | --- |
  | Llama-1B | 0.771 | 0.986 |
  | Qwen-0.6B | 0.871 | 0.943 (écart résiduel 5,7 pts) |
  | Qwen-4B | 1.000 | 1.000 |

  Les échecs de sémantique d'instruction résistent au décodage contraint. — [arXiv 2609.23742](https://arxiv.org/html/2609.23742)
- Coût du décodage contraint : XGrammar compile en 4–8 ms et réduit le débit de 1,6 à 3,7 %. Outlines compile en 2 à 4,9 s (jusqu'à 19,5 s) et ralentit la génération de 9 à 13 %. Le GBNF de llama.cpp n'est pas évalué. — [arXiv 2609.23742](https://arxiv.org/html/2609.23742)
- Hy-MT2-1.8B fournit des modèles de prompt pour préserver JSON/XML et les délimiteurs. HY-MT1.5 annonce une « formatted translation » qui préserve les balises. — [HF Hy-MT2-1.8B](https://huggingface.co/tencent/Hy-MT2-1.8B) ; [HF HY-MT1.5-1.8B-GGUF](https://huggingface.co/tencent/HY-MT1.5-1.8B-GGUF)
- Seed-X n'est pas un modèle de chat et « unexpected to support other tasks ». Il ne faut donc pas compter sur lui pour une sortie JSON à consignes complexes. — [HF Seed-X-PPO-7B](https://huggingface.co/ByteDance-Seed/Seed-X-PPO-7B)
- Sugoi 14B Ultra : sortie JSON et préservation des tags de contrôle RPG Maker annoncées comme expérimentales. Trop de contexte dégrade la traduction. — [Blog Sugoi](https://blog.sugoitoolkit.com/sugoi-llm-14b-ultra/)
- TranslateGemma n'accepte que 2K tokens en entrée, ce qui limite la taille des lots et le contexte. — [HF translategemma-4b-it](https://huggingface.co/google/translategemma-4b-it)
- Template Ollama incorrect pour HY-MT1.5-1.8B : sortie tronquée ou corrompue. — [HF discussion](https://huggingface.co/tencent/HY-MT1.5-1.8B-GGUF/discussions/6)

### Inferences
- Pour Hoshi2Star, avec un petit modèle, mieux vaut privilégier des lots courts (une ligne ou quelques lignes numérotées), une grammaire GBNF ou un schéma JSON côté serveur (Ollama et llama.cpp acceptent un format/grammaire), et une validation stricte des placeholders avec retry ou split. Les modèles MT spécialisés sans chat (Seed-X, et probablement TranslateGemma) conviennent mieux en mode « un segment en entrée, un segment en sortie » qu'en JSON par lot.

### Gaps
- Aucune mesure publique du taux de préservation de placeholders du type `\V[n]` / `⟦ph⟧` par ces modèles.

## Q5 — Licences (usage commercial ?)

### Takeaway
Plusieurs licences permissives existent : Apache 2.0 pour Hy-MT2, Qwen3.5, Gemma 4, MADLAD-400 et OPUS-MT ; Apache/MIT pour Shisa V2. Seed-X est sous OpenMDW, dont le texte n'a pas été lu. Deux pièges : **Hunyuan-MT-7B** (Tencent Hunyuan Community License, non applicable dans l'UE, au Royaume-Uni et en Corée du Sud) et **NLLB-200** (CC-BY-NC, non commercial). TranslateGemma reste sous les Gemma Terms. Pour un utilisateur basé en France, c'est décisif.

### Cited Findings
- Hunyuan-MT-7B relève de la « Tencent Hunyuan Community License Agreement », qui « DOES NOT APPLY IN THE EUROPEAN UNION, UNITED KINGDOM AND SOUTH KOREA ». Une licence séparée est requise au-delà de 100 M d'utilisateurs actifs mensuels. Interdiction d'utiliser les sorties pour améliorer un autre modèle d'IA. — [License.txt Hunyuan-MT](https://raw.githubusercontent.com/Tencent-Hunyuan/Hunyuan-MT/main/License.txt)
- Hy-MT2 (1.8B, 7B, 30B-A3B) est sous Apache 2.0, ce que la presse présente comme un assouplissement commercial. — [HF tencent/Hy-MT2-1.8B](https://huggingface.co/tencent/Hy-MT2-1.8B) ; [Startup Fortune](https://startupfortune.com/tencent-makes-hy-mt2-easier-for-startups-to-use-commercially/)
- Seed-X : OpenMDW. — [HF Seed-X-PPO-7B](https://huggingface.co/ByteDance-Seed/Seed-X-PPO-7B) ; [GitHub Seed-X-7B](https://github.com/ByteDance-Seed/Seed-X-7B)
- TranslateGemma : Gemma Terms, avec acceptation requise sur HF. — [HF translategemma-4b-it](https://huggingface.co/google/translategemma-4b-it)
- Gemma 4 : Apache 2.0, contre les Gemma Terms pour les générations précédentes. — [Codersera](https://codersera.com/blog/gemma-4-complete-guide-2026/) ; [HF gemma-4-12B-it](https://huggingface.co/google/gemma-4-12B-it)
- Qwen3.5 Small : Apache 2.0. — [The Rundown](https://www.therundown.ai/tools/qwen3-5-small)
- Shisa V2 : Apache 2.0 ou MIT selon le modèle de base. Le blog annonce un usage commercial autorisé. — [Blog Shisa V2](https://blog.shisa.ai/posts/shisa-v2/)
- NLLB-200 : CC-BY-NC-4.0, sans usage commercial. MADLAD-400 et OPUS-MT : Apache 2.0. — [HF facebook/nllb-200-distilled-600M](https://huggingface.co/facebook/nllb-200-distilled-600M) ; [Picovoice](https://picovoice.ai/blog/open-source-translation/)

### Inferences
- Pour un outil distribué depuis la France, la famille Hy-MT2 est la variante Tencent sûre à recommander. Hunyuan-MT-7B et, par prudence, HY-MT1.5 sont à éviter tant que leur licence n'est pas vérifiée. Le logiciel ne distribue pas les poids : l'utilisateur les télécharge via Ollama. La licence reste pourtant pertinente pour l'utilisateur UE.

### Gaps
- Licences de HY-MT1.5, Sugoi 14B Ultra, vntl-llama3-8b-v2 et Qwen3-8B non vérifiées dans cette session.
- Le texte d'OpenMDW (obligations exactes) n'a pas été lu.
