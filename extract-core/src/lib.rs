//! Extract-RS is a library that extracts text from various file formats.
//! * Supports many file formats such as Word, Excel, PowerPoint, PDF, and many more.
//! * Strives to be simple fast and efficient
//!
//! # Quick Start
//! Extract-RS API entry point is the [`Extractor`] struct.
//! All public apis are accessible through an extractor.
//! The extractor provides functions to extract text from files, Urls, and byte arrays.
//! To use an extractor, you need to:
//! - [create and configure new the extractor](#create-and-config-an-extractor)
//! - [use the extractor to extract text](#extract-text)
//!
//! ## Create and config an extractor
//!
//! ```no_run
//! use extract_rs::Extractor;
//! use extract_rs::PdfParserConfig;
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
//! use extract_rs::Extractor;
//! use extract_rs::PdfParserConfig;
//!
//! // Create a new extractor. Note it uses a consuming builder pattern
//! let mut extractor = Extractor::new().set_extract_string_max_length(1000);
//!
//! // Extract text from a file
//! let text = extractor.extract_file_to_string("README.md").unwrap();
//! println!("{}", text);
//!
//! ```

// errors module
mod errors;
pub use errors::*;

// extract module main outside interface
mod extract {
    mod config;
    pub use config::*;
    mod extractor;
    pub use extractor::*;
}
pub use extract::*;

// tika module, not outside this crate
mod tika {
    mod jni_utils;
    mod parse;
    mod wrappers;
    pub use parse::*;
    pub use wrappers::JReaderInputStream;
}
