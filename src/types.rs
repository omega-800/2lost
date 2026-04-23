use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Studies {
    pub version_kuerzel: String,
    pub form: Option<String>,
    pub zuordnungen: Vec<StudiesZuordnungen>, // Zuordnung
    pub spezialisierungen: Option<Vec<StudiesSpezialisierungen>>, // Studies
    pub id: i64,
    pub parent: Option<StudiesParent>, // Studies
    pub uebersetzungen: Option<Vec<StudiesUebersetzungen>>, // Uebersetzungen
    pub kredits: Option<Vec<StudiesKredits>>, // Kredits
    pub studienberater: Option<StudiesStudienberater>, // Dozent?
    pub typ: Option<String>,
    pub art: String,
    pub kuerzel: String,
    pub bezeichnung: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StudiesZuordnungen {
    pub url: String, // dup
    pub typ: String,
    pub kategorien: Option<Vec<StudiesZuordnungenKategorien>>, // Kategorie
    pub sem_empfehlung: i64,
    pub bezeichnung: String, // dup
    pub kuerzel: String, // dup
    pub ist_abschluss_arbeit: bool,
    pub ist_pflichtmodul: bool,
    pub id: i64, // Modules
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StudiesZuordnungenKategorien {
    pub id: i64,
    pub kuerzel: String,
    pub bezeichnung: String,
    pub kreditpunkte: i64,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StudiesSpezialisierungen {
    pub kuerzel: String,
    pub id: i64,
    pub bezeichnung: String,
    pub url: String,
    pub uebersetzungen: Option<Vec<StudiesSpezialisierungenUebersetzungen>>, // Uebersetzungen
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StudiesSpezialisierungenUebersetzungen {
    pub bezeichnung: String,
    pub id: i64,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StudiesParent {
    pub url: String,
    pub id: i64,
    pub bezeichnung: String,
    pub kuerzel: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StudiesUebersetzungen {
    pub bezeichnung: String,
    pub id: i64,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StudiesKredits {
    pub min_kredits: i64,
    pub id: i64,
    pub kategorien: Vec<StudiesKreditsKategorien>, // Kategorie
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StudiesKreditsKategorien {
    pub kuerzel: String,
    pub id: i64,
    pub bezeichnung: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StudiesStudienberater {
    pub vorname: String,
    pub id: i64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Modules {
    pub orte: Option<Vec<ModulesOrte>>,
    pub vorausg_kenntnisse: Option<String>, // html?
    pub lernziele: Option<String>, // html
    pub sprache: String, // [ISO-639]-[ISO-3166]
    pub methoden: Option<String>, // html?
    pub id: i64,
    pub uebersetzungen: Option<Vec<ModulesUebersetzungen>>, // Uebersetzungen
    pub nachfolger: Option<ModulesNachfolger>, // Modules
    pub skript_ablage_link: Option<String>,
    pub kurse: Option<Vec<ModulesKurse>>, 
    pub durchfuehrungen: Option<ModulesDurchfuehrungen>,
    pub mengen: Option<Vec<ModulesMengen>>,
    pub pruefung: Option<Vec<ModulesPruefung>>,
    pub zuordnungen: Option<Vec<ModulesZuordnungen>>, // Zuordnung
    pub bezeichnung: String,
    pub zustand: String,
    pub vorgaenger: Option<ModulesVorgaenger>, // Modules
    pub semester_bewertung: String,
    pub voraussetzungen: Option<Vec<ModulesVoraussetzungen>>, // Modules
    pub dozenten: Option<Vec<ModulesDozenten>>, // Dozent
    pub inhalt: Option<String>, // html
    pub kuerzel: String,
    pub empfehlungen: Option<Vec<ModulesEmpfehlungen>>, // Modules
    pub anschlussmodule: Option<Vec<ModulesAnschlussmodule>>, // Modules
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
    pub uebersetzungen: Option<Vec<ModulesKurseUebersetzungen>>, // Uebersetzungen
    pub bezeichnung: String,
    pub lernziele: Option<String>,
    pub dozenten: Option<Vec<ModulesKurseDozenten>>, // Dozent
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
    pub module: Vec<ModulesMengenModule>, // Modules
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
    pub kategorien: Option<Vec<ModulesZuordnungenKategorien>>, // Kategorie
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
    pub kuerzel: String, // TODO: 
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
