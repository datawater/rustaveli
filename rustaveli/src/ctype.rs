use super::statics::*;

pub(crate) trait CTypeable {
    fn to_string(&self) -> String;
    fn default_value(&self) -> String;
    fn random_value(&self) -> String;
    fn size(&self) -> String;
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CType {
    pub value: String,
}

impl CTypeable for CType {
    fn to_string(&self) -> String {
        return self.value.clone();
    }

    fn default_value(&self) -> String {
        return AVAILABLE_TYPES
            .get(&self.to_string())
            .unwrap()
            .default
            .clone();
    }

    fn random_value(&self) -> String {
        let x = AVAILABLE_TYPES.get(&self.to_string()).unwrap();

        let self_ = x
            .self_
            .as_ref()
            .map(|x| Box::new(x.clone()) as Box<dyn CTypeable>);

        return (x.random)(self_).clone();
    }

    fn size(&self) -> String {
        return AVAILABLE_TYPES.get(&self.to_string()).unwrap().size.clone();
    }
}

impl CType {
    pub fn new(string: String) -> Self {
        if !AVAILABLE_TYPES.contains_key(&string) {
            panic!("Invalid type {string}");
        }

        let mut r = Self::default();
        r.value = string.clone();

        return r;
    }
}
