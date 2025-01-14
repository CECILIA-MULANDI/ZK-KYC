use chrono::NaiveDate;

use std::Error;
pub struct KycUpload {
    pub name: String,
    pub date_of_birth: NaiveDate,
    pub gender: Option<String>,
    pub signature: String,
    pub address: String,
    pub id_number: String,
    pub id_type: String,
    pub document_issue_date: NaiveDate,
    pub document_expiry_date: Option<NaiveDate>,
}
impl KycUpload {
    pub fn new(
        name: String,
        dob: String,
        gender: Option<String>,
        signature: String,
        address: String,
        id_number: String,
        id_type: String,
        issue_date: String,
        expiry_date: Option<String>,
    ) -> Self {
        KycUpload {
            name,
            date_of_birth: NaiveDate::parse_from_str(&dob, "%Y-%m-%d")
                .expect("Invalid date format"),
            gender,
            signature,
            address,
            id_number,
            id_type,
            document_issue_date,
            document_expiry_date,
            document_issue_date: NaiveDate::parse_from_str(&issue_date, "%Y-%m-%d")
                .expect("Invalid issue date format"),
            document_expiry_date: expiry_date.map(|date| {
                NaiveDate::parse_from_str(&date, "%Y-%m-%d").expect("Invalid expiry date format")
            }),
        }
    }
}
