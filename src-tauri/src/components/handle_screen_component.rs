use serde_json::json;

#[tauri::command(rename_all = "snake_case")]
pub fn get_screen(boardX: f64, boardY: f64) -> serde_json::Value {
    let screens = find_screen();

    screens
}

/*
   This function for find screen in network and return list
*/
fn find_screen() -> serde_json::Value {
    let data = json!([
        {
            "name": "com1",
            "width": 1920,
            "height": 1080,
            "fill": "red",
            "mac": "asdasmdkasdkasmjdkasmdkamsd",
            "role": "main"
        },
        {
            "name": "com2",
            "width": 2560,
            "height": 1440,
            "fill": "yellow",
            "mac": "fsdfsdagagcasfasvsdbfsbfsdcf"
        },
        {
            "name": "com3",
            "width": 3480,
            "height": 2160,
            "fill": "blue",
            "mac": "sdfbrydtmhdnhfhfgdhnfdtnhhdn"
        }
    ]);

    data
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_find_screen() {
        let json = find_screen();
        assert_eq!(*json.get(0).unwrap().get("width").unwrap(), json!(1920));
    }
}
