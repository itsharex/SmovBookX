
pub struct SmovName;

impl SmovName {
    pub fn format_smov_name(name:&String)-> String {
        name.to_uppercase().replace("-C", "").replace("-", "")
    }
}