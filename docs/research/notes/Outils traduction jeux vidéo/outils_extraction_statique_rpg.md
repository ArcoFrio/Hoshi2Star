# Static extraction/reinjection translation tools for Japanese RPG engines (RPG Maker XP/VX/VX Ace/MV/MZ/2000/2003, Wolf RPG Editor) — state as of September 2026

Research date: 2026-09-25. Evidence levels are marked inline:
- **[code]**: read directly from source in a shallow clone of DazedMTLTool at commit dated 2026-09-20.
- **[API]**: GitHub REST API metadata (pushed_at, release tags, license).
- **[README]**: README read directly.
- **[summary]**: page summarized by a fetch model. May be less precise.
- **[snippet]**: search-result snippet only. Primary page not reachable.
- **[vendor]**: self-reported marketing by the tool's vendor.

Tools covered: Translator++ (Dreamsavior), SLR Translator (Shisaye), RPGMaker Trans / JapeTrans, "Translator Aggregator", DazedMTLTool / DazedTL (dazedanon), MTool (AdventCirno), WolfTL, UberWolf, WolfDec and RV2JSON (Sinflower), Wolf Trans (elizagamedev), RPGMakerDecrypter (uuksu), rvpacker-txt-rs and rpgmtranslate(-qt), LinguaGacha, AiNiee, SExtractor, GalTransl, RuneTranslate, and a few smaller 2024–2026 GitHub tools.

---

## Q1. How does each tool extract text (parsers, file formats, event codes, plugins)?

### Takeaway
Tools fall into two families. **Engine-aware static parsers** decode the game data, dump strings and later rewrite the original files: Translator++, DazedTL, rvpacker-txt-rs, WolfTL, Wolf Trans, RPGMaker Trans and RuneTranslate. **Downstream LLM translators** do not parse games themselves. They consume exports from Translator++ or MTool: LinguaGacha and AiNiee. MTool is mostly a runtime ("external") translator, not an embedded static patcher. Event-code coverage is explicit only in DazedTL: message codes are on by default, while script, plugin and variable codes are opt-in.

### Cited Findings

