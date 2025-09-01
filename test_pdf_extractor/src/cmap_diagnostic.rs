use std::fs;
use std::collections::HashMap;
use regex::Regex;

// Enhanced diagnostic to understand the specific CMap content
fn analyze_cmap_content(pdf_path: &str) {
    println!("=== Detailed CMap Analysis ===");
    println!("PDF path: {}", pdf_path);
    
    match fs::read(pdf_path) {
        Ok(pdf_data) => {
            let pdf_str = String::from_utf8_lossy(&pdf_data);
            
            // Extract ToUnicode stream content
            if let Some(cmap_content) = extract_cmap_content(&pdf_str) {
                println!("✓ Found CMap content ({} bytes)", cmap_content.len());
                println!("First 500 characters of CMap:");
                println!("{}", &cmap_content[..cmap_content.len().min(500)]);
                
                // Analyze CMap structure
                analyze_cmap_structure(&cmap_content);
            } else {
                println!("✗ No CMap content found");
            }
        }
        Err(e) => {
            println!("✗ Failed to read PDF file: {:?}", e);
        }
    }
}

fn extract_cmap_content(pdf_str: &str) -> Option<String> {
    // Look for ToUnicode stream content
    let stream_pattern = r"/ToUnicode\s+(\d+)\s+(\d+)\s+R";
    if let Ok(regex) = Regex::new(stream_pattern) {
        for cap in regex.captures_iter(pdf_str) {
            if let (Some(obj_num), Some(gen_num)) = (cap.get(1), cap.get(2)) {
                let obj_id = format!("{} {} obj", obj_num.as_str(), gen_num.as_str());
                let end_obj = format!("{} {} endobj", obj_num.as_str(), gen_num.as_str());
                
                if let Some(start) = pdf_str.find(&obj_id) {
                    if let Some(end) = pdf_str[start..].find(&end_obj) {
                        let stream_content = &pdf_str[start..start + end];
                        
                        // Extract the actual stream data
                        if let Some(stream_start) = stream_content.find("stream") {
                            if let Some(stream_end) = stream_content[stream_start..].find("endstream") {
                                let data_start = stream_start + "stream".len();
                                let data_end = stream_start + stream_end;
                                return Some(stream_content[data_start..data_end].to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

fn analyze_cmap_structure(cmap_content: &str) {
    println!("\n--- CMap Structure Analysis ---");
    
    // Count different CMap sections
    let bfchar_count = cmap_content.matches("beginbfchar").count();
    let bfrange_count = cmap_content.matches("beginbfrange").count();
    
    println!("bfchar sections: {}", bfchar_count);
    println!("bfrange sections: {}", bfrange_count);
    
    // Extract sample mappings
    let mapping_pattern = r"<([0-9A-Fa-f]+)>\s+<([0-9A-Fa-f]+)>";
    if let Ok(regex) = Regex::new(mapping_pattern) {
        let mut sample_count = 0;
        for cap in regex.captures_iter(cmap_content) {
            if sample_count < 10 {
                if let (Some(src), Some(dst)) = (cap.get(1), cap.get(2)) {
                    let src_code = u32::from_str_radix(src.as_str(), 16).unwrap_or(0);
                    let dst_code = u32::from_str_radix(dst.as_str(), 16).unwrap_or(0);
                    
                    println!("  Mapping {}: 0x{} (code {}) -> 0x{} (code {})", 
                             sample_count + 1, src.as_str(), src_code, dst.as_str(), dst_code);
                    
                    // Try to convert to Unicode
                    if let Some(ch) = char::from_u32(dst_code) {
                        println!("    Unicode: '{}' (U+{:04X})", ch, dst_code);
                    } else {
                        println!("    Unicode: Invalid code point");
                    }
                    
                    sample_count += 1;
                }
            } else {
                break;
            }
        }
        
        let total_mappings = regex.captures_iter(cmap_content).count();
        if total_mappings > 10 {
            println!("  ... and {} more mappings", total_mappings - 10);
        }
    }
}

fn main() {
    println!("Enhanced CMap Diagnostic Tool");
    println!("============================\n");
    
    // Analyze the Alipay PDF's CMap content
    let alipay_pdf = "../电子回单_2025082300085000022244077482480011350582.pdf";
    analyze_cmap_content(alipay_pdf);
}
