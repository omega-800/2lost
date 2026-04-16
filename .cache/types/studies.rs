use serde::{Deserialize, Serialize};

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
struct Zuordnungen {
  typ: String,
  bezeichnung: String,
  sem_empfehlung: i64,
  ist_pflichtmodul: bool,
  id: i64,
  url: String,
  kategorien: Option<Vec<Kategorien>>,
  kuerzel: String,
  ist_abschluss_arbeit: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Kategorien {
  kreditpunkte: i64,
  kuerzel: String,
  bezeichnung: String,
  id: i64,
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
struct Uebersetzungen {
  bezeichnung: String,
  id: i64,
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
struct Kategorien {
  bezeichnung: String,
  id: i64,
  kuerzel: String,
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
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Uebersetzungen {
  bezeichnung: String,
  id: i64,
}
