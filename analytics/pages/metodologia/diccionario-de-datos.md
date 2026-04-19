# Diccionario de Datos y Referencia Técnica

Este documento sirve como referencia para la interpretación de las variables y el esquema físico de la base de datos analítica procesada a partir del CENAGRO 2011.

---

## Mapeo de Categorías (Variables VARCHAR / ENUM original)

Las siguientes variables fueron transformadas de códigos numéricos a etiquetas categóricas mediante la codificación de diccionario de Parquet para facilitar el análisis semántico.

### Actividad Principal (`principal_activity`)
* **1**: autoconsumo
* **2**: mercado interno
* **3**: exportacion_tradicional
* **4**: exportacion_no_tradicional
* **5**: otros agricolas
* **6**: ganaderia leche
* **7**: ganaderia_carne
* **8**: doble_proposito
* **9**: ganaderia_menor
* **10**: crianza
* **11**: otros_pecuarios
* **12**: acuicola
* **13**: forestal
* **14**: apicola
* **15**: abandono inactiva
* **99**: ignorado

### Estructura Operacional de la EA (`farm_operational_structure`)
* **1**: individual
* **2**: cooperativa
* **3**: colectivo familiar
* **4**: empresa
* **5**: comunidad_indigena
* **6**: administracion publica
* **7**: otra
* **9**: ignorado

### Género del Productor (`producer_gender`)
* **1**: hombre
* **2**: mujer
* **9**: ignorado

---

## Esquema de la Tabla: `farms`

Contiene los datos generales de las explotaciones agropecuarias, incluyendo demografía, métricas laborales, tecnología y matrices de crédito.

### Clave Compuesta de Identificación
* **`department_id`** (UTINYINT): Código de departamento.
* **`municipality_id`** (USMALLINT): Código de municipio.
* **`census_segment_id`** (UINTEGER): Número de Área de supervisión (SEA).
* **`farm_id`** (USMALLINT): Número de identificación de la explotación.
* **`legal_status_id`** (UTINYINT): Identificador del estado legal.

### Características Generales y Demográficas
* **`total_area_mz`** (FLOAT): Área total en manzanas.
* **`total_area_sqm`** (FLOAT): Área total en metros cuadrados.
* **`producer_gender`** (VARCHAR): Clasificación de sexo del productor.
* **`farm_operational_structure`** (VARCHAR): Estructura legal/operacional de la EA.
* **`principal_activity`** (VARCHAR): Actividad económica principal de la explotación.

### Métricas Laborales
* **`hired_workers`** (BOOLEAN): Indica si la explotación contrató trabajadores.
* **`permanent_workers_total`** (USMALLINT): Trabajadores permanentes (6 meses o más).
* **`temporal_workers_total`** (USMALLINT): Trabajadores temporales (menos de 6 meses).

### Tecnología y Mecanización
* **`has_irrigation_system`** (BOOLEAN): Uso de sistema de riego.
* **`traction_animal`** (BOOLEAN): Uso de animales de tiro/yunta.
* **`traction_tractor`** (BOOLEAN): Uso de tractor para labores agrícolas.

### Matriz de Demanda y Acceso a Crédito
* **`requested_loan`** (BOOLEAN): Demanda global; TRUE si solicitó crédito.
* **`received_loan`** (BOOLEAN): Acceso global; TRUE si recibió crédito.
* **`has_any_loan`** (BOOLEAN): Máscara global basada en el reporte de fuentes.
* **Sectores de Solicitud (BOOLEAN):** `req_crop`, `req_livestock`, `req_forestry`, `req_aquaculture`.
* **Sectores de Recepción (BOOLEAN):** `rec_crop`, `rec_livestock`, `rec_forestry`, `rec_aquaculture`.
* **Fuentes de Financiamiento (BOOLEAN):** `loan_banco`, `loan_banco_produzcamos`, `loan_cooperativa`, `loan_comercial`, `loan_acopiador`, `loan_ong`, `loan_gobierno`, `loan_prestamista`, `loan_otro`.

---

## Esquema de la Tabla: `parcels`

Contiene el desglose detallado del aprovechamiento de la tierra.

### Clave Compuesta de Identificación
* **`department_id`** (UTINYINT)
* **`municipality_id`** (USMALLINT)
* **`census_segment_id`** (UINTEGER)
* **`farm_id`** (USMALLINT)
* **`legal_status_id`** (UTINYINT)

### Agregados Generales
* **`total_parcels`** (FLOAT): Número de subdivisiones o parcelas.
* **`total_farm_manzanas`** (FLOAT): Suma agregada de todos los usos de la tierra en la explotación.

### Atributos de Uso de Suelo (Medidos en Manzanas)
* **`mz_annual_crops`** (FLOAT): Cultivos anuales o temporales.
* **`mz_permanent_crops`** (FLOAT): Cultivos permanentes o semipermanentes.
* **`mz_cultivated_pasture`** (FLOAT): Pastos cultivados o sembrados.
* **`mz_natural_pasture`** (FLOAT): Pastos naturales.
* **`mz_forest`** (FLOAT): Bosques.
* **`mz_fallow`** (FLOAT): Tierra en descanso o tacotales.
* **`mz_infrastructure`** (FLOAT): Instalaciones y viales (casas, galeras, caminos).
* **`mz_unusable`** (FLOAT): Pantanos, pedregales, otras tierras no cultivables.

---

## Explorador de Datos Estructurados

```sql sample_farms
SELECT 
    department AS departamento, 
    municipality AS municipio, 
    farm_operational_structure AS estructura_operacional,
    principal_activity AS actividad_principal, 
    total_area_mz AS area_total_mz, 
    received_loan AS recibio_credito
FROM warehouse.sample_farms
```

<DataTable data={sample_farms}/>