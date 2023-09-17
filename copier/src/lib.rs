use schema::Schema;

pub mod replacer;
pub mod file_copier;

pub trait YamlParser {
    fn parse_yml(&self) -> Schema;
}
