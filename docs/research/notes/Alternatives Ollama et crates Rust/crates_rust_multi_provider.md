# Crates Rust et passerelles multi-fournisseurs LLM pour Hoshi2Star (état au 2026-09-25)

Contexte de l'application : backend Tauri v2, `tokio = "1" (full)`, `reqwest = { version = "0.12", default-features = false, features = ["json", "rustls-tls"] }` (vérifié dans `src-tauri/Cargo.toml`, lignes 25 et 32). Le trait `LlmProvider` se trouve dans `src-tauri/src/llm/provider.rs:100`.
Méthode : les chiffres crates.io viennent de l'API `https://crates.io/api/v1/crates/<nom>`, interrogée le 2026-09-25. Les étoiles et dates de push GitHub viennent de `gh api repos/<owner>/<repo>`, à la même date. Les dépendances viennent de `https://crates.io/api/v1/crates/<nom>/<version>/dependencies`.

## Q1 — Quelle crate convient le mieux : multi-fournisseur, Anthropic, Gemini et OpenAI-compatible natifs, sorties structurées, tokio et reqwest 0.12/rustls, maintenue, licence permissive ?

### Takeaway
**genai** (jeremychone) est la crate la plus proche du besoin :
- protocoles natifs Anthropic, Gemini, Ollama et OpenAI, plus 26 fournisseurs ou plus ;
- `ChatResponseFormat::JsonSpec` ;
- licence MIT/Apache ;
- publications très fréquentes.

Deux réserves vérifiées :
- elle dépend de **reqwest 0.13**, pas 0.12 ;
- son adaptateur Ollama **natif** ignore `JsonSpec` et n'envoie que `"format":"json"`. Cela régresse par rapport au JSON structuré Ollama actuel de l'app.

**rig-core** est la plus populaire, mais elle casse son API à chaque version mineure, environ toutes les trois semaines. **graniet/llm** reste sur reqwest 0.12, mais son rythme ralentit. **async-openai** ne couvre que le protocole OpenAI.

### Cited Findings

**Tableau comparatif (crates multi-fournisseurs et SDK dédiés)**

