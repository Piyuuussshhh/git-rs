use std::collections::HashMap;

use crate::{
    constants::COMMIT_MESSAGE,
    traits::{ObjectDeserialize, ObjectSerialize},
    utils::kvlm_parse,
};

pub struct Commit {
    fields: HashMap<String, String>,
}

impl Commit {
    pub fn new(tree: &str, parent: &str, author: &str, committer: &str, gpgsig: &str) -> Self {
        let mut fields = HashMap::new();
        fields.insert("tree".to_string(), tree.to_string());
        fields.insert("parent".to_string(), parent.to_string());
        fields.insert("author".to_string(), author.to_string());
        fields.insert("committer".to_string(), committer.to_string());
        fields.insert("gpgsig".to_string(), gpgsig.to_string());

        Self { fields }
    }
}

impl ObjectSerialize for Commit {
    fn serialize(&self, _: &str) -> String {
        let mut content = String::new();

        for key in self.fields.keys() {
            if key == COMMIT_MESSAGE {
                continue;
            }

            let values = self.fields.get(key);
            match values {
                Some(values) => {
                    let val_list = values.split("\n").collect::<Vec<&str>>();

                    for val in val_list {
                        content += &(key.to_owned() + " " + &val.replace("\n", "\n ") + "\n");
                    }
                }
                None => panic!("[ERROR] Could not find entry for {key}"),
            }
        }

        content
            + "\n"
            + self
                .fields
                .get(COMMIT_MESSAGE)
                .expect("[ERROR] No commit message")
    }
}

impl ObjectDeserialize<Self> for Commit {
    fn deserialize(data: &str) -> Self {
        let fields = kvlm_parse(data);
        Self { fields }
    }
}
