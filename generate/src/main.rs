use openapi_client_generator::{OpenAPI, read_spec, GenerateLibraryOptions};
use openapi_client_generator::generate_library;
use openapi_client_generator::openapiv3::{ReferenceOr, Schema, SchemaKind, Type};
use openapi_client_generator::sourcegen::SourceGen;


fn get_component_name(reference: &str) -> Option<&str> {
    let mut parts = reference.split('/');
    if parts.next() != Some("#") {
        return None;
    }
    if parts.next() != Some("components") {
        return None;
    }
    if parts.next() != Some("schemas") {
        return None;
    }
    parts.next()
}


fn resolve_mut<'a>(schema: &'a mut ReferenceOr<Schema>, spec: &'a mut OpenAPI) -> &'a mut Schema {
    match schema {
        ReferenceOr::Reference { reference } => {
            let name = get_component_name(&reference).unwrap();
            let components = spec.components.as_mut().unwrap();
            let ref_or_schema = components.schemas.get_mut(name).unwrap();
            match ref_or_schema {
                ReferenceOr::Item(schema) => schema,
                ReferenceOr::Reference { .. } => panic!("circular reference"),
            }
        }
        ReferenceOr::Item(ref mut schema) => schema,
    }
}

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
    let mut spec = read_spec("openapi.yaml".as_ref()).unwrap();

    modify_spec(&mut spec);

    generate_library(spec, GenerateLibraryOptions {
        package_name: "plaid-openapi".to_string(),
        service_name: "Plaid".to_string(),
        qualified_github_repo: "libninjacom/plaid-rs".to_string(),
        dest_path: "..".into(),
        lib_rs_path: Some("template/src/lib.rs".into()),
        model_rs_path: Some("template/src/model.rs".into()),
        package_version: "1.0.0".to_string(),
        generator: SourceGen::Rust
    }).unwrap();

    std::fs::copy("template/Justfile", "../Justfile").unwrap();
}