| Crate | Dernière stable (date) | Pré-release | Téléch. totaux / récents (90 j) | Étoiles GitHub | reqwest / TLS par défaut | Licence | Source |
|---|---|---|---|---|---|---|---|
| rig-core | 0.42.0 (2026-08-17) | — | 2 998 891 / 1 641 440 | 8 725 | ^0.13, défaut `rustls` | MIT | [crates.io](https://crates.io/crates/rig-core), [GitHub](https://github.com/0xPlaygrounds/rig) |
| genai | 0.6.5 (2026-06-06) | 0.7.0-beta.24 (2026-09-23) | 410 019 / 166 088 | 892 | ^0.13, défaut `rustls-tls` | MIT OR Apache-2.0 (crates.io) ; GitHub détecte Apache-2.0 | [crates.io](https://crates.io/crates/genai), [GitHub](https://github.com/jeremychone/rust-genai) |
| async-openai | 0.42.0 (2026-09-09) | — | 8 570 666 / 2 632 010 | 2 017 | ^0.13, défaut `rustls` | MIT | [crates.io](https://crates.io/crates/async-openai), [GitHub](https://github.com/64bit/async-openai) |
| llm (graniet) | 1.3.8 (2026-04-19) | — | 120 996 / 18 032 | 365 | ^0.12.12, défaut `cli` + `default-tls` (native) ; feature `rustls-tls` disponible | MIT | [crates.io](https://crates.io/crates/llm), [GitHub](https://github.com/graniet/llm) |
| openai-api-rs | 10.0.1 (2026-04-17) | — | 731 229 / 95 121 | 486 | ^0.12, défaut `default-tls` ; feature `rustls` | MIT | [crates.io](https://crates.io/crates/openai-api-rs), [GitHub](https://github.com/dongri/openai-api-rs) |
| ollama-rs | 0.3.6 (2026-07-24) | — | 509 555 / 141 183 | 1 063 | ^0.12.28, défaut `default-tls` ; feature `rustls` | crates.io : « non-standard » ; GitHub : MIT | [crates.io](https://crates.io/crates/ollama-rs), [GitHub](https://github.com/pepperoni21/ollama-rs) |
| gemini-rust | 2.1.0 (2026-09-05) | — | 51 647 / 15 826 | 78 | ^0.12.15 avec `rustls-tls` fixé | MIT | [crates.io](https://crates.io/crates/gemini-rust), [GitHub](https://github.com/flachesis/gemini-rust) |
| allms | 0.43.0 (2026-09-06) | — | 49 163 / 717 | 111 | **^0.11** + `rustls-tls` ; schemars 0.8 ; backoff 0.4 | MIT (crates.io) ; dépôt avec LICENSE-MIT et LICENSE-APACHE | [crates.io](https://crates.io/crates/allms), [GitHub](https://github.com/neferdata/allms) |
| siumai | 0.10.3 (2025-10-12) | 0.11.0-beta.10 (2026-08-13) | 9 397 / 1 644 | 30 | ^0.12 **avec features par défaut** ; backoff 0.4 | MIT OR Apache-2.0 | [crates.io](https://crates.io/crates/siumai), [GitHub](https://github.com/YumchaLabs/siumai) |
| swiftide | 0.32.1 (2025-11-15) | — | 84 568 / 2 091 | 786 | (framework RAG/agents) | MIT | [crates.io](https://crates.io/crates/swiftide), [GitHub](https://github.com/bosun-ai/swiftide) |
| langchain-rust | 4.6.0 (**2024-10-06**) | — | 155 951 / 10 423 | 1 349 | — | MIT | [crates.io](https://crates.io/crates/langchain-rust), [GitHub](https://github.com/Abraxas-365/langchain-rust) |
| llm-chain | 0.13.0 (**2023-11-15**) | — | 97 208 / 7 701 | 1 603 (dernier push 2024-10-31) | — | MIT | [crates.io](https://crates.io/crates/llm-chain), [GitHub](https://github.com/sobelio/llm-chain) |
| llm-connector | 1.4.0 (2026-05-07) | — | 14 485 / 1 920 | n.d. | — | MIT | [crates.io](https://crates.io/crates/llm-connector) |
| ai-lib | 0.4.0 (2025-12-18) | — | 5 771 / 123 | n.d. | — | MIT OR Apache-2.0 | [crates.io](https://crates.io/crates/ai-lib) |
| multi-llm | 1.0.0 (2025-11-28) | — | 315 / 209 | n.d. | — | Apache-2.0 | [crates.io](https://crates.io/crates/multi-llm) |
| rllm (graniet) | 1.1.9 (2025-05-20) | — | 13 692 / 600 | n.d. | — | MIT | [crates.io](https://crates.io/crates/rllm) |
| misanthropy (Anthropic) | 0.0.8 (2025-06-08) | — | 12 808 / 317 | n.d. | — | MIT | [crates.io](https://crates.io/crates/misanthropy) |
| clust (Anthropic) | 0.9.0 (**2024-06-30**) | — | 14 800 / 364 | n.d. | — | MIT OR Apache-2.0 | [crates.io](https://crates.io/crates/clust) |
| anthropic-rs | 0.1.7 (2024-09-07) | — | 11 416 / 294 | n.d. | — | MIT AND Apache-2.0 | [crates.io](https://crates.io/crates/anthropic-rs) |
| anthropic-sdk | 0.1.5 (2024-07-23) | — | 78 250 / 6 465 | n.d. | — | MIT | [crates.io](https://crates.io/crates/anthropic-sdk) |
| anthropic-sdk-rust | 0.1.1 (2025-06-11) | — | 13 254 / 3 906 | n.d. | — | — | [crates.io](https://crates.io/crates/anthropic-sdk-rust) |
| async-anthropic (bosun) | 0.6.0 (2025-05-03) | — | 47 487 / 5 528 | n.d. | — | MIT | [crates.io](https://crates.io/crates/async-anthropic) |

**SDK officiels**
- Anthropic ne publie **aucun SDK Rust officiel**. Au 2026-05-17, la gamme officielle couvre Python, TypeScript, Java, Go, Ruby, C#, PHP et la CLI `ant`. Une issue ouverte réclame un SDK Rust et décrit l'écosystème actuel comme des « single-maintainer wrappers … all lagging API features » — [anthropic-sdk-python#1559](https://github.com/anthropics/anthropic-sdk-python/issues/1559)
- async-openai est la référence communautaire côté OpenAI : 8,57 M de téléchargements, la plus téléchargée des crates examinées — [async-openai sur crates.io](https://crates.io/crates/async-openai). L'absence de SDK Rust officiel OpenAI n'est pas confirmée par une source primaire ; voir Gaps.

**genai (jeremychone/rust-genai)**
- Couverture fournisseurs :
  - « native-protocol multi-AI provider access » ;
  - « 26+ LLM providers » ;
  - adaptateurs `openai`, `openai_resp`, `anthropic`, `gemini`, `ollama`, `ollama_cloud`, `vertex`, `bedrock_api`, `bedrock_sigv4`, `groq`, `together`, `fireworks`, `cohere`, `deepseek`, `open_router`, `xai`, `moonshot`/`kimi`, `zai`, `aliyun`, `qwen_cloud`, entre autres.

  Pas d'adaptateur Mistral ni DeepL listé : Mistral passerait par un endpoint OpenAI-compatible personnalisé (`ServiceTargetResolver`). Source : [README](https://github.com/jeremychone/rust-genai)
- Sélection du fournisseur par préfixe de nom de modèle (`claude-*` → Anthropic, `gemini-*` → Gemini, repli sur Ollama pour les noms inconnus), ou forcée par `adapter::model`, par exemple `open_router::google/gemini-2.0-flash-001` — [README](https://github.com/jeremychone/rust-genai)
- `ChatOptions` expose notamment :
  - `with_response_format(ChatResponseFormat::JsonMode | JsonSpec(name, schema))` ;
  - `with_reasoning_effort`, `with_tool_choice`, `with_prompt_cache_key`, `with_cache_control` ;
  - `with_extra_body` (« provider-specific fields in OpenAI-compatible chat payloads ») et `with_extra_headers`.

  Source : [api-reference-for-llm.md](https://github.com/jeremychone/rust-genai/blob/main/docs/for-llm/api-reference-for-llm.md)
- Embeddings : `exec_embed`, `embed`, `embed_batch` — [api-reference-for-llm.md](https://github.com/jeremychone/rust-genai/blob/main/docs/for-llm/api-reference-for-llm.md)
- Streaming : `exec_chat_stream`. La v0.7 ajoute `ChatStreamEvent::Heartbeat` — [README](https://github.com/jeremychone/rust-genai)
- Configuration HTTP :
  - `with_web_config` (timeouts, proxies) ;
  - `with_reqwest(reqwest_client)` pour injecter un client ;
  - TLS par défaut `rustls-tls` (rustls + aws-lc-rs + magasin OS via `rustls-platform-verifier`) ;
  - `native-tls` en option, exclusif.

  Source : [README, section TLS Backends](https://github.com/jeremychone/rust-genai)
- Normalisation des schémas Gemini/Vertex (`const`, nullable, `additionalProperties`) depuis la v0.6 — [README v0.6.x](https://github.com/jeremychone/rust-genai)
- **Anthropic + JsonSpec** : l'adaptateur transforme le schéma en `output_config.format` avec `additionalProperties: false`. Un test de non-régression couvre `reasoning_effort` combiné à `JsonSpec` — [adapter_shared.rs l.462](https://github.com/jeremychone/rust-genai/blob/main/src/adapter/adapters/anthropic/adapter_shared.rs), [adapter_shared_tests.rs](https://github.com/jeremychone/rust-genai/blob/main/src/adapter/adapters/anthropic/adapter_shared_tests.rs)
- **Ollama + JsonSpec : non transmis.** Dans l'adaptateur natif, seul `JsonMode` produit `"format": "json"`, et le schéma d'un `JsonSpec` est ignoré. C'est vérifié sur `main` et sur le tag `v0.6.5`, et le code est identique pour `ollama_cloud` — [ollama/adapter_impl.rs l.108-113](https://github.com/jeremychone/rust-genai/blob/main/src/adapter/adapters/ollama/adapter_impl.rs), [v0.6.5](https://github.com/jeremychone/rust-genai/blob/v0.6.5/src/adapter/adapters/ollama/adapter_impl.rs)
- Depuis la v0.6.0 (2026-05-23), « Ollama and Ollama Cloud: Now use the native Ollama API protocol » — [README v0.6.x](https://github.com/jeremychone/rust-genai)
- La v0.7.0 visait « first half of Sept 2026 ». Au 2026-09-25, la dernière version publiée reste `0.7.0-beta.24` (2026-09-23) — [README](https://github.com/jeremychone/rust-genai) ; [versions crates.io](https://crates.io/crates/genai/versions)
- genai déclare privilégier « ergonomics and commonality, while depth is secondary ». Pour une API complète, elle renvoie vers async-openai et ollama-rs — [README, Library Focus](https://github.com/jeremychone/rust-genai)
- 142 versions publiées depuis 2024-06-01, dont de nombreuses bêtas — [crates.io](https://crates.io/crates/genai/versions)

**rig-core (0xPlaygrounds/rig)**
- Le README prévient : « future updates **will** contain **breaking changes** » — [README](https://github.com/0xPlaygrounds/rig)
- Le CHANGELOG de rig-core 0.42.0 (2026-08-17) contient une douzaine d'entrées `[**breaking**]` :
  - `OneOrMany<T>` devient `Vec<T>` ;
  - `AssistantContent` devient taggé, et un historique sérialisé en 0.41 « does not load » sans migration ;
  - identités de tool-call typées ;
  - grammaire de streaming refondue.

  Rythme observé : 0.34 (2026-03-29), 0.35 (04-12), 0.36 (04-28), 0.37 (05-13), 0.38 (06-02), 0.39 (06-19), 0.40 (07-10), 0.41 (07-28), 0.42 (08-17). Source : [CHANGELOG rig-core](https://github.com/0xPlaygrounds/rig/blob/main/crates/rig-core/CHANGELOG.md)
- Modules fournisseurs présents dans `crates/rig-core/src/providers/` sur `main` : anthropic, azure, chatgpt, cohere, copilot, deepseek, doubleword, gemini, groq, huggingface, hyperbolic, llamacpp, minimax, mira, mistral, moonshot, ollama, openai, openrouter, perplexity, together, venice, voyageai, xai, xiaomimimo, zai — [arbre du dépôt](https://github.com/0xPlaygrounds/rig/tree/main/crates/rig-core/src/providers)
- Découpage du dépôt :
  - `rig-core` contient les contrats neutres ;
  - `rig-agent` porte le builder d'agent, le streaming et l'extraction. Ce dernier est publié séparément en 0.42.0 depuis le 2026-07-19.

  La page docs.rs `rig-core/0.42.0/rig/extractor` renvoie 404 : l'extractor a quitté rig-core. Sources : [README](https://github.com/0xPlaygrounds/rig), [crates.io rig-agent](https://crates.io/crates/rig-agent), [arbre crates/rig-agent/src/extractor.rs](https://github.com/0xPlaygrounds/rig/tree/main/crates/rig-agent/src)
- Des cassettes de tests couvrent les sorties structurées Anthropic (`structured_output_smoke`, `strict_schema_integrations`) et l'extractor — [rig-cassette fixtures](https://github.com/0xPlaygrounds/rig/tree/main/crates/rig-cassette/fixtures/cassettes/anthropic)
- Features : `default = ["reqwest","derive","rustls"]` ; `reqwest-middleware-rustls` pour brancher un middleware de retry ; exemple `retry_on_truncation.rs` dans rig-agent — [métadonnées crates.io 0.42.0](https://crates.io/crates/rig-core/0.42.0), [exemple](https://github.com/0xPlaygrounds/rig/tree/main/crates/rig-agent/examples)
- Embeddings et 10 intégrations de vector store ou plus (lancedb, qdrant, sqlite, fastembed…) — [README](https://github.com/0xPlaygrounds/rig)

**llm (graniet/llm)**
- Backends : OpenAI, Anthropic, Ollama, DeepSeek, xAI, Phind, Groq, Google (Gemini), Cohere, Mistral, OpenRouter, Hugging Face, ElevenLabs. Chaque backend correspond à une feature cargo — [README](https://github.com/graniet/llm)
- Fonctions annoncées :
  - « Resilience (retry/backoff) … exponential backoff and jitter » ;
  - « Structured Output … based on a provided JSON schema » ;
  - streaming et embeddings (OpenAI, Google) ;
  - tool calling ;
  - serveur REST au format OpenAI.

  Source : [README](https://github.com/graniet/llm)
- Features par défaut : `["cli", "default-tls"]`. Il faut `default-features = false, features = ["rustls-tls", …]` pour rester en rustls — [métadonnées crates.io 1.3.8](https://crates.io/crates/llm/1.3.8)
- Rythme : 1.3.4 (2025-08-30), 1.3.6 (2025-11-15), 1.3.7 (2026-01-09), 1.3.8 (2026-04-19). Dernier push GitHub le 2026-06-06 — [crates.io](https://crates.io/crates/llm/versions), [GitHub](https://github.com/graniet/llm)

**async-openai**
- La crate vise l'API OpenAI officielle, mais elle est « very configurable to work with compatible providers » :
  - feature `byot` : méthodes `*_byot` à types génériques, pour les fournisseurs dont les requêtes ou réponses s'écartent d'OpenAI ;
  - retry avec backoff exponentiel sur rate limit.

  Source : [README](https://github.com/64bit/async-openai)
- reqwest ^0.13, défaut `rustls` ; MSRV 1.75 — [crates.io 0.42.0](https://crates.io/crates/async-openai/0.42.0)

**Crates mono-fournisseur**
- **ollama-rs** : reqwest ^0.12.28, schemars ^1.2.1 (sorties structurées via schéma), défaut native TLS ; dernier push 2026-09-21 — [crates.io](https://crates.io/crates/ollama-rs), [GitHub](https://github.com/pepperoni21/ollama-rs)
- **gemini-rust** : function calling (Google Search, Maps), batch, embeddings, content caching, streaming SSE, thinking mode, Gemini 3 Pro ; reqwest 0.12 rustls fixé ; 2.0.0 (2026-07-10) puis 2.1.0 (2026-09-05) — [README](https://github.com/flachesis/gemini-rust), [crates.io](https://crates.io/crates/gemini-rust)
- **allms** : toujours sur reqwest 0.11 et schemars 0.8 à la 0.43.0 (2026-09-06) — [dépendances crates.io](https://crates.io/crates/allms/0.43.0)

**Frameworks et crates en perte de vitesse**
- **langchain-rust** : dernier commit sur la branche par défaut le 2025-04-30 (« llm add qwen and deepseek support »), dernière crate le 2024-10-06. Le `pushed_at` GitHub du 2026-09-24 ne reflète pas de commit récent sur la branche par défaut — [commits](https://github.com/Abraxas-365/langchain-rust/commits), [crates.io](https://crates.io/crates/langchain-rust)
- **llm-chain** : dernière release 2023-11-15, dernier push 2024-10-31 — [crates.io](https://crates.io/crates/llm-chain), [GitHub](https://github.com/sobelio/llm-chain)
- **swiftide** : framework d'indexation et d'agents, dernière crate le 2025-11-15 alors que le dépôt est actif (push 2026-09-21) — [crates.io](https://crates.io/crates/swiftide), [GitHub](https://github.com/bosun-ai/swiftide)

### Inferences
- **Compatibilité reqwest.** genai, rig-core et async-openai tirent reqwest 0.13, alors que l'app utilise 0.12. Ce n'est pas un conflit de compilation, car Cargo accepte deux versions semver-incompatibles côte à côte. Coûts probables :
  - deux piles HTTP/hyper/TLS dans le binaire ;
  - `genai::ClientBuilder::with_reqwest()` attend un `reqwest::Client` 0.13 et ne peut pas réutiliser le client 0.12 de l'app, sauf si l'app migre elle-même en 0.13.

  graniet/llm, ollama-rs, gemini-rust et openai-api-rs restent sur 0.12.
- **Classement pour Hoshi2Star**, selon le besoin « Anthropic et Gemini natifs, OpenAI-compatible, JSON structuré, MIT/Apache, maintenu » :
  1. **genai 0.6.x**, puis 0.7 une fois stable. C'est la couverture native la plus large, avec l'API la plus simple (`client.exec_chat(model, req, opts)`), JsonSpec traduit nativement pour OpenAI, Anthropic et Gemini, et des publications très actives. Le pipeline Ollama actuel perdrait le JSON contraint par schéma, sauf à garder le `OpenAiCompatibleProvider` existant pour Ollama ou à patcher ou contribuer l'adaptateur.
  2. **Approche hybride minimale** : garder le trait `LlmProvider` et ajouter des implémentations ciblées, par exemple Anthropic et Gemini écrits à la main avec le reqwest 0.12 existant, ou gemini-rust pour Gemini. Les dépendances restent en 0.12 et le contrôle total est conservé, au prix de plus de code à maintenir.
  3. **graniet/llm** : même version de reqwest et retry intégré, mais un rythme plus lent (4 releases en 8 mois) et une feature CLI par défaut à désactiver.
  4. **rig-core** : techniquement le plus riche, mais le churn (breaking changes à chaque mineure toutes les 2 à 4 semaines, découpage rig-agent en cours) est disproportionné pour un simple « chat(system, user) → texte ou JSON ».
- async-openai et openai-api-rs ne résolvent pas le problème Anthropic/Gemini natif. L'app sait déjà faire de l'OpenAI-compatible à la main.
- Les crates Anthropic dédiées sont figées ou mono-mainteneur, et aucune n'est officielle. Elles n'apportent rien par rapport à genai ou à un client maison.

### Gaps
- Je n'ai pas vérifié si `ChatOptions::with_extra_body` est appliqué par l'adaptateur **Ollama natif** de genai. La doc le décrit pour les payloads OpenAI-compatibles. Si c'est le cas, il permettrait d'injecter `"format": <schema>` à la main.
- Je n'ai pas trouvé de retry ou de backoff intégré dans la référence d'API de genai. Cela ne prouve pas qu'il n'en existe pas ; le CHANGELOG n'a pas été lu en entier.
- rustls et fournisseur crypto : à tester si l'app garde reqwest 0.12 `rustls-tls` (ring par défaut dans 0.12, non vérifié) à côté de reqwest 0.13 `rustls` (aws-lc-rs selon le README de genai). La question est de savoir si `rustls::crypto::CryptoProvider::install_default()` devient nécessaire au démarrage. Non vérifié.
- siumai : sa dépendance reqwest ^0.12 garde les features par défaut, ce qui tirerait probablement native-tls. C'est une inférence, non vérifiée dans le `Cargo.lock`.
- Les étoiles GitHub de llm-connector, ai-lib, multi-llm, misanthropy, clust et anthropic-rs n'ont pas été collectées.
- DeepL n'est couvert par aucune des crates LLM examinées (non trouvé). Il reste à implémenter à part.
- Absence de SDK Rust officiel OpenAI : c'est une affirmation d'une synthèse de recherche web, sans page primaire d'OpenAI consultée.

## Q2 — À quoi ressemble un changement de fournisseur dans les meilleurs candidats ? (extraits de doc)

### Takeaway
genai change de fournisseur **par une simple chaîne de modèle**, ce qui est l'approche la plus légère pour une app où l'utilisateur choisit son fournisseur dans l'UI. graniet/llm utilise un enum `LLMBackend` dans un builder. rig utilise un client typé par fournisseur (`openai::Client`, `anthropic::Client`…), ce qui impose des types différents ou du `dyn` par fournisseur. async-openai change d'`api_base` dans une config.

### Cited Findings
- **genai**, extrait du README. Un seul `Client` ; le fournisseur est résolu d'après le nom de modèle, et un suffixe d'effort de raisonnement est possible — [README](https://github.com/jeremychone/rust-genai) :
  ```rust
  let client = Client::new()?;
  let chat_req = ChatRequest::new(vec![
      ChatMessage::system("Answer in one sentence"),
      ChatMessage::user(question),
  ]);
  let chat_res = client.exec_chat("gpt-5.4-mini-high", chat_req, None).await?;
  // autres exemples du README : "claude-haiku-4-5", "gemini-3.1-flash-lite",
  // "gemma4:e2b" (Ollama), "open_router::google/gemini-2.0-flash-001", "omlx::my-model"
  ```
  Sortie structurée : `ChatOptions::default().with_response_format(JsonSpec::new(name, schema))` — [api-reference-for-llm.md](https://github.com/jeremychone/rust-genai/blob/main/docs/for-llm/api-reference-for-llm.md), [tests Anthropic](https://github.com/jeremychone/rust-genai/blob/main/src/adapter/adapters/anthropic/adapter_shared_tests.rs)
- **graniet/llm**, extrait du README. Le changement de fournisseur passe par `.backend(LLMBackend::X)` — [README](https://github.com/graniet/llm) :
  ```rust
  let llm = LLMBuilder::new()
      .backend(LLMBackend::OpenAI)   // ou Anthropic, Ollama, DeepSeek, Google, Mistral…
      .api_key(api_key)
      .model("gpt-4.1-nano")
      .max_tokens(512)
      .temperature(0.7)
      .normalize_response(true)
      .build()
      .expect("Failed to build LLM");
  ```
- **rig**, extrait du README de la branche `main`. Attention : ce code utilise `.bound()` et une API que la 0.42.0 publiée n'a peut-être pas ; il peut ne pas compiler en 0.42.0 — [README](https://github.com/0xPlaygrounds/rig) :
  ```rust
  use rig::prelude::*;
  use rig::providers::openai::{self, OpenAI};
  let comedian_agent = OpenAI::from_env()?
      .bound()?
      .agent(openai::GPT_5_2)
      .preamble("You are a comedian here to entertain the user using humour and jokes.")
      .build();
  let response = comedian_agent.prompt("Entertain me!").await?;
  ```
  Changer de fournisseur revient à importer un autre module (`rig::providers::anthropic`, `gemini`, `ollama`…) — [arbre providers](https://github.com/0xPlaygrounds/rig/tree/main/crates/rig-core/src/providers)
- **async-openai** : seul le protocole OpenAI est couvert. On cible un autre fournisseur compatible via la config (`api_base`) et, pour les formes non standard, via les méthodes `create_byot(json!({...}))` — [README, « OpenAI Compatible Providers »](https://github.com/64bit/async-openai)

### Inferences
- Le modèle genai (chaîne `adapter::model` + `ChatOptions`) se rapproche le plus du trait `LlmProvider { translate, health_check, chat }` de l'app. Un adaptateur `GenaiProvider` unique pourrait implémenter `chat(system, user)` pour tous les fournisseurs. `health_check` se ferait via `all_model_names(adapter_kind, provider_config)`, cité dans le [README v0.6.x](https://github.com/jeremychone/rust-genai).
- Le typage par fournisseur de rig complique une sélection à l'exécution depuis l'UI. rig-agent propose un exemple `runtime_model_routing.rs` ([exemples](https://github.com/0xPlaygrounds/rig/tree/main/crates/rig-agent/examples)), mais je ne l'ai pas lu.

### Gaps
- Snippet rig garanti conforme à 0.42.0 non obtenu : la doc docs.rs de 0.42 n'a pas été lue.
- Extrait exact de configuration `OpenAIConfig::new().with_api_base(...)` d'async-openai non recopié depuis la doc. Seule la description du README est citée.

## Q3 — Risques : churn d'API, breaking changes, fuites d'abstraction (reasoning_effort, response_format, prompt caching)

### Takeaway
Le principal risque vient de **l'asymétrie des fonctions entre fournisseurs** : JSON par schéma, cache de prompt et raisonnement ne se mappent pas uniformément. genai l'illustre déjà, avec JsonSpec ignoré pour Ollama natif et `max_tokens` Anthropic à 1024 par défaut. Vient ensuite le **churn** : rig casse l'API à chaque mineure, et genai est en 0.x avec une 0.7 en bêta prolongée et des guides de migration. Côté passerelles, TensorZero est archivé, Helicone ai-gateway est dormant et LiteLLM a été compromis sur PyPI.

### Cited Findings

**Fuites d'abstraction observées dans genai**
- `max_tokens` vaut 1024 par défaut pour Anthropic, faute d'être fixé — [README, tableau ChatOptions](https://github.com/jeremychone/rust-genai)
- Ollama native : `JsonSpec` est perdu et seul `JsonMode` est transmis — [adapter_impl.rs](https://github.com/jeremychone/rust-genai/blob/main/src/adapter/adapters/ollama/adapter_impl.rs)
- Prompt caching spécifique à chaque fournisseur :
  - OpenAI : `prompt_cache_key` ;
  - Anthropic : `Tool::with_cache_control` et un breakpoint automatique sur le préfixe tools+system.

  Source : [README v0.6.x](https://github.com/jeremychone/rust-genai)
- `ReasoningEffort::Max` est réservé à Anthropic et `XHigh` à OpenAI. En 0.7, `ReasoningEffort::Zero` remplace `None` — [README](https://github.com/jeremychone/rust-genai)
- Usage tokens : Ollama n'émet pas les tokens en streaming via la couche OpenAI-compatible ; le cumul Gemini en streaming est « not clear » — [README, section Usage](https://github.com/jeremychone/rust-genai)
- Échappatoire prévue : `with_extra_body(...)` pour les champs propres à un fournisseur dans les payloads OpenAI-compatibles, et `ClientBuilder::with_adapter_kind` pour les passerelles ou modèles aux noms non standard — [README v0.6.x](https://github.com/jeremychone/rust-genai)
- Migrations documentées : [0.5 → 0.6](https://github.com/jeremychone/rust-genai/blob/main/docs/migration/migration_v_0_5_to_0_6.md) et [0.6 → 0.7](https://github.com/jeremychone/rust-genai/blob/main/docs/migration/migration-v_0_6_to_0_7.md), liées depuis le [README](https://github.com/jeremychone/rust-genai)

**Churn de rig**
- Voir Q1 : environ 12 entrées `[**breaking**]` rien qu'en 0.42.0, dont une incompatibilité de sérialisation d'historique — [CHANGELOG](https://github.com/0xPlaygrounds/rig/blob/main/crates/rig-core/CHANGELOG.md)

**Passerelles et proxys (sidecar)**

| Passerelle | Langage | Licence | État au 2026-09-25 | Source |
|---|---|---|---|---|
| LiteLLM | Python | MIT hors `enterprise/` | v1.102.1 (2026-09-23), 59 601 étoiles, 5 260 issues ouvertes | [GitHub](https://github.com/BerriAI/litellm), [LICENSE](https://github.com/BerriAI/litellm/blob/main/LICENSE) |
| TensorZero | Rust | Apache-2.0 | **Archivé le 2026-06-12**, dépôt en lecture seule (société en liquidation) | [GitHub](https://github.com/tensorzero/tensorzero), [ai-tldr](https://ai-tldr.dev/releases/tensorzero-archived-jun12/), [byteiota](https://byteiota.com/tensorzero-shuts-down-what-oss-llmops-cant-survive/) |
| Helicone ai-gateway | Rust | **Conflit** : l'API GitHub dit GPL-3.0, le badge du README dit « APACHE » | Dernier push 2025-11-21, badge « Public Beta », 632 étoiles | [GitHub](https://github.com/Helicone/ai-gateway) |
| Bifrost (maximhq) | Go | Apache-2.0 | core/v1.10.4 (2026-09-25), 8 354 étoiles ; « 23+ providers » via une API OpenAI-compatible ; fallbacks et load balancing ; lancement par `npx -y @maximhq/bifrost` ou Docker | [GitHub](https://github.com/maximhq/bifrost) |
| Portkey gateway | TypeScript | MIT | Dernier push 2026-05-25, 13 081 étoiles | [GitHub](https://github.com/Portkey-AI/gateway) |
| any-llm (Mozilla.ai) | Python (SDK, Python 3.11+) | Apache-2.0 | 1.29.0 (2026-09-24), 2 203 étoiles ; se présente comme alternative à LiteLLM | [GitHub](https://github.com/mozilla-ai/any-llm) |
| OpenRouter | Service hébergé | — | Frais de 5,5 % sur l'achat de crédits (plan Standard), 8 % en Business ; BYOK gratuit jusqu'à 25 000 $/mois de prix catalogue, puis 5 % | [openrouter.ai/pricing](https://openrouter.ai/pricing) |

- **LiteLLM, incident de chaîne d'approvisionnement** : les versions 1.82.7 et 1.82.8 publiées sur PyPI le 2026-03-24 contenaient un `.pth` malveillant (vol d'identifiants, backdoor systemd). Elles sont restées en ligne environ 40 minutes. Les attaquants (TeamPCP) avaient obtenu les identifiants PyPI via une compromission préalable de Trivy dans la CI — [issue #24518](https://github.com/BerriAI/litellm/issues/24518), [LiteLLM security update](https://docs.litellm.ai/blog/security-update-march-2026), [Snyk](https://snyk.io/blog/poisoned-security-scanner-backdooring-litellm/)
- **OpenRouter, sources en conflit** : des agrégateurs citent « 1M requêtes BYOK gratuites puis 5 % » ([aireiter](https://aireiter.com/blog/openrouter-byok-fees-fallback-guide)). La page officielle indique un seuil en dollars (25 k$/mois). Retenir la [page officielle](https://openrouter.ai/pricing).
- **burn-lm** (tracel-ai) : « Burn-LM has been archived and is no longer maintained » — [GitHub](https://github.com/tracel-ai/burn-lm)

**Inférence locale embarquée en Rust (question « bundlable dans Tauri ? »)**

| Crate | Version (date) | Téléch. totaux / récents | Étoiles | Notes | Source |
|---|---|---|---|---|---|
| candle-core (HF) | 0.11.0 (2026-06-26) | 8 579 781 / 3 017 994 | 21 100 | MIT OR Apache-2.0, framework ML en Rust | [crates.io](https://crates.io/crates/candle-core), [GitHub](https://github.com/huggingface/candle) |
| mistralrs | 0.8.1 (2026-04-02) | 200 282 / 135 985 | 7 718 | MIT ; MSRV 1.88 ; dépend de reqwest ^0.13 avec `blocking` + `rustls` et de tokio full ; chargement GGUF, quantification ISQ/UQFF, serveur OpenAI **et** Anthropic `/v1/messages` compatible | [crates.io](https://crates.io/crates/mistralrs), [GitHub](https://github.com/EricLBuehler/mistral.rs) |
| llama-cpp-2 (utilityai) | 0.1.157 (2026-09-22) | 1 365 392 / 679 915 | 652 | MIT OR Apache-2.0 ; binding de llama.cpp (C++) ; features par défaut `openmp`, `common` ; `--features cuda` pour le GPU | [crates.io](https://crates.io/crates/llama-cpp-2), [GitHub](https://github.com/utilityai/llama-cpp-rs) |
| llama_cpp (edgenai) | 0.3.2 (2024-04-29) | 43 027 / 6 405 | n.d. | abandonné de fait | [crates.io](https://crates.io/crates/llama_cpp) |
| kalosm (floneum) | 0.4.0 (**2025-02-09**) | 24 748 / 2 139 | 2 230 | crate figée depuis 02/2025, alors que le dépôt est actif (push 2026-09-21) | [crates.io](https://crates.io/crates/kalosm), [GitHub](https://github.com/floneum/kalosm) |

- Point à noter sur mistral.rs : le README évoque « v0.8.2 CUDA performance », mais crates.io s'arrête à 0.8.1 au 2026-09-25 — [GitHub](https://github.com/EricLBuehler/mistral.rs), [crates.io](https://crates.io/crates/mistralrs/versions)

### Inferences
- **Sidecar ou crate en processus, pour une app desktop** : une passerelle (LiteLLM, Bifrost, any-llm) impose d'embarquer un runtime Python ou un binaire Go via `externalBin` de Tauri, avec gestion du cycle de vie du processus et d'un port local. Cela alourdit le packaging multi-OS et élargit la surface d'attaque, comme l'a montré l'incident LiteLLM. Ces passerelles visent des serveurs multi-utilisateurs (observabilité, budgets, clés virtuelles), dont un éditeur desktop mono-utilisateur n'a pas besoin. Une crate en processus reste préférable.
- **OpenRouter** peut servir d'option « un seul endpoint OpenAI-compatible » **sans code nouveau** : le `OpenAiCompatibleProvider` existant suffit, au prix de 5,5 % sur les crédits et d'un tiers de plus dans la chaîne. genai a d'ailleurs un adaptateur `open_router`.
- **Inférence embarquée** : candle est du Rust pur sous licence permissive, donc le plus simple à lier. llama-cpp-2 compile du C++ (llama.cpp) et les features GPU (CUDA…) compliquent la CI multi-OS. mistralrs tire reqwest 0.13 et une MSRV 1.88. Pour tous, le poids des modèles (Go) et le GPU rendent le bundling dans l'installeur Tauri peu réaliste. Le téléchargement de modèles à la demande ou le maintien d'Ollama comme runtime externe restent plus raisonnables. Pour ce volet, voir aussi `runtimes_locaux.md` dans ce dossier.
- **Stratégie anti-churn** recommandée par ces constats : garder le trait `LlmProvider` de l'app comme frontière stable et encapsuler la crate choisie derrière un seul adaptateur. Épingler la version exacte (`=0.6.5`) et garder le chemin Ollama JSON-schema maison tant que genai ne transmet pas `JsonSpec` à Ollama.

### Gaps
- Exigences de build de llama-cpp-2 (cmake, clang, toolchain C++ selon l'OS) : non confirmées par source, la recherche n'a renvoyé que des logs CUDA.
- La taille binaire ajoutée par genai, rig ou graniet/llm dans un build Tauri release n'a pas été mesurée.
- Portkey gateway : pas de release récente vérifiée (seulement le dernier push, 2026-05-25).
- La cause de la licence divergente de Helicone ai-gateway (fichier LICENSE GPL-3.0 contre badge Apache) n'a pas été investiguée.
- La compatibilité d'un sidecar Bifrost (`npx`/Docker) avec le mécanisme `externalBin` de Tauri v2 n'a pas été vérifiée.
