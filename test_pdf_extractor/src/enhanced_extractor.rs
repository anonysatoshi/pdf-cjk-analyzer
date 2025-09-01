use std::fs;
use std::collections::HashMap;
use regex::Regex;

// Let's create an enhanced version of the extractor with better CMap handling
fn enhanced_extract_text(pdf_path: &str) -> Result<Vec<String>, String> {
    println!("=== Enhanced PDF Text Extraction ===");
    println!("PDF path: {}", pdf_path);
    
    match fs::read(pdf_path) {
        Ok(pdf_data) => {
            println!("✓ Successfully read PDF file ({} bytes)", pdf_data.len());
            
            // Use the original extractor first
            match extractor::extract_text(pdf_data.clone()) {
                Ok(text_per_page) => {
                    println!("✓ Original extractor result:");
                    for (i, text) in text_per_page.iter().enumerate() {
                        println!("  Page {}: {} characters", i + 1, text.len());
                        let replacement_count = text.chars().filter(|c| *c == '�').count();
                        println!("    Replacement characters: {}", replacement_count);
                    }
                    
                    // Now let's try our enhanced approach
                    let enhanced_text = enhanced_extract_with_cmap(&pdf_data)?;
                    
                    println!("\n✓ Enhanced extraction result:");
                    for (i, text) in enhanced_text.iter().enumerate() {
                        println!("  Page {}: {} characters", i + 1, text.len());
                        let replacement_count = text.chars().filter(|c| *c == '�').count();
                        println!("    Replacement characters: {}", replacement_count);
                        
                        // Show some sample text
                        let preview: String = text.chars().take(100).collect();
                        println!("    Preview: {}", preview);
                    }
                    
                    Ok(enhanced_text)
                }
                Err(e) => {
                    println!("✗ Original extractor failed: {:?}", e);
                    Err(e.to_string())
                }
            }
        }
        Err(e) => {
            println!("✗ Failed to read PDF file: {:?}", e);
            Err(e.to_string())
        }
    }
}

fn enhanced_extract_with_cmap(pdf_data: &[u8]) -> Result<Vec<String>, String> {
    // This is where we'll implement our enhanced CMap handling
    // For now, let's return a simple result
    let pdf_str = String::from_utf8_lossy(pdf_data);
    
    // Look for text content streams
    let text_streams = extract_text_streams(&pdf_str);
    
    let mut pages = Vec::new();
    for (i, stream) in text_streams.iter().enumerate() {
        println!("  Processing text stream {} ({} bytes)", i + 1, stream.len());
        
        // Try to decode with enhanced CMap handling
        let decoded_text = enhanced_decode_text_stream(stream);
        pages.push(decoded_text);
    }
    
    Ok(pages)
}

fn extract_text_streams(pdf_str: &str) -> Vec<String> {
    let mut streams = Vec::new();
    
    // Look for text content streams
    let stream_pattern = r"stream\s*\r?\n([\s\S]*?)endstream";
    if let Ok(regex) = Regex::new(stream_pattern) {
        for cap in regex.captures_iter(pdf_str) {
            if let Some(stream_data) = cap.get(1) {
                streams.push(stream_data.as_str().to_string());
            }
        }
    }
    
    streams
}

fn enhanced_decode_text_stream(stream_data: &str) -> String {
    // Enhanced text decoding with better CMap handling
    let mut result = String::new();
    
    // Look for text operators like Tj, TJ, etc.
    let text_pattern = r"\(([^)]+)\)\s*Tj";
    if let Ok(regex) = Regex::new(text_pattern) {
        for cap in regex.captures_iter(stream_data) {
            if let Some(text_content) = cap.get(1) {
                let decoded = enhanced_decode_text_content(text_content.as_str());
                result.push_str(&decoded);
                result.push(' ');
            }
        }
    }
    
    // Also look for array-based text
    let array_pattern = r"\[([^\]]+)\]\s*TJ";
    if let Ok(regex) = Regex::new(array_pattern) {
        for cap in regex.captures_iter(stream_data) {
            if let Some(array_content) = cap.get(1) {
                let decoded = enhanced_decode_array_content(array_content.as_str());
                result.push_str(&decoded);
                result.push(' ');
            }
        }
    }
    
    result.trim().to_string()
}

fn enhanced_decode_text_content(text_content: &str) -> String {
    // Enhanced text content decoding
    let mut result = String::new();
    
    // Handle hex strings
    if text_content.starts_with('<') && text_content.ends_with('>') {
        let hex_content = &text_content[1..text_content.len()-1];
        if let Ok(bytes) = hex::decode(hex_content) {
            // Try different decoding approaches
            result = enhanced_decode_bytes(&bytes);
        }
    } else {
        // Handle literal strings
        result = text_content.to_string();
    }
    
    result
}

fn enhanced_decode_array_content(array_content: &str) -> String {
    // Enhanced array content decoding
    let mut result = String::new();
    
    // Split by whitespace and process each element
    let elements: Vec<&str> = array_content.split_whitespace().collect();
    for element in elements {
        if element.starts_with('(') && element.ends_with(')') {
            // Literal string
            let content = &element[1..element.len()-1];
            result.push_str(content);
            result.push(' ');
        } else if element.starts_with('<') && element.ends_with('>') {
            // Hex string
            let hex_content = &element[1..element.len()-1];
            if let Ok(bytes) = hex::decode(hex_content) {
                let decoded = enhanced_decode_bytes(&bytes);
                result.push_str(&decoded);
                result.push(' ');
            }
        }
    }
    
    result.trim().to_string()
}

fn enhanced_decode_bytes(bytes: &[u8]) -> String {
    // Enhanced byte decoding with multiple fallback strategies
    let mut result = String::new();
    
    // Strategy 1: Try UTF-8
    if let Ok(utf8_str) = String::from_utf8(bytes.to_vec()) {
        result = utf8_str;
    } else {
        // Strategy 2: Try UTF-16
        if bytes.len() >= 2 {
            let mut utf16_chars = Vec::new();
            let mut i = 0;
            while i < bytes.len() - 1 {
                let code = ((bytes[i] as u16) << 8) | (bytes[i + 1] as u16);
                if let Some(ch) = char::from_u32(code as u32) {
                    utf16_chars.push(ch);
                }
                i += 2;
            }
            result = utf16_chars.into_iter().collect();
        }
        
        // Strategy 3: Try individual bytes as Latin-1
        if result.is_empty() {
            result = bytes.iter()
                .map(|&b| b as char)
                .collect();
        }
    }
    
    result
}

fn main() {
    println!("Enhanced PDF Extractor");
    println!("=====================\n");
    
    // Test with the Alipay PDF
    let alipay_pdf = "../电子回单_2025082300085000022244077482480011350582.pdf";
    match enhanced_extract_text(alipay_pdf) {
        Ok(text_pages) => {
            println!("\n=== Final Results ===");
            for (i, text) in text_pages.iter().enumerate() {
                println!("Page {}: {}", i + 1, text);
            }
        }
        Err(e) => {
            println!("✗ Enhanced extraction failed: {}", e);
        }
    }
}
