type NOpt<N,P> = {
  name: N,
  path: P,
}
type Opt<P> = {
  name: P,
  path: P,
}

type ModuleFilter = {
  orte: NOpt<["orte","bezeichnung"], ["orte", "id"]>,
  kreditpunkte: Opt<["kreditpunkte"]>,
  jahr: Opt<["durchfuehrungen", ["begin_jahr", "end_jahr"]]>,
  semester: Opt<["durchfuehrungen", ["begin_semester", "end_semester"]]>,
  studiengang: NOpt<["zuordnungen", "bezeichnung"], ["zuordnungen", "kuerzel"]>, // kuerzel / id?
  ist_abschluss_arbeit: Opt<["zuordnungen", "ist_abschluss_arbeit"]>,
  sem_empfehlung: Opt<["zuordnungen", "sem_empfehlung"]>,
  ist_pflichtmodul: Opt<["zuordnungen", "ist_pflichtmodul"]>,
  voraussetzungen: NOpt<["voraussetzungen", "bezeichnung"], ["voraussetzungen", "kuerzel"]>,
  zustand: Opt<["zustand"]>,
  semester_bewertung: Opt<["semester_bewertung"]>,
  vorgaenger: NOpt<["vorgaenger", "bezeichnung"], ["vorgaenger", "kuerzel"]>,
  sprache: Opt<["sprache"]>,
  nachfolger: NOpt<["nachfolger", "bezeichnung"], ["nachfolger", "kuerzel"]>,
}
