#[derive(Clone)]
pub struct Sentance {
    pub en: String,
    pub jp: String,
}
pub fn load_sentances() -> Vec<Sentance> {
    let txt = std::fs::read_to_string("data/Sentence pairs in Japanese-English - 2022-09-03.tsv")
        .unwrap();
    let rows = txt.lines().map(|l| l.split("\t"));
    rows.map(|mut v| {
        let record: Vec<&str> = v.collect();

        Sentance {
            en: record[3].to_owned(),
            jp: record[1].to_owned(),
        }
    })
    .collect()
}
