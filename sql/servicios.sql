CREATE TABLE Servicios (
    id_servicio SERIAL PRIMARY KEY,
    descripcion_falla TEXT NOT NULL,
    precio_estimado DECIMAL(10,2)
);

INSERT INTO Servicios (descripcion_falla, precio_estimado)
VALUES
('Cambio de aceite', 25.00),
('Cambio de frenos', 80.00),
('Diagnóstico general', 15.00);
