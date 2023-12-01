use weaksignal::ft8;

fn read_wav(filename: &str) -> Result<Vec<f32>, hound::Error> {
    let mut reader = hound::WavReader::open(filename)?;
    Ok(reader
        .samples::<i16>()
        .map(|s| s.unwrap() as f32 / 32768.0)
        .collect())
}

#[test]
fn it_works() {
    let mut decoder = ft8::FT8Decoder::new(&Default::default());
    let samples = read_wav("tests/test.wav").unwrap();

    let messages = decoder.decode_messages(samples.as_slice()).unwrap();

    /*assert!(messages
        .iter()
        .find(|m| m.to_string() == "JL1TZQ R3BV R-12")
        .is_some());*/

    messages.iter().for_each(|f| println!("{}", f));

    messages
        .iter()
        .flat_map(|m| m.callsigns())
        .filter(|c| !c.starts_with('<') && !c.ends_with('>'))
        .for_each(|c| {
            decoder.insert_callsign(&c);
        });
}
