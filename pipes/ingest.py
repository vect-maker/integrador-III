import json
import pyreadstat

def convert_to_parquet(spss_file, file_name, outdir=""):
    # Open the file
    df, meta = pyreadstat.read_sav(spss_file, apply_value_formats=False)

    # Save metadata
    with open(f"{outdir}/{file_name}-metadata.json", "w", encoding="utf-8") as f:
        json.dump({
            "variable_value_labels":  meta.variable_value_labels, 
            "column_names_to_labels": meta.column_names_to_labels
        }, f, ensure_ascii=False, indent=4)

    # Save to parquet
    df.to_parquet(f"{outdir}/{file_name}.parquet", engine='pyarrow', index=False)

convert_to_parquet("data/cenagro-2011-explotaciones-agropecuarias.sav", "cenagro-2011-explotaciones-agropecuarias" ,"data")
convert_to_parquet("data/cenagro-2011-parcelas-aprovechamiento-tierra.sav", "cenagro-2011-parcelas-aprovechamiento-tierra" ,"data")