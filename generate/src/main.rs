use std::env;
use openapi_client_generator::{OpenAPI, read_spec, GenerateLibraryOptions};
use openapi_client_generator::generate_library;
use openapi_client_generator::openapiv3::{SchemaKind, Type};
use openapi_client_generator::sourcegen::SourceGen;


fn modify_spec(spec: &mut OpenAPI) {
    spec.paths.paths.iter_mut().for_each(|(_path, item)| {
        let item = item.as_mut().unwrap();
        item.iter_mut().for_each(|(_method, operation)| {
            if let Some(ref mut docs) = operation.external_docs {
                docs.url = format!("https://plaid.com/docs{}", docs.url);
            }
        });
    });
    spec.components.as_mut().unwrap().schemas.iter_mut().for_each(|(_name, schema)| {
        let schema = schema.as_mut().unwrap();
        match &mut schema.schema_kind {
            SchemaKind::Type(Type::Object(ref mut o)) => {
                let props = &mut o.properties;
                props.shift_remove("client_id");
                props.shift_remove("secret")
            }
            _ => { return; }
        };
    });
}


fn main() {
    let version = env::var("VERSION").expect("VERSION is not set.");
    let yaml_path = env::var("YAML_FILE").expect("YAML_FILE is not set.");
    let mut spec = read_spec(yaml_path.as_ref()).unwrap();

    modify_spec(&mut spec);

    generate_library(spec, GenerateLibraryOptions {
        package_name: "plaid-openapi".to_string(),
        service_name: "Plaid".to_string(),
        qualified_github_repo: "libninjacom/plaid-rs".to_string(),
        dest_path: "..".into(),
        lib_rs_path: Some("template/src/lib.rs".into()),
        model_rs_path: Some("template/src/model.rs".into()),
        package_version: version.to_string(),
        generator: SourceGen::Rust
    }).unwrap();
}
