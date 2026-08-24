//! Модуль обработки ошибок конвертера

use thiserror::Error;

/// Типы ошибок конвертации SVG в DXF
#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("Ошибка чтения SVG файла: {0}")]
    SvgReadError(String),

    #[error("Ошибка парсинга SVG: {0}")]
    SvgParseError(String),

    #[error("Неподдерживаемый SVG элемент: {element}")]
    UnsupportedElement { element: String },

    #[error("Ошибка создания DXF: {0}")]
    DxfCreationError(String),

    #[error("Ошибка записи DXF файла: {0}")]
    DxfWriteError(String),

    #[error("Ошибка файловой системы: {0}")]
    FileSystemError(String),

    #[error("Ошибка трансформации координат: {0}")]
    TransformError(String),

    #[error("Ошибка обработки пути: {0}")]
    PathError(String),

    #[error("Неподдерживаемый формат: {0}")]
    UnsupportedFormat(String),
}

impl ConversionError {
    /// Создает ошибку чтения SVG
    pub fn svg_read_error(msg: impl Into<String>) -> Self {
        Self::SvgReadError(msg.into())
    }

    /// Создает ошибку парсинга SVG
    pub fn svg_parse_error(msg: impl Into<String>) -> Self {
        Self::SvgParseError(msg.into())
    }

    /// Создает ошибку неподдерживаемого элемента
    pub fn unsupported_element(element: impl Into<String>) -> Self {
        Self::UnsupportedElement {
            element: element.into(),
        }
    }

    /// Создает ошибку создания DXF
    pub fn dxf_creation_error(msg: impl Into<String>) -> Self {
        Self::DxfCreationError(msg.into())
    }

    /// Создает ошибку записи DXF
    pub fn dxf_write_error(msg: impl Into<String>) -> Self {
        Self::DxfWriteError(msg.into())
    }

    /// Создает ошибку файловой системы
    pub fn file_system_error(msg: impl Into<String>) -> Self {
        Self::FileSystemError(msg.into())
    }

    /// Создает ошибку трансформации
    pub fn transform_error(msg: impl Into<String>) -> Self {
        Self::TransformError(msg.into())
    }

    /// Создает ошибку обработки пути
    pub fn path_error(msg: impl Into<String>) -> Self {
        Self::PathError(msg.into())
    }

    /// Создает ошибку неподдерживаемого формата
    pub fn unsupported_format(msg: impl Into<String>) -> Self {
        Self::UnsupportedFormat(msg.into())
    }
}

/// Тип результата операций конвертера
pub type ConversionResult<T> = Result<T, ConversionError>;
