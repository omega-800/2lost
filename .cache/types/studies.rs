use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Studies {
  pub form: Option<String>,
  pub id: i64,
  pub art: String,
  pub kredits: Option<Vec<StudiesKredits>>,
  pub typ: Option<String>,
  pub uebersetzungen: Option<Vec<StudiesUebersetzungen>>,
  pub bezeichnung: String,
  pub studienberater: Option<StudiesStudienberater>,
  pub parent: Option<StudiesParent>,
  pub zuordnungen: Vec<StudiesZuordnungen>,
  pub kuerzel: String,
  pub version_kuerzel: String,
  pub spezialisierungen: Option<Vec<StudiesSpezialisierungen>>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StudiesKredits {
  pub min_kredits: i64,
  pub id: i64,
  pub kategorien: Vec<StudiesKreditsKategorien>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StudiesKreditsKategorien {
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
pub struct StudiesStudienberater {
  pub vorname: String,
  pub id: i64,
  pub name: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StudiesParent {
  pub kuerzel: String,
  pub bezeichnung: String,
  pub id: i64,
  pub url: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StudiesZuordnungen {
  pub sem_empfehlung: i64,
  pub kuerzel: String,
  pub url: String,
  pub ist_pflichtmodul: bool,
  pub typ: String,
  pub bezeichnung: String,
  pub id: i64,
  pub ist_abschluss_arbeit: bool,
  pub kategorien: Option<Vec<StudiesZuordnungenKategorien>>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StudiesZuordnungenKategorien {
  pub kreditpunkte: i64,
  pub bezeichnung: String,
  pub id: i64,
  pub kuerzel: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StudiesSpezialisierungen {
  pub url: String,
  pub id: i64,
  pub bezeichnung: String,
  pub uebersetzungen: Option<Vec<StudiesSpezialisierungenUebersetzungen>>,
  pub kuerzel: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StudiesSpezialisierungenUebersetzungen {
  pub bezeichnung: String,
  pub id: i64,
}
