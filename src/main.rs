use serde::Serialize;
use std::fs;
use std::io::{self, BufRead};

#[derive(Serialize)]
#[serde(rename = "people")]
struct People {
    #[serde(rename = "person")]
    persons: Vec<Person>
}

#[derive(Serialize)]
struct Person {
    firstname: String,
    lastname: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<Phone>,
    #[serde(rename = "family")]
    family: Vec<FamilyMember>
}

#[derive(Serialize)]
struct Address {
    street: String,
    city: String,
    zip: String,
}

#[derive(Serialize)]
struct Phone {
    mobile: String,
    landline: String,
}

#[derive(Serialize)]
struct FamilyMember {
    name: String,
    born: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<Phone>
}

struct TreeBuilder {
    persons: Vec<Person>,
    active_person_idx: Option<usize>,
    active_family_idx: Option<usize>,
}

impl TreeBuilder {
    fn new() -> Self {
        TreeBuilder {
            persons: Vec::new(),
            active_person_idx: None,
            active_family_idx: None,
        }
    }

    fn add_person(&mut self, firstname: &str, lastname: &str) {
        let person = Person {
            firstname: firstname.to_string(),
            lastname: lastname.to_string(),
            address: None,
            phone: None,
            family: Vec::new(),
        };
        self.persons.push(person);
        self.active_person_idx = Some(self.persons.len() - 1);
        self.active_family_idx = None;
    }

    fn add_family_member(&mut self, name: &str, born: &str) {
        if let Some(p) = self.active_person_idx {
            let member = FamilyMember {
                name: name.to_string(),
                born: born.to_string(),
                address: None,
                phone: None,
            };
            self.persons[p].family.push(member);
            self.active_family_idx = Some(self.persons[p].family.len() - 1);
        }
    }

    fn add_address(&mut self, street: &str, city: &str, zip: &str) {
        let address = Address {
            street: street.to_string(),
            city: city.to_string(),
            zip: zip.to_string(),
        };

        if let Some(p) = self.active_person_idx {
            if let Some(f) = self.active_family_idx {
                self.persons[p].family[f].address = Some(address);
            } else {
                self.persons[p].address = Some(address);
            }
        }
    }

    fn add_phone(&mut self, mobile: &str, landline: &str) {
        let phone = Phone {
            mobile: mobile.to_string(),
            landline: landline.to_string(),
        };

        if let Some(p) = self.active_person_idx {
            if let Some(f) = self.active_family_idx {
                self.persons[p].family[f].phone = Some(phone);
            } else {
                self.persons[p].phone = Some(phone);
            }
        }
    }

    fn build(self) -> Vec<Person> {
        self.persons
    }
}

fn main() -> io::Result<()> {
    let file_path = "testFiles/test4.txt";
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);
    
    let mut builder = TreeBuilder::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split('|').collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "P" => builder.add_person(parts.get(1).unwrap_or(&""), parts.get(2).unwrap_or(&"")),
            "F" => builder.add_family_member(parts.get(1).unwrap_or(&""), parts.get(2).unwrap_or(&"")),
            "A" => builder.add_address(
                parts.get(1).unwrap_or(&""), 
                parts.get(2).unwrap_or(&""), 
                parts.get(3).unwrap_or(&"")
            ),
            "T" => builder.add_phone(parts.get(1).unwrap_or(&""), parts.get(2).unwrap_or(&"")),
            _ => {} 
        }
    }

    let root = People { persons: builder.build() };

    let mut pretty_xml = String::new();
    let mut serializer = quick_xml::se::Serializer::new(&mut pretty_xml);
    serializer.indent(' ', 4);
    
    root.serialize(serializer).expect("Failed to serialize XML");
    
    fs::create_dir_all("xmlResult")?;
    
    let mut file_name = String::from("xmlResult/test.xml");
    let mut counter = 1;

    while std::path::Path::new(&file_name).exists() {
        file_name = format!("xmlResult/test_{}.xml", counter);
        counter += 1;
    }

    fs::write(&file_name, &pretty_xml)?;
    
    println!("Complete, saved in {}", file_name);

    Ok(())
}