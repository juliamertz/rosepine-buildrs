use anyhow::Result;
use palette::Variant;
use strum::IntoEnumIterator;
use tera::{Context, Tera};

mod filters {
    use std::collections::HashMap;

    // credit: https://github.com/catppuccin/whiskers/blob/c36c9fe101448cd6755d91c83321ceb4346b9ae6/src/filters.rs#L123C1-L134C2
    pub fn trunc(
        value: &tera::Value,
        args: &HashMap<String, tera::Value>,
    ) -> Result<tera::Value, tera::Error> {
        let value: f64 = tera::from_value(value.clone())?;
        let places: usize = tera::from_value(
            args.get("places")
                .ok_or_else(|| tera::Error::msg("number of places is required"))?
                .clone(),
        )?;
        Ok(tera::to_value(format!("{value:.places$}"))?)
    }
}

fn create_context(variant: &Variant) -> Context {
    let mut ctx = Context::new();
    for (key, value) in variant.metadata() {
        ctx.insert(key, &value);
    }
    for (role, color) in variant.colors() {
        ctx.insert(role, &color);
    }

    ctx
}

pub fn generate_variants(template: String) -> Result<Vec<(Variant, String)>> {
    let mut tera = Tera::default();
    tera.register_filter("trunc", filters::trunc);
    tera.add_raw_template("content", &template)?;

    // TODO:
    Ok(Variant::iter()
        .map(|v| (v, tera.render("content", &create_context(&v)).unwrap()))
        .collect())
}
