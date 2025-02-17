use crate::specs::Specification;
use crate::{verb, LogLevel};
use serde_json::Value;
use serde_yml;
use std::fs;
use tera::{Context, Tera};

#[derive(serde::Deserialize)]
struct TemplateConfig {
    files: Vec<String>,
}

pub fn process_templates(
    spec: &Specification,
    dbg: LogLevel,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // Create Tera context from spec.compiled
    let mut context = Context::new();
    if let Value::Object(map) = &spec.compiled {
        if let Some(Value::Object(data_map)) = map.get("data") {
            for (key, value) in data_map {
                context.insert(key, value);
            }
        }
    }

    // Get the base directory path
    let base_dir = format!("./tmpl/{}", spec.get_layer());

    // Read structure.yaml
    let structure_path = format!("{}/structure.yaml", base_dir);
    let structure_content = fs::read_to_string(&structure_path)?;
    let config: TemplateConfig = serde_yml::from_str(&structure_content)?;

    // Initialize Tera with the specific directory
    let mut tera = Tera::default();

    // Process each template
    verb!(dbg, "Rendering Templates");
    let mut rendered_templates = Vec::new();
    for template_name in config.files {
        // Read the template file directly
        let template_path = format!("{}/{}.tmpl", base_dir, template_name);
        verb!(dbg, " | {}", &template_path);
        let template_content = fs::read_to_string(&template_path)?;

        // Add this specific template to Tera
        tera.add_raw_template(&template_name, &template_content)?;

        // Render the template
        let rendered = tera.render(&template_name, &context)?;
        rendered_templates.push(rendered);
    }

    Ok(rendered_templates)
}
