mod stato_pronto_soccorsi;
use serde_json::from_str;
use stato_pronto_soccorsi::StatoProntoSoccorsi;

#[tokio::main]
async fn main() {
    let future = get_hospital_data();
    let _ = futures::executor::block_on(future);
}

async fn get_hospital_data() -> anyhow::Result<()> {
    let body = reqwest::get("https://servizi.apss.tn.it/opendata/STATOPS001.json")
        .await?
        .text()
        .await?;

    let stato: StatoProntoSoccorsi = from_str::<StatoProntoSoccorsi>(&body)?;

    println!("{:?}", stato);
    Ok(())
}
