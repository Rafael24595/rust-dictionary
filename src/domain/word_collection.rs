use crate::configuration::dependency::Dependency;
use crate::configuration::word::Word;

pub trait WordCollection: Dependency {
    fn find(&mut self, code: &String) -> Option<&Word>;
    fn find_lax(&self, code: &String) ->  Vec<&Word>;
    fn find_includes(&self, code: &String, position: Option<i8>, lax: Option<bool>, size: Option<i64>) -> Vec<&Word>;
    fn find_random(&self, size: Option<i64>) ->  Vec<&Word>;
    fn find_permute(&mut self, combo: &String, min: Option<i8>, exists: Option<bool>, lax: Option<bool>, includes: Option<i8>, size: Option<i64>) -> Vec<Word>;
}