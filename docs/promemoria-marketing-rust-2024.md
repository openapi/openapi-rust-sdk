# Promemoria per il marketing: Openapi® passa a Rust 2024

> Nota interna per chi prepara la comunicazione (blog post, social, newsletter).
> Spiega in parole semplici cosa è successo, perché conta e a chi va il merito.

## In una frase

L'SDK ufficiale di Openapi® per il linguaggio Rust è passato a **Rust 2024**, l'edizione più recente del linguaggio. Il cambio è arrivato grazie al contributo di un volontario della community open source, **Luca Cappelletti**.

## Cos'è, detto semplice

- **Rust** è un linguaggio di programmazione molto apprezzato per velocità e sicurezza. Lo usano aziende come Microsoft, Google, Amazon e Cloudflare.
- **L'SDK Rust di Openapi®** è la libreria gratuita e open source che permette agli sviluppatori Rust di usare le API di Openapi® con poche righe di codice.
- Ogni tre anni circa Rust pubblica una nuova **"edizione"**, un aggiornamento importante che introduce regole più moderne e più sicure. **Rust 2024** è la più recente ed è la più grande dal 2018.

Passare a Rust 2024 è un po' come aggiornare un'auto all'ultimo standard di sicurezza: fuori sembra uguale, dentro è più affidabile.

## Perché l'abbiamo fatto: i motivi

1. **Più sicurezza.** Aggiornando le librerie da cui dipende l'SDK sono state eliminate **10 vulnerabilità note** che gli strumenti di controllo segnalavano.
2. **Di nuovo stabile.** Prima dell'intervento l'SDK non si compilava più con le versioni più recenti di Rust. Ora sì.
3. **Codice più moderno e meno soggetto a errori.** Rust 2024 elimina alcune trappole classiche, per esempio blocchi e crash dovuti a risorse tenute occupate più del necessario, ed è più severo con le parti di codice "pericolose".
4. **Regole chiare per gli sviluppatori.** Abbiamo dichiarato in modo esplicito la versione minima di Rust richiesta (**1.85**), così chi usa l'SDK sa subito di cosa ha bisogno.
5. **Nessun disagio per chi lo usa già.** Il funzionamento dell'SDK non cambia e chi lo usa non deve riscrivere nulla. Basta avere Rust 1.85 o più recente.
6. **Base solida per il futuro.** È il primo passo di un percorso per rafforzare i test e i controlli automatici dell'SDK prima di aggiungere nuove funzionalità.

## Il valore della community (il messaggio chiave)

Il lavoro è stato proposto e realizzato da **Luca Cappelletti** ([@LucaCappelletti94](https://github.com/LucaCappelletti94) su GitHub), un contributor esterno. Questo dimostra che:

- i progetti open source di Openapi® sono **vivi e aperti** a contributi reali;
- la community aiuta Openapi® a mantenere i suoi strumenti **sicuri e aggiornati**;
- chi contribuisce riceve **riconoscimento pubblico** e può entrare nel [registro ufficiale dei contributor Openapi®](https://github.com/openapi/contributors).

Nella comunicazione è importante **citare e ringraziare Luca per nome**, con il link al suo profilo GitHub.

## Cosa succede adesso

Sulla scia di questo lavoro abbiamo aperto delle attività simili, adatte anche a chi è alle prime armi, per portare allo stesso livello anche la **CLI di Openapi®** (lo strumento a riga di comando): [issue aperte su openapi-cli](https://github.com/openapi/openapi-cli/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22). È un buon invito all'azione per attirare nuovi volontari.

## Link utili

- **La modifica tecnica (Pull Request #14):** https://github.com/openapi/openapi-rust-sdk/pull/14
- **L'annuncio ufficiale nelle Discussions:** https://github.com/orgs/openapi/discussions/86
- **Il repository dell'SDK Rust:** https://github.com/openapi/openapi-rust-sdk
- **Il profilo di Luca Cappelletti:** https://github.com/LucaCappelletti94
- **Registro dei contributor Openapi®:** https://github.com/openapi/contributors
- **Attività aperte per nuovi volontari (CLI):** https://github.com/openapi/openapi-cli/issues

## Spunti per il titolo

- "Openapi® passa a Rust 2024: più sicurezza e più futuro per il nostro SDK"
- "Grazie alla community, l'SDK Rust di Openapi® si aggiorna a Rust 2024"
- "Open source che funziona: un contributor porta Openapi® su Rust 2024"

## Cosa evitare

- Non promettere nuove funzionalità: questo aggiornamento riguarda sicurezza, stabilità e basi solide, non novità visibili.
- Non dire che gli utenti "erano a rischio": le vulnerabilità riguardavano l'ambiente di sviluppo del progetto e sono state risolte in via preventiva.
