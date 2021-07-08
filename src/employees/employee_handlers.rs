#[derive(Debug, Serialize, Deserialize)]
pub struct InputEmployee {
    pub employee_id: String,
    pub title: Title,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub gender: Gender,
    pub dob: chrono::NaiveDate
}

pub async fn get_employees(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    
}