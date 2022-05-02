use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize, Debug)]
pub struct StatoProntoSoccorsi {
    pub risposte: Vec<Risposta>,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct Risposta {
    timestamp: String,
    pronto_soccorso: ProntoSoccorso,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct ProntoSoccorso {
    reparto: Reparto,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct Reparto {
    descrizione: String,
    attesa: Stato,
    ambulatorio: Stato,
    osservazione: Stato,
}

#[derive(Deserialize, Debug)]
pub struct Stato {
    bianco: String,
    verde: String,
    azzurro: String,
    arancio: String,
    giallo: String,
    rosso: String,
}
