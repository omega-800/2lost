use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Modules {
  pub orte: Option<Vec<ModulesOrte>>,
  pub vorausg_kenntnisse: Option<String>,
  pub lernziele: Option<String>,
  pub sprache: String,
  pub methoden: Option<String>,
  pub id: i64,
  pub uebersetzungen: Option<Vec<ModulesUebersetzungen>>,
  pub nachfolger: Option<ModulesNachfolger>,
  pub skript_ablage_link: Option<String>,
  pub kurse: Option<Vec<ModulesKurse>>,
  pub durchfuehrungen: Option<ModulesDurchfuehrungen>,
  pub mengen: Option<Vec<ModulesMengen>>,
  pub pruefung: Option<Vec<ModulesPruefung>>,
  pub zuordnungen: Option<Vec<ModulesZuordnungen>>,
  pub bezeichnung: String,
  pub zustand: String,
  pub vorgaenger: Option<ModulesVorgaenger>,
  pub semester_bewertung: String,
  pub voraussetzungen: Option<Vec<ModulesVoraussetzungen>>,
  pub dozenten: Option<Vec<ModulesDozenten>>,
  pub inhalt: Option<String>,
  pub kuerzel: String,
  pub empfehlungen: Option<Vec<ModulesEmpfehlungen>>,
  pub anschlussmodule: Option<Vec<ModulesAnschlussmodule>>,
  pub kreditpunkte: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesOrte {
  pub id: i64,
  pub ort: String,
  pub bezeichnung: String,
  pub kuerzel: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesUebersetzungen {
  pub bezeichnung: String,
  pub id: i64,
  pub sprache: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesNachfolger {
  pub id: i64,
  pub kuerzel: String,
  pub bezeichnung: String,
  pub url: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesKurse {
  pub uebersetzungen: Option<Vec<ModulesKurseUebersetzungen>>,
  pub bezeichnung: String,
  pub lernziele: Option<String>,
  pub dozenten: Option<Vec<ModulesKurseDozenten>>,
  pub bibliographie: Option<String>,
  pub kuerzel: String,
  pub kurselemente: Option<Vec<ModulesKurseKurselemente>>,
  pub leistungsnachweis: Option<String>,
  pub kreditpunkte: Option<f64>,
  pub empf_lehrmittel: Option<String>,
  pub id: i64,
  pub planinhalt: Option<String>,
  pub bemerkung_kurs: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesKurseUebersetzungen {
  pub bezeichnung: String,
  pub id: i64,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesKurseDozenten {
  pub vorname: String,
  pub name: String,
  pub id: i64,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesKurseKurselemente {
  pub id: i64,
  pub anz_lektionen: Option<f64>,
  pub harte_grenze: Option<bool>,
  pub max_teilnehmer: i64,
  pub art: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesDurchfuehrungen {
  pub begin_semester: String,
  pub begin_jahr: i64,
  pub end_semester: String,
  pub end_jahr: i64,
  pub count: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesMengen {
  pub module: Vec<ModulesMengenModule>,
  pub d_mod_menge_art: i64,
  pub id: i64,
  pub name: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesMengenModule {
  pub id: i64,
  pub bezeichnung: String,
  pub kuerzel: String,
  pub url: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesPruefung {
  pub dauer_prf_schr: Option<i64>,
  pub art: String,
  pub zulassung: bool,
  pub zulassungs_bedingung: Option<String>,
  pub bemerkung_pruefung: Option<String>,
  pub pruefung_mue: bool,
  pub dauer_prf_mue: Option<i64>,
  pub pruefung_schr: bool,
  pub id: i64,
  pub prf_abmeldbar: bool,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesZuordnungen {
  pub ist_pflichtmodul: bool,
  pub id: i64,
  pub url: String,
  pub kategorien: Option<Vec<ModulesZuordnungenKategorien>>,
  pub bezeichnung: String,
  pub kuerzel: String,
  pub sem_empfehlung: i64,
  pub typ: String,
  pub ist_abschluss_arbeit: bool,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesZuordnungenKategorien {
  pub bezeichnung: String,
  pub kreditpunkte: i64,
  pub kuerzel: String,
  pub id: i64,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesVorgaenger {
  pub id: i64,
  pub bezeichnung: String,
  pub kuerzel: String,
  pub url: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesVoraussetzungen {
  pub bezeichnung: String,
  pub id: i64,
  pub kuerzel: String,
  pub url: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesDozenten {
  pub vorname: String,
  pub name: String,
  pub id: i64,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesEmpfehlungen {
  pub id: i64,
  pub bezeichnung: String,
  pub kuerzel: String,
  pub url: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulesAnschlussmodule {
  pub kuerzel: String,
  pub bezeichnung: String,
  pub id: i64,
  pub url: String,
}
