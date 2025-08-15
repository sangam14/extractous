//! Extractous is a library that extracts text from various file formats.
//! * Supports many file formats such as Word, Excel, PowerPoint, PDF, and many more.
//! * Strives to be simple fast and efficient
//!
//! # Quick Start
//! Extractous API entry point is the [`Extractor`] struct.
//! All public apis are accessible through an extractor.
//! The extractor provides functions to extract text from files, Urls, and byte arrays.
//! To use an extractor, you need to:
//! - [create and configure new the extractor](#create-and-config-an-extractor)
//! - [use the extractor to extract text](#extract-text)
//! - [enable OCR for the extractor](#extract-text-with-ocr)
//!
//! ## Create and config an extractor
//!
//! ```no_run
//! use extractous::Extractor;
//! use extractous::PdfParserConfig;
//!
//! // Create a new extractor. Note it uses a consuming builder pattern
//! let mut extractor = Extractor::new()
//!                       .set_extract_string_max_length(1000);
//!
//! // can also perform conditional configuration
//! let custom_pdf_config = true;
//! if custom_pdf_config {
//!     extractor = extractor.set_pdf_config(
//!         PdfParserConfig::new().set_extract_annotation_text(false)
//!     );
//! }
//!
//! ```
//!
//! ## Extract text
//!
//! ```no_run
//! use extractous::Extractor;
//! use extractous::PdfParserConfig;
//!
//! // Create a new extractor. Note it uses a consuming builder pattern
//! let mut extractor = Extractor::new().set_extract_string_max_length(1000);
//!
//! // Extract text from a file
//! let (text, metadata) = extractor.extract_file_to_string("README.md").unwrap();
//! println!("{}", text);
//!
//! ```
//!
//! ## Extract text with OCR
//! * Make sure Tesseract is installed with the corresponding language packs. For example on debian `sudo apt install tesseract-ocr tesseract-ocr-deu` to install tesseract with German language pack.
//! * If you get `Parse error occurred : Unable to extract PDF content`, it is most likely that the OCR language pack is not installed
//!
//! ```no_run
//! use extractous::{Extractor, TesseractOcrConfig, PdfParserConfig, PdfOcrStrategy};
//!
//! let file_path = "../test_files/documents/deu-ocr.pdf";
//!
//! // Create a new extractor. Note it uses a consuming builder pattern
//! let extractor = Extractor::new()
//!  .set_ocr_config(TesseractOcrConfig::new().set_language("deu"))
//!  .set_pdf_config(PdfParserConfig::new().set_ocr_strategy(PdfOcrStrategy::OCR_ONLY));
//!
//! // extract file with extractor
//! let (content, metadata) = extractor.extract_file_to_string(file_path).unwrap();
//! println!("{}", content);
//!
//! ```

/// Default buffer size - optimized for better performance
/// Increased from 32KB to 256KB for better throughput based on benchmarks
pub const DEFAULT_BUF_SIZE: usize = 262144; // 256KB

/// Large buffer size for memory-mapped operations
pub const LARGE_BUF_SIZE: usize = 1048576; // 1MB

/// Threshold for using memory-mapped I/O - lowered based on benchmarks
pub const MMAP_THRESHOLD: usize = 512 * 1024; // 512KB

// errors module
mod errors;
pub use errors::*;

// config module
mod config;
pub use config::*;

// extractor module is the main public api interface
mod extractor;
pub use extractor::*;

// format detection module
mod format_detection;
pub use format_detection::*;

// pure rust parsers for performance optimization
mod pure_rust_parsers;
pub use pure_rust_parsers::*;

// SIMD-optimized text processing
mod simd_text;
pub use simd_text::*;

// tika module, not exposed outside this crate
mod tika {
    mod jni_utils;
    mod parse;
    mod wrappers;
    pub use parse::*;
    pub use wrappers::JReaderInputStream;
}
