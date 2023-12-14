use std::{env, fs};
use serde_yaml::Value;
use std::fs::File;
use openapiv3::{Type, SchemaKind, OpenAPI};

fn main() {
    let mut args = env::args().skip(1);
    let fpath: String = args.next().expect("Pass in an input fpath to an openapi spec");
    let out_fpath: String = args.next().expect("Need an output fpath");

    let f = File::open(fpath).expect("Could not open file");
    let mut yaml: Value = serde_yaml::from_reader(f).expect("Could not deserialize yaml from file");
    yaml["components"]["schemas"]["PartnerCustomersCreateRequest"]["type"] = Value::from("object");
    yaml["components"]["schemas"]["UserName"]["type"] = Value::from("object");

    let mut spec: OpenAPI = serde_yaml::from_value(yaml).expect("Could not structure OpenAPI file");

    spec.operations_mut().for_each(|(_, _, operation)| {
        if let Some(ref mut docs) = operation.external_docs {
            docs.url = format!("https://plaid.com/docs{}", docs.url);
        }
    });
    spec.schemas_mut().iter_mut().for_each(|(_, schema)| {
        let schema = schema.as_mut().unwrap();
        if let SchemaKind::Type(Type::Object(ref mut o)) = &mut schema.schema_kind {
            let props = &mut o.properties;
            props.shift_remove("client_id");
            props.shift_remove("secret");
        }
    });
    fs::write(&out_fpath, serde_yaml::to_string(&spec).unwrap()).expect("Could not write to file");
    eprintln!("Wrote to {}", out_fpath);
}