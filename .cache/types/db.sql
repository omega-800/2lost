CREATE TABLE studien (
  typ TEXT,
  id INTEGER PRIMARY KEY NOT NULL,
  version_kuerzel TEXT,
  studienberater REFERENCES dozenten(id),
  url TEXT,
  kredits REFERENCES studien_kredits(id),
  form TEXT,
  parent REFERENCES studien(id),
  zuordnungen REFERENCES studien_zuordnungen(id),
  uebersetzungen REFERENCES studien_uebersetzungen(id),
  kuerzel TEXT NOT NULL,
  bezeichnung TEXT NOT NULL,
  spezialisierungen REFERENCES studien_spezialisierungen(id),
  art TEXT);

CREATE TABLE studien_kredits (
  id INTEGER PRIMARY KEY REFERENCES kredits(id),
  studien REFERENCES studien(id)
);

CREATE TABLE studien_zuordnungen (
  id INTEGER PRIMARY KEY REFERENCES zuordnungen(id),
  studien REFERENCES studien(id)
);

CREATE TABLE studien_uebersetzungen (
  id INTEGER PRIMARY KEY REFERENCES uebersetzungen(id),
  studien REFERENCES studien(id)
);

CREATE TABLE studien_spezialisierungen (
  id INTEGER PRIMARY KEY REFERENCES studien(id),
  studien REFERENCES studien(id)
);

CREATE TABLE module (
  methoden TEXT,
  bezeichnung TEXT NOT NULL,
  nachfolger REFERENCES module(id),
  skript_ablage_link TEXT,
  lernziele TEXT,
  sprache TEXT,
  id INTEGER PRIMARY KEY NOT NULL,
  uebersetzungen REFERENCES module_uebersetzungen(id),
  pruefung REFERENCES module_pruefung(id),
  zustand TEXT,
  durchfuehrungen REFERENCES durchfuehrungen(id),
  zuordnungen REFERENCES module_zuordnungen(id),
  dozenten REFERENCES module_dozenten(id),
  mengen REFERENCES module_mengen(id),
  voraussetzungen REFERENCES module_voraussetzungen(id),
  url TEXT,
  vorgaenger REFERENCES module(id),
  inhalt TEXT,
  kurse REFERENCES module_kurse(id),
  vorausg_kenntnisse TEXT,
  empfehlungen REFERENCES module_empfehlungen(id),
  anschlussmodule REFERENCES module_anschlussmodule(id),
  orte REFERENCES module_orte(id),
  kreditpunkte INTEGER ,
  semester_bewertung TEXT,
  kuerzel TEXT NOT NULL);

CREATE TABLE module_uebersetzungen (
  id INTEGER PRIMARY KEY REFERENCES uebersetzungen(id),
  module REFERENCES module(id)
);

CREATE TABLE module_pruefung (
  id INTEGER PRIMARY KEY REFERENCES pruefung(id),
  module REFERENCES module(id)
);

CREATE TABLE module_zuordnungen (
  id INTEGER PRIMARY KEY REFERENCES zuordnungen(id),
  module REFERENCES module(id)
);

CREATE TABLE module_dozenten (
  id INTEGER PRIMARY KEY REFERENCES nachfolger(id),
  module REFERENCES module(id)
);

CREATE TABLE module_mengen (
  id INTEGER PRIMARY KEY REFERENCES mengen(id),
  module REFERENCES module(id)
);

CREATE TABLE module_voraussetzungen (
  id INTEGER PRIMARY KEY REFERENCES module(id),
  module REFERENCES module(id)
);

CREATE TABLE module_kurse (
  id INTEGER PRIMARY KEY REFERENCES kurse(id),
  module REFERENCES module(id)
);

CREATE TABLE module_empfehlungen (
  id INTEGER PRIMARY KEY REFERENCES module(id),
  module REFERENCES module(id)
);

CREATE TABLE module_anschlussmodule (
  id INTEGER PRIMARY KEY REFERENCES module(id),
  module REFERENCES module(id)
);

CREATE TABLE module_orte (
  id INTEGER PRIMARY KEY REFERENCES orte(id),
  module REFERENCES module(id)
);

