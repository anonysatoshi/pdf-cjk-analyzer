use std::fs;
use std::collections::HashMap;
use extractor::extract_text;

// Enhanced test to capture ALL text content
fn comprehensive_extract_text(pdf_path: &str) {
    println!("=== Comprehensive PDF Text Extraction ===");
    println!("PDF path: {}", pdf_path);
    
    match fs::read(pdf_path) {
        Ok(pdf_data) => {
            println!("✓ Successfully read PDF file ({} bytes)", pdf_data.len());
            
            // Use the enhanced extractor
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
                            (0x4E00..=0x9FFF).contains(&code) || // CJK Unified Ideographs
                            (0x3400..=0x4DBF).contains(&code) || // CJK Unified Ideographs Extension A
                            (0x20000..=0x2A6DF).contains(&code)  // CJK Unified Ideographs Extension B
                        }).count();
                        
                        println!("ASCII characters: {}", ascii_count);
                        println!("Non-ASCII characters: {}", non_ascii_count);
                        println!("Replacement characters (�): {}", replacement_count);
                        println!("Chinese characters: {}", chinese_count);
                        
                        // Show FULL text content
                        println!("\n=== FULL TEXT CONTENT ===");
                        println!("{}", text);
                        println!("=== END TEXT CONTENT ===");
                        
                        // Look for specific Chinese words and phrases
                        let chinese_phrases = [
                            "回单生成时间", "付款方", "收款方", "支付宝流水号", 
                            "付款时间", "付款金额", "摘要", "转账",
                            "王阳", "单珍品初", "壹仟零陆拾元整", "1060.00",
                            "2025-08-23", "2024-12-11", "15941190353", "139******41"
                        ];
                        
                        println!("\n=== KEY PHRASES SEARCH ===");
                        for phrase in chinese_phrases.iter() {
                            if text.contains(phrase) {
                                println!("✓ Found: {}", phrase);
                            } else {
                                println!("✗ Missing: {}", phrase);
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
    println!("Comprehensive PDF Text Extractor");
    println!("================================\n");
    
    // Test with the Alipay PDF
    let alipay_pdf = "../电子回单_2025082300085000022244077482480011350582.pdf";
    comprehensive_extract_text(alipay_pdf);
}
