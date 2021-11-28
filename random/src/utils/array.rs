pub fn array_as_json(array: Vec<&str>) -> String {
    let array: Vec<String> = array
                            .iter()
                            .map(|element| {element.replace("\"", "\\\"")})
                            .collect();
    let mut result = array.join("\",\"");
    result.insert_str(0, "[\"");
    result.push_str("\"]");
    result
}