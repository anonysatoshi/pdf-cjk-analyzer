use std::fs;
use std::collections::HashMap;
use extractor::extract_text;

// Test program to use enhanced font extraction
fn test_enhanced_extraction(pdf_path: &str) {
    println!("=== Testing Enhanced Font Extraction ===");
    println!("PDF path: {}", pdf_path);
    
    match fs::read(pdf_path) {
        Ok(pdf_data) => {
            println!("✓ Successfully read PDF file ({} bytes)", pdf_data.len());
            
            // Use the original extractor with enhanced debugging
            match extract_text(pdf_data) {
                Ok(text_per_page) => {
                    println!("✓ Extraction completed!");
                    println!("Number of pages: {}", text_per_page.len());
                    
                    for (i, text) in text_per_page.iter().enumerate() {
                        println!("\n--- Page {} ---", i + 1);
                        println!("Text length: {} characters", text.len());
                        
                        // Count character types
                        let ascii_count = text.chars().filter(|c| c.is_ascii()).count();
                        let non_ascii_count = text.chars().filter(|c| !c.is_ascii()).count();
                        let replacement_count = text.chars().filter(|c| *c == '�').count();
                        let chinese_count = text.chars().filter(|c| {
                            let code = *c as u32;
                            // Chinese Unicode ranges
                            (0x4E00..=0x9FFF).contains(&code) || // CJK Unified Ideographs
                            (0x3400..=0x4DBF).contains(&code) || // CJK Unified Ideographs Extension A
                            (0x20000..=0x2A6DF).contains(&code)  // CJK Unified Ideographs Extension B
                        }).count();
                        
                        println!("ASCII characters: {}", ascii_count);
                        println!("Non-ASCII characters: {}", non_ascii_count);
                        println!("Replacement characters (�): {}", replacement_count);
                        println!("Chinese characters: {}", chinese_count);
                        
                        // Show text preview
                        let preview: String = text.chars().take(200).collect();
                        println!("Text preview: {}", preview);
                        
                        // Look for specific Chinese words
                        let chinese_words = ["回单", "支付宝", "电子", "银行", "金额"];
                        for word in chinese_words.iter() {
                            if text.contains(word) {
                                println!("✓ Found Chinese word: {}", word);
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("✗ Extraction failed: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("✗ Failed to read PDF file: {:?}", e);
        }
    }
}

fn main() {
    println!("Enhanced Font Extraction Test");
    println!("============================\n");
    
    // Test with the Alipay PDF
    let alipay_pdf = "../电子回单_2025082300085000022244077482480011350582.pdf";
    test_enhanced_extraction(alipay_pdf);
    
    println!("\n{}", "=".repeat(50));
    
    // Test with sample PDF for comparison
    let sample_pdf = "digitally_signed.pdf";
    test_enhanced_extraction(sample_pdf);
}
