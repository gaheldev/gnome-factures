use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;

use handlebars::{to_json, Context, Helper, Output, RenderContext, RenderError, RenderErrorReason, Handlebars};
use serde_json::Map;

use crate::app::{Author, Client, Product};


// /// Example usage including all functionalities:
// /// 
// /// Template::new()
// ///     .fill(invoice)?
// ///     .to_file("./filled-template.tex")?
// ///     .export("./export.pdf")?
// ///     .open()?;

const TMP_DIR: &str = "/tmp/gnome-factures/";
const TEMPLATE_STR: &str = include_str!("../assets/template.tex");

/// let bill = BillType::Devis;
/// let invoice = InvoiceData {
///     is_devis: matches!(bill, BillType::Devis),
///     number: "013".to_string(),
///     client: Client {
///         name: "robert".to_string(),
///         address: Address {
///             number_and_street: "32 rue truc".to_string(),
///             postcode: "39200".to_string(),
///             city: "Ville".to_string()
///         },
///         ..Client::default()
///     },
///     // nature: "this is nature".to_string(),
///     nature: "oui".to_string(),
///     products: vec![product.clone(), product2],
///     diffuseur: false,
/// };
#[derive(Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InvoiceData {
    pub author: Author,
    pub is_devis: bool,
    pub number: String,
    pub client: Client,
    pub nature: String,
    pub diffuseur: bool,
    pub dispense_path: String,
    pub products: Vec<Product>,
    /// date with format dd/mm/yyyy
    pub date: String,
}


#[derive(Clone)]
pub struct Template {
    pub content: String,
    filled: bool
}
impl Template {
    #[allow(dead_code)]
    pub fn from(path: &str) -> Result<Template, Box<dyn std::error::Error>> {
        Ok(Template {
            content: read_to_string(Path::new(path))?,
            filled: false,
        })
    }

    pub fn new() ->  Template {
        Template { content: TEMPLATE_STR.to_string(), filled: false }
    }

    pub fn fill(&self, invoice_data: InvoiceData) -> Result<Template, Box<dyn std::error::Error>> {
        let mut data = Map::new();
        data.insert("invoice".to_string(), to_json(invoice_data));
        let mut reg = Handlebars::new();
        reg.register_helper("frfloat", Box::new(french_float));
        reg.register_helper("multiline", Box::new(multiline));
        reg.register_helper("includepdf", Box::new(includepdf));
        reg.register_helper("override_braces", Box::new(override_braces));
        reg.set_dev_mode(true);  // This enables alternative delimiters
        Ok(Template {
            content: reg.render_template(&self.content, &data)?,
            filled: true,
        })
    }

    pub fn to_file(&self, file_path: &str) -> Result<Template, Box<dyn std::error::Error>> {
        if !self.filled {
            return Err("Template is not filled".into());
        }
        let mut output = File::create(file_path)?;
        write!(output, "{}", self.content)?;
        Ok(self.clone())
    }

    pub fn export(self, output_file: &str) -> Result<PdfFile, Box<dyn std::error::Error>> {
        if !self.filled { return Err("Template was not filled".into()) };
        let tmp_pdf = latex_to_pdf(&self.content)?;
        std::fs::copy(tmp_pdf.path, output_file)?;
        Ok(PdfFile{ path: output_file.to_string() })
    }

    pub fn compile(self) -> Result<PdfFile, Box<dyn std::error::Error>> {
        if !self.filled { return Err("Template was not filled".into()) };
        latex_to_pdf(&self.content)
    }
}


