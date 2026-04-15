# Producer Demographics

```sql gender_stats
SELECT 
    producer_gender, 
    count
FROM warehouse.producer_gender  
ORDER BY count DESC
```

### Producers by Gender
<BarChart 
    data={gender_stats} 
    x="producer_gender" 
    y="count" 
    swapXY={true}
    title="Total Registered Producers"
/>
