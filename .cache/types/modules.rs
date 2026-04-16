use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Modules {
  pub lernziele: Option<String>,
  pub methoden: Option<String>,
  pub orte: Option<Vec<ModulesOrte>>,
  pub zustand: String,
  pub kurse: Option<Vec<ModulesKurse>>,
  pub id: i64,
  pub inhalt: Option<String>,
  pub sprache: String,
  pub nachfolger: Option<ModulesNachfolger>,
  pub durchfuehrungen: Option<ModulesDurchfuehrungen>,
  pub kuerzel: String,
  pub pruefung: Option<Vec<ModulesPruefung>>,
  pub uebersetzungen: Option<Vec<ModulesUebersetzungen>>,
  pub dozenten: Option<Vec<ModulesDozenten>>,
  pub skript_ablage_link: Option<String>,
  pub semester_bewertung: String,
  pub bezeichnung: String,
  pub zuordnungen: Option<Vec<ModulesZuordnungen>>,
  pub kreditpunkte: Option<i64>,
  pub anschlussmodule: Option<Vec<ModulesAnschlussmodule>>,
  pub empfehlungen: Option<Vec<ModulesEmpfehlungen>>,
  pub vorgaenger: Option<ModulesVorgaenger>,
  pub mengen: Option<Vec<ModulesMengen>>,
  pub voraussetzungen: Option<Vec<ModulesVoraussetzungen>>,
  pub vorausg_kenntnisse: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesOrte {
  pub kuerzel: String,
  pub ort: String,
  pub id: i64,
  pub bezeichnung: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesKurse {
  pub uebersetzungen: Option<Vec<ModulesKurseUebersetzungen>>,
  pub empf_lehrmittel: Option<String>,
  pub kurselemente: Option<Vec<ModulesKurseKurselemente>>,
  pub bezeichnung: String,
  pub id: i64,
  pub lernziele: Option<String>,
  pub kreditpunkte: Option<f64>,
  pub planinhalt: Option<String>,
  pub kuerzel: String,
  pub dozenten: Option<Vec<ModulesKurseDozenten>>,
  pub leistungsnachweis: Option<String>,
  pub bemerkung_kurs: Option<String>,
  pub bibliographie: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesKurseUebersetzungen {
  pub bezeichnung: String,
  pub id: i64,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesKurseKurselemente {
  pub anz_lektionen: Option<f64>,
  pub id: i64,
  pub harte_grenze: Option<bool>,
  pub max_teilnehmer: i64,
  pub art: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesKurseDozenten {
  pub name: String,
  pub id: i64,
  pub vorname: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesNachfolger {
  pub url: String,
  pub kuerzel: String,
  pub id: i64,
  pub bezeichnung: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesDurchfuehrungen {
  pub begin_semester: String,
  pub begin_jahr: i64,
  pub end_semester: String,
  pub count: Option<i64>,
  pub end_jahr: i64,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesPruefung {
  pub bemerkung_pruefung: Option<String>,
  pub zulassungs_bedingung: Option<String>,
  pub pruefung_mue: bool,
  pub prf_abmeldbar: bool,
  pub id: i64,
  pub art: String,
  pub pruefung_schr: bool,
  pub dauer_prf_mue: Option<i64>,
  pub dauer_prf_schr: Option<i64>,
  pub zulassung: bool,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesUebersetzungen {
  pub sprache: String,
  pub bezeichnung: String,
  pub id: i64,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesDozenten {
  pub name: String,
  pub id: i64,
  pub vorname: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesZuordnungen {
  pub ist_pflichtmodul: bool,
  pub ist_abschluss_arbeit: bool,
  pub typ: String,
  pub kuerzel: String,
  pub bezeichnung: String,
  pub kategorien: Option<Vec<ModulesZuordnungenKategorien>>,
  pub sem_empfehlung: i64,
  pub id: i64,
  pub url: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesZuordnungenKategorien {
  pub bezeichnung: String,
  pub kreditpunkte: i64,
  pub id: i64,
  pub kuerzel: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesAnschlussmodule {
  pub url: String,
  pub bezeichnung: String,
  pub id: i64,
  pub kuerzel: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesEmpfehlungen {
  pub bezeichnung: String,
  pub kuerzel: String,
  pub url: String,
  pub id: i64,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesVorgaenger {
  pub id: i64,
  pub kuerzel: String,
  pub bezeichnung: String,
  pub url: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesMengen {
  pub name: String,
  pub id: i64,
  pub d_mod_menge_art: i64,
  pub module: Vec<ModulesMengenModule>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesMengenModule {
  pub id: i64,
  pub url: String,
  pub kuerzel: String,
  pub bezeichnung: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesVoraussetzungen {
  pub bezeichnung: String,
  pub kuerzel: String,
  pub url: String,
  pub id: i64,
}
