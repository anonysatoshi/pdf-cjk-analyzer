use std::fs;
use extractor::extract_text;

fn main() {
    // Read the Alipay PDF
    let pdf_path = "../../../电子回单_2025082300085000022244077482480011350582.pdf";
    
    println!("Testing PDF extractor with Alipay PDF...");
    println!("PDF path: {}", pdf_path);
    
    match fs::read(pdf_path) {
        Ok(pdf_data) => {
            println!("✓ Successfully read PDF file ({} bytes)", pdf_data.len());
            
            match extract_text(pdf_data) {
                Ok(text_per_page) => {
                    println!("✓ Successfully extracted text from PDF!");
                    println!("Number of pages: {}", text_per_page.len());
                    
                    for (i, text) in text_per_page.iter().enumerate() {
                        println!("\n--- Page {} ---", i + 1);
                        println!("Text length: {} characters", text.len());
                        println!("First 200 characters:");
                        println!("{}", &text[..text.len().min(200)]);
                        
                        if text.len() > 200 {
                            println!("... (truncated)");
                        }
                    }
                }
                Err(e) => {
                    println!("✗ Failed to extract text: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("✗ Failed to read PDF file: {:?}", e);
        }
    }
}
