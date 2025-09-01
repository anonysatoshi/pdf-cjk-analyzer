use std::fs;
use std::collections::HashMap;
use regex::Regex;

// We'll need to access the internal functions, so let's create our own diagnostic version
fn analyze_pdf_structure(pdf_path: &str) {
    println!("=== PDF Structure Analysis ===");
    println!("PDF path: {}", pdf_path);
    
    match fs::read(pdf_path) {
        Ok(pdf_data) => {
            println!("✓ Successfully read PDF file ({} bytes)", pdf_data.len());
            
            // Convert to string for analysis
            let pdf_str = String::from_utf8_lossy(&pdf_data);
            
            // Extract font information
            analyze_fonts(&pdf_str);
            
            // Extract ToUnicode mappings
            analyze_tounicode(&pdf_str);
            
            // Extract encoding information
            analyze_encoding(&pdf_str);
            
            // Look for Chinese text patterns
            analyze_chinese_content(&pdf_str);
            
        }
        Err(e) => {
            println!("✗ Failed to read PDF file: {:?}", e);
        }
    }
}

fn analyze_fonts(pdf_str: &str) {
    println!("\n--- Font Analysis ---");
    
    // Find font definitions
    let font_patterns = [
        ("Type0", "Type0 fonts (CID fonts)"),
        ("Type1", "Type1 fonts"),
        ("TrueType", "TrueType fonts"),
        ("/Font", "Font objects"),
        ("/BaseFont", "Base font names"),
        ("/Subtype", "Font subtypes"),
    ];
    
    for (pattern, description) in font_patterns.iter() {
        let count = pdf_str.matches(pattern).count();
        if count > 0 {
            println!("✓ Found {} {}: {}", count, description, pattern);
        }
    }
    
    // Extract specific font names
    let font_name_pattern = r"/BaseFont\s+/([^\s/]+)";
    if let Ok(regex) = Regex::new(font_name_pattern) {
        for cap in regex.captures_iter(pdf_str) {
            if let Some(font_name) = cap.get(1) {
                println!("  Font name: {}", font_name.as_str());
            }
        }
    }
}

fn analyze_tounicode(pdf_str: &str) {
    println!("\n--- ToUnicode Analysis ---");
    
    if pdf_str.contains("/ToUnicode") {
        println!("✓ PDF contains ToUnicode mappings");
        
        // Look for CMap content
        if pdf_str.contains("beginbfchar") {
            println!("✓ Contains bfchar mappings");
        }
        if pdf_str.contains("beginbfrange") {
            println!("✓ Contains bfrange mappings");
        }
        
        // Extract some sample mappings
        let cmap_pattern = r"<([0-9A-Fa-f]+)>\s+<([0-9A-Fa-f]+)>";
        if let Ok(regex) = Regex::new(cmap_pattern) {
            let mut sample_count = 0;
            for cap in regex.captures_iter(pdf_str) {
                if sample_count < 5 {
                    if let (Some(src), Some(dst)) = (cap.get(1), cap.get(2)) {
                        println!("  Sample mapping: {} -> {}", src.as_str(), dst.as_str());
                        sample_count += 1;
                    }
                } else {
                    break;
                }
            }
            let total_count = regex.captures_iter(pdf_str).count();
            if total_count > 5 {
                println!("  ... and {} more mappings", total_count - 5);
            }
        }
    } else {
        println!("✗ No ToUnicode mappings found");
    }
}

fn analyze_encoding(pdf_str: &str) {
    println!("\n--- Encoding Analysis ---");
    
    let encoding_patterns = [
        ("WinAnsiEncoding", "Windows-1252"),
        ("MacRomanEncoding", "Mac Roman"),
        ("StandardEncoding", "Adobe Standard"),
        ("PDFDocEncoding", "PDF Document"),
        ("Identity-H", "Identity-H (Unicode)"),
        ("Identity-V", "Identity-V (Unicode)"),
    ];
    
    for (pattern, description) in encoding_patterns.iter() {
        if pdf_str.contains(pattern) {
            println!("✓ Found encoding: {} ({})", pattern, description);
        }
    }
}

fn analyze_chinese_content(pdf_str: &str) {
    println!("\n--- Chinese Content Analysis ---");
    
    // Look for Chinese characters in the raw PDF
    let chinese_patterns = [
        ("回单", "回单 (receipt)"),
        ("支付宝", "支付宝 (Alipay)"),
        ("电子", "电子 (electronic)"),
        ("银行", "银行 (bank)"),
        ("金额", "金额 (amount)"),
    ];
    
    for (pattern, description) in chinese_patterns.iter() {
        if pdf_str.contains(pattern) {
            println!("✓ Found Chinese text: {} ({})", pattern, description);
        }
    }
    
    // Count Unicode characters
    let unicode_chars: Vec<char> = pdf_str.chars().filter(|c| *c as u32 > 127).collect();
    if !unicode_chars.is_empty() {
        println!("✓ Found {} Unicode characters", unicode_chars.len());
        
        // Show some sample Unicode characters
        let sample: String = unicode_chars.iter().take(20).collect();
        println!("  Sample Unicode chars: {}", sample);
    }
}

fn main() {
    println!("PDF Extractor Diagnostic Tool");
    println!("============================\n");
    
    // Analyze the Alipay PDF
    let alipay_pdf = "../电子回单_2025082300085000022244077482480011350582.pdf";
    analyze_pdf_structure(alipay_pdf);
    
    println!("\n{}", "=".repeat(50));
    
    // Compare with sample PDF
    let sample_pdf = "digitally_signed.pdf";
    analyze_pdf_structure(sample_pdf);
}
