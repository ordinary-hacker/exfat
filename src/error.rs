use thiserror::Error;

#[derive(Debug)]
pub enum DataError {
    NotExFAT,
    BootChecksum,
    AllocationBitmapMissing,
    UpcaseTableMissing,
    UpcaseTableChecksum,
    FATChain,
    Metadata,
}

impl core::fmt::Display for DataError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DataError::NotExFAT => write!(f, "Not exFAT filesystem"),
            DataError::BootChecksum => write!(f, "Bad boot sector checksum"),
            DataError::AllocationBitmapMissing => write!(f, "Allocation bitmap missing"),
            DataError::UpcaseTableMissing => write!(f, "Upcase table missing"),
            DataError::UpcaseTableChecksum => write!(f, "Bad upcase table checksum"),
            DataError::FATChain => write!(f, "Broken FAT chain"),
            DataError::Metadata => write!(f, "Broken file or directory metadata"),
        }
    }
}

impl core::error::Error for DataError {}

#[derive(Debug)]
pub enum ImplementationError {
    TexFATNotSupported,
    CreateDirectoryNotSupported,
}

impl core::fmt::Display for ImplementationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ImplementationError::TexFATNotSupported => write!(f, "TexFAT not supported"),
            ImplementationError::CreateDirectoryNotSupported => write!(f, "Create directory not supported"),
        }
    }
}

impl core::error::Error for ImplementationError {}

#[derive(Debug)]
pub enum InputError {
    NameTooLong,
    SeekPosition,
    Size,
}

impl core::fmt::Display for InputError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            InputError::NameTooLong => write!(f, "Name too long"),
            InputError::SeekPosition => write!(f, "Seek position out of range"),
            InputError::Size => write!(f, "Size out of range"),
        }
    }
}

impl core::error::Error for InputError {}

#[derive(Debug)]
pub enum AllocationError {
    NotPossible,
    Fragment,
    NoMoreCluster,
}

impl core::fmt::Display for AllocationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AllocationError::NotPossible => write!(f, "Allocation-not-possible is set in file metadata"),
            AllocationError::Fragment => write!(f, "Need fragment while dont-fragment is set in file options"),
            AllocationError::NoMoreCluster => write!(f, "No more cluster available"),
        }
    }
}

impl core::error::Error for AllocationError {}

#[derive(Debug)]
pub enum OperationError {
    AlreadyOpen,
    NotFound,
    NotFile,
    NotDirectory,
    AlreadyExists,
    DirectoryNotEmpty,
    EOF,
}

impl core::fmt::Display for OperationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            OperationError::AlreadyOpen => write!(f, "File or directory already open"),
            OperationError::NotFound => write!(f, "File or directory not found"),
            OperationError::NotFile => write!(f, "Not a file"),
            OperationError::NotDirectory => write!(f, "Not a directory"),
            OperationError::AlreadyExists => write!(f, "File or directory already exists"),
            OperationError::DirectoryNotEmpty => write!(f, "Directory not empty when deleting"),
            OperationError::EOF => write!(f, "End of file"),
        }
    }
}

impl core::error::Error for OperationError {}

#[derive(Error, Debug)]
pub enum Error<E> {
    #[error("IO({0:?})")]
    IO(E),
    #[error("{0:?}")]
    Data(#[from] DataError),
    #[error("{0:?}")]
    Implementation(#[from] ImplementationError),
    #[error("{0:?}")]
    Input(#[from] InputError),
    #[error("{0:?}")]
    Operation(#[from] OperationError),
    #[error("{0:?}")]
    Allocation(#[from] AllocationError),
}