CREATE TABLE kurse (
  kuerzel TEXT NOT NULL,
  lernziele TEXT,
  id INTEGER PRIMARY KEY NOT NULL,
  bezeichnung TEXT NOT NULL,
  empf_lehrmittel TEXT,
  bibliographie TEXT,
  leistungsnachweis TEXT,
  uebersetzungen REFERENCES kurse_uebersetzungen(id),
  kurselemente REFERENCES kurse_kurselemente(id),
  dozenten REFERENCES kurse_dozenten(id),
  planinhalt TEXT,
  bemerkung_kurs TEXT,
  kreditpunkte REAL);

CREATE TABLE kurse_uebersetzungen (
  id INTEGER PRIMARY KEY REFERENCES uebersetzungen(id),
  kurse REFERENCES kurse(id)
);

CREATE TABLE kurse_kurselemente (
  id INTEGER PRIMARY KEY REFERENCES kurselemente(id),
  kurse REFERENCES kurse(id)
);

CREATE TABLE kurse_dozenten (
  id INTEGER PRIMARY KEY REFERENCES nachfolger(id),
  kurse REFERENCES kurse(id)
);

CREATE TABLE kategorien (
  kreditpunkte INTEGER ,
  kuerzel TEXT NOT NULL,
  id INTEGER PRIMARY KEY NOT NULL,
  bezeichnung TEXT NOT NULL);

CREATE TABLE zuordnungen (
  ist_abschluss_arbeit INTEGER  NOT NULL,
  id INTEGER PRIMARY KEY NOT NULL,
  typ TEXT NOT NULL,
  ist_pflichtmodul INTEGER  NOT NULL,
  bezeichnung TEXT NOT NULL,
  url TEXT NOT NULL,
  kuerzel TEXT NOT NULL,
  kategorien REFERENCES zuordnungen_kategorien(id),
  sem_empfehlung INTEGER  NOT NULL);

CREATE TABLE zuordnungen_kategorien (
  id INTEGER PRIMARY KEY REFERENCES kategorien(id),
  zuordnungen REFERENCES zuordnungen(id)
);

CREATE TABLE kurselemente (
  id INTEGER PRIMARY KEY NOT NULL,
  harte_grenze INTEGER ,
  anz_lektionen REAL,
  max_teilnehmer INTEGER  NOT NULL,
  art TEXT NOT NULL);

CREATE TABLE mengen (
  d_mod_menge_art INTEGER  NOT NULL,
  module REFERENCES mengen_module(id) NOT NULL,
  name TEXT NOT NULL,
  id INTEGER PRIMARY KEY NOT NULL);

CREATE TABLE mengen_module (
  id INTEGER PRIMARY KEY REFERENCES vorgaenger(id),
  mengen REFERENCES mengen(id)
);

CREATE TABLE uebersetzungen (
  bezeichnung TEXT NOT NULL,
  sprache TEXT,
  id INTEGER PRIMARY KEY NOT NULL);

CREATE TABLE durchfuehrungen (
  id INTEGER PRIMARY KEY,
  begin_semester TEXT NOT NULL,
  begin_jahr INTEGER  NOT NULL,
  count INTEGER ,
  end_semester TEXT NOT NULL,
  end_jahr INTEGER  NOT NULL);

CREATE TABLE orte (
  kuerzel TEXT NOT NULL,
  bezeichnung TEXT NOT NULL,
  ort TEXT NOT NULL,
  id INTEGER PRIMARY KEY NOT NULL);

CREATE TABLE pruefung (
  zulassungs_bedingung TEXT,
  art TEXT NOT NULL,
  pruefung_mue INTEGER  NOT NULL,
  dauer_prf_schr INTEGER ,
  dauer_prf_mue INTEGER ,
  id INTEGER PRIMARY KEY NOT NULL,
  prf_abmeldbar INTEGER  NOT NULL,
  pruefung_schr INTEGER  NOT NULL,
  zulassung INTEGER  NOT NULL,
  bemerkung_pruefung TEXT);

CREATE TABLE kredits (
  kategorien REFERENCES kredits_kategorien(id) NOT NULL,
  min_kredits INTEGER  NOT NULL,
  id INTEGER PRIMARY KEY NOT NULL);

CREATE TABLE kredits_kategorien (
  id INTEGER PRIMARY KEY REFERENCES kategorien(id),
  kredits REFERENCES kredits(id)
);

CREATE TABLE dozenten (
  id INTEGER PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  vorname TEXT NOT NULL);
