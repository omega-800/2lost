use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Studien {
  pub typ: Option<String>,
  pub kuerzel: String,
  pub zuordnungen: Vec<StudienZuordnungen>,
  pub form: Option<String>,
  pub spezialisierungen: Option<Vec<StudienSpezialisierungen>>,
  pub version_kuerzel: String,
  pub studienberater: Option<StudienStudienberater>,
  pub id: i64,
  pub bezeichnung: String,
  pub parent: Option<StudienParent>,
  pub kredits: Option<Vec<StudienKredits>>,
  pub art: String,
  pub uebersetzungen: Option<Vec<StudienUebersetzungen>>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StudienZuordnungen {
  pub ist_abschluss_arbeit: bool,
  pub id: i64,
  pub kuerzel: String,
  pub ist_pflichtmodul: bool,
  pub url: String,
  pub sem_empfehlung: i64,
  pub kategorien: Option<Vec<StudienZuordnungenKategorien>>,
  pub bezeichnung: String,
  pub typ: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StudienZuordnungenKategorien {
  pub kreditpunkte: i64,
  pub bezeichnung: String,
  pub id: i64,
  pub kuerzel: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StudienSpezialisierungen {
  pub url: String,
  pub id: i64,
  pub kuerzel: String,
  pub uebersetzungen: Option<Vec<StudienSpezialisierungenUebersetzungen>>,
  pub bezeichnung: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StudienSpezialisierungenUebersetzungen {
  pub bezeichnung: String,
  pub id: i64,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StudienStudienberater {
  pub vorname: String,
  pub id: i64,
  pub name: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StudienParent {
  pub bezeichnung: String,
  pub kuerzel: String,
  pub url: String,
  pub id: i64,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StudienKredits {
  pub kategorien: Vec<StudienKreditsKategorien>,
  pub min_kredits: i64,
  pub id: i64,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StudienKreditsKategorien {
  pub bezeichnung: String,
  pub id: i64,
  pub kuerzel: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StudienUebersetzungen {
  pub id: i64,
  pub bezeichnung: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Module {
  pub orte: Option<Vec<ModuleOrte>>,
  pub kreditpunkte: Option<i64>,
  pub methoden: Option<String>,
  pub durchfuehrungen: Option<ModuleDurchfuehrungen>,
  pub zuordnungen: Option<Vec<ModuleZuordnungen>>,
  pub inhalt: Option<String>,
  pub skript_ablage_link: Option<String>,
  pub voraussetzungen: Option<Vec<ModuleVoraussetzungen>>,
  pub zustand: String,
  pub semester_bewertung: String,
  pub vorgaenger: Option<ModuleVorgaenger>,
  pub sprache: String,
  pub mengen: Option<Vec<ModuleMengen>>,
  pub uebersetzungen: Option<Vec<ModuleUebersetzungen>>,
  pub nachfolger: Option<ModuleNachfolger>,
  pub pruefung: Option<Vec<ModulePruefung>>,
  pub kurse: Option<Vec<ModuleKurse>>,
  pub anschlussmodule: Option<Vec<ModuleAnschlussmodule>>,
  pub id: i64,
  pub dozenten: Option<Vec<ModuleDozenten>>,
  pub vorausg_kenntnisse: Option<String>,
  pub bezeichnung: String,
  pub empfehlungen: Option<Vec<ModuleEmpfehlungen>>,
  pub kuerzel: String,
  pub lernziele: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModuleOrte {
  pub ort: String,
  pub bezeichnung: String,
  pub id: i64,
  pub kuerzel: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModuleDurchfuehrungen {
  pub count: Option<i64>,
  pub begin_semester: String,
  pub end_jahr: i64,
  pub end_semester: String,
  pub begin_jahr: i64,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModuleZuordnungen {
  pub ist_pflichtmodul: bool,
  pub kategorien: Option<Vec<ModuleZuordnungenKategorien>>,
  pub typ: String,
  pub bezeichnung: String,
  pub id: i64,
  pub ist_abschluss_arbeit: bool,
  pub url: String,
  pub sem_empfehlung: i64,
  pub kuerzel: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModuleZuordnungenKategorien {
  pub bezeichnung: String,
  pub id: i64,
  pub kreditpunkte: i64,
  pub kuerzel: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModuleVoraussetzungen {
  pub url: String,
  pub kuerzel: String,
  pub bezeichnung: String,
  pub id: i64,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModuleVorgaenger {
  pub url: String,
  pub kuerzel: String,
  pub bezeichnung: String,
  pub id: i64,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModuleMengen {
  pub d_mod_menge_art: i64,
  pub module: Vec<ModuleMengenModule>,
  pub name: String,
  pub id: i64,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModuleMengenModule {
  pub id: i64,
  pub bezeichnung: String,
  pub kuerzel: String,
  pub url: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModuleUebersetzungen {
  pub sprache: String,
  pub bezeichnung: String,
  pub id: i64,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModuleNachfolger {
  pub kuerzel: String,
  pub bezeichnung: String,
  pub id: i64,
  pub url: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModulePruefung {
  pub dauer_prf_mue: Option<i64>,
  pub pruefung_mue: bool,
  pub dauer_prf_schr: Option<i64>,
  pub id: i64,
  pub pruefung_schr: bool,
  pub bemerkung_pruefung: Option<String>,
  pub zulassung: bool,
  pub prf_abmeldbar: bool,
  pub art: String,
  pub zulassungs_bedingung: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModuleKurse {
  pub empf_lehrmittel: Option<String>,
  pub bemerkung_kurs: Option<String>,
  pub planinhalt: Option<String>,
  pub leistungsnachweis: Option<String>,
  pub kuerzel: String,
  pub bezeichnung: String,
  pub lernziele: Option<String>,
  pub dozenten: Option<Vec<ModuleKurseDozenten>>,
  pub kurselemente: Option<Vec<ModuleKurseKurselemente>>,
  pub bibliographie: Option<String>,
  pub kreditpunkte: Option<f64>,
  pub id: i64,
  pub uebersetzungen: Option<Vec<ModuleKurseUebersetzungen>>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModuleKurseDozenten {
  pub name: String,
  pub id: i64,
  pub vorname: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModuleKurseKurselemente {
  pub harte_grenze: Option<bool>,
  pub max_teilnehmer: i64,
  pub anz_lektionen: Option<f64>,
  pub id: i64,
  pub art: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModuleKurseUebersetzungen {
  pub bezeichnung: String,
  pub id: i64,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModuleAnschlussmodule {
  pub bezeichnung: String,
  pub id: i64,
  pub url: String,
  pub kuerzel: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModuleDozenten {
  pub name: String,
  pub id: i64,
  pub vorname: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModuleEmpfehlungen {
  pub url: String,
  pub bezeichnung: String,
  pub kuerzel: String,
  pub id: i64,
}
