# Impacto del Financiamiento Agropecuario en Nicaragua (CENAGRO 2011)

En el sector agropecuario nicaragüense, el acceso al financiamiento representa un factor fundamental que influye directamente en el desarrollo productivo y en las condiciones socioeconómicas de las explotaciones. A pesar de la vital importancia del financiamiento rural, existe una limitada evidencia empírica, particularmente desde un enfoque cuantitativo, que permita comparar de manera rigurosa las unidades productivas. Esta falta de análisis estadístico dificulta la comprensión del verdadero impacto del acceso al crédito en la dinámica productiva y social, lo cual hace necesario el uso de la ciencia de datos para impulsar mejores decisiones en políticas públicas y estrategias de desarrollo rural.

---

## Objetivos de la Investigación

**Objetivo General**
* Analizar el impacto del financiamiento en la generación de empleo y la diversificación productiva de las explotaciones agropecuarias en Nicaragua, utilizando datos del CENAGRO 2011.

**Objetivos Específicos**
* Comparar el volumen de generación de empleo entre explotaciones agropecuarias financiadas y no financiadas durante el período de estudio.
* Evaluar el nivel de diversificación productiva en las explotaciones agropecuarias financiadas frente a las no financiadas.
* Determinar si existen diferencias estadísticamente significativas en la generación de empleo y la diversificación productiva según el acceso al financiamiento.

---

## Panorama Global del Censo

```sql global_summary
SELECT 
    total_farms,
    total_area_mz,
    financed_farms
FROM global_summary
```

<Grid cols=3>
    <BigValue 
      data={global_summary} 
      value=total_farms base
      title="Total de Explotaciones" 
      fmt="num0"
    />
    <BigValue 
      data={global_summary} 
      value=total_area_mz 
      title="Área Total (Manzanas)" 
      fmt="num0"
    />
    <BigValue 
      data={global_summary} 
      value=financed_farms 
      title="Fincas con Financiamiento" 
      fmt="num0"
    />
</Grid>