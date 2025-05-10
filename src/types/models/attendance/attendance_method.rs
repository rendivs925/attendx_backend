use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum AttendanceMethod {
    QRCode,
    FacialRecognition,
    NFC,
    Manual,
    GPS,
    Biometric,
}
