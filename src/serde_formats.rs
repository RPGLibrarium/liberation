pub mod naive_date{
    use serde::{Serializer};
    use chrono::{NaiveDate};
    const DATE_FORMAT: &'static str = "%Y-%m-%d";


    pub fn serialize<S> (
        date: &NaiveDate,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        let s = format!("{}", date.format(DATE_FORMAT));
        serializer.serialize_str(&s)
    }
}
