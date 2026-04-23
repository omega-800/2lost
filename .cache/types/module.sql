CREATE TABLE module (
  anschlussmodule REFERENCES module_module(id),
  pruefung REFERENCES module_pruefung(id),
  sprache TEXT NOT NULL,
  methoden TEXT NOT NULL,
  semester_bewertung TEXT NOT NULL,
  vorausg_kenntnisse TEXT NOT NULL,
  kurse REFERENCES module_kurse(id),
  voraussetzungen REFERENCES module_module(id),
  mengen REFERENCES module_mengen(id),
  kreditpunkte INTEGER NOT NULL,
  id INTEGER PRIMARY KEY,
  vorgaenger REFERENCES module(id),
  lernziele TEXT NOT NULL,
  dozenten REFERENCES module_dozenten(id),
  empfehlungen REFERENCES module_module(id),
  orte REFERENCES module_orte(id),
  kuerzel TEXT NOT NULL,
  inhalt TEXT NOT NULL,
  zuordnungen REFERENCES module_zuordnungen(id),
  zustand TEXT NOT NULL,
  durchfuehrungen REFERENCES durchfuehrungen(id),
  uebersetzungen REFERENCES module_uebersetzungen(id),
  bezeichnung TEXT NOT NULL,
  skript_ablage_link TEXT NOT NULL,
  nachfolger REFERENCES module(id),
);
CREATE TABLE module (
  kuerzel TEXT NOT NULL,
  id INTEGER PRIMARY KEY,
  url TEXT NOT NULL,
  bezeichnung TEXT NOT NULL,
);
CREATE TABLE module_module (
  id INTEGER PRIMARY KEY REFERENCES module(id),
  module REFERENCES module(id)
);
CREATE TABLE pruefung (
  pruefung_mue INTEGER NOT NULL,
  bemerkung_pruefung TEXT NOT NULL,
  prf_abmeldbar INTEGER NOT NULL,
  zulassung INTEGER NOT NULL,
  zulassungs_bedingung TEXT NOT NULL,
  dauer_prf_mue INTEGER NOT NULL,
  pruefung_schr INTEGER NOT NULL,
  art TEXT NOT NULL,
  dauer_prf_schr INTEGER NOT NULL,
  id INTEGER PRIMARY KEY,
);
CREATE TABLE module_pruefung (
  id INTEGER PRIMARY KEY REFERENCES pruefung(id),
  module REFERENCES module(id)
);
CREATE TABLE kurse (
  bemerkung_kurs TEXT NOT NULL,
  bezeichnung TEXT NOT NULL,
  kurselemente REFERENCES kurse_kurselemente(id),
  bibliographie TEXT NOT NULL,
  uebersetzungen REFERENCES kurse_uebersetzungen(id),
  kuerzel TEXT NOT NULL,
  leistungsnachweis TEXT NOT NULL,
  empf_lehrmittel TEXT NOT NULL,
  id INTEGER PRIMARY KEY,
  lernziele TEXT NOT NULL,
  dozenten REFERENCES kurse_dozenten(id),
  kreditpunkte REAL NOT NULL,
  planinhalt TEXT NOT NULL,
);
CREATE TABLE kurselemente (
  art TEXT NOT NULL,
  harte_grenze INTEGER NOT NULL,
  max_teilnehmer INTEGER NOT NULL,
  anz_lektionen REAL NOT NULL,
  id INTEGER PRIMARY KEY,
);
CREATE TABLE kurse_kurselemente (
  id INTEGER PRIMARY KEY REFERENCES kurselemente(id),
  kurse REFERENCES kurse(id)
);
CREATE TABLE uebersetzungen (
  bezeichnung TEXT NOT NULL,
  id INTEGER PRIMARY KEY,
);
CREATE TABLE kurse_uebersetzungen (
  id INTEGER PRIMARY KEY REFERENCES uebersetzungen(id),
  kurse REFERENCES kurse(id)
);
CREATE TABLE dozenten (
  vorname TEXT NOT NULL,
  name TEXT NOT NULL,
  id INTEGER PRIMARY KEY,
);
CREATE TABLE kurse_dozenten (
  id INTEGER PRIMARY KEY REFERENCES dozenten(id),
  kurse REFERENCES kurse(id)
);
CREATE TABLE module_kurse (
  id INTEGER PRIMARY KEY REFERENCES kurse(id),
  module REFERENCES module(id)
);
CREATE TABLE module (
  bezeichnung TEXT NOT NULL,
  url TEXT NOT NULL,
  id INTEGER PRIMARY KEY,
  kuerzel TEXT NOT NULL,
);
CREATE TABLE module_module (
  id INTEGER PRIMARY KEY REFERENCES module(id),
  module REFERENCES module(id)
);
CREATE TABLE mengen (
  name TEXT NOT NULL,
  id INTEGER PRIMARY KEY,
  module REFERENCES mengen_module(id),
  d_mod_menge_art INTEGER NOT NULL,
);
CREATE TABLE module (
  id INTEGER PRIMARY KEY,
  bezeichnung TEXT NOT NULL,
  kuerzel TEXT NOT NULL,
  url TEXT NOT NULL,
);
CREATE TABLE mengen_module (
  id INTEGER PRIMARY KEY REFERENCES module(id),
  mengen REFERENCES mengen(id)
);
CREATE TABLE module_mengen (
  id INTEGER PRIMARY KEY REFERENCES mengen(id),
  module REFERENCES module(id)
);
CREATE TABLE module (
  bezeichnung TEXT NOT NULL,
  kuerzel TEXT NOT NULL,
  id INTEGER PRIMARY KEY,
  url TEXT NOT NULL,
);
CREATE TABLE dozenten (
  name TEXT NOT NULL,
  vorname TEXT NOT NULL,
  id INTEGER PRIMARY KEY,
);
CREATE TABLE module_dozenten (
  id INTEGER PRIMARY KEY REFERENCES dozenten(id),
  module REFERENCES module(id)
);
CREATE TABLE module (
  url TEXT NOT NULL,
  id INTEGER PRIMARY KEY,
  bezeichnung TEXT NOT NULL,
  kuerzel TEXT NOT NULL,
);
CREATE TABLE module_module (
  id INTEGER PRIMARY KEY REFERENCES module(id),
  module REFERENCES module(id)
);
CREATE TABLE orte (
  bezeichnung TEXT NOT NULL,
  id INTEGER PRIMARY KEY,
  kuerzel TEXT NOT NULL,
  ort TEXT NOT NULL,
);
CREATE TABLE module_orte (
  id INTEGER PRIMARY KEY REFERENCES orte(id),
  module REFERENCES module(id)
);
CREATE TABLE zuordnungen (
  sem_empfehlung INTEGER NOT NULL,
  ist_abschluss_arbeit INTEGER NOT NULL,
  typ TEXT NOT NULL,
  ist_pflichtmodul INTEGER NOT NULL,
  bezeichnung TEXT NOT NULL,
  kuerzel TEXT NOT NULL,
  kategorien REFERENCES zuordnungen_kategorien(id),
  url TEXT NOT NULL,
  id INTEGER PRIMARY KEY,
);
CREATE TABLE kategorien (
  bezeichnung TEXT NOT NULL,
  id INTEGER PRIMARY KEY,
  kuerzel TEXT NOT NULL,
  kreditpunkte INTEGER NOT NULL,
);
CREATE TABLE zuordnungen_kategorien (
  id INTEGER PRIMARY KEY REFERENCES kategorien(id),
  zuordnungen REFERENCES zuordnungen(id)
);
CREATE TABLE module_zuordnungen (
  id INTEGER PRIMARY KEY REFERENCES zuordnungen(id),
  module REFERENCES module(id)
);
CREATE TABLE durchfuehrungen (
  end_semester TEXT NOT NULL,
  begin_jahr INTEGER NOT NULL,
  begin_semester TEXT NOT NULL,
  end_jahr INTEGER NOT NULL,
  count INTEGER NOT NULL,
  id INTEGER PRIMARY KEY,
);
CREATE TABLE uebersetzungen (
  bezeichnung TEXT NOT NULL,
  id INTEGER PRIMARY KEY,
  sprache TEXT NOT NULL,
);
CREATE TABLE module_uebersetzungen (
  id INTEGER PRIMARY KEY REFERENCES uebersetzungen(id),
  module REFERENCES module(id)
);
CREATE TABLE module (
  url TEXT NOT NULL,
  bezeichnung TEXT NOT NULL,
  id INTEGER PRIMARY KEY,
  kuerzel TEXT NOT NULL,
);
