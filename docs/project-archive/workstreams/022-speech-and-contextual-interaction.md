# Speech and contextual interaction

Repository: Jordan-Hall/browser

Retrieved from 2026-09-18T19:04:57+00:00 to 2026-09-18T19:05:07+00:00. This is a non-atomic point-in-time export. Descriptions and comments can contain historical or conflicting status claims. GitHub states, task references and source-authored checkboxes are preserved; none establishes accepted implementation or production readiness. Original description and comment strings are retained without edits in snapshot.json. Markdown headings are nested for navigation. Attachments remain links; deleted content, revision history, PR code diffs, inline code reviews and CI logs are not included.

**Issues in this document:** 29

## Contents

- [#22 — EPIC: Speech and contextual interaction](#issue-22)
- [#65 — [P0][VOICE-01] Local speech-to-text](#issue-65)
- [#66 — [P1][VOICE-02] Semantic commands and dictation](#issue-66)
- [#67 — [P3][VOICE-03] Read-aloud, multilingual and optional wake word](#issue-67)
- [#247 — [TASK][EPIC-VOICE.T01] Ratify audio and command activation contracts](#issue-247)
- [#248 — [TASK][EPIC-VOICE.T02] Integrate local speech with explicit selection context](#issue-248)
- [#249 — [TASK][EPIC-VOICE.T03] Integrate optional output, languages and wake mode](#issue-249)
- [#250 — [TASK][EPIC-VOICE.T04] Qualify voice under load and adversarial audio](#issue-250)
- [#515 — [TASK][VOICE-01.T01] Implement microphone consent and capture state](#issue-515)
- [#516 — [TASK][VOICE-01.T02] Build local audio preprocessing](#issue-516)
- [#517 — [TASK][VOICE-01.T03] Integrate the local transcription worker](#issue-517)
- [#518 — [TASK][VOICE-01.T04] Implement editable partial/final transcript UX](#issue-518)
- [#519 — [TASK][VOICE-01.T05] Implement cancellation and audio retention](#issue-519)
- [#520 — [TASK][VOICE-01.T06] Add vocabulary and device recovery hooks](#issue-520)
- [#521 — [TASK][VOICE-01.T07] Qualify accuracy, latency and false activation](#issue-521)
- [#522 — [TASK][VOICE-02.T01] Implement explicit dictation and command modes](#issue-522)
- [#523 — [TASK][VOICE-02.T02] Capture scoped semantic foreground context](#issue-523)
- [#524 — [TASK][VOICE-02.T03] Parse common commands and resolve ambiguity](#issue-524)
- [#525 — [TASK][VOICE-02.T04] Integrate correction and technical vocabulary](#issue-525)
- [#526 — [TASK][VOICE-02.T05] Route commands through ordinary authority](#issue-526)
- [#527 — [TASK][VOICE-02.T06] Implement interruption and task revision](#issue-527)
- [#528 — [TASK][VOICE-02.T07] Test contextual and adversarial speech journeys](#issue-528)
- [#529 — [TASK][VOICE-03.T01] Define read-aloud and language-pack contracts](#issue-529)
- [#530 — [TASK][VOICE-03.T02] Implement local TTS playback](#issue-530)
- [#531 — [TASK][VOICE-03.T03] Enforce sensitivity before reading aloud](#issue-531)
- [#532 — [TASK][VOICE-03.T04] Implement multilingual and pronunciation behavior](#issue-532)
- [#533 — [TASK][VOICE-03.T05] Implement optional local wake word](#issue-533)
- [#534 — [TASK][VOICE-03.T06] Handle audio routing and interruption](#issue-534)
- [#535 — [TASK][VOICE-03.T07] Qualify languages, wake-word errors and privacy](#issue-535)

---

<a id="issue-22"></a>
## #22 — EPIC: Speech and contextual interaction

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/22
**Created:** 2026-09-15T12:07:51Z | **Updated:** 2026-09-15T14:23:31Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1

Own local speech-to-text, command/dictation separation, foreground semantic context, correction vocabulary and optional read-aloud/voice output.

#### Child issues
- [ ] #65 VOICE-01 — Local speech-to-text
- [ ] #66 VOICE-02 — Semantic commands and dictation
- [ ] #67 VOICE-03 — Read-aloud, multilingual and optional wake word

#### Cross-cutting gates
Microphone state is visible, audio retention is opt-in, risky actions always use trusted confirmation, contextual references resolve to selected/shared objects rather than unrestricted desktop surveillance, and keyboard stop remains available.

### Discussion (1 comments)

#### Comment 5681875913 — Jordan-Hall — 2026-09-15T14:23:31Z

Source: https://github.com/Jordan-Hall/browser/issues/22#issuecomment-5681875913 | Updated: 2026-09-15T14:23:31Z

<!-- intent-implementation-v1:EPIC-VOICE -->
###### Workstream implementation and integration tasks

Integrate #65–#67 with local inference, workspace selection and trusted authorization. Voice is an input channel, not a separate permission system.

- [ ] **EPIC-VOICE.T01 — Ratify audio/command activation contracts.** Define visible capture state, local transcription, audio retention, command/dictation modes and contextual target resolution. **Proof:** capture and command activation can be controlled independently.
- [ ] **EPIC-VOICE.T02 — Integrate speech with selection context.** Connect editable transcripts, vocabulary corrections and selected object/panel/file IDs to command/goal processing. **Proof:** “hide these” targets the explicit selection; ambiguous references do not become guessed writes.
- [ ] **EPIC-VOICE.T03 — Integrate output/languages/wake mode.** Add local read-aloud, language packs and optional wake detection with sensitivity checks and interruption. **Proof:** sensitive text is not spoken automatically, and wake mode stays opt-in and visibly active.
- [ ] **EPIC-VOICE.T04 — Qualify load/adversarial audio.** Test accents, noise, silence, code symbols, quoted/nearby speech, permission loss and simultaneous coding inference. **Proof:** dictation remains text, consequential actions require trusted authority, and keyboard stop remains responsive.

**Demonstration:** select two offers, hide them by voice, correct a technical identifier, dictate a quoted imperative without executing it, then interrupt while a coding model is busy.

**Review boundary:** wake detection and speaker matching do not authorize transactions. Do not solve pronoun resolution with unrestricted continuous desktop recording. Publish accuracy and latency per supported device/language corpus.


---

<a id="issue-65"></a>
## #65 — [P0][VOICE-01] Local speech-to-text

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/65
**Created:** 2026-09-15T12:14:39Z | **Updated:** 2026-09-15T19:37:23Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #22

#### Objective
Ship low-latency local speech input as a release requirement, with explicit microphone state, editable transcripts and no internet dependency.

#### Scope
- Local streaming/push-to-talk transcription worker (whisper.cpp-class runtime initially).
- Visible microphone capture state and hardware permission handling.
- Partial transcript, final transcript, edit/correct and cancel flows.
- Audio lifecycle: discard raw audio by default after transcription; separate opt-in retention/export.
- Technical vocabulary hooks and per-user local vocabulary storage.
- Independent keyboard/global stop path while speech/inference workers are busy.
- Measurements for latency, WER/command accuracy, CPU/GPU/memory and noise behavior.

#### Privacy / safety rules
- Recording state must never be hidden.
- Captured audio cannot automatically become durable memory/training data.
- Speech recognition alone never authorizes consequential writes.

#### Acceptance criteria
- [ ] Transcription works with networking disabled on declared reference hardware.
- [ ] Microphone activity is always visibly indicated.
- [ ] Partial/final text can be edited before being used as task input.
- [ ] Audio is discarded by default according to documented retention behavior.
- [ ] Keyboard stop works even under heavy model load.
- [ ] Accent/noise/silence/code-symbol fixtures are measured and tracked.

#### Dependencies
- LOCAL-01

**First phase:** P0  
**Maturity target:** P1  
**Owner:** local-ai-speech

### Discussion (2 comments)

#### Comment 5682209417 — Jordan-Hall — 2026-09-15T14:41:17Z

Source: https://github.com/Jordan-Hall/browser/issues/65#issuecomment-5682209417 | Updated: 2026-09-15T14:41:17Z

<!-- intent-implementation-v1:VOICE-01 -->
###### Implementation proposal — VOICE-01

Build capture, preprocessing and a supervised local transcription worker with explicit AudioSession and transcript revision contracts. Integrate #61 scheduling/privacy; speech ships in the first local runtime slice.

- [ ] **VOICE-01.T01 — Microphone consent/state.** Acquire platform permission through trusted UI, select devices and expose listening/recording/stopped states independently of inference. Use bounded transient buffers. **Verify:** capture cannot occur without a visible state and appropriate OS permission.
- [ ] **VOICE-01.T02 — Audio preprocessing.** Normalize sample rates/channels and add bounded voice activity/endpoint detection with timestamps. **Verify:** silence, device buffering and malformed input cannot cause unbounded memory or phantom commands.
- [ ] **VOICE-01.T03 — Local transcription.** Load a verified speech pack, process bounded chunks and return provisional/final segment revisions under foreground priority. **Verify:** offline configuration makes no network requests and speech failure does not freeze the shell.
- [ ] **VOICE-01.T04 — Editable transcript.** Keep provisional text distinct from committed input; preserve user corrections against late recognizer updates. **Verify:** partial hypotheses cannot automatically submit a command or overwrite edits.
- [ ] **VOICE-01.T05 — Stop/retention.** Route keyboard stop independently, stop capture/cancel decoding and discard raw buffers by default. Require separate opt-in for recording/export. **Verify:** stalled decoding cannot prevent capture shutdown or silently retain audio.
- [ ] **VOICE-01.T06 — Vocabulary/device recovery.** Support explicit technical-name hints where the backend supports them; handle device change, suspend and worker restart. **Verify:** recovery retains transcript state without hidden recording or losing user edits.
- [ ] **VOICE-01.T07 — Accuracy/latency qualification.** Test accents, noise, silence, interruptions and code punctuation on labelled audio. **Verify:** report word error separately from command/identifier accuracy, with hardware/backend versions.

**Reference:** [whisper.cpp](https://github.com/ggml-org/whisper.cpp). Low-latency partials require engineered chunking/endpointing; do not assume every model is natively streaming. Transcription and speaker recognition never confer transaction authority; #66 supplies the explicit command path.

#### Comment 5687011120 — Jordan-Hall — 2026-09-15T19:37:23Z

Source: https://github.com/Jordan-Hall/browser/issues/65#issuecomment-5687011120 | Updated: 2026-09-15T19:37:23Z

###### Task issues
- [ ] #515 `VOICE-01.T01` — Implement microphone consent and capture state
- [ ] #516 `VOICE-01.T02` — Build local audio preprocessing
- [ ] #517 `VOICE-01.T03` — Integrate the local transcription worker
- [ ] #518 `VOICE-01.T04` — Implement editable partial/final transcript UX
- [ ] #519 `VOICE-01.T05` — Implement cancellation and audio retention
- [ ] #520 `VOICE-01.T06` — Add vocabulary and device recovery hooks
- [ ] #521 `VOICE-01.T07` — Qualify accuracy, latency and false activation


---

<a id="issue-66"></a>
## #66 — [P1][VOICE-02] Semantic commands and dictation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/66
**Created:** 2026-09-15T12:14:49Z | **Updated:** 2026-09-15T19:38:21Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #22

#### Objective
Turn speech into reliable contextual interaction by separating dictation from commands and resolving references against explicit semantic foreground context.

#### Scope
- Distinct command activation and dictation modes.
- Resolve “this/these/current/that one” against WS-04 selected object/panel/evidence/file IDs.
- Command parsing into GoalContract edits, deterministic UI actions or bounded task requests.
- User-managed technical vocabulary and corrections for names, repositories, identifiers and product terms.
- Confirmation/revision before ambiguous semantic targeting.
- Nearby/quoted speech handling that remains content rather than executable command.
- History showing transcript → interpreted command → resolved target.

#### Safety rules
- Voice matching is not authorization.
- Purchases, bids, sharing, deletion, messages and security changes still require the normal trusted approval policy.
- No unrestricted continuous desktop observation just to resolve pronouns.

#### Acceptance criteria
- [ ] Dictated text is never executed merely because it contains imperative language.
- [ ] Commands resolve to explicit selected/context IDs and fail when ambiguity is material.
- [ ] Quoted/article/nearby speech cannot authorize side effects.
- [ ] Corrections update the intended target/text without restarting the whole task.
- [ ] Target/account/recipient changes invalidate stale command interpretation where needed.
- [ ] Command interpretation and resolved context are inspectable in task history.

#### Dependencies
- VOICE-01
- WS-04
- DATA-04

**First phase:** P1  
**Maturity target:** P3  
**Owner:** local-ai-speech

### Discussion (2 comments)

#### Comment 5682215572 — Jordan-Hall — 2026-09-15T14:41:38Z

Source: https://github.com/Jordan-Hall/browser/issues/66#issuecomment-5682215572 | Updated: 2026-09-15T14:41:38Z

<!-- intent-implementation-v1:VOICE-02 -->
###### Implementation proposal — VOICE-02

Connect #65 speech to #40 explicit selection and #44 goal contracts. Store original transcript, corrected text, interpretation and resolved targets separately so the user can inspect mistakes.

- [ ] **VOICE-02.T01 — Dictation versus commands.** Provide distinct visible activation modes; route dictation to text insertion only. **Verify:** imperative sentences inside dictated text are not executed.
- [ ] **VOICE-02.T02 — Scoped foreground context.** Snapshot permitted selected objects, focus, file/panel/task and revisions. **Verify:** context cannot include unrequested continuous desktop capture or unrelated account state.
- [ ] **VOICE-02.T03 — Typed command parsing.** Deterministically map common hide/sort/open/save actions; use bounded model interpretation for novelty and surface material target/account/amount ambiguity. **Verify:** pronouns resolve current semantic IDs, not a guessed screenshot target.
- [ ] **VOICE-02.T04 — Corrections/vocabulary.** Preserve transcript/interpretation revisions and allow local name, number and target correction without restarting the task. **Verify:** late recognizer output cannot reinstate an earlier incorrect recipient or amount.
- [ ] **VOICE-02.T05 — Ordinary authority path.** Send deterministic view changes to their services and consequential commands to canonical broker/transaction proposals. **Verify:** exact account, recipient, destination and amount appear in trusted confirmation; voice is not a grant.
- [ ] **VOICE-02.T06 — Interrupt/revise.** Resolve the current task explicitly and send pause/cancel/revise through reserved control paths. **Verify:** stopping interpretation/capture leaves the last valid workspace intact and revokes intended execution only.
- [ ] **VOICE-02.T07 — Context/adversarial tests.** Combine changed selections, multiple accounts, quoted/nearby voices and stale notification targets with labelled transcripts. **Verify:** report exact target/recipient/amount accuracy and correction burden, not only transcription accuracy.

**Implementation rule:** source audio, an article read aloud, a wake word or voice identity cannot substitute for trusted authorization. Selection changes between interpretation and dispatch require revalidation; material changes invalidate the proposal. Keep common commands usable deterministically without frontier inference.

#### Comment 5687022961 — Jordan-Hall — 2026-09-15T19:38:21Z

Source: https://github.com/Jordan-Hall/browser/issues/66#issuecomment-5687022961 | Updated: 2026-09-15T19:38:21Z

###### Task issues
- [ ] #522 `VOICE-02.T01` — Implement explicit dictation and command modes
- [ ] #523 `VOICE-02.T02` — Capture scoped semantic foreground context
- [ ] #524 `VOICE-02.T03` — Parse common commands and resolve ambiguity
- [ ] #525 `VOICE-02.T04` — Integrate correction and technical vocabulary
- [ ] #526 `VOICE-02.T05` — Route commands through ordinary authority
- [ ] #527 `VOICE-02.T06` — Implement interruption and task revision
- [ ] #528 `VOICE-02.T07` — Test contextual and adversarial speech journeys


---

<a id="issue-67"></a>
## #67 — [P3][VOICE-03] Read-aloud, multilingual and optional wake word

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/67
**Created:** 2026-09-15T12:14:59Z | **Updated:** 2026-09-15T19:39:13Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #1

### Original description

Programme: #1
Epic: #22

#### Objective
Extend the speech layer with opt-in local output, multilingual packs and optional wake-word interaction without weakening privacy or command safety.

#### Scope
- Local TTS/read-aloud engine abstraction and voice/language packs.
- Pronunciation dictionary and user technical vocabulary reuse.
- Per-workspace/content sensitivity checks before reading aloud.
- Optional local wake word with explicit on-device state and enable/disable UX.
- Multilingual STT/TTS pack selection and mixed technical vocabulary.
- Audio-device routing, interruption/barge-in and mute behavior.
- Evaluation corpus across target accents, languages, noise and sensitive-content scenarios.

#### Privacy/safety rules
- Wake word is opt-in and visibly active.
- Sensitive content is never read aloud merely because TTS is available.
- Wake detection/voice identity is not authorization for consequential actions.

#### Acceptance criteria
- [ ] Target language/accent/noise suites meet declared support thresholds.
- [ ] Sensitive content requires context-appropriate user choice before read-aloud.
- [ ] Wake-word mode remains local for supported configuration and exposes clear recording/listening state.
- [ ] User can interrupt output immediately.
- [ ] Language-pack installation exposes size/license/privacy metadata.
- [ ] No retained audio is created without the configured opt-in.

#### Dependencies
- VOICE-01
- SEC-03

**First phase:** P3  
**Maturity target:** P5  
**Owner:** local-ai-speech

### Discussion (2 comments)

#### Comment 5682220736 — Jordan-Hall — 2026-09-15T14:41:53Z

Source: https://github.com/Jordan-Hall/browser/issues/67#issuecomment-5682220736 | Updated: 2026-09-15T14:41:53Z

<!-- intent-implementation-v1:VOICE-03 -->
###### Implementation proposal — VOICE-03

Extend #65 with local TTS, qualified language packs and opt-in wake detection; enforce #8 sensitivity labels at audio output as well as at inference.

- [ ] **VOICE-03.T01 — Output/pack contracts.** Define text/source sensitivity, language/voice, device, retention and interruption; version pronunciation dictionaries and pack hashes/licenses. **Verify:** changing a voice pack cannot silently change privacy or output routing.
- [ ] **VOICE-03.T02 — Local playback.** Run the selected backend in a supervised worker with bounded audio streaming and pause/stop/seek where supported. **Verify:** output remains linked to visible text/evidence and can stop without waiting for full synthesis.
- [ ] **VOICE-03.T03 — Sensitive read-aloud checks.** Evaluate current source labels/workspace/user choice before speaker output; prompt for private material as appropriate. **Verify:** headphones or device labels alone are not assumed to prove privacy.
- [ ] **VOICE-03.T04 — Multilingual/pronunciation support.** Install qualified packs, reuse approved vocabulary and preserve names, numbers and code identifiers. **Verify:** code-switching and technical terms have targeted tests rather than assumed multilingual correctness.
- [ ] **VOICE-03.T05 — Optional local wake word.** Require explicit enablement, bounded transient buffers, visible listening state and immediate disablement. **Verify:** activation only enters the documented capture/command path; it never authorizes consequential actions.
- [ ] **VOICE-03.T06 — Routing/interruption.** Handle device unplug/suspend and coordinate playback/microphone interruption. **Verify:** the application's own read-aloud cannot become an authorized new command.
- [ ] **VOICE-03.T07 — Qualification.** Test languages, noise, accents, false accept/reject rates, interruption and private-content scenarios with versioned hardware/model metadata. **Verify:** evaluation audio is retained only under explicit consent and reported limitations remain visible.

**Review decisions:** backend/license/hardware/language support need measured selection; no default promise that every language or identifier is handled accurately. Read-aloud and wake mode remain fully in scope but do not weaken command/dictation separation or trusted confirmation requirements.

#### Comment 5687033645 — Jordan-Hall — 2026-09-15T19:39:13Z

Source: https://github.com/Jordan-Hall/browser/issues/67#issuecomment-5687033645 | Updated: 2026-09-15T19:39:13Z

###### Task issues
- [ ] #529 `VOICE-03.T01` — Define read-aloud and language-pack contracts
- [ ] #530 `VOICE-03.T02` — Implement local TTS playback
- [ ] #531 `VOICE-03.T03` — Enforce sensitivity before reading aloud
- [ ] #532 `VOICE-03.T04` — Implement multilingual and pronunciation behavior
- [ ] #533 `VOICE-03.T05` — Implement optional local wake word
- [ ] #534 `VOICE-03.T06` — Handle audio routing and interruption
- [ ] #535 `VOICE-03.T07` — Qualify languages, wake-word errors and privacy


---

<a id="issue-247"></a>
## #247 — [TASK][EPIC-VOICE.T01] Ratify audio and command activation contracts

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/247
**Created:** 2026-09-15T15:28:10Z | **Updated:** 2026-09-15T15:28:10Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #22

### Original description

Parent: #22

Task ID: `EPIC-VOICE.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-248"></a>
## #248 — [TASK][EPIC-VOICE.T02] Integrate local speech with explicit selection context

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/248
**Created:** 2026-09-15T15:28:15Z | **Updated:** 2026-09-15T15:28:15Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #22

### Original description

Parent: #22

Task ID: `EPIC-VOICE.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-249"></a>
## #249 — [TASK][EPIC-VOICE.T03] Integrate optional output, languages and wake mode

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/249
**Created:** 2026-09-15T15:28:21Z | **Updated:** 2026-09-15T15:28:21Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #22

### Original description

Parent: #22

Task ID: `EPIC-VOICE.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-250"></a>
## #250 — [TASK][EPIC-VOICE.T04] Qualify voice under load and adversarial audio

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/250
**Created:** 2026-09-15T15:28:28Z | **Updated:** 2026-09-15T15:28:28Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #22

### Original description

Parent: #22

Task ID: `EPIC-VOICE.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-515"></a>
## #515 — [TASK][VOICE-01.T01] Implement microphone consent and capture state

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/515
**Created:** 2026-09-15T19:36:47Z | **Updated:** 2026-09-15T19:36:47Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #65

### Original description

Parent: #65

Task ID: `VOICE-01.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-516"></a>
## #516 — [TASK][VOICE-01.T02] Build local audio preprocessing

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/516
**Created:** 2026-09-15T19:36:52Z | **Updated:** 2026-09-15T19:36:52Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #65

### Original description

Parent: #65

Task ID: `VOICE-01.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-517"></a>
## #517 — [TASK][VOICE-01.T03] Integrate the local transcription worker

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/517
**Created:** 2026-09-15T19:36:58Z | **Updated:** 2026-09-15T19:36:58Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #65

### Original description

Parent: #65

Task ID: `VOICE-01.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-518"></a>
## #518 — [TASK][VOICE-01.T04] Implement editable partial/final transcript UX

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/518
**Created:** 2026-09-15T19:37:02Z | **Updated:** 2026-09-15T19:37:02Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #65

### Original description

Parent: #65

Task ID: `VOICE-01.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-519"></a>
## #519 — [TASK][VOICE-01.T05] Implement cancellation and audio retention

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/519
**Created:** 2026-09-15T19:37:06Z | **Updated:** 2026-09-15T19:37:06Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #65

### Original description

Parent: #65

Task ID: `VOICE-01.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-520"></a>
## #520 — [TASK][VOICE-01.T06] Add vocabulary and device recovery hooks

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/520
**Created:** 2026-09-15T19:37:11Z | **Updated:** 2026-09-15T19:37:11Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #65

### Original description

Parent: #65

Task ID: `VOICE-01.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-521"></a>
## #521 — [TASK][VOICE-01.T07] Qualify accuracy, latency and false activation

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/521
**Created:** 2026-09-15T19:37:17Z | **Updated:** 2026-09-15T19:37:17Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #65

### Original description

Parent: #65

Task ID: `VOICE-01.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-522"></a>
## #522 — [TASK][VOICE-02.T01] Implement explicit dictation and command modes

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/522
**Created:** 2026-09-15T19:37:29Z | **Updated:** 2026-09-15T19:37:29Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #66

### Original description

Parent: #66

Task ID: `VOICE-02.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-523"></a>
## #523 — [TASK][VOICE-02.T02] Capture scoped semantic foreground context

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/523
**Created:** 2026-09-15T19:37:33Z | **Updated:** 2026-09-15T19:37:33Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #66

### Original description

Parent: #66

Task ID: `VOICE-02.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-524"></a>
## #524 — [TASK][VOICE-02.T03] Parse common commands and resolve ambiguity

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/524
**Created:** 2026-09-15T19:37:39Z | **Updated:** 2026-09-15T19:37:39Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #66

### Original description

Parent: #66

Task ID: `VOICE-02.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-525"></a>
## #525 — [TASK][VOICE-02.T04] Integrate correction and technical vocabulary

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/525
**Created:** 2026-09-15T19:37:45Z | **Updated:** 2026-09-15T19:37:45Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #66

### Original description

Parent: #66

Task ID: `VOICE-02.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-526"></a>
## #526 — [TASK][VOICE-02.T05] Route commands through ordinary authority

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/526
**Created:** 2026-09-15T19:37:50Z | **Updated:** 2026-09-15T19:37:50Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #66

### Original description

Parent: #66

Task ID: `VOICE-02.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-527"></a>
## #527 — [TASK][VOICE-02.T06] Implement interruption and task revision

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/527
**Created:** 2026-09-15T19:37:56Z | **Updated:** 2026-09-15T19:37:56Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #66

### Original description

Parent: #66

Task ID: `VOICE-02.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-528"></a>
## #528 — [TASK][VOICE-02.T07] Test contextual and adversarial speech journeys

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/528
**Created:** 2026-09-15T19:38:15Z | **Updated:** 2026-09-15T19:38:15Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #66

### Original description

Parent: #66

Task ID: `VOICE-02.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-529"></a>
## #529 — [TASK][VOICE-03.T01] Define read-aloud and language-pack contracts

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/529
**Created:** 2026-09-15T19:38:26Z | **Updated:** 2026-09-15T19:38:26Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #67

### Original description

Parent: #67

Task ID: `VOICE-03.T01`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-530"></a>
## #530 — [TASK][VOICE-03.T02] Implement local TTS playback

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/530
**Created:** 2026-09-15T19:38:32Z | **Updated:** 2026-09-15T19:38:32Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #67

### Original description

Parent: #67

Task ID: `VOICE-03.T02`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-531"></a>
## #531 — [TASK][VOICE-03.T03] Enforce sensitivity before reading aloud

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/531
**Created:** 2026-09-15T19:38:39Z | **Updated:** 2026-09-15T19:38:39Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #67

### Original description

Parent: #67

Task ID: `VOICE-03.T03`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-532"></a>
## #532 — [TASK][VOICE-03.T04] Implement multilingual and pronunciation behavior

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/532
**Created:** 2026-09-15T19:38:44Z | **Updated:** 2026-09-15T19:38:44Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #67

### Original description

Parent: #67

Task ID: `VOICE-03.T04`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-533"></a>
## #533 — [TASK][VOICE-03.T05] Implement optional local wake word

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/533
**Created:** 2026-09-15T19:38:51Z | **Updated:** 2026-09-15T19:38:51Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #67

### Original description

Parent: #67

Task ID: `VOICE-03.T05`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-534"></a>
## #534 — [TASK][VOICE-03.T06] Handle audio routing and interruption

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/534
**Created:** 2026-09-15T19:38:58Z | **Updated:** 2026-09-15T19:38:58Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #67

### Original description

Parent: #67

Task ID: `VOICE-03.T06`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

<a id="issue-535"></a>
## #535 — [TASK][VOICE-03.T07] Qualify languages, wake-word errors and privacy

**GitHub state:** open | **State reason:** not supplied
**Source:** https://github.com/Jordan-Hall/browser/issues/535
**Created:** 2026-09-15T19:39:05Z | **Updated:** 2026-09-15T19:39:05Z
**Author:** Jordan-Hall | **Assignees:** none
**Labels:** none | **Milestone:** none
**Native parent:** none recorded | **Body-declared parent:** #67

### Original description

Parent: #67

Task ID: `VOICE-03.T07`

Review: pending
Implementation: not started

### Discussion (0 comments)

No issue-conversation comments were returned at export time.

---

