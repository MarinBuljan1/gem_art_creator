use serde::Deserialize;


#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct DmcColor {
    #[serde(rename = "Floss")]
    pub floss: String,
    #[serde(rename = "DMC Name")]
    pub name: String,
    #[serde(rename = "R")]
    pub r: u8,
    #[serde(rename = "G")]
    pub g: u8,
    #[serde(rename = "B")]
    pub b: u8,
    #[serde(rename = "Hex")]
    pub hex: String,
}

pub fn get_dmc_colors() -> Vec<DmcColor> {
    let csv_data = include_str!("../list_of_DMC_colours.csv");
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_data.as_bytes());

    let mut colors = Vec::new();
    for result in reader.deserialize() {
        let color: DmcColor = result.unwrap();
        colors.push(color);
    }
    colors
}
