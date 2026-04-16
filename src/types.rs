use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Modules {
  id: i64,
  empfehlungen: Option<Vec<Empfehlungen>>,
  bezeichnung: String,
  lernziele: Option<String>,
  inhalt: Option<String>,
  voraussetzungen: Option<Vec<Voraussetzungen>>,
  durchfuehrungen: Option<Durchfuehrungen>,
  mengen: Option<Vec<Mengen>>,
  sprache: String,
  kreditpunkte: Option<i64>,
  kurse: Option<Vec<Kurse>>,
  vorausg_kenntnisse: Option<String>,
  kuerzel: String,
  skript_ablage_link: Option<String>,
  semester_bewertung: String,
  methoden: Option<String>,
  pruefung: Option<Vec<Pruefung>>,
  anschlussmodule: Option<Vec<Anschlussmodule>>,
  uebersetzungen: Option<Vec<Uebersetzungen>>,
  nachfolger: Option<Nachfolger>,
  zustand: String,
  vorgaenger: Option<Vorgaenger>,
  orte: Option<Vec<Orte>>,
  dozenten: Option<Vec<Dozenten>>,
  zuordnungen: Option<Vec<Zuordnungen>>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Empfehlungen {
  kuerzel: String,
  url: String,
  id: i64,
  bezeichnung: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Voraussetzungen {
  bezeichnung: String,
  url: String,
  id: i64,
  kuerzel: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Durchfuehrungen {
  end_semester: String,
  begin_jahr: i64,
  end_jahr: i64,
  begin_semester: String,
  count: Option<i64>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Mengen {
  module: Vec<Module>,
  id: i64,
  d_mod_menge_art: i64,
  name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Module {
  bezeichnung: String,
  id: i64,
  url: String,
  kuerzel: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Kurse {
  kurselemente: Option<Vec<Kurselemente>>,
  leistungsnachweis: Option<String>,
  kreditpunkte: Option<f64>,
  bemerkung_kurs: Option<String>,
  dozenten: Option<Vec<Dozenten>>,
  uebersetzungen: Option<Vec<Uebersetzungen>>,
  planinhalt: Option<String>,
  kuerzel: String,
  empf_lehrmittel: Option<String>,
  bibliographie: Option<String>,
  bezeichnung: String,
  id: i64,
  lernziele: Option<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Kurselemente {
  id: i64,
  art: String,
  max_teilnehmer: i64,
  harte_grenze: Option<bool>,
  anz_lektionen: Option<f64>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Dozenten {
  id: i64,
  name: String,
  vorname: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Pruefung {
  prf_abmeldbar: bool,
  dauer_prf_mue: Option<i64>,
  id: i64,
  pruefung_mue: bool,
  art: String,
  pruefung_schr: bool,
  bemerkung_pruefung: Option<String>,
  dauer_prf_schr: Option<i64>,
  zulassung: bool,
  zulassungs_bedingung: Option<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Anschlussmodule {
  bezeichnung: String,
  id: i64,
  kuerzel: String,
  url: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Uebersetzungen {
  id: i64,
  bezeichnung: String,
  sprache: Option<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Nachfolger {
  kuerzel: String,
  url: String,
  bezeichnung: String,
  id: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Vorgaenger {
  kuerzel: String,
  id: i64,
  url: String,
  bezeichnung: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Orte {
  bezeichnung: String,
  id: i64,
  ort: String,
  kuerzel: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Zuordnungen {
  typ: String,
  url: String,
  kuerzel: String,
  id: i64,
  ist_abschluss_arbeit: bool,
  sem_empfehlung: i64,
  ist_pflichtmodul: bool,
  bezeichnung: String,
  kategorien: Option<Vec<Kategorien>>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Kategorien {
  kuerzel: String,
  bezeichnung: String,
  id: i64,
  kreditpunkte: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Studies {
  art: String,
  bezeichnung: String,
  zuordnungen: Vec<Zuordnungen>,
  id: i64,
  studienberater: Option<Studienberater>,
  typ: Option<String>,
  kuerzel: String,
  parent: Option<Parent>,
  version_kuerzel: String,
  form: Option<String>,
  uebersetzungen: Option<Vec<Uebersetzungen>>,
  kredits: Option<Vec<Kredits>>,
  spezialisierungen: Option<Vec<Spezialisierungen>>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Studienberater {
  name: String,
  id: i64,
  vorname: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Parent {
  url: String,
  bezeichnung: String,
  id: i64,
  kuerzel: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Kredits {
  kategorien: Vec<Kategorien>,
  id: i64,
  min_kredits: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Spezialisierungen {
  url: String,
  uebersetzungen: Option<Vec<Uebersetzungen>>,
  bezeichnung: String,
  id: i64,
  kuerzel: String,
}
