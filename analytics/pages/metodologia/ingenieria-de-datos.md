```sql global_summary
SELECT 
    total_farms,    
FROM global_summary
```

# Ingeniería de Datos: Metodología y Arquitectura

Este documento técnico detalla la infraestructura, el diseño del pipeline y las decisiones de ingeniería tomadas para procesar los **{fmt(global_summary[0].total_farms, 'num0')}** registros del CENAGRO 2011 de manera eficiente y reproducible.

---

## Arquitectura y Entorno de Ejecución

El procesamiento se basa en una arquitectura desacoplada y determinista, donde la base de datos original se trata como un artefacto estático e inmutable.

* **Infraestructura Local:** La ejecución se realiza en hardware con procesador Intel Core i7 y 16 GB de RAM, utilizando **Aurora Fedora**, una variante atómica de Linux que garantiza la estabilidad del sistema base.
* **Contenerización con Podman:** Todo el entorno de desarrollo y compilación está orquestado mediante **Podman**. Se seleccionó esta herramienta por su arquitectura *daemonless* y su capacidad nativa de ejecución *rootless*, eliminando conflictos de dependencias y garantizando la reproducibilidad total del entorno.

---

## Pipeline ETL (Extracción, Transformación y Carga)

La canalización de datos se divide en fases especializadas para maximizar el rendimiento y minimizar el uso de memoria.

### 1. Extracción e Ingesta Cruda
Se utiliza un script aislado en **Python** con el propósito exclusivo de leer los archivos binarios propietarios de SPSS. Los datos se vuelcan inmediatamente al formato columnar **Parquet**, logrando una reducción de peso crítica (de 100 MB a 10 MB) y aislando la carga de memoria de Python del resto del flujo.

### 2. Transformación y Tipificación Estricta (Rust + Polars)
El núcleo del procesamiento se ejecuta en **Rust** utilizando el motor **Polars**.
* **Optimización de Tipos:** Los formatos de coma flotante por defecto se reducen a tipos enteros mínimos viables (UTINYINT, USMALLINT).
* **Mapeo de Categorías:** Las variables categóricas se transforman al tipo nativo **ENUM**, sustituyendo cadenas de texto por punteros numéricos y aprovechando la codificación de diccionario de Parquet.
* **Manejo de Binarios:** Las variables de crédito y empleo se convierten a tipos booleanos con manejo explícito de nulos.

### 3. Motor Analítico (DuckDB)
Para las consultas de investigación, se emplea **DuckDB**. Este motor permite realizar lecturas *zero-copy* sobre los archivos Parquet, manteniendo la integridad relacional mediante claves compuestas para los cruces (*joins*) entre las tablas de explotaciones y parcelas.

---

## Estrategia de Business Intelligence (BI)

Dado que el volumen de datos con **{fmt(global_summary[0].total_farms, 'num0')}** registros, la transmisión directa al navegador comprometería la experiencia del usuario.

* **Materialización Estática:** Se utiliza **Evidence.dev** para realizar la agregación de datos durante el tiempo de compilación.
* **Tablas de Resumen:** Las métricas principales se precomputan y se materializan en tablas de resumen. Esto evita bloqueos en el navegador, ya que los gráficos interactivos consumen únicamente los resultados procesados y no la base de datos completa.
* **Localización:** La traducción de etiquetas y formatos se gestiona exclusivamente en la capa semántica de las consultas SQL dentro de los archivos Markdown.

---

## Licencia de Datos Abiertos (Open Data)

La política de distribución de este proyecto fomenta la transparencia y el acceso abierto a la investigación.

* **Derechos de Origen:** Se reconoce explícitamente que los derechos sobre el conjunto de datos original pertenecen al **Gobierno de Nicaragua**, responsable de la recolección de datos en el CENAGRO 2011 y de su puesta a disposición pública.
* **Acceso Público:** Como parte del compromiso académico, tanto los archivos Parquet optimizados como la base de datos DuckDB resultante de esta ingeniería se publican de forma abierta para que otros investigadores puedan realizar sus propios análisis.
* **Protocolo de Atribución:** Cualquier proyecto derivado debe otorgar crédito compartido tanto a la fuente gubernamental original como al trabajo de ingeniería de datos desarrollado en este proyecto.
* **Despliegue:** El reporte y los artefactos finales se distribuyen a través de la plataforma Netlify para asegurar alta disponibilidad.