**Translator++ (Dreamsavior)**
- Supported engines per docs: RPG Maker XP, VX, VX Ace, MV, MZ, 2000 and 2003. Also Wolf RPG Editor, KiriKiri/KAG, Ren'Py, Unity (including IL2CPP), Live Maker, YU-RIS, Tyrano, Windows binaries (DLL/EXE) and RPG Developer Bakin — [Dreamsavior docs](https://dreamsavior.net/docs/translator/) [summary; docs page footer dated 2024-09-11]
- Includes a "Parser Model Creator" whose rules can be JavaScript functions or regexes, for custom formats — [Dreamsavior docs](https://dreamsavior.net/docs/translator/) [summary; footer dated 2024-09-11]
- Text is grouped by **context**. For example, MV conversation text sits in the "Message" context, and word-wrap or queries select by context or pattern — [T++ word wrapping](https://dreamsavior.net/docs/translator/getting-started/word-wrapping-in-translator/) [summary]
- Recommended LinguaGacha workflow: new project, "RPGMaker Parser v2", point it at the game folder and allow JavaScript file loading. The prompt implies it parses plugins/js. The project is saved as a `.trans` file — [LinguaGacha wiki, RPG Maker best practice](https://github.com/neavo/LinguaGacha/wiki/BestPracticeForRPGMakerEN)
- Added 2026: SRPG Studio and Bakin parsers, stable as of v8.3.25 — [Patreon post v8.3.25](https://www.patreon.com/dreamsavior/posts/translator-ver-8-154383386). UnrealParser 0.1 arrived in August 2026, with "WolfTL" compatibility fixes in the same update — [Dreamsavior monthly update, 2026-08-30](https://dreamsavior.net/translator-monthly-update-image-translation-free-openai-support-and-unreal-engine-projects/) [summary]
- Wolf RPG reinjection has existed since T++ v2.4.16 — [Dreamsavior, "Apply translation into Wolf RPG games"](https://dreamsavior.net/translator-ver-2-4-16-apply-translation-into-wolf-rpg-games/) (title only)

**DazedMTLTool / DazedTL (dazedanon)**
- Engines: RPG Maker MV, MZ and VX Ace; WOLF (including a guided "WolfDawn" workflow); SRPG Studio; Ren'Py; TyranoScript; Kirikiri; NScripter; Unity; CSV, JSON, text, subtitles, regex-driven formats; prepared Aquedi4 data. "Support depth varies by engine. RPG Maker MV/MZ and WOLF have the most complete guided workflows" — [README](https://github.com/dazedanon/DazedMTLTool) [README]
- Adapters are Python modules: `rpgmakermvmz.py` (about 6,700 lines), `rpgmakerplugin.py`, `wolf.py`, `wolf2.py`, `wolfdawn.py`, `srpg.py`, `renpy.py`, `tyrano.py`, `kirikiri.py`, `nscript.py`, `unity.py`, `yuris.py`, `lune.py`, `aquedi4.py`, `csv.py`, `json.py`, `regex.py` and `text.py` — [modules/](https://github.com/dazedanon/DazedMTLTool/tree/main/modules) [code]
- Per-event-code toggles in the MV/MZ adapter — [modules/rpgmakermvmz.py](https://github.com/dazedanon/DazedMTLTool/blob/main/modules/rpgmakermvmz.py) [code]
  - **On by default:** `CODE101` (speaker/face header), `CODE401` (Show Text line), `CODE405` (Scrolling Text), `CODE102` (Choices).
  - **Off by default (opt-in):** `CODE408` (comment continuation, limited to markers such as `選択肢ヘルプ` per `util/rpgmaker_markers.py`), `CODE122` (Control Variables with script string, restricted by `CODE122_VAR_RANGES`), `CODE355655` (Script), `CODE356` (MV plugin command), `CODE357` (MZ plugin command), `CODE657` (MZ plugin command args), `CODE320` / `CODE324` / `CODE325` (Change Name / Nickname / Profile), `CODE111` (Conditional Branch), `CODE108` (Comment).
  - Other flags: `TLSYSTEMVARIABLES` and `TLSYSTEMSWITCHES` (off), `FIXTEXTWRAP` (on), `IGNORETLTEXT` (on), `PRESERVEORIGINAL` (on).
- It also contains a plugin registry (`util/rpgmaker_plugin_registry.py`) and AutoNamePopup speaker support — [util/](https://github.com/dazedanon/DazedMTLTool/tree/main/util) and [README](https://github.com/dazedanon/DazedMTLTool) [code]
- VX Ace goes through Sinflower's **RV2JSON** (rvdata2 to JSON and back, version 1.2.1 bundled), downloaded by `util/ace/update_tools.py` — [util/ace/update_tools.py](https://github.com/dazedanon/DazedMTLTool/blob/main/util/ace/update_tools.py) [code]. RV2JSON is Ruby. It dumps data to JSON and `Scripts.rvdata2` to `.rb` files, writes them back, and backs up files on in-place update — [RV2JSON README](https://github.com/Sinflower/RV2JSON) [README]
- Wolf unpacking is done by a bundled `UberWolfCli.exe` — [gameupdate/](https://github.com/dazedanon/DazedMTLTool/tree/main/gameupdate) [code]

**rvpacker-txt-rs (RPG-Maker-Translation-Tools org, Rust)**
- "extracts the translatable text from RPG Maker 2000/2003/XP/VX/VX Ace/MV/MZ game files into plain `.txt` files, and writes translated `.txt` files back"; it "decrypts `.rgss` archives" on the fly — [README](https://github.com/RPG-Maker-Translation-Tools/rvpacker-txt-rs) [README]
- 2000/2003 input is detected from `RPG_RT.ldb`, `RPG_RT.lmt` and `MapNNNN.lmu`. The tool guesses legacy encoding (UTF-8, Shift-JIS, GB18030, CP1252, CP1251) and writes UTF-8 by default — [README](https://github.com/RPG-Maker-Translation-Tools/rvpacker-txt-rs) [README]
- File format: one entry per line, `source<#>translation`. In-string line breaks become `\#`. Comment lines are prefixed `<!>` (for example `<!>ID`, `<!>NAME`, `<!>IN-GAME DISPLAYED NAME`). The separators are configurable — [README](https://github.com/RPG-Maker-Translation-Tools/rvpacker-txt-rs) [README]
- Modes: `read`, `write`, `purge`, `json` (binary to JSON), `serde` (CSV/XLSX/XML/JSON/YAML) — [README](https://github.com/RPG-Maker-Translation-Tools/rvpacker-txt-rs) [summary]
- GUI companions: `rpgmtranslate` (older) and `rpgmtranslate-qt` ("the fastest and lightest GUI…", created 2026-03, v1.1.4 on 2026-09-16) — [rpgmtranslate-qt](https://github.com/RPG-Maker-Translation-Tools/rpgmtranslate-qt) [API]. It is a successor of savannstm's `rvpacker-txt` (XP/VX/VXAce to .txt) — [savannstm/rvpacker-txt](https://github.com/savannstm/rvpacker-txt) [API]

**Wolf RPG: WolfTL, Wolf Trans, UberWolf**
- **WolfTL** extracts translation data from `.dat` and `.mps` files (CommonEvents, DataBases, Maps and Game.dat) to JSON. Its modes are `create`, `patch` and `patch_ip`. Output goes to `dump/{CommonEvents,Databases,Maps}` and then `patched/data`. "WolfPro games are supported. After patching, the files will be unencrypted." The parsing logic comes from Wolf Trans — [WolfTL README](https://github.com/Sinflower/WolfTL) [README]
- WolfTL documents the Wolf command-code table: 101 Message, 102 Choices, 103 Comment, 106 DebugMessage, 112 StringCondition, 122 SetString, 150 Picture, 1000 ProFeature, and others — [WolfTL README](https://github.com/Sinflower/WolfTL) [README]
- WolfTL v0.6.0 (2026-03-01) properly implemented Game.dat parsing/patching and rewrote the map parser to scan all folders for `.mps`. v0.6.1 (2026-05-18) added font modifications and fixed Game.dat parsing for Pro v1/v2. v0.6.2 (2026-06-28) fixed Game.dat and added invalid-map checks — [WolfTL releases](https://github.com/Sinflower/WolfTL/releases) [API]
- **Wolf Trans** (Ruby gem, elizagamedev) runs as `wolftrans game_dir patch_dir out_dir`. It extracts to plaintext patch files in the RPGMaker Trans syntax, with contexts like `> CONTEXT MPS:TitleMap/events/0/pages/1/65/Message`. The README warns: "OUT_DIR WILL BE *DELETED* IF IT EXISTS". Last push 2019-12-19 — [wolftrans README](https://github.com/elizagamedev/wolftrans) [README]/[API]
- **UberWolf** is a GUI and CLI decrypter for `.wolf`, `.data`, `.pak`, `.bin`, `.assets`, `.content`, `.res` and `.resource`. It auto-detects the key and identifies Pro Editor protection keys — [UberWolf README](https://github.com/Sinflower/UberWolf) [summary]

**RPGMaker Trans / JapeTrans (Habisain, a.k.a. Aleph Fell)**
- Extracts dialogue from RPG Maker games into editable text files, then reinserts it — [JapeTrans resources](https://gametrans.bitbucket.io/resources.html) [summary]
- The patch syntax is `> BEGIN STRING` / source / `> CONTEXT <path>` / translation / `> END STRING`. The source for this is Wolf Trans, which states it shares RPGMaker Trans's design — [wolftrans README](https://github.com/elizagamedev/wolftrans)
- A patch folder is identified by an `RPGMKTRANSPATCH` marker file — [archive.org RPGMaker Trans v4.5](https://archive.org/details/rpgmaker-transv-4.5) [snippet]

**MTool (AdventCirno)**
- Engines: RPG Maker 2000/2003, XP, VX, VX Ace, MKXP-Z, RGU, MV and MZ. Also Wolf RPG, SMILE GAME BUILDER, Bakin, TyranoBuilder, Kirikiri 2/Z, Ren'Py, Visual Novel Maker, ChoiceScript, SRPG Studio, Pixel Game Maker MV. Runs on Windows 7+ and Android 11+ — [mtool.app](https://mtool.app/?lang=en) [summary]
- The AiNiee README classifies MTool as "外挂式" (external/overlay translation). It classifies Translator++ as "内嵌式" (embedded into files) — [AiNiee README](https://github.com/NEKOparapa/AiNiee) [README]
- LinguaGacha's wiki recommends T++ over MTool because MTool fragments sentences into "clauses or phrases" that lack context — [LinguaGacha wiki](https://github.com/neavo/LinguaGacha/wiki/BestPracticeForRPGMakerEN)
- The MTool **export file** is a JSON of strings. Third-party tools consume it, e.g. [WWWee80/MTool-Translator](https://github.com/WWWee80/MTool-Translator) ("JSON batch translation tool", 44 stars, pushed 2026-09-24) [API]. T++ can also open an MTool translation file — [YouTube tutorial](https://www.youtube.com/watch?v=IHDC6A_aEM8) (title only)

**RuneTranslate (2026, closed source, Windows)** [vendor]
- A file patcher (no hook, no OCR) for 17 engines and formats, including RPG Maker, Ren'Py, Kirikiri, Wolf RPG, Unity, Unreal, Godot and NScripter. It "identifies the engine, opens the archives, pulls every translatable string" and exports a playable build — [RuneTranslatePublic README](https://github.com/3nginius/RuneTranslatePublic). The repo was created 2026-06-05 [API]

**Visual-novel side tools (related, not RPG-first)**
- **SExtractor** (satan53x) extracts and imports text from GalGame scripts. GPL-3.0, 507 stars, pushed 2026-09-20 — [SExtractor](https://github.com/satan53x/SExtractor) [API]
- **GalTransl** is an automated VN translation solution supporting GPT-4, Claude, DeepSeek and Sakura. GPL-3.0, 2,278 stars, release 8.0.0 on 2026-09-18 — [GalTransl](https://github.com/XD2333/GalTransl) [API]. It is VN-oriented, not an RPG Maker parser.

**Smaller or legacy GitHub tools**
- [RPG-Maker-Translator/RPG-Maker-Translator](https://github.com/RPG-Maker-Translator/RPG-Maker-Translator): auto-translate JP RPG Maker games. MIT, last push 2022-05-24 [API]
- [JaimeDevCode/RPGMakerTranslator](https://github.com/JaimeDevCode/RPGMakerTranslator): "Automatic, free translation of RPG Maker MV / MZ games". Created 2026-02, 7 stars [API]
- [LuoTat/RpgMakerTransTextTool](https://github.com/LuoTat/RpgMakerTransTextTool): extracts strings from RPGVXA games. Last update 2024-09 [API]
- [Dclef/RPGmaker-Translation](https://github.com/Dclef/RPGmaker-Translation): Chinese MV/MZ localization tutorial series. 52 stars, pushed 2026-09 [API]

### Inferences
- Only DazedTL publishes an explicit per-event-code policy. Its defaults (text 101/401/405/102 on; scripts, plugins, variables and conditional branches off) match the conventional risk split. Translating 122, 355/655, 356/357 and 111 can break game logic because strings there are often identifiers compared at runtime.
- The Wolf ecosystem converges on Sinflower's WolfTL (JSON) plus UberWolf (decrypt). DazedTL bundles UberWolf, and T++'s August 2026 fix list mentions WolfTL. That suggests T++ also relies on WolfTL for Wolf, but this is **not confirmed**.

### Gaps
- Which exact MV/MZ event codes and plugin parameters Translator++'s "RPGMaker Parser v2" extracts. Not found in public docs.
- MTool's internal extraction method. The public site does not describe hook vs static, and MTool is closed source.
- RPGMaker Trans documentation page (`rpgmakertrans.bitbucket.io/guidocs.html`) returned 404. Details come only from secondary sources.

---

## Q2. How are escape codes (\V[n], \C[n], \N[n], %1) protected before machine translation?

### Takeaway
Translator++ has the most elaborate, selectable masking system: hex placeholders by default, with HTML, XML and JSON cloaking and aggressive splitting as alternatives. DazedTL masks control codes as `__PROTECTED_n__` and adds extra guards. LinguaGacha (an LLM-downstream tool) relies on automatic code recognition and admits omissions. Every source concedes that masking is not fool-proof.

### Cited Findings
- **Translator++ "Code Escaping Algorithm"**, with a selectable algorithm — [T++ docs](https://dreamsavior.net/docs/translator/execution-flow/code-escaping-algorithm/) [summary; page undated]:
  - `hexPlaceholder` (default): `Hello \C[1]World\C[0]! Gold: %1` becomes `Hello 0xF0000World0xF0001! Gold: 0xF0002`, restored with regex `/0[xX][fF][\dA-Fa-f]{4}/g`. A case-insensitive variant also exists.
  - `HTMLCloaking` (`<hr id="n">`), `XMLCloaking` (`<hr id="1" />`).
  - `JSONCloaking` / `JSONObjectCloaking` (`${dat[n]}` sent as JSON arrays or objects, meant for LLMs) and `JSTemplateCloaking`.
  - `SubstituteNumber` (numeric tokens like `9012349`) and "Meaningless Word".
  - "Aggressive Splitting": tags are kept in memory and removed from the text sent, which "almost guaranteed" preservation at the cost of context.
  - `none`, plus user-defined custom patterns.
  - The docs warn that machine translators "will always find a way to mess your code up. It is guaranteed!"
- **DazedTL** "Placeholder Protection System" — [util/translation.py](https://github.com/dazedanon/DazedMTLTool/blob/main/util/translation.py) [code]:
  - Protected patterns: `\SE[..]`, `\ME[..]`, `\BGM[..]`, `\BGS[..]`, `_pum[..]`, `\VS[..]`, and a general control regex. The general regex matches any backslash run followed by an identifier with optional nested `[...]` (for example `\C[3]`, `\V[12]`) or punctuation codes `\{ \} \. \! \| \^ \> \< \# \$ \@ \, \-`.
  - An "orphan backslash" guard covers the case of a backslash before a non-ASCII word, where translation could create a new control code (`\ヘレン` becoming `\Helen`).
  - Placeholders are `__PROTECTED_{n}__`. Post-validation reports "missing protected codes" per line.
  - The MV/MZ adapter also removes a dummy subject "Taro" inserted for battle-log sentences and rejects `{{name}}` plugin placeholders — [rpgmakermvmz.py](https://github.com/dazedanon/DazedMTLTool/blob/main/modules/rpgmakermvmz.py) [code]
- **LinguaGacha** "automatically recognizes and preserves control codes like `\c[1]` and `\n[2]`" but concedes "more or less some omissions". Proofreading then flags code errors that could cause crashes — [LinguaGacha wiki](https://github.com/neavo/LinguaGacha/wiki/BestPracticeForRPGMakerEN) [summary]
- **SLR Translator** notes mention "improvements to how scripts are escaped" — [search snippet of an F95 mirror thread](https://auxiliardelimpeza.com/threads/slr-translator-offline-jp-to-en-translation-for-rpg-maker-vx-vx-ace-mv-and-mz.1528101/) [snippet, domain unreachable at fetch time]
- **rvpacker-txt-rs** does no MT masking. It is a pure text-file format. Only line breaks are escaped (`\#`) — [README](https://github.com/RPG-Maker-Translation-Tools/rvpacker-txt-rs) [README]

### Inferences
- The industry-standard pattern (seen in T++ and DazedTL) is: mask with an opaque token, send, restore, then validate that the token set matches. Hoshi2Star's `⟦ph_001⟧` scheme (ADR-002) sits in this family. DazedTL's orphan-backslash guard and T++'s "SubstituteNumber" are niche protections worth noting.

### Gaps
- Whether RuneTranslate, MTool or AiNiee use explicit tokens or rely on the model. No public documentation found.

---

## Q3. Translation memory, glossary, QA checks, word wrap and fonts

### Takeaway
Translator++ offers TM (MySQL/MariaDB), context-based wrapping and a custom parser/escaper. DazedTL offers a per-game glossary, "skills" prompt files, visible-width wrap and a deterministic QA bundle system for AI reviewers. RuneTranslate paywalls TM and glossary behind a $3/month Supporter tier [vendor]. WolfTL added font modification in 2026.

### Cited Findings
- **Translator++**: TM backed by MySQL/MariaDB databases, and "Context-Based Translation" — [T++ docs](https://dreamsavior.net/docs/translator/) [summary; footer dated 2024-09-11]
- **Translator++ wrapping**: a batch "Wrap texts" tool selects by context or manual query, since boxes with and without faces have different widths. It can split a dialog so "the dialog box will appear twice". An "On the fly word wrapping" mode is tag-based — [T++ word wrapping](https://dreamsavior.net/docs/translator/getting-started/word-wrapping-in-translator/) and [On the fly word wrapping](https://dreamsavior.net/docs/translator/getting-started/word-wrapping-in-translator/word-wrapping-by-tags/) [summary; pages undated]
- **Translator++ add-ons**: Rikaikun dictionary, ChatGPT Translator, Translation Proxy, GPT4All, JSON Parser — [T++ addons](https://dreamsavior.net/docs/translator/addons/) [summary]. The August 2026 update added an **ImageTranslator** (OCR plus image editing) and a Batch Proofreader with column and context options — [Dreamsavior update 2026-08-30](https://dreamsavior.net/translator-monthly-update-image-translation-free-openai-support-and-unreal-engine-projects/) [summary]
- **DazedTL context**:
  - Per-game `<game>/.dazedtl/glossary.txt` for names and recurring terms.
  - `skills/game.md` (setting, register, naming), `skills/quirks.md` (voice and formatting habits), and a local definitions-only Japanese sound-effect reference. Only entries that match the current source are injected — [README](https://github.com/dazedanon/DazedMTLTool) [README]
  - A seed glossary ships in `data/glossary_base.txt` — [data/](https://github.com/dazedanon/DazedMTLTool/tree/main/data) [code]
- **DazedTL QA**: `util/rpgmaker_qa.py` provides "Deterministic local tooling for AI-helper RPG Maker translation QA". It covers inventory, compact bundles, checkpoints, context expansion, result validation and correction maps, with versioned JSON schemas (`rpgmaker-qa-bundle-v3`, etc.). The module "never calls a model provider" — [rpgmaker_qa.py](https://github.com/dazedanon/DazedMTLTool/blob/main/util/rpgmaker_qa.py) [code]. It also has an "Evaluation" page to compare models on a deterministic sample — [README](https://github.com/dazedanon/DazedMTLTool) [README]
- **DazedTL wrap**: `util/dazedwrap.py` computes visible width, ignoring Wolf `\c[]` / `\f[]` codes, so that wrap checks match in-game boxes. `util/rpgmaker_rewrap.py` handles RPG Maker — [dazedwrap.py](https://github.com/dazedanon/DazedMTLTool/blob/main/util/dazedwrap.py) [code]
- **WolfTL** v0.6.1 (2026-05-18) "Added support for font modifications" — [WolfTL releases](https://github.com/Sinflower/WolfTL/releases) [API]
- **RuneTranslate** [vendor]:
  - Supporter tier: cross-project TM synced to account with CSV import/export, glossary forced into every batch, auto-glossary (katakana names, speaker labels).
  - Free: "AI refiner" second pass with neighbours as context, custom LLM prompt, and "Game diagnosis" (fonts, encoding, crash log, lines whose shape broke builds) — [features.md](https://github.com/3nginius/RuneTranslatePublic/blob/main/docs/features.md)
- **LinguaGacha**: glossary import, plus pre-translation replacement that imports `Actors.json` names for MV/MZ. Post-translation tags (red, blue, gold) mark entries that need review — [LinguaGacha wiki](https://github.com/neavo/LinguaGacha/wiki/BestPracticeForRPGMakerEN) [summary]
- **SLR Translator**: word-wrap updates "to work with more box types" and fixes where wrapping affected plugins — [snippet](https://auxiliardelimpeza.com/threads/slr-translator-offline-jp-to-en-translation-for-rpg-maker-vx-vx-ace-mv-and-mz.1528101/) [snippet]

### Inferences
- A QA engine that is independent of the LLM (deterministic checks: placeholder parity, width, untranslated text) is now standard in the leading tools. DazedTL formalises it with JSON schemas, and RuneTranslate markets "Game diagnosis".

### Gaps
- Whether T++ has a built-in QA checker beyond the Batch Proofreader. The documentation summary did not mention one.
- Font replacement handling in T++ and MTool. Not documented in reachable sources.

---

## Q4. How do they handle game updates (re-import, merge, diff)?

### Takeaway
Approaches differ widely:
- **Git-branch cherry-pick with no semantic merge**: DazedTL.
- **Content-hash metadata** preserving earlier decisions: rvpacker-txt-rs.
- **Re-scan and merge new lines**: RuneTranslate [vendor].
- **Project update and merge**: Translator++.
- **Source-keyed patch files re-applied to a fresh game**: RPGMaker Trans and Wolf Trans.

### Cited Findings
- **DazedTL "Version Update"**: two Git branches. `original` records official releases, and the translated branch receives each release by cherry-pick. Conflicting files default to the new official copy and are listed for review. JSON is normalized to 4-space indentation to avoid formatting-only diffs. "No engine-aware or semantic merge is performed" — [README](https://github.com/dazedanon/DazedMTLTool) [README]. A player-side `gameupdate/` component ships `GameUpdate.bat` / `.sh` plus `UberWolfCli.exe` — [gameupdate/](https://github.com/dazedanon/DazedMTLTool/tree/main/gameupdate) [code]
- **rvpacker-txt-rs**: records encoding, format settings and content hashes in `.rvpacker-metadata` so that later operations recognize changes while preserving previous translations. There is also an `append` operation — [README](https://github.com/RPG-Maker-Translation-Tools/rvpacker-txt-rs) [README/summary]
- **Translator++**: supports "project updating when source material changes and merging updates with existing translations" — [T++ docs](https://dreamsavior.net/docs/translator/) [summary; footer dated 2024-09-11]
- **RuneTranslate**: "Re-scan source: When a game updates, re-scan: new lines merge in, your translations stay, and your exclusion filters are re-applied" (free) — [features.md](https://github.com/3nginius/RuneTranslatePublic/blob/main/docs/features.md) [vendor]
- **Wolf Trans / RPGMaker Trans**: the patch directory is separate from the game. The tool is rerun as `game_dir patch_dir out_dir`, and existing patch strings are keyed by source text plus context — [wolftrans README](https://github.com/elizagamedev/wolftrans) [README]

### Inferences
- Keying by source text (RMT-style, rvpacker `<#>` lines) makes updates robust to reordering but fragile to edits of the source line. Git-based approaches (DazedTL) keep file history but push the conflict work onto the user.

### Gaps
- Exact T++ merge algorithm (key = context path? source text?). Not documented publicly.

---

## Q5. MT/LLM providers, batching and context given to the model

### Takeaway
By 2026 every active tool supports LLMs:
- **Translator++**: Google, DeepL, Sugoi, ChatGPT, DeepSeek, LiteLLM, LM Studio, Gemini, OpenRouter batch. The OpenAI-compatible add-on became free in August 2026.
- **DazedTL**: OpenAI, Gemini, Mistral, Anthropic and compatibles, with provider batch jobs and a coding-assistant mode ("Len's Method").
- **SLR Translator**: historically offline Sugoi/SugoiLevi.
- **MTool**: Sugoi and ChatGPT on Patreon tiers (older info).
- **RuneTranslate**: 10 providers, including a local model [vendor].

### Cited Findings
- **Translator++ engines**: Google Cloud, DeepL (standard and Pro), Sugoi Translator, ChatGPT/GPT-4, DeepSeek, LiteLLM, LM Studio — [T++ docs](https://dreamsavior.net/docs/translator/) and [translator-engine docs](https://dreamsavior.net/docs/translator/translator-engine/) [summary; docs footer dated 2024-09-11, list may be outdated]. A Sugoi offline engine page exists — [Sugoi Translator doc](https://dreamsavior.net/docs/translator/translator-engine/sugoi-translator/)
- **Translator++ August 2026**: the "OpenAI-compatible translator add-on is now free and included", with configurable reasoning effort, also covering the Proofreader, Gemini and Batch API models, and OpenRouter support for Batch LLM — [Dreamsavior 2026-08-30](https://dreamsavior.net/translator-monthly-update-image-translation-free-openai-support-and-unreal-engine-projects/) [summary]
- **Translator++ 8.4.15** (2026-04-30) was headlined "Built-in AI Assistants, Native Offline Translation", and a snippet saying T++ "now natively supports" zh, ja, ko, ru, es, id, ar, vi, pt… It is ambiguous whether this means UI or translation languages — [Dreamsavior post](https://dreamsavior.net/translator-annual-public-release-ver-8-4-15-built-in-ai-assistants-native-offline-translation-and-much-more/) [summary; details paywalled]
- **DazedTL**:
  - Providers: OpenAI, Gemini, Mistral, Anthropic and compatible providers.
  - A "Batches" page submits and resumes provider batch jobs, with sequential native OpenAI Batch queues.
  - "Len's Method" provides whole-game translation driven by a coding assistant: copy a starting prompt and paste it into the assistant.
  - "Evaluation" compares models.
  - Credits: Len for adaptive rate limiting and batch mode — [README](https://github.com/dazedanon/DazedMTLTool) [README]
  - Cost tracking: `util/provider_costs.py` and a "cache-discounted cost" running total in `translation.py` — [util/translation.py](https://github.com/dazedanon/DazedMTLTool/blob/main/util/translation.py) [code]
  - Speaker handling: WolfDawn reshapes `Name\nbody` into `[Name]: body` for the model and restores it on inject — [data/skills/wolf_speakers.md](https://github.com/dazedanon/DazedMTLTool/blob/main/data/skills/wolf_speakers.md) [code]
- **SLR Translator** (Shisaye) v1.124 requires a working SugoiV4 server on local port 14366, or the SugoiLevi model on port 14370. Windows x64, at least 8 GB RAM. JP to EN offline — [snippet](https://auxiliardelimpeza.com/threads/slr-translator-offline-jp-to-en-translation-for-rpg-maker-vx-vx-ace-mv-and-mz.1528101/) [snippet, undated]
- **MTool**: "additional translation engines such as Sugoi and ChatGPT 3.5, but these are only for Patreon subscribers" — [AdventCirno Patreon](https://www.patreon.com/AdventCirno/about) [search summary; originating page unconfirmed; the model name suggests 2023-era info]
- **RuneTranslate**: "10 translation providers — DeepL, OpenAI, Claude, DeepSeek, Google, a local model, and more. Five need no API key." Supporter-tier "provider routing" sends short strings to a cheap engine and dialogue to a premium one. Free runs use half the batch size, which is about 2x slower — [README](https://github.com/3nginius/RuneTranslatePublic) and [features.md](https://github.com/3nginius/RuneTranslatePublic/blob/main/docs/features.md) [vendor]
- **LinguaGacha**: AI "next-generation text translator" for novels, games and subtitles. 2,498 stars, build v0.123.1 on 2026-09-23 — [LinguaGacha](https://github.com/neavo/LinguaGacha) [API]
- **AiNiee**: AI translation of RPG/SLG games and documents. It supports Mtool, Ren'Py, Translator++ (`.trans`), ParaTranz, VNText and SExtractor exports. AGPL-3.0, 6,298 stars, v7.2.4 on 2026-09-01 — [AiNiee](https://github.com/NEKOparapa/AiNiee) [README/API]. A separate project, independent of the AiNiee app and aimed at novels, uses Claude Code as its translation engine — [xuanji86/ainiee-translate-skill](https://github.com/xuanji86/ainiee-translate-skill) [API]

### Inferences
- Two 2026 trends stand out: (1) batch APIs and cost-aware routing (T++ OpenRouter batch, DazedTL batches, RuneTranslate routing); (2) coding-assistant-driven translation ("Len's Method", ainiee-translate-skill), where the agent reads context files instead of receiving fixed prompts.

### Gaps
- Exact context window sent per request (neighbouring lines, speaker, file) for T++ and MTool. Not documented publicly.
- Whether current SLR Translator versions add LLM backends. Not verifiable, since F95zone and gitgud were not reachable.

---

## Q6. Output format (patch, full folder, ZIP) and decryption of encrypted archives and images

### Takeaway
Most tools write a patched copy of the data folder: T++ "Export to Folder", WolfTL `patched/`, Wolf Trans `out_dir`, rvpacker `write`. Some also produce shareable patches without game files (T++ exported patch, RMT patch folder, RuneTranslate `.rtpatch`). Decryption is usually delegated to specialised tools: RPGMakerDecrypter, rvpacker's rgss decryption, UberWolf and WolfDec.

### Cited Findings
- **T++**: export to folder (target `www/` if present). Exported patches can be distributed — [LinguaGacha wiki](https://github.com/neavo/LinguaGacha/wiki/BestPracticeForRPGMakerEN) and [T++ patching guide](https://dreamsavior.net/docs/translator/getting-started/patching-with-translator-exported-patch/) [summary]
- **WolfTL**: `patched/data`, or in-place with `patch_ip`. Patched files end up **unencrypted** — [WolfTL README](https://github.com/Sinflower/WolfTL) [README]
- **UberWolf** v0.6.x (2026):
  - v0.6.1 fixed Pro unpacking without a data folder.
  - v0.6.2 fixed the WolfRPG parser and moved WolfX cracking after protection removal.
  - v0.6.4 (2026-08-08) fixed unpacking of **WolfRPG Pro games protected with ChaCha20**, and unpacking no longer stops at the first invalid `.wolf` file — [UberWolf releases](https://github.com/Sinflower/UberWolf/releases) [API]
- **WolfDec** (Sinflower): "A Simple Wolf RPG File Decrypter". v0.3.3 (2023-04-08), last push 2023-11. Effectively superseded by UberWolf — [WolfDec](https://github.com/Sinflower/WolfDec) [API]
- **RPGMakerDecrypter** (uuksu, .NET 8 CLI):
  - Extracts `.rgssad`, `.rgss2a` and `.rgss3a` (XP, VX, VX Ace) and MV/MZ encrypted files.
  - `--recreate-project` produces a "best guess" of the original project.
  - GUI support has ended.
  - v3.0.4 released 2024-12-30, MIT, 328 stars — [README](https://github.com/uuksu/RPGMakerDecrypter) [README]/[API]
- **LynxShu/RPGMakerDecrypter**: decrypts and encrypts MV/MZ resources and edits saves. v1.0.1 (2026-08-30) — [repo](https://github.com/LynxShu/RPGMakerDecrypter) [API]
- **rvpacker-txt-rs** ecosystem: on-the-fly `.rgss` decryption, plus sibling CLIs `rpgm-asset-decrypter-rs` (assets) and `rpgm-archive-decrypter` (archives) — [README](https://github.com/RPG-Maker-Translation-Tools/rvpacker-txt-rs) [README]
- **RuneTranslate**: exports a playable build. `.rtpatch` contains "translations plus the source lines they attach to, and no game files", and can also export `.xlsx`. It warns before exporting over the game folder or saves. It "automatically detects encryption schemes and recovers hidden keys". For Unreal it writes an override `_P.pak` — [features.md](https://github.com/3nginius/RuneTranslatePublic/blob/main/docs/features.md), [file-formats.md](https://github.com/3nginius/RuneTranslatePublic/blob/main/docs/file-formats.md), [compare](https://runetranslate.com/compare) [vendor]
- **DazedTL**: an Images page to "Extract, edit, review, and safely patch translatable images", and a release packager (`util/release_package.py`) — [README](https://github.com/dazedanon/DazedMTLTool) [README/code]
- **RPGMaker Trans** uses a patch folder with an `RPGMKTRANSPATCH` marker and can take zips or folders. The GitHub tool [softashell/rpgmaker-patch-translator](https://github.com/softashell/rpgmaker-patch-translator) machine-translates RMT patches — [search result](https://archive.org/details/rpgmaker-transv-4.5) [snippet]

### Inferences
- Shareable "text-only patch" formats (RMT patch, `.rtpatch`, T++ patch) exist to avoid redistributing copyrighted game files. That is a relevant design pattern for Hoshi2Star's `.h2s` format.

### Gaps
- Whether Translator++ bundles its own decrypters for rgss3a and MV `.rpgmvp`, or relies on external ones.

---

## Q7. Business model, community, maintenance status (2026) and known limits

### Takeaway
- **Translator++**: free public build once a year, patron early access. Very active (v8.8.x in August 2026).
- **DazedTL**: open source, very active. License is inconsistent: MIT in LICENSE, GPL in README.
- **Sinflower Wolf tools**: MIT, active in 2026.
- **RPGMakerDecrypter**: stable, last release December 2024.
- **RPGMaker Trans and Wolf Trans**: abandoned.
- **MTool and RuneTranslate**: freemium Patreon.
- **SLR Translator**: status unverifiable.

### Cited Findings

**Per-tool status (as of 2026-09-25)**

| Tool | Model / licence | Latest known | Status |
|---|---|---|---|
| Translator++ | Free public build — [Patreon download post](https://www.patreon.com/posts/download-public-103339245). Supporters try new features early (search summary). The 8.4.15 announcement itself was gated at the Patreon "$1 or more" tier — [8.4.15 post](https://dreamsavior.net/translator-annual-public-release-ver-8-4-15-built-in-ai-assistants-native-offline-translation-and-much-more/) | Public 8.4.15 (2026-04-30). Patrons 8.8.5–8.8.24 (2026-08-30) — [update post](https://dreamsavior.net/translator-monthly-update-image-translation-free-openai-support-and-unreal-engine-projects/) | Very active |
| DazedTL / DazedMTLTool | Open source. **Licence conflict**: README says GPL-3.0 — [README](https://github.com/dazedanon/DazedMTLTool); `LICENSE.md` and the GitHub API say MIT [code/API] | No releases; last commit 2026-09-20; about 1,544 commits (from a fetch summary of the repo page) | Very active; Python 3.12–3.14, Win/Linux/macOS |
| WolfTL | MIT | v0.6.2 (2026-06-28) [API] | Active |
| UberWolf | MIT, 184 stars | v0.6.4 (2026-08-08) [API] | Active |
| WolfDec | none declared | v0.3.3 (2023-04-08) [API] | No push since 2023-11 |
| RV2JSON | MIT | repo created 2026-03, pushed 2026-05 [API] | New |
| Wolf Trans | MPL-2.0 | last push 2019-12-19 [API] | Abandoned |
| RPGMakerDecrypter (uuksu) | MIT, 328 stars | v3.0.4 (2024-12-30) [API] | Maintained but no release since 2024-12 |
| rvpacker-txt-rs / rpgmtranslate-qt | WTFPL | pushed 2026-08-31 / v1.1.4 (2026-09-16) [API] | Active |
| RPGMaker Trans | not verified | v4.5 (2017) — [archive.org](https://archive.org/details/rpgmaker-transv-4.5) | Abandoned; renamed JapeTrans — [gametrans](https://gametrans.bitbucket.io/resources.html); a Dreamsavior fork "of the abandoned RPGMakertrans project" (0 stars, last push 2024-04) — [dreamsavior/rpgmakertrans](https://github.com/dreamsavior/rpgmakertrans) |
| MTool | Freemium, Patreon (AdventCirno) — [Patreon](https://www.patreon.com/AdventCirno/about); "download first… then subscribe" — [mtool.app](https://mtool.app/?lang=en) | not found | Site reachable 2026-09; version and date not found |
| SLR Translator | UI under GPLv3 "similar to Translator++" [snippet] | v1.124 [snippet, undated] | Unverified |
| RuneTranslate | Free tier; Supporter $3/mo, Pro $5/mo per features.md; the compare page says $3.99/mo [vendor, **internal inconsistency**] — [features.md](https://github.com/3nginius/RuneTranslatePublic/blob/main/docs/features.md), [compare](https://runetranslate.com/compare) | public repo created 2026-06 | New; Windows, Linux via Wine; not code-signed; requires Patreon sign-in |
| LinguaGacha | no licence declared (API) | v0.123.1 (2026-09-23) [API] | Very active |
| AiNiee | AGPL-3.0 | 7.2.4 (2026-09-01) [API] | Very active |

**Known limits and complaints**
- T++ docs admit that code masking can fail ("guaranteed") — [T++ escaping doc](https://dreamsavior.net/docs/translator/execution-flow/code-escaping-algorithm/)
- MTool fragments sentences, which loses context — [LinguaGacha wiki](https://github.com/neavo/LinguaGacha/wiki/BestPracticeForRPGMakerEN). Its runtime/"external" approach means each player needs the tool — [AiNiee README](https://github.com/NEKOparapa/AiNiee); [RuneTranslate compare](https://runetranslate.com/compare) [vendor]
- Wolf Trans deletes `out_dir` if it exists — [wolftrans README](https://github.com/elizagamedev/wolftrans)
- WolfTL leaves patched files unencrypted — [WolfTL README](https://github.com/Sinflower/WolfTL)
- RPGMakerDecrypter's GUI has been discontinued (CLI only) — [README](https://github.com/uuksu/RPGMakerDecrypter)
- DazedTL does no semantic merge on updates. It says support depth varies by engine, and it requires Python plus API keys — [README](https://github.com/dazedanon/DazedMTLTool)
- Communities: the Chinese community prefers MTool, T++ and AiNiee, with Bilibili tutorials — [AiNiee README](https://github.com/NEKOparapa/AiNiee). The English community centres on F95zone, where the SLR thread lives. The RPG Maker Forums have a T++ thread — [rpgmakerweb forum](https://forums.rpgmakerweb.com/threads/translator-game-translation-tool.102706/)

### Inferences
- The competitive landscape in 2026 has three tiers. (1) Open-source developer pipelines: DazedTL, rvpacker-txt-rs, the Sinflower tools, LinguaGacha and AiNiee. (2) Freemium GUI suites monetised via Patreon: Translator++, MTool and RuneTranslate. (3) Legacy or abandoned patchers: RPGMaker Trans and Wolf Trans. Hoshi2Star (CAT plus LLM, TM, QA) competes most directly with Translator++ and RuneTranslate on features, and with DazedTL on LLM pipeline depth.

### Gaps
- **"Translator Aggregator (Silverwolfceh)"**: no such repo or tool found. The silverwolfceh GitHub account's repos show nothing translation-related. The closest match is **Translation Aggregator**, a *runtime* tool that sends hooked text to MT services (used with ITH or Textractor), not a static extractor — [gametrans resources](https://gametrans.bitbucket.io/resources.html). This is probably a name mix-up. Not described further.
- **SLR Translator**: F95zone (not indexed; the fetch landed on an unrelated thread) and gitgud (403, and the API search returned nothing) were unreachable. Version, date, current LLM support and licence are based only on snippets.
- Translator++ 8.4.15 full changelog and exact patron tier prices are behind the Patreon paywall. The tier required for beta builds is unknown.
- WolfDec vs UberWolf: WolfDec has had no push since 2023, while UberWolf is updated in 2026 and covers the same job. It is likely superseded, but that is an inference.
- **Original Ruby rvpacker** (`ludovicchabant/rvpacker`, `Lanza-/rvpacker-ng`): both returned 404 from the GitHub API. Only the `.txt` successors (savannstm/rvpacker-txt, rvpacker-txt-rs) were found.
- **"RPGMaker-LLM-Translator"**: no repo by that name was found. The GitHub search "RPG Maker LLM translate" returned nothing. The nearest matches are JaimeDevCode/RPGMakerTranslator and the MTool-JSON LLM translators (WWWee80/MTool-Translator, Richard-WYZ/MToolTranslator).
- MTool's current version, price and engine list per tier were not found.
- Quantitative community complaints (F95 and Anime-Sharing thread statistics) could not be accessed.
