
### Impacto del Financiamiento en la Generación de Empleo (Intensidad Laboral)

```sql labor_stats
SELECT 
    CASE 
        WHEN received_loan = true THEN 'Financiada'
        ELSE 'No Financiada'
    END AS loan_status, 
    labor_to_land_ratio 
FROM warehouse.labor_to_land_ratio_by_received_loan
```

<Grid cols=2>
<BigValue
data={labor_stats.filter(d => d.loan_status === 'Financiada')}
value=labor_to_land_ratio
title="Finca con financiación"
subtitle="Trabajadores por manzana"
fmt='0.00'
/>
<BigValue
data={labor_stats.filter(d => d.loan_status === 'No Financiada')}
value=labor_to_land_ratio
title="Finca sin financiación"
subtitle="Trabajadores por manzana"
fmt='0.00'
/>
</Grid>

<BarChart
data={labor_stats}
x=loan_status
y=labor_to_land_ratio
/>


Los datos demuestran que el acceso a crédito durante el ciclo agrícola 2010-2011 no generó un desplazamiento masivo de la mano de obra humana por capital tecnológico (mecanización).

Por el contrario, el financiamiento actuó como un catalizador directo para la absorción laboral. Las unidades productivas financiadas generaron casi el triple (2.9 veces más) de empleos por manzana en comparación con su grupo de control (no financiadas).

Esto sugiere que los fondos obtenidos a través de la banca formal, cooperativas o programas estatales se destinaron mayoritariamente a la expansión de las áreas de siembra o a la gestión de rubros altamente demandantes de mano de obra, requiriendo un incremento proporcional en la contratación de empleo temporal y permanente.


