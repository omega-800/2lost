CREATE TABLE kredits (
  min_kredits INTEGER   NOT NULL,
  id INTEGER PRIMARY KEY  NOT NULL,
  kategorien REFERENCES kredits_kategorien(id)  NOT NULL,
);

CREATE TABLE kredits_kategorien (
  id INTEGER PRIMARY KEY REFERENCES kategorien(id),
  kredits REFERENCES kredits(id)
);

CREATE TABLE uebersetzungen (
  bezeichnung TEXT  NOT NULL,
  id INTEGER PRIMARY KEY  NOT NULL,
  sprache TEXT ,
);

CREATE TABLE kategorien (
  kuerzel TEXT  NOT NULL,
  id INTEGER PRIMARY KEY  NOT NULL,
  bezeichnung TEXT  NOT NULL,
  kreditpunkte INTEGER  ,
);

CREATE TABLE zuordnungen (
  url TEXT  NOT NULL,
  typ TEXT  NOT NULL,
  kategorien REFERENCES zuordnungen_kategorien(id) ,
  ist_abschluss_arbeit INTEGER   NOT NULL,
  id INTEGER PRIMARY KEY  NOT NULL,
  sem_empfehlung INTEGER   NOT NULL,
  ist_pflichtmodul INTEGER   NOT NULL,
  kuerzel TEXT  NOT NULL,
  bezeichnung TEXT  NOT NULL,
);

CREATE TABLE zuordnungen_kategorien (
  id INTEGER PRIMARY KEY REFERENCES kategorien(id),
  zuordnungen REFERENCES zuordnungen(id)
);

CREATE TABLE module (
  empfehlungen REFERENCES module_empfehlungen(id) ,
  nachfolger REFERENCES module(id) ,
  zuordnungen REFERENCES module_zuordnungen(id) ,
  mengen REFERENCES module_mengen(id) ,
  durchfuehrungen REFERENCES durchfuehrungen(id) ,
  skript_ablage_link TEXT ,
  dozenten REFERENCES module_dozenten(id) ,
  pruefung REFERENCES module_pruefung(id) ,
  sprache TEXT ,
  vorausg_kenntnisse TEXT ,
  kurse REFERENCES module_kurse(id) ,
  anschlussmodule REFERENCES module_anschlussmodule(id) ,
  uebersetzungen REFERENCES module_uebersetzungen(id) ,
  kuerzel TEXT  NOT NULL,
  inhalt TEXT ,
  bezeichnung TEXT  NOT NULL,
  vorgaenger REFERENCES module(id) ,
  id INTEGER PRIMARY KEY  NOT NULL,
  url TEXT ,
  lernziele TEXT ,
  kreditpunkte INTEGER  ,
  voraussetzungen REFERENCES module_voraussetzungen(id) ,
  orte REFERENCES module_orte(id) ,
  semester_bewertung TEXT ,
  zustand TEXT ,
  methoden TEXT ,
);

CREATE TABLE module_empfehlungen (
  id INTEGER PRIMARY KEY REFERENCES module(id),
  module REFERENCES module(id)
);

CREATE TABLE module_zuordnungen (
  id INTEGER PRIMARY KEY REFERENCES zuordnungen(id),
  module REFERENCES module(id)
);

CREATE TABLE module_mengen (
  id INTEGER PRIMARY KEY REFERENCES mengen(id),
  module REFERENCES module(id)
);

CREATE TABLE module_dozenten (
  id INTEGER PRIMARY KEY REFERENCES nachfolger(id),
  module REFERENCES module(id)
);

CREATE TABLE module_pruefung (
  id INTEGER PRIMARY KEY REFERENCES pruefung(id),
  module REFERENCES module(id)
);

CREATE TABLE module_kurse (
  id INTEGER PRIMARY KEY REFERENCES kurse(id),
  module REFERENCES module(id)
);

CREATE TABLE module_anschlussmodule (
  id INTEGER PRIMARY KEY REFERENCES module(id),
  module REFERENCES module(id)
);

CREATE TABLE module_uebersetzungen (
  id INTEGER PRIMARY KEY REFERENCES uebersetzungen(id),
  module REFERENCES module(id)
);

CREATE TABLE module_voraussetzungen (
  id INTEGER PRIMARY KEY REFERENCES module(id),
  module REFERENCES module(id)
);

CREATE TABLE module_orte (
  id INTEGER PRIMARY KEY REFERENCES orte(id),
  module REFERENCES module(id)
);

