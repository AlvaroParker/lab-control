\c fingerprints
CREATE TABLE IF NOT EXISTS personas(
  nombre VARCHAR(64) NOT NULL,
  apellido_1 VARCHAR(64) NOT NULL,
  apellido_2 VARCHAR(64) NOT NULL,
  rut VARCHAR(10) NOT NULL,
  print_path VARCHAR(128) UNIQUE NOT NULL,
  correo_uai VARCHAR(64) UNIQUE NOT NULL,
  is_disabled BOOLEAN NOT NULL,
  rol VARCHAR(10) CHECK (rol IN ('alumno', 'ayudante', 'docente')) NOT NULL DEFAULT 'alumno',
  PRIMARY KEY(rut),
  CONSTRAINT no_empty_strings CHECK (
    nombre <> ''
    and apellido_1 <> ''
    and apellido_2 <> ''
    and rut <> ''
    and correo_uai<> ''
  )
);

--  ALTER TABLE registros DROP CONSTRAINT registros_motivo_check;
-- ALTER TABLE registros ADD CONSTRAINT registros_motivo_check CHECK (motivo IN ('ventana', 'investigacion', 'ramo', 'salida', 'uso libre'));
-- ALTER TABLE registros ALTER COLUMN motivo SET DEFAULT 'uso libre';
CREATE TABLE IF NOT EXISTS registros(
  id SERIAL PRIMARY KEY,
  rut VARCHAR(10) NOT NULL,
  FOREIGN KEY (rut) REFERENCES personas(rut) ON DELETE CASCADE ON UPDATE CASCADE,
  fecha TIMESTAMP WITH TIME ZONE NOT NULL,
  salida BOOLEAN NOT NULL,
  motivo VARCHAR(20) NOT NULL DEFAULT 'Uso libre'
);

CREATE TABLE IF NOT EXISTS motivos(
  id SERIAL PRIMARY KEY,
  motivo VARCHAR(128) UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS admins(
  nombre VARCHAR(64) NOT NULL,
  apellido_1 VARCHAR(64) NOT NULL,
  apellido_2 VARCHAR(64) NOT NULL,
  email VARCHAR(64) NOT NULL,
  pswd VARCHAR(128) NOT NULL,
  PRIMARY KEY(email)
);

INSERT INTO admins (
  nombre,
  apellido_1,
  apellido_2,
  email,
  pswd
) VALUES
(
  'Alvaro',
  'Parker',
  'Del Fierro',
  'aparkerdf@gmail.com',
  -- Hashed Password for Testing purpposes: 'admin'
  '$2b$10$Siy.4RyYygdYblLB1Pg1reQerp3VxQGhi1fp/owXg3qmpRr4/8/DS' 
);

