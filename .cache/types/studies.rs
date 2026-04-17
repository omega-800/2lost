use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Studies {
  pub version_kuerzel: String,
  pub form: Option<String>,
  pub zuordnungen: Vec<StudiesZuordnungen>,
  pub spezialisierungen: Option<Vec<StudiesSpezialisierungen>>,
  pub id: i64,
  pub parent: Option<StudiesParent>,
  pub uebersetzungen: Option<Vec<StudiesUebersetzungen>>,
  pub kredits: Option<Vec<StudiesKredits>>,
  pub studienberater: Option<StudiesStudienberater>,
  pub typ: Option<String>,
  pub art: String,
  pub kuerzel: String,
  pub bezeichnung: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StudiesZuordnungen {
  pub url: String,
  pub typ: String,
  pub kategorien: Option<Vec<StudiesZuordnungenKategorien>>,
  pub sem_empfehlung: i64,
  pub bezeichnung: String,
  pub kuerzel: String,
  pub ist_abschluss_arbeit: bool,
  pub ist_pflichtmodul: bool,
  pub id: i64,
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
  pub uebersetzungen: Option<Vec<StudiesSpezialisierungenUebersetzungen>>,
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
  pub kategorien: Vec<StudiesKreditsKategorien>,
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
