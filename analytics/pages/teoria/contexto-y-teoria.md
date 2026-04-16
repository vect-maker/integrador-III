# Contexto Económico y Marco Teórico

Esta sección detalla los fundamentos que sustentan el análisis del impacto del financiamiento en el sector agropecuario nicaragüense, permitiendo una interpretación precisa de las métricas presentadas en este reporte.

---

## Justificación Económica

El acceso al financiamiento es un **catalizador crítico** para el desarrollo productivo y la mejora de las condiciones socioeconómicas en el sector rural. En Nicaragua, el crédito actúa como un mecanismo de **inclusión o exclusión productiva**:

* **Mecanismo de Inclusión:** Las explotaciones con acceso a apoyo financiero poseen mayores capacidades para invertir en tecnología, innovar en sus procesos y diversificar sus actividades. Esto se traduce potencialmente en una mayor generación de empleo y un uso más eficiente del suelo.
* **Limitaciones Estructurales:** Aquellas unidades que no logran acceder a recursos financieros enfrentan restricciones estructurales que restringen su crecimiento y competitividad, afectando su sostenibilidad económica a largo plazo.

---

## Marco Conceptual

Para asegurar la rigurosidad del análisis, se establecen las siguientes definiciones basadas en el diseño del **CENAGRO 2011**:

### 1. Financiamiento Agropecuario
Se define como la provisión de **capital líquido o en especie** para el desarrollo de actividades productivas. Se clasifica según su origen en:
* **Formal:** Bancos, microfinancieras, cooperativas y Banco Produzcamos.
* **Informal:** Prestamistas y acopiadores.

### 2. Generación de Empleo
Representa la intensidad de la fuerza laboral absorbida por la unidad productiva, dividida en dos métricas operativas:
* **Empleo Permanente:** Trabajadores contratados de manera regular por un período igual o superior a **seis meses** (ej. administradores, mandadores).
* **Empleo Temporal:** Mano de obra contratada por un tiempo fijo **menor a seis meses**, generalmente asociada a picos de demanda laboral estacional (siembra, cosecha, recolección).

### 3. Diversificación Productiva (Uso del Suelo)
Es la estrategia de asignación de tierras a una variedad de propósitos (agrícolas, pecuarios o forestales) dentro de la misma Explotación Agropecuaria (EA). Desde una perspectiva microeconómica, la diversificación reduce la vulnerabilidad ante shocks climáticos y fluctuaciones de precios, optimizando la resiliencia del ingreso rural.

---

## Métricas de Estudio

Para evaluar el impacto del financiamiento, es necesario aislar el tamaño de la finca como variable de confusión. Por ello, se establece la **Intensidad Laboral**. 

Para normalizar la generación de empleo y evitar que el tamaño de la finca sesgue los resultados, la métrica de empleo se evalúa en función del área total explotada.

### Fórmula de Intensidad Laboral (Factor Trabajo por Manzana)

```sql
-- Definición de la métrica de intensidad laboral
-- Basado en las variables censales S1068A, S1069A y S427

labor_to_land_ratio = (permanent_workers_total + temporal_workers_total) / total_area_mz
```

*Donde:*
* **`permanent_workers_total`**: Trabajadores permanentes (6 meses o más).
* **`temporal_workers_total`**: Trabajadores temporales (menos de 6 meses).
* **`total_area_mz`**: Área total de la explotación agropecuaria medida en manzanas.