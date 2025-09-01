use std::fs;

use extractor::extract_text;

fn main() {
    println!("Testing PDF extractor with different PDFs...\n");
    
    // Test 1: Sample PDF with CJK content
let sample_pdf = "../sample_cjk_document.pdf";
println!("=== Test 1: Sample PDF with CJK Content ===");
test_pdf(sample_pdf);
    
    // Test 2: Sample PDF from the project
    let sample_pdf = "digitally_signed.pdf";
    println!("\n=== Test 2: Sample PDF ===");
    test_pdf(sample_pdf);
}

fn test_pdf(pdf_path: &str) {
    println!("PDF path: {}", pdf_path);
    
    match fs::read(pdf_path) {
        Ok(pdf_data) => {
            println!("✓ Successfully read PDF file ({} bytes)", pdf_data.len());
            
            // Let's also check the raw PDF content for font information
            let pdf_str = String::from_utf8_lossy(&pdf_data);
            
            // Look for font information
            if pdf_str.contains("/Font") {
                println!("✓ PDF contains font definitions");
                
                // Look for specific font types
                if pdf_str.contains("Type0") {
                    println!("✓ Contains Type0 fonts (CID fonts - often used for CJK)");
                }
                if pdf_str.contains("Type1") {
                    println!("✓ Contains Type1 fonts");
                }
                if pdf_str.contains("TrueType") {
                    println!("✓ Contains TrueType fonts");
                }
                if pdf_str.contains("ToUnicode") {
                    println!("✓ Contains ToUnicode mappings");
                }
                if pdf_str.contains("Encoding") {
                    println!("✓ Contains encoding information");
                }
                
                // Look for Chinese-related content
                if pdf_str.contains("回单") {
                    println!("✓ Contains Chinese text '回单' in raw PDF");
                }
                if pdf_str.contains("支付宝") {
                    println!("✓ Contains Chinese text '支付宝' in raw PDF");
                }
                    if pdf_str.contains("支付宝") {
        println!("✓ Contains '支付宝' in raw PDF");
    }
            }
            
            match extract_text(pdf_data) {
                Ok(text_per_page) => {
                    println!("✓ Successfully extracted text from PDF!");
                    println!("Number of pages: {}", text_per_page.len());
                    
                    for (i, text) in text_per_page.iter().enumerate() {
                        println!("\n--- Page {} ---", i + 1);
                        println!("Text length: {} characters", text.len());
                        println!("First 200 characters:");
                        
                        // Handle Unicode character boundaries properly
                        let preview: String = text.chars().take(200).collect();
                        println!("{}", preview);
                        
                        if text.len() > 200 {
                            println!("... (truncated)");
                        }
                        
                        // Show some key information about the content
                        println!("\nContent analysis:");
                        println!("Contains Chinese characters: {}", text.chars().any(|c| c.is_ascii() == false));
                        println!("Contains numbers: {}", text.chars().any(|c| c.is_numeric()));
                        println!("Contains '回单': {}", text.contains("回单"));
                        println!("Contains '支付宝': {}", text.contains("支付宝"));
                        println!("Contains '支付宝': {}", text.contains("支付宝"));
                        println!("Contains 'Sample': {}", text.contains("Sample"));
                        println!("Contains 'Signed': {}", text.contains("Signed"));
                        
                        // Count different character types
                        let ascii_count = text.chars().filter(|c| c.is_ascii()).count();
                        let non_ascii_count = text.chars().filter(|c| !c.is_ascii()).count();
                        let replacement_count = text.chars().filter(|c| *c == '�').count();
                        
                        println!("ASCII characters: {}", ascii_count);
                        println!("Non-ASCII characters: {}", non_ascii_count);
                        println!("Replacement characters (): {}", replacement_count);
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
