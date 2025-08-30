use crate::{Result, ForgeTreeError};
use handlebars::Handlebars;
use serde_json::Value;
use std::collections::HashMap;

pub struct TemplateEngine {
    handlebars: Handlebars<'static>,
}

impl TemplateEngine {
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();
        
        // Register built-in helpers
        handlebars.register_helper("uppercase", Box::new(uppercase_helper));
        handlebars.register_helper("lowercase", Box::new(lowercase_helper));
        handlebars.register_helper("snake_case", Box::new(snake_case_helper));
        handlebars.register_helper("pascal_case", Box::new(pascal_case_helper));

        Self { handlebars }
    }

    pub fn render_template(&self, template: &str, variables: &HashMap<String, String>) -> Result<String> {
        let json_vars: Value = serde_json::to_value(variables)
            .map_err(|e| ForgeTreeError::Parse(format!("Failed to serialize variables: {}", e)))?;
        
        self.handlebars
            .render_template(template, &json_vars)
            .map_err(ForgeTreeError::Template)
    }

    pub fn register_template(&mut self, name: &str, template: &str) -> Result<()> {
        self.handlebars
            .register_template_string(name, template)
            .map_err(ForgeTreeError::Template)
    }
}

// Helper functions
fn uppercase_helper(
    h: &handlebars::Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let param = h.param(0)
        .and_then(|v| v.value().as_str())
        .unwrap_or("");
    out.write(&param.to_uppercase())?;
    Ok(())
}

fn lowercase_helper(
    h: &handlebars::Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let param = h.param(0)
        .and_then(|v| v.value().as_str())
        .unwrap_or("");
    out.write(&param.to_lowercase())?;
    Ok(())
}

fn snake_case_helper(
    h: &handlebars::Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let param = h.param(0)
        .and_then(|v| v.value().as_str())
        .unwrap_or("");
    
    let snake_case = param
        .chars()
        .map(|c| if c.is_uppercase() { format!("_{}", c.to_lowercase()) } else { c.to_string() })
        .collect::<String>()
        .trim_start_matches('_')
        .to_string();
    
    out.write(&snake_case)?;
    Ok(())
}

fn pascal_case_helper(
    h: &handlebars::Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let param = h.param(0)
        .and_then(|v| v.value().as_str())
        .unwrap_or("");
    
    let pascal_case = param
        .split(['_', '-', ' '])
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + &chars.collect::<String>(),
            }
        })
        .collect::<String>();
    
    out.write(&pascal_case)?;
    Ok(())
}

impl Default for TemplateEngine {
    fn default() -> Self {
        Self::new()
    }
}
