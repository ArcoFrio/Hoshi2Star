# Runtimes locaux d'inférence LLM alternatifs à Ollama (état septembre 2026, focus matériel modeste)

> Notes de recherche (Claude, 2026-09-25). Convention : **[vérifié 2026]** = confirmé par une source consultée dans cette session ; **[non re-vérifié]** = fait connu (connaissances antérieures) mais pas re-confirmé par une source 2026 dans cette session, à revérifier avant décision. Les articles de type « review 2026 » d'agrégateurs (promptquorum, codersera, runaihome…) sont de qualité variable ; préférer les dépôts officiels.

## Q1 — Panorama par runtime : base technique, licence, statut de maintenance

### Takeaway
En 2026, l'écosystème grand public repose massivement sur **llama.cpp/ggml** : LM Studio, Jan, KoboldCpp, llamafile, Lemonade, LocalAI, Docker Model Runner, GPT4All l'embarquent ; Ollama s'en est partiellement détaché (moteur maison sur ggml). Les moteurs non-ggml (vLLM, SGLang, ExLlama, mistral.rs, OpenVINO, MLC) visent surtout GPU NVIDIA ou matériel spécifique. Deux projets à éviter pour du neuf : **IPEX-LLM (archivé janv. 2026)** et **GPT4All (déclaré EOL par un mainteneur)**.

### Cited Findings

