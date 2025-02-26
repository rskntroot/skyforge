use crate::specs::Specification;
use crate::{info, verb, LogLevel};
use serde_json::Value;
use serde_yml;
use std::fs;
use tera::{Context, Tera};

#[derive(serde::Deserialize)]
struct TemplateConfig {
    files: Vec<String>,
}

pub struct RenderedConfig {
    pub hostname: String,
    pub configs: Vec<(String, String)>,
    pub spec: Value,
}

pub fn process_templates(
    spec: &Specification,
    dbg: LogLevel,
) -> Result<RenderedConfig, Box<dyn std::error::Error>> {
    // Create Tera context from spec.compiled
    let mut context = Context::new();
    if let Value::Object(map) = &spec.compiled {
        if let Some(Value::Object(data_map)) = map.get("data") {
            for (key, value) in data_map {
                context.insert(key, value);
            }
        }
    }

    let device_name = match context.get("hostname") {
        Some(value) => value.as_str().unwrap_or("default_hostname"),
        None => "default_hostname",
    };

    // Get the base directory path
    let base_dir = format!("./tmpl/{}", spec.get_layer());

    // Read structure.yaml
    let structure_path = format!("{}/structure.yaml", base_dir);
    let structure_content = fs::read_to_string(&structure_path)?;
    let config: TemplateConfig = serde_yml::from_str(&structure_content)?;

    // Initialize Tera with the specific directory
    let mut tera = Tera::default();

    // Process each template
    info!(dbg, "Rendering {}", &device_name);
    let mut configs = Vec::new();
    for template_name in config.files {
        // Read the template file directly
        let template_path = format!("{}/{}.tmpl", base_dir, template_name);
        verb!(dbg, " | {}", &template_path);
        let template_content = fs::read_to_string(&template_path)?;

        // Add this specific template to Tera
        tera.add_raw_template(&template_name, &template_content)?;

        // Render the template
        let rendered = tera.render(&template_name, &context)?;
        configs.push((String::from(&template_name), rendered));
    }

    Ok(RenderedConfig {
        hostname: String::from(device_name),
        configs,
        spec: spec.compiled.clone(),
    })
}
