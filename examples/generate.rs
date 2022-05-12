use openapi_client_generator::{generate_library, GenerateLibrary, OpenAPI, read_spec};

fn modify_spec(spec: &mut OpenAPI) {
    spec.paths.paths.iter_mut().for_each(|(path, item)| {
        let item = item.as_mut().unwrap();
        item.iter_mut().for_each(|(method, operation)| {
            if let Some(ref mut docs) = operation.external_docs {
                docs.url = format!("https://plaid.com/docs{}", docs.url);
            }
        });
    });
}


fn main() {
    let mut spec = read_spec("openapi.yaml".as_ref()).unwrap();

    modify_spec(&mut spec);

    generate_library(spec, GenerateLibrary {
        name: "Plaid".to_string(),
        dest_path: "src".into(),
        lib_rs_path: Some("template/lib.rs".into()),
        model_rs_path: Some("template/model.rs".into()),
    }).unwrap();
}