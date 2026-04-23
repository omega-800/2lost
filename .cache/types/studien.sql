CREATE TABLE studien (
  kuerzel TEXT NOT NULL,
  uebersetzungen REFERENCES studien_uebersetzungen(id),
  version_kuerzel TEXT NOT NULL,
  studienberater REFERENCES dozenten(id),
  parent REFERENCES studien(id),
  spezialisierungen REFERENCES studien_studien(id),
  art TEXT NOT NULL,
  typ TEXT NOT NULL,
  zuordnungen REFERENCES studien_zuordnungen(id),
  kredits REFERENCES studien_kredits(id),
  bezeichnung TEXT NOT NULL,
  id INTEGER PRIMARY KEY,
  form TEXT NOT NULL,
);
CREATE TABLE uebersetzungen (
  id INTEGER PRIMARY KEY,
  bezeichnung TEXT NOT NULL,
);
CREATE TABLE studien_uebersetzungen (
  id INTEGER PRIMARY KEY REFERENCES uebersetzungen(id),
  studien REFERENCES studien(id)
);
CREATE TABLE dozenten (
  vorname TEXT NOT NULL,
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
);
CREATE TABLE studien (
  url TEXT NOT NULL,
  kuerzel TEXT NOT NULL,
  bezeichnung TEXT NOT NULL,
  id INTEGER PRIMARY KEY,
);
CREATE TABLE studien (
  url TEXT NOT NULL,
  id INTEGER PRIMARY KEY,
  uebersetzungen REFERENCES studien_uebersetzungen(id),
  bezeichnung TEXT NOT NULL,
  kuerzel TEXT NOT NULL,
);
CREATE TABLE uebersetzungen (
  bezeichnung TEXT NOT NULL,
  id INTEGER PRIMARY KEY,
);
CREATE TABLE studien_uebersetzungen (
  id INTEGER PRIMARY KEY REFERENCES uebersetzungen(id),
  studien REFERENCES studien(id)
);
CREATE TABLE studien_studien (
  id INTEGER PRIMARY KEY REFERENCES studien(id),
  studien REFERENCES studien(id)
);
CREATE TABLE zuordnungen (
  ist_abschluss_arbeit INTEGER NOT NULL,
  bezeichnung TEXT NOT NULL,
  url TEXT NOT NULL,
  kuerzel TEXT NOT NULL,
  sem_empfehlung INTEGER NOT NULL,
  kategorien REFERENCES zuordnungen_kategorien(id),
  ist_pflichtmodul INTEGER NOT NULL,
  id INTEGER PRIMARY KEY,
  typ TEXT NOT NULL,
);
CREATE TABLE kategorien (
  kreditpunkte INTEGER NOT NULL,
  bezeichnung TEXT NOT NULL,
  id INTEGER PRIMARY KEY,
  kuerzel TEXT NOT NULL,
);
CREATE TABLE zuordnungen_kategorien (
  id INTEGER PRIMARY KEY REFERENCES kategorien(id),
  zuordnungen REFERENCES zuordnungen(id)
);
CREATE TABLE studien_zuordnungen (
  id INTEGER PRIMARY KEY REFERENCES zuordnungen(id),
  studien REFERENCES studien(id)
);
CREATE TABLE kredits (
  kategorien REFERENCES kredits_kategorien(id),
  min_kredits INTEGER NOT NULL,
  id INTEGER PRIMARY KEY,
);
CREATE TABLE kategorien (
  id INTEGER PRIMARY KEY,
  kuerzel TEXT NOT NULL,
  bezeichnung TEXT NOT NULL,
);
CREATE TABLE kredits_kategorien (
  id INTEGER PRIMARY KEY REFERENCES kategorien(id),
  kredits REFERENCES kredits(id)
);
CREATE TABLE studien_kredits (
  id INTEGER PRIMARY KEY REFERENCES kredits(id),
  studien REFERENCES studien(id)
);