CREATE TABLE mengen (
  d_mod_menge_art INTEGER   NOT NULL,
  id INTEGER PRIMARY KEY  NOT NULL,
  name TEXT  NOT NULL,
  module REFERENCES mengen_module(id)  NOT NULL,
);

CREATE TABLE mengen_module (
  id INTEGER PRIMARY KEY REFERENCES vorgaenger(id),
  mengen REFERENCES mengen(id)
);

CREATE TABLE dozenten (
  name TEXT  NOT NULL,
  vorname TEXT  NOT NULL,
  id INTEGER PRIMARY KEY  NOT NULL,
);

CREATE TABLE kurse (
  dozenten REFERENCES kurse_dozenten(id) ,
  bibliographie TEXT ,
  empf_lehrmittel TEXT ,
  kuerzel TEXT  NOT NULL,
  uebersetzungen REFERENCES kurse_uebersetzungen(id) ,
  kurselemente REFERENCES kurse_kurselemente(id) ,
  lernziele TEXT ,
  bezeichnung TEXT  NOT NULL,
  leistungsnachweis TEXT ,
  bemerkung_kurs TEXT ,
  id INTEGER PRIMARY KEY  NOT NULL,
  planinhalt TEXT ,
  kreditpunkte REAL ,
);

CREATE TABLE kurse_dozenten (
  id INTEGER PRIMARY KEY REFERENCES nachfolger(id),
  kurse REFERENCES kurse(id)
);

CREATE TABLE kurse_uebersetzungen (
  id INTEGER PRIMARY KEY REFERENCES uebersetzungen(id),
  kurse REFERENCES kurse(id)
);

CREATE TABLE kurse_kurselemente (
  id INTEGER PRIMARY KEY REFERENCES kurselemente(id),
  kurse REFERENCES kurse(id)
);

CREATE TABLE durchfuehrungen (
  begin_jahr INTEGER   NOT NULL,
  begin_semester TEXT  NOT NULL,
  count INTEGER  ,
  end_jahr INTEGER   NOT NULL,
  end_semester TEXT  NOT NULL,
  id INTEGER PRIMARY KEY,
);

CREATE TABLE kurselemente (
  art TEXT  NOT NULL,
  max_teilnehmer INTEGER   NOT NULL,
  id INTEGER PRIMARY KEY  NOT NULL,
  anz_lektionen REAL ,
  harte_grenze INTEGER  ,
);

CREATE TABLE pruefung (
  zulassung INTEGER   NOT NULL,
  id INTEGER PRIMARY KEY  NOT NULL,
  zulassungs_bedingung TEXT ,
  dauer_prf_mue INTEGER  ,
  bemerkung_pruefung TEXT ,
  pruefung_schr INTEGER   NOT NULL,
  art TEXT  NOT NULL,
  pruefung_mue INTEGER   NOT NULL,
  prf_abmeldbar INTEGER   NOT NULL,
  dauer_prf_schr INTEGER  ,
);

CREATE TABLE studien (
  id INTEGER PRIMARY KEY  NOT NULL,
  art TEXT ,
  kuerzel TEXT  NOT NULL,
  parent REFERENCES studien(id) ,
  studienberater REFERENCES dozenten(id) ,
  zuordnungen REFERENCES studien_zuordnungen(id) ,
  bezeichnung TEXT  NOT NULL,
  url TEXT ,
  typ TEXT ,
  spezialisierungen REFERENCES studien_spezialisierungen(id) ,
  kredits REFERENCES studien_kredits(id) ,
  form TEXT ,
  uebersetzungen REFERENCES studien_uebersetzungen(id) ,
  version_kuerzel TEXT ,
);

CREATE TABLE studien_zuordnungen (
  id INTEGER PRIMARY KEY REFERENCES zuordnungen(id),
  studien REFERENCES studien(id)
);

CREATE TABLE studien_spezialisierungen (
  id INTEGER PRIMARY KEY REFERENCES studien(id),
  studien REFERENCES studien(id)
);

CREATE TABLE studien_kredits (
  id INTEGER PRIMARY KEY REFERENCES kredits(id),
  studien REFERENCES studien(id)
);

CREATE TABLE studien_uebersetzungen (
  id INTEGER PRIMARY KEY REFERENCES uebersetzungen(id),
  studien REFERENCES studien(id)
);

CREATE TABLE orte (
  bezeichnung TEXT  NOT NULL,
  ort TEXT  NOT NULL,
  kuerzel TEXT  NOT NULL,
  id INTEGER PRIMARY KEY  NOT NULL,
);