fn latex_to_pdf(latex_content: &str) -> Result<PdfFile, Box<dyn std::error::Error>> {
    let output_dir = Path::new(TMP_DIR);

    // Create output directory if it doesn't exist
    std::fs::create_dir_all(output_dir)?;
    
    // Path for the temporary tex file
    let tex_path = output_dir.join("document.tex");
    
    // Write LaTeX content to file
    let mut file = File::create(&tex_path)?;
    file.write_all(latex_content.as_bytes())?;
    
    // Ensure the file is closed before pdflatex tries to access it
    drop(file);
    
    // Run pdflatex - avoid changing directory and use absolute paths
    let absolute_tex_path = tex_path.canonicalize()?;
    let absolute_output_dir = output_dir.canonicalize()?;
    
    println!("Running xelatex on: {}", absolute_tex_path.display());
    println!("Output directory: {}", absolute_output_dir.display());
    
    let output = Command::new("xelatex")
        .arg("-interaction=nonstopmode")
        .arg(format!("-output-directory={}", absolute_output_dir.display()))
        .arg(absolute_tex_path)
        .output()?;
    
    // Print the full output for debugging
    println!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
    println!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
    
    if !output.status.success() {
        println!("Some errors occured during xelatex command");
    }
    
    // Check if PDF was actually created
    let pdf_path = output_dir.join("document.pdf");
    if !pdf_path.exists() {
        return Err("PDF file wasn't created".into());
    } else {
        println!("PDF file successfully created at {}", pdf_path.to_str().unwrap());
    }

    Ok(PdfFile{ path: pdf_path.to_str().unwrap().to_string() })
}


#[derive(Debug,Clone)]
pub struct PdfFile {
    pub path: String,
}
impl PdfFile {
    pub fn open(&self) -> Result<(), std::io::Error> {
        open::that(&self.path)
    }

    pub fn export(&self, output_file: &str) -> Result<(), std::io::Error> {
        std::fs::copy(&self.path, output_file)?;
        Ok(())
    }
}

/// include pdf file
/// -> workaround delimiter error (can't use space in \includepdf{}
fn includepdf(h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> Result<(), RenderError> {
    let param = h.param(0).ok_or(
        RenderErrorReason::ParamNotFoundForIndex(
            "includepdf helper requires at least one parameter",
            0,
    ))?;

    let content = param.value().as_str().ok_or(
        RenderErrorReason::InvalidParamType(
            "includepdf helper requires a string parameter",
    ))?;

    // Format with comma as decimal separator
    let formatted = format!("\\includepdf{{{}}}", content);

    // Write the formatted string to output
    out.write(&formatted)?;

    Ok(())
}

/// replace {{content}} with actual latex {content}
/// -> workaround delimiter error (can't use space in \includepdf{}
fn override_braces(h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> Result<(), RenderError> {
    let param = h.param(0).ok_or(
        RenderErrorReason::ParamNotFoundForIndex(
            "override_braces helper requires at least one parameter",
            0,
    ))?;

    let content = param.value().as_str().ok_or(
        RenderErrorReason::InvalidParamType(
            "override_braces helper requires a string parameter",
    ))?;

    // Format with comma as decimal separator
    // let formatted = format!("\\includegraphics[width=0.3\\textwidth]{{{}}}", content);
    let formatted = format!("{{{}}}", content);

    // Write the formatted string to output
    out.write(&formatted)?;

    Ok(())
}

/// convert multiline string for latex
///     "\n" -> "\\"
fn multiline(h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> Result<(), RenderError> {
    let param = h.param(0).ok_or(
        RenderErrorReason::ParamNotFoundForIndex(
            "multiline helper requires at least one parameter",
            0,
    ))?;

    let content = param.value().as_str().ok_or(
        RenderErrorReason::InvalidParamType(
            "multiline helper requires a string parameter",
    ))?;

    // Format with comma as decimal separator
    let formatted = content.replace("\n", " \\\\\n");

    // Write the formatted string to output
    out.write(&formatted)?;

    Ok(())
}

// floats should be represented with a , not a .
//     -> 21,4€ 
// otherwise latex template will get errors
// /!\ only works if input param is a valid number with 1 or 0 "."
fn french_float (h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> Result<(), RenderError> {
    // Get the first parameter (the price)
    let param = h.param(0).ok_or(
        RenderErrorReason::ParamNotFoundForIndex(
            "french_float helper requires at least one parameter",
            0,
    ))?;

    // Convert to f64
    let price = param.value().as_f64().ok_or(
        RenderErrorReason::InvalidParamType(
            "french_float helper requires a number parameter",
    ))?;

    // Format with comma as decimal separator
    let formatted = format!("{:.2}", price).replace(".", ",");

    // Write the formatted string to output
    out.write(&formatted)?;

    Ok(())
}