**Ollama (baseline)**
- Ollama a annoncé (mi-2025) un nouveau moteur qui n'utilise plus llama.cpp mais une implémentation maison directement sur **ggml** (la bibliothèque tenseur sous llama.cpp) [vérifié] — [GitHub issue #9959](https://github.com/ollama/ollama/issues/9959) ; [HN « Ollama's new engine for multimodal models »](https://news.ycombinator.com/item?id=44001087) ; [Digital Spaceport](https://digitalspaceport.com/ollama-multimodal-gets-launched-and-the-controversy-surrounding-llama-cpp/)
- Controverse ancienne : attribution insuffisante de llama.cpp pendant la croissance initiale [vérifié] — [Arsturn](https://www.arsturn.com/blog/is-ollama-stealing-from-llamacpp-the-open-source-ai-controversy-explained) ; [HN](https://news.ycombinator.com/item?id=44001087)
- Article critique « Friends Don't Let Friends Use Ollama » (15 avril 2026, ~1 620 upvotes sur r/LocalLLaMA selon codersera) : absence de notice MIT llama.cpp dans les binaires (issue #3185 sans réponse >400 jours), moteur ggml ayant « réintroduit des bugs » (structured output, vision), app GUI de juillet 2025 publiée depuis un dépôt privé sans licence puis fusionnée dans le dépôt principal en nov. 2025, pivot cloud fin 2025, CVE-2025-51471 (exfiltration de token), registre à noms hashés, quantisations limitées (Q4_K_S, Q4_K_M, Q8_0, F16, F32 à l'import selon l'auteur), nommage trompeur « DeepSeek-R1 » pour des distillats (janv. 2025). **Source partisane, chiffres de perf non reproduits** — [Sleeping Robots](https://sleepingrobots.com/dreams/stop-using-ollama/) ; réception r/LocalLLaMA résumée par [codersera](https://codersera.com/blog/best-ollama-alternatives-2026/)
- Contre-argument le plus voté du fil (≈455 upvotes) : aucune alternative ne remplace encore l'UX d'Ollama comme porte d'entrée — [codersera](https://codersera.com/blog/best-ollama-alternatives-2026/) ; voir aussi [falcao.org « Ollama in 2026: The Boring Choice »](https://falcao.org/posts/ollama-the-boring-standard/)
- Critique récurrente : contexte par défaut court (`num_ctx` 2048) tronqué silencieusement — [codersera](https://codersera.com/blog/best-ollama-alternatives-2026/). **Attention** : la valeur par défaut a évolué selon les versions ; à vérifier sur la version ciblée [non re-vérifié].
- Licence MIT [non re-vérifié] — [ollama/ollama](https://github.com/ollama/ollama)

**llama.cpp / llama-server (ggml-org)**
- Serveur HTTP C/C++ avec endpoints OpenAI, Anthropic Messages et Responses, grammaires, continuous batching, spéculatif, KV cache quantifié, router multi-modèles [vérifié] — [README server](https://github.com/ggml-org/llama.cpp/blob/master/tools/server/README.md)
- **Router mode** livré en décembre 2025 (annonce 11 déc.) : `llama-server` sans `-m` devient un proxy qui découvre les GGUF du cache et lance chaque modèle dans un processus enfant, chargement/déchargement à la demande → fonctionnalité « à la Ollama » en natif [vérifié] — [HF blog « New in llama.cpp: Model Management »](https://huggingface.co/blog/ggml-org/model-management-in-llamacpp) ; [runaihome](https://runaihome.com/blog/llama-server-router-mode-multi-model-setup-2026/)
- Licence MIT [non re-vérifié] — [ggml-org/llama.cpp](https://github.com/ggml-org/llama.cpp)

**LM Studio (Element Labs)**
- App desktop **propriétaire mais gratuite pour usage personnel et commercial** depuis juillet 2025 (suppression de la licence commerciale séparée) ; termes : usage « personnel et/ou interne à l'entreprise » [vérifié] — [codersera guide 2026](https://codersera.com/blog/lm-studio-complete-guide-2026/) ; [Thunder Compute](https://www.thundercompute.com/blog/lm-studio-local-llm)
- Moteurs : llama.cpp (GGUF) et MLX (Apple) ; CLI `lms`, SDK Python/TS sous MIT ; démon headless **llmster** (cœur de LM Studio sans GUI) [vérifié] — [docs headless](https://lmstudio.ai/docs/developer/core/headless) ; [codersera](https://codersera.com/blog/lm-studio-complete-guide-2026/)
- Version observée : 0.4.20 [vérifié, source secondaire] — [QWE AI Academy](https://www.qwe.edu.pl/ai-tools/install-lm-studio-local-llm-desktop-app/)

**KoboldCpp (LostRuins)**
- Fork/distribution llama.cpp « One File. Zero Install. », **AGPL-3.0**, UI KoboldAI intégrée ; build `nocuda` avec Vulkan recommandé pour AMD ; releases très fréquentes (v1.121 en sept. 2026 selon un agrégateur) [vérifié] — [GitHub](https://github.com/lostruins/koboldcpp) ; [releases](https://github.com/LostRuins/koboldcpp/releases) ; [promptquorum](https://www.promptquorum.com/power-local-llm/koboldcpp-review)
- Expose API KoboldAI + endpoints OpenAI-compatibles [non re-vérifié]

**Jan (Menlo / janhq)**
- App desktop **Apache-2.0**, wrapper **Tauri** (TypeScript) autour de llama.cpp [vérifié] — [Petronella](https://petronellatech.com/blog/jan-ai-local-first-llm/) ; [docs llama.cpp engine](https://www.jan.ai/docs/desktop/local-engine/llama-cpp)
- v0.8.0 (22 mai 2026) : un seul process « router » llama.cpp qui charge/décharge les modèles à la demande, Multi-Token Prediction ; v0.8.1 (29 mai 2026) : providers Anthropic-compatibles, refonte des réglages llama.cpp ; 0.8.4 publiée ensuite [vérifié] — [changelog v0.8.0](https://www.jan.ai/changelog/2026-05-22-jan-v0.8.0) ; [changelog v0.8.1](https://www.jan.ai/changelog/2026-05-29-jan-v0.8.1)
- Pertinent pour Hoshi2Star : même pile (Tauri) → exemple de référence d'embarquement de llama-server en sidecar [inférence, voir Q5].

**LocalAI (mudler)**
- **MIT**, API OpenAI-compatible devant llama.cpp et de nombreux autres backends (image, voix, vidéo) ; « No GPU required » [vérifié] — [GitHub](https://github.com/mudler/localai) ; [backends](https://localai.io/docs/backends/index.html)
- v4.3.0 (mai 2026) : prompt cache llama.cpp activé par défaut, backends distribués en images OCI signées cosign, attribution par clé API ; juin 2026 : Gemma 4 QAT + paires MTP pour décodage spéculatif [vérifié] — [News LocalAI](https://localai.io/basics/news/)
- Orientation self-hosting/serveur (Docker), pas app grand public [inférence].

**llamafile (Mozilla.ai)**
- v0.10.0 (19 mars 2026) : cœur reconstruit pour suivre llama.cpp upstream plus facilement ; exécutable unique Cosmopolitan multi-OS avec poids embarqués possibles ; Metal out-of-the-box, CUDA restauré, whisper.cpp et stable-diffusion en sous-modules ; v0.10.5 au 3 août 2026 [vérifié] — [blog Mozilla.ai](https://blog.mozilla.ai/llamafile-reloaded-whats-new-in-v0-10-0/) ; [Help Net Security](https://www.helpnetsecurity.com/2026/03/20/llamafile-0-10-0-released/) ; [Phoronix](https://www.phoronix.com/news/Mozilla-AI-Llamafile-0.10)
- Licence Apache-2.0 (llamafile) + MIT (llama.cpp) [non re-vérifié] — [mozilla-ai/llamafile](https://github.com/mozilla-ai/llamafile)

**vLLM**
- Serveur haut débit (PagedAttention, continuous batching) orienté GPU datacenter ; **GGUF expérimental, GPU uniquement** (plugin), pas de noyau CPU GGUF [vérifié] — [docs GGUF](https://docs.vllm.ai/en/latest/features/quantization/gguf/) ; [DEV](https://dev.to/mrsaynothing/can-vllm-run-gguf-yes-on-gpu-only-1hpj)
- Backend CPU : sans AVX-512, repli AVX2 « non recommandé, fonctionnalités de base » ; une RFC propose un backend CPU llama.cpp [vérifié] — [docs CPU](https://docs.vllm.ai/en/v0.10.1/getting_started/installation/cpu.html) ; [RFC #25590](https://github.com/vllm-project/vllm/issues/25590) ; [issue AVX2 #6178](https://github.com/vllm-project/vllm/issues/6178)
- Licence Apache-2.0 [non re-vérifié] — [vllm-project/vllm](https://github.com/vllm-project/vllm)

**SGLang** — serveur haut débit (RadixAttention) orienté GPU NVIDIA/AMD Instinct, Apache-2.0 [non re-vérifié] — [sgl-project/sglang](https://github.com/sgl-project/sglang). Pas de source 2026 consultée ; non pertinent pour laptop sans GPU [inférence].

**ExLlamaV3 + TabbyAPI (turboderp)**
- Format **EXL3** (basé sur QTIP), cache KV quantifié 2–8 bits ; noyaux **CUDA uniquement** (CUDA 12.4+), tensor/expert-parallel, offload CPU pour MoE ; TabbyAPI = serveur OpenAI-compatible officiel [vérifié] — [turboderp-org/exllamav3](https://github.com/turboderp-org/exllamav3) ; [hysenlabs](https://hysenlabs.com/en/projects/turboderp-org-exllamav3)
- ExLlamaV3 MIT, TabbyAPI AGPL-3.0 [non re-vérifié] — [TabbyAPI](https://github.com/theroyallab/tabbyAPI)

**text-generation-webui (oobabooga)** — UI Gradio multi-backends (llama.cpp, ExLlamaV2/V3, Transformers), API OpenAI-compatible, AGPL-3.0 [non re-vérifié] — [oobabooga/text-generation-webui](https://github.com/oobabooga/text-generation-webui). Aucune source 2026 consultée.

**MLC LLM / WebLLM** — compilation TVM vers Vulkan/Metal/CUDA/ROCm/WebGPU, serveur OpenAI-compatible, WebLLM dans le navigateur (WebGPU), Apache-2.0 [non re-vérifié] — [mlc-ai/mlc-llm](https://github.com/mlc-ai/mlc-llm) ; [mlc-ai/web-llm](https://github.com/mlc-ai/web-llm). Aucune source 2026 consultée sur l'activité.

**GPT4All (Nomic)**
- Statut contradictoire : un mainteneur a déclaré GPT4All **EOL** (ne supporte que Q4_0, Q4_1, FP16) ; issues « Is GPT4all dead? » (#3605) et « Development stopped? » (#3558) ; liste officielle de modèles >12 mois ; un article affirme au contraire des mises à jour régulières (source faible) [vérifié : contradiction] — [issue #3605](https://github.com/nomic-ai/gpt4all/issues/3605) ; [issue #3558](https://github.com/nomic-ai/gpt4all/issues/3558) ; contredit par [ML Journey](https://mljourney.com/gpt4all-review-2026-is-it-still-worth-using/)
- Licence MIT [non re-vérifié] — [nomic-ai/gpt4all](https://github.com/nomic-ai/gpt4all)

**Msty** — app desktop propriétaire (freemium) s'appuyant sur un moteur local dérivé d'Ollama [non re-vérifié] — [msty.ai](https://msty.ai). Aucune source 2026 consultée.

**Lemonade (AMD)**
- Serveur local **Apache-2.0**, API OpenAI ; orchestre llama.cpp (GGUF, **Vulkan par défaut**, ROCm), **FastFlowLM** et **ONNX Runtime GenAI** ; seul des trois à exploiter le **NPU XDNA2** (Ryzen AI 300 / Strix Point minimum, Strix Halo) ; Windows (NPU), Linux (NPU expérimental via FastFlowLM), macOS (Metal) ; installeurs one-click [vérifié] — [FAQ Lemonade](https://lemonade-server.ai/docs/guide/faq/) ; [docs backends llama.cpp](https://lemonade-server.ai/docs/guide/configuration/llamacpp/) ; [AMD](https://www.amd.com/en/developer/resources/technical-articles/2025/ryzen-ai-radeon-llms-with-lemonade.html) ; [HN](https://news.ycombinator.com/item?id=47612724)
- Vulkan de Lemonade couvre aussi iGPU/dGPU AMD, NVIDIA, Intel [vérifié] — [docs backends](https://lemonade-server.ai/docs/guide/configuration/llamacpp/)

**Intel : OpenVINO Model Server (OVMS) / OpenVINO GenAI / IPEX-LLM**
- OVMS : serveur C++ ; LLM via continuous batching + paged attention exposés en OpenAI `chat/completions`, `completions`, `responses`, embeddings, rerank [vérifié] — [docs OVMS 2026](https://docs.openvino.ai/2026/model-server/ovms_demos_continuous_batching.html) ; [présentation](https://docs.openvino.ai/2026/model-server/ovms_what_is_openvino_model_server.html)
- OpenVINO 2026.0 : meilleure gestion NPU Core Ultra, GPT-OSS-20B CPU/GPU ; sur NPU seulement de petits modèles listés (Qwen2.5-1B-Instruct, Qwen-2.5-coder-0.5B…) ; 2026.3 corrige TTFT/streaming NPU [vérifié] — [Phoronix](https://www.phoronix.com/news/Intel-OpenVINO-2026.0-Released) ; [OpenVINO GenAI NPU](https://docs.openvino.ai/2026/openvino-workflow-generative/inference-with-genai/inference-with-genai-on-npu.html) ; [release notes](https://docs.openvino.ai/2026/about-openvino/release-notes-openvino.html)
- Nécessite des modèles convertis au format OpenVINO IR (pas GGUF natif) [non re-vérifié]. Licence Apache-2.0 [non re-vérifié].
- **IPEX-LLM archivé (lecture seule) le 28 janvier 2026** ; correctifs remontés dans PyTorch 2.9 ; les « portable zip » llama.cpp/Ollama Intel GPU/NPU datent de 2025 [vérifié] — [intel/ipex-llm](https://github.com/intel/ipex-llm) ; [issue #13325](https://github.com/intel/ipex-llm/issues/13325)

**mistral.rs (EricLBuehler)**
- Rust, **MIT**, crate `mistralrs` sur crates.io + bindings Python ; backends CUDA (FlashAttention), Metal, CPU (pas de Vulkan mentionné) ; formats GGUF 2–8 bits, GPTQ, AWQ, HQQ, FP8, BNB, ISQ (quantification à la volée) ; OpenAI `/v1` + Anthropic Messages ; tool calling avec grammaire / schéma strict ; continuous batching par défaut, PagedAttention (CUDA, Apple), prefix caching ; v0.8.2 [vérifié] — [GitHub mistral.rs](https://github.com/EricLBuehler/mistral.rs)

**llama-swap (mostlygeek)**
- Proxy Go qui expose une porte OpenAI/Anthropic unique et démarre/arrête à la demande l'upstream configuré par modèle (YAML) — llama-server, vLLM, etc. ; actif en 2026 [vérifié] — [GitHub](https://github.com/mostlygeek/llama-swap) ; [releases](https://github.com/mostlygeek/llama-swap/releases) ; [glukhov.org](https://www.glukhov.org/llm-hosting/llama-swap/)
- En partie concurrencé par le router mode natif de llama-server (déc. 2025) [inférence d'après HF blog ci-dessus].

**Nouveaux venus 2025–2026**
- **Docker Model Runner** (avril 2025) : processus natifs sur llama.cpp, modèles en artefacts OCI via Docker Hub, API OpenAI port 12434 [vérifié] — [Docker blog](https://www.docker.com/blog/run-llms-locally/) ; [codersera](https://codersera.com/blog/docker-model-runner-local-llm-guide-2026/)
- **Microsoft Foundry Local** : cité comme cible supportée par Copilot CLI depuis avril 2026 [vérifié, source secondaire] — [DEV Community](https://dev.to/thegatewayguy/the-local-llm-stack-in-2026-what-actually-works-ib1). Détails techniques (ONNX Runtime, NPU Windows) [non re-vérifié].
- Lemonade (2025–2026) et llamafile 0.10 (2026) : voir ci-dessus.

### Inferences
- Pour un public non technique sur PC modeste, les options « un clic » sérieuses et maintenues sont LM Studio (propriétaire), Jan (Apache-2.0), KoboldCpp (AGPL), Lemonade (Apache-2.0, surtout AMD) et Ollama.
- Les runtimes « serveur haut débit » (vLLM, SGLang, ExLlama/TabbyAPI, OVMS) sont hors cible pour 8–16 Go RAM sans GPU NVIDIA.

### Gaps
- Maintenance (dernier push GitHub, vérifié le 2026-09-25, détails en Q6) : SGLang, WebLLM, textgen (ex-text-generation-webui, dépôt renommé `oobabooga/textgen`), MLC LLM (2026-08-17) sont actifs ; **GPT4All sans push depuis le 2025-05-27** (confirme l'arrêt de fait) ; ExLlamaV2 ralenti (dernier push 2026-03-04) au profit de V3. Leurs fonctionnalités 2026 n'ont pas été re-vérifiées par une source 2026.
- Msty : aucune source consultée.
- Licence exacte de la GUI Ollama après fusion de nov. 2025 non confirmée par source primaire (le dépôt entier est déclaré MIT).

## Q2 — Compatibilité API : OpenAI, structured output / JSON schema / GBNF, streaming, requêtes parallèles

### Takeaway
Quasiment tous exposent une API OpenAI-compatible. **llama-server** est la référence la plus complète pour du local : `/v1/chat/completions`, `/v1/models`, JSON schema, grammaires GBNF, streaming, slots parallèles + continuous batching, clé API. mistral.rs et OVMS offrent aussi continuous batching ; Ollama gère le parallélisme via slots (`OLLAMA_NUM_PARALLEL`) avec un coût mémoire multiplié.

### Cited Findings
- llama-server : `/v1/chat/completions` (streaming), `/v1/completions`, `/v1/models`, `/v1/embeddings`, `/v1/responses`, Anthropic Messages, `/reranking` ; `response_format` JSON simple ou contraint par JSON Schema ; grammaires BNF (GBNF) ; continuous batching, slots parallèles ; `--api-key` / `--api-key-file` ; CORS configurable [vérifié] — [README server](https://github.com/ggml-org/llama.cpp/blob/master/tools/server/README.md)
- Ollama : structured outputs par JSON Schema (paramètre `format`) [non re-vérifié : l'extrait de recherche ne correspondait pas clairement à la source ; à confirmer sur docs.ollama.com] ; `OLLAMA_NUM_PARALLEL` = conversations simultanées, chacune avec son KV cache, contexte alloué = `num_ctx × slots` [vérifié] — [Easton](https://eastondev.com/blog/en/posts/ai/20260410-ollama-performance-optimization/) ; API OpenAI-compatible `/v1` [non re-vérifié] — [ollama/ollama](https://github.com/ollama/ollama)
- Critique : le moteur ggml d'Ollama aurait cassé le structured output à un moment (source partisane) — [Sleeping Robots](https://sleepingrobots.com/dreams/stop-using-ollama/)
- mistral.rs : OpenAI `/v1` + Anthropic, tool calling à grammaire / schéma strict, continuous batching par défaut [vérifié] — [GitHub](https://github.com/EricLBuehler/mistral.rs)
- OVMS : OpenAI `chat/completions`, `completions`, `responses`, embeddings, rerank ; continuous batching + paged attention ; streaming conforme OpenAI corrigé sur NPU en 2026.3 [vérifié] — [docs OVMS](https://docs.openvino.ai/2026/model-server/ovms_demos_continuous_batching.html) ; [release notes](https://docs.openvino.ai/2026/about-openvino/release-notes-openvino.html)
- Lemonade : API OpenAI standard [vérifié] — [FAQ](https://lemonade-server.ai/docs/guide/faq/)
- LocalAI : API OpenAI-compatible ; en juin 2026 scoring/NER concurrents via la file de tâches de llama.cpp server [vérifié] — [News LocalAI](https://localai.io/basics/news/)
- Docker Model Runner : API OpenAI port 12434 [vérifié] — [codersera](https://codersera.com/blog/docker-model-runner-local-llm-guide-2026/)
- TabbyAPI : API OpenAI-compatible, templates Jinja2 HF [vérifié] — [exllamav3](https://github.com/turboderp-org/exllamav3)
- llama-swap : façade OpenAI + Anthropic unique [vérifié] — [GitHub](https://github.com/mostlygeek/llama-swap)
- LM Studio : serveur API local + `lms` + llmster [vérifié] — [docs headless](https://lmstudio.ai/docs/developer/core/headless) ; endpoints OpenAI `/v1/*` et structured output JSON schema [non re-vérifié]
- vLLM/SGLang : OpenAI-compatible, structured output (xgrammar/outlines), continuous batching [non re-vérifié] — [vLLM](https://github.com/vllm-project/vllm)

### Inferences
- Tout runtime basé sur llama-server hérite des grammaires GBNF et du JSON schema ; c'est le choix le plus sûr pour une app qui a besoin de sortie JSON fiable (ex. lots de segments à traduire).
- Pour Hoshi2Star, coder contre l'API OpenAI `/v1/chat/completions` + `response_format: json_schema` rend l'app compatible avec llama-server, LM Studio, Jan, Lemonade, LocalAI, Ollama `/v1` et mistral.rs sans code spécifique ; les champs GBNF (`grammar`) restent une extension llama.cpp.

### Gaps
- Pas de matrice primaire vérifiée pour le support `json_schema` strict de LM Studio, KoboldCpp, Jan (hérité de llama.cpp mais non confirmé).

## Q3 — Backends matériels et adéquation au matériel modeste (CPU seul, iGPU, AMD/Intel, 4–8 Go VRAM)

### Takeaway
Sur machine sans GPU NVIDIA, **Vulkan via llama.cpp** est devenu le chemin universel (AMD/Intel iGPU et dGPU, Windows et Linux) ; Ollama l'a adopté (expérimental à partir de 0.12.6, désormais backend par défaut quand installé). Les NPU restent de niche : AMD XDNA2 via Lemonade/FastFlowLM, Intel via OpenVINO (petits modèles). vLLM/SGLang/ExLlama sont inadaptés sans GPU NVIDIA.

### Cited Findings
- Ollama GPU : CUDA (compute ≥5.0, driver ≥550), ROCm v7 (liste restreinte de Radeon RX 7000/9000, override `HSA_OVERRIDE_GFX_VERSION`), Metal, **Vulkan sur Windows et Linux, backend par défaut lorsqu'il est installé**, `GGML_VK_VISIBLE_DEVICES=-1` pour désactiver ; peut nécessiter `cap_perfmon` pour le reporting VRAM [vérifié] — [docs.ollama.com/gpu](https://docs.ollama.com/gpu)
- Vulkan introduit expérimentalement dans Ollama 0.12.6 (build from source au départ) pour couvrir AMD/Intel sans ROCm/SYCL [vérifié] — [Phoronix](https://www.phoronix.com/news/ollama-Experimental-Vulkan) ; [issue #11247](https://github.com/ollama/ollama/issues/11247)
- Lemonade : Vulkan par défaut (AMD iGPU/dGPU, NVIDIA, Intel, Adreno ARM64 Linux), ROCm, CPU x86, NPU XDNA2 (Ryzen AI 300+) [vérifié] — [docs llama.cpp Lemonade](https://lemonade-server.ai/docs/guide/configuration/llamacpp/) ; [FAQ](https://lemonade-server.ai/docs/guide/faq/)
- KoboldCpp : pour AMD, essayer d'abord Vulkan (build `nocuda`) [vérifié] — [promptquorum](https://www.promptquorum.com/power-local-llm/koboldcpp-review) ; [GitHub](https://github.com/lostruins/koboldcpp)
- Benchmark Mozilla.ai sur **Steam Deck OLED (APU AMD Van Gogh, 16 Go partagés, Vulkan)** : une meilleure chaîne de compilation de shaders Vulkan a donné **+63,3 % en prompt processing** sur Qwen 9B Q4_K_M ; llama.cpp/llamafile en tête [vérifié] — [Mozilla.ai, 17 sept. 2026](https://blog.mozilla.ai/benchmarking-local-llm-servers-llama-cpp-llamafile-lm-studio-and-ollama/)
- vLLM : AVX2 sans AVX-512 « non recommandé » ; GGUF GPU uniquement [vérifié] — [docs CPU](https://docs.vllm.ai/en/v0.10.1/getting_started/installation/cpu.html) ; [DEV](https://dev.to/mrsaynothing/can-vllm-run-gguf-yes-on-gpu-only-1hpj)
- ExLlamaV3 : CUDA 12.4+ uniquement [vérifié] — [hysenlabs](https://hysenlabs.com/en/projects/turboderp-org-exllamav3)
- mistral.rs : CUDA, Metal, CPU ; pas de Vulkan/ROCm annoncé [vérifié par absence dans README] — [GitHub](https://github.com/EricLBuehler/mistral.rs)
- OpenVINO : CPU/iGPU/NPU Intel, NPU limité à de petits modèles (0,5–1B listés) en 2026.0 [vérifié] — [Phoronix](https://www.phoronix.com/news/Intel-OpenVINO-2026.0-Released)
- IPEX-LLM (SYCL Intel) archivé janv. 2026 → ne plus miser dessus [vérifié] — [intel/ipex-llm](https://github.com/intel/ipex-llm)
- Quantisation / KV cache : Ollama `OLLAMA_KV_CACHE_TYPE=q8_0` ≈ moitié mémoire KV, q4_0 ≈ quart ; bloc q8_0 = 34 octets vs 64 en f16 [vérifié] — [SSD Nodes](https://www.ssdnodes.com/learn/ollama-kv-cache-quantization) ; [smcleod.net (déc. 2024)](https://smcleod.net/2024/12/bringing-k/v-context-quantisation-to-ollama/) ; llama-server : types KV f32/f16/bf16/q8_0/q4_0… [vérifié] — [README server](https://github.com/ggml-org/llama.cpp/blob/master/tools/server/README.md)
- Formats : GGUF (llama.cpp et dérivés, toutes quantisations K/IQ), EXL3 (ExLlamaV3, CUDA), OpenVINO IR (Intel), MLC (compilé), mistral.rs multi-formats + ISQ [vérifié pour GGUF/EXL3/mistral.rs ; OpenVINO IR et MLC non re-vérifiés]. Critique : registre Ollama limité surtout à Q4_K_M/Q8_0 (source partisane) — [Sleeping Robots](https://sleepingrobots.com/dreams/stop-using-ollama/)

### Inferences
- Pour 8–16 Go RAM sans dGPU : llama.cpp CPU (AVX2) + Vulkan iGPU avec modèles 1–8B en Q4_K_M/IQ4 et KV cache q8_0 est la combinaison la plus portable. Le backend CPU tire profit de la bande passante mémoire plutôt que du nombre de cœurs [inférence générale, non chiffrée ici].
- Pour 4–8 Go VRAM : GGUF avec offload partiel de couches (llama.cpp `-ngl`) reste le plus flexible ; EXL3 est plus efficace en VRAM mais NVIDIA-only.

### Gaps
- Pas de benchmark tokens/s CPU-seul sur laptop 8–16 Go trouvé dans cette session.
- Performance réelle Vulkan vs ROCm sur Radeon grand public non chiffrée ici.

## Q4 — Benchmarks comparatifs (tokens/s), décodage spéculatif, KV cache

### Takeaway
Même modèle, même machine : llama.cpp est généralement le plus rapide ; Ollama perd ~5–15 % en génération (jusqu'à ~40 % sur un petit modèle en CUDA dans le test Mozilla.ai) ; LM Studio est à égalité en CUDA mais nettement derrière sur Mac en GGUF. Mozilla.ai conclut que la **configuration** (CUDA graphs, toolchain Vulkan, draft length du spéculatif) pèse plus que le choix du runtime, puisque tous partagent le cœur llama.cpp.

### Cited Findings
- **Mozilla.ai (17 sept. 2026)** — llama.cpp vs llamafile vs LM Studio vs Ollama ; Mac Studio M4 Max, Linux L40S (CUDA), Steam Deck OLED (Vulkan) ; Qwen 0.8B Q8, 9B et 27B Q4_K_M. L40S 0.8B : PP ~600 tok/s pour tous ; TG llama.cpp/llamafile ~67 tok/s vs **Ollama ~40 tok/s**. Mac 27B : PP à ±3 % pour llama.cpp/llamafile/Ollama, LM Studio en retrait sur petits modèles ; TG ~24–25 tok/s (draft length 2). Gains de config : CUDA graphs +16,8 % (0.8B), toolchain shaders Vulkan +63,3 % PP (9B), longueur de draft spéculatif optimale variable selon plateforme. Pas de test concurrent — [Mozilla.ai](https://blog.mozilla.ai/benchmarking-local-llm-servers-llama-cpp-llamafile-lm-studio-and-ollama/)
- **InventiveHQ** : llama.cpp +10,3 % vs Ollama sur RTX 5060 Ti (77,0 vs 69,1 tok/s) et ~+14 % sur M3 Max (53,5 vs 46,2) ; LM Studio −0,3 % (bruit) sur NVIDIA, −29 % sur Mac (d'après extrait de recherche ; page 403 en fetch) — [InventiveHQ](https://inventivehq.com/blog/ollama-vs-llama-cpp-vs-lm-studio-benchmark)
- RTX 4090 : Ollama 34–48 tok/s vs llama.cpp 36–52 tok/s, écart 5–10 % (agrégateur) — [promptquorum](https://www.promptquorum.com/local-llms/llamacpp-vs-ollama-vs-vllm)
- Apple : MLX ~130 tok/s vs Ollama (backend llama.cpp legacy) ~43 tok/s sur Qwen3-Coder-30B-A3B, Mac mini M4 Pro ; MLX de LM Studio +20–25 % vs GGUF Metal (agrégateur) — [codersera](https://codersera.com/blog/ollama-vs-lm-studio-vs-vllm-vs-llama-cpp-vs-mlx-2026/)
- Source critique : llama.cpp 1,8× plus rapide qu'Ollama (161 vs 89 tok/s), 30–50 % sur CPU — protocole non précisé, **à traiter avec prudence** — [Sleeping Robots](https://sleepingrobots.com/dreams/stop-using-ollama/)
- Débit multi-utilisateurs : un article comparatif intitulé « llama.cpp vs Ollama vs vLLM 2026: Speed, Batching & GPU Benchmarks » existe, mais seul son titre a été vu ; **contenu non vérifié** — [promptquorum](https://www.promptquorum.com/local-llms/llamacpp-vs-ollama-vs-vllm)
- Décodage spéculatif : llama-server supporte les draft models [vérifié] — [README server](https://github.com/ggml-org/llama.cpp/blob/master/tools/server/README.md) ; Jan 0.8.0 intègre Multi-Token Prediction [vérifié] — [changelog](https://www.jan.ai/changelog/2026-05-22-jan-v0.8.0) ; LocalAI fournit des paires MTP Gemma 4 [vérifié] — [News LocalAI](https://localai.io/basics/news/)

### Inferences
- Pour du débit batch (traduction de milliers de segments) sur machine modeste, le levier principal est de régler llama-server (slots parallèles, taille de batch, KV q8_0, CUDA graphs/Vulkan à jour) plutôt que de changer de runtime.
- Les écarts Ollama vs llama.cpp tiennent aux builds/flags et au moteur ggml propre d'Ollama ; ils varient fortement selon modèle et version.

### Gaps
- Aucun benchmark trouvé sur laptop CPU-only 8–16 Go ou iGPU Intel (Xe/Arc) comparant les runtimes.
- Pas de chiffres vérifiés sur le gain concret du spéculatif sur CPU.

## Q5 — Embarquement dans une application desktop (sidecar, bibliothèque)

### Takeaway
Les options les plus réalistes pour bundler dans une app Tauri : **llama-server en sidecar** (binaire MIT par backend CPU/Vulkan/CUDA, exemple éprouvé : Jan, lui-même Tauri), **llamafile** (exécutable unique), ou **mistral.rs** en crate Rust (MIT, mais sans Vulkan). LM Studio (propriétaire) et Ollama se prêtent plutôt à une détection d'un service installé par l'utilisateur.

### Cited Findings
- Jan (Tauri, Apache-2.0) embarque llama.cpp et, depuis v0.8.0, un seul process router llama.cpp qui charge/décharge les modèles [vérifié] — [changelog v0.8.0](https://www.jan.ai/changelog/2026-05-22-jan-v0.8.0) ; [Petronella](https://petronellatech.com/blog/jan-ai-local-first-llm/)
- llama-server router mode : un process parent, un enfant par modèle, isolation des crashs, auto-découverte du cache [vérifié] — [HF blog](https://huggingface.co/blog/ggml-org/model-management-in-llamacpp) ; [glukhov.org](https://www.glukhov.org/llm-hosting/llama-cpp/llama-server-router-mode/)
- llamafile : un seul exécutable multi-OS, poids embarquables [vérifié] — [blog Mozilla.ai](https://blog.mozilla.ai/llamafile-reloaded-whats-new-in-v0-10-0/)
- KoboldCpp : binaire unique « zero install » [vérifié] — [GitHub](https://github.com/lostruins/koboldcpp) (mais AGPL, voir Q6)
- mistral.rs : crate `mistralrs` + serveur ; CUDA/Metal/CPU [vérifié] — [GitHub](https://github.com/EricLBuehler/mistral.rs)
- LM Studio : démon llmster headless + SDK MIT, mais application propriétaire (termes d'usage) [vérifié] — [docs headless](https://lmstudio.ai/docs/developer/core/headless) ; [codersera](https://codersera.com/blog/lm-studio-complete-guide-2026/)
- Lemonade : installeurs one-click, intégration par API standard [vérifié] — [FAQ](https://lemonade-server.ai/docs/guide/faq/)
- Docker Model Runner : nécessite Docker → peu adapté à un utilisateur final non technique [inférence d'après [Docker blog](https://www.docker.com/blog/run-llms-locally/)]

### Inferences
- Stratégie probable pour une app grand public : se connecter à un endpoint OpenAI-compatible configurable (détection auto Ollama :11434, LM Studio :1234, llama-server :8080, Lemonade), et optionnellement proposer un sidecar llama-server (build Vulkan pour AMD/Intel, CPU fallback) géré par l'app.
- Bindings Rust directs vers llama.cpp (crates type `llama-cpp-2`) sont une alternative au sidecar — hors périmètre de cette note (couvert par la recherche « crates Rust »).

### Gaps
- Taille des binaires sidecar par backend et contraintes de signature/notarisation non documentées ici.

## Q6 — Licences et contraintes d'usage commercial

### Takeaway
Permissifs (redistribution dans une app OK avec notice) : llama.cpp (MIT), Ollama (MIT), mistral.rs (MIT), LocalAI (MIT), Jan (Apache-2.0), Lemonade (Apache-2.0), llamafile (Apache-2.0), vLLM/SGLang/MLC (Apache-2.0). Copyleft fort : **KoboldCpp (AGPL-3.0)**, TabbyAPI et text-generation-webui (AGPL-3.0, non re-vérifié). Propriétaire : LM Studio (gratuit, y compris usage pro, mais non redistribuable librement), Msty.

### Cited Findings
- KoboldCpp AGPL-3.0 [vérifié] — [GitHub](https://github.com/lostruins/koboldcpp)
- LM Studio propriétaire, gratuit personnel/commercial depuis juillet 2025, termes « personal and/or internal business purposes » ; SDK et `lms` MIT [vérifié] — [codersera](https://codersera.com/blog/lm-studio-complete-guide-2026/) ; [Thunder Compute](https://www.thundercompute.com/blog/lm-studio-local-llm)
- Jan Apache-2.0 [vérifié] — [Petronella](https://petronellatech.com/blog/jan-ai-local-first-llm/)
- Lemonade Apache-2.0 [vérifié] — [FAQ](https://lemonade-server.ai/docs/guide/faq/)
- LocalAI MIT [vérifié] — [LocalAI](https://localai.io/)
- mistral.rs MIT [vérifié] — [GitHub](https://github.com/EricLBuehler/mistral.rs)
- Ollama : binaires distribués sans la notice MIT de llama.cpp pendant longtemps (issue #3185) ; GUI initialement sans licence avec dépendances AGPL potentielles (source critique) — [Sleeping Robots](https://sleepingrobots.com/dreams/stop-using-ollama/)
- **Tableau vérifié via l'API GitHub le 2026-09-25** (champ `license.spdx_id`, `archived`, `pushed_at` = dernier push, étoiles). Quand SPDX = NOASSERTION, le fichier LICENSE a été lu directement.

| Dépôt | Licence | Archivé | Dernier push | Étoiles |
|---|---|---|---|---|
| [ggml-org/llama.cpp](https://github.com/ggml-org/llama.cpp) | MIT | non | 2026-09-25 | 129 481 |
| [ollama/ollama](https://github.com/ollama/ollama) | MIT | non | 2026-09-25 | 181 673 |
| [mozilla-ai/llamafile](https://github.com/mozilla-ai/llamafile) | Apache-2.0 (fichier LICENSE lu) | non | 2026-09-24 | 26 057 |
| [vllm-project/vllm](https://github.com/vllm-project/vllm) | Apache-2.0 | non | 2026-09-25 | 92 662 |
| [sgl-project/sglang](https://github.com/sgl-project/sglang) | Apache-2.0 | non | 2026-09-25 | 36 422 |
| [mlc-ai/mlc-llm](https://github.com/mlc-ai/mlc-llm) | Apache-2.0 | non | 2026-08-17 | 23 191 |
| [mlc-ai/web-llm](https://github.com/mlc-ai/web-llm) | Apache-2.0 | non | 2026-09-15 | 19 187 |
| [nomic-ai/gpt4all](https://github.com/nomic-ai/gpt4all) | MIT | non | **2025-05-27** (aucun push depuis 16 mois) | 77 383 |
| [turboderp-org/exllamav2](https://github.com/turboderp-org/exllamav2) | MIT | non | 2026-03-04 (ralenti, remplacé par V3) | 4 628 |
| [turboderp-org/exllamav3](https://github.com/turboderp-org/exllamav3) | MIT | non | 2026-09-24 | 1 512 |
| [theroyallab/tabbyAPI](https://github.com/theroyallab/tabbyAPI) | AGPL-3.0 | non | 2026-09-22 | 1 427 |
| [oobabooga/textgen](https://github.com/oobabooga/text-generation-webui) (dépôt **renommé** de `text-generation-webui` en `textgen`) | AGPL-3.0 | non | 2026-08-17 | 47 709 |
| [LostRuins/koboldcpp](https://github.com/LostRuins/koboldcpp) | AGPL-3.0 | non | 2026-09-24 | 11 860 |
| [janhq/jan](https://github.com/janhq/jan) | Apache-2.0 (fichier LICENSE lu, © Menlo Research) | non | 2026-09-25 | 44 646 |
| [mudler/LocalAI](https://github.com/mudler/LocalAI) | MIT | non | 2026-09-25 | 49 263 |
| [lemonade-sdk/lemonade](https://github.com/lemonade-sdk/lemonade) | Apache-2.0 | non | 2026-09-25 | 5 781 |
| [openvinotoolkit/model_server](https://github.com/openvinotoolkit/model_server) | Apache-2.0 | non | 2026-09-25 | 936 |
| [intel/ipex-llm](https://github.com/intel/ipex-llm) | Apache-2.0 | **oui** | 2026-01-28 | 8 853 |
| [EricLBuehler/mistral.rs](https://github.com/EricLBuehler/mistral.rs) | MIT | non | 2026-09-25 | 7 718 |
| [mostlygeek/llama-swap](https://github.com/mostlygeek/llama-swap) | MIT | non | 2026-09-25 | 5 750 |
| [microsoft/foundry-local](https://github.com/microsoft/foundry-local) | « FOUNDRY LOCAL LICENSE TERMS » : SDK sous MIT, le runtime relève de termes Microsoft propres (fichier LICENSE lu, partiellement) | non | 2026-09-25 | 2 564 |

- Non couverts par l'API : LM Studio (propriétaire, voir plus haut), Msty (propriétaire, non vérifié), SGLang/TabbyAPI (couverts ci-dessus).

### Inferences
- Redistribuer llama-server (MIT) dans Hoshi2Star impose seulement d'inclure la notice MIT (leçon tirée du cas Ollama). Bundler KoboldCpp ou TabbyAPI impliquerait des obligations AGPL (source du composant à fournir ; interaction réseau). Se connecter à un LM Studio installé par l'utilisateur ne pose pas de problème de licence côté app.
- Les licences des **poids de modèles** (Llama, Gemma, Qwen…) sont une contrainte distincte, non traitée ici.

### Gaps
- Termes d'utilisation précis de Msty non consultés ; termes complets du runtime Foundry Local non lus (seul l'en-tête du fichier LICENSE a été lu).
- Les mentions [non re-vérifié] de licence dans Q1 sont remplacées par le tableau ci-dessus (vérifié le 2026-09-25) ; Ollama, llama.cpp, vLLM, SGLang, MLC, GPT4All, ExLlamaV3, TabbyAPI, textgen et llamafile y sont désormais confirmés.
