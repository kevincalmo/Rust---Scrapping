use csv;
use serde_json::json;

struct JobList {
    url: Option<String>,
    title: Option<String>,
    enterprise: Option<String>,
    created_at: Option<String>,
    type_of_contract: Option<String>,
}

fn main() {
    let response = reqwest::blocking::get("https://www.domemploi.com/emploi/?id_com_pays=MTQ&id_de_job_contrat=CDI");
    let html_content = response.unwrap().text().unwrap();
    /* println!("{html_content}"); */
    let document = scraper::Html::parse_document(&html_content);
    let html_product_selector = scraper::Selector::parse("li.list__ul__li").unwrap();
    let html_products = document.select(&html_product_selector);

    let mut job_list: Vec<JobList> = Vec::new();

    for html_product in html_products {
        let url = html_product
            .select(&scraper::Selector::parse("a.list__ul__li__a").unwrap())
            .next()
            .and_then(|a| a.value().attr("href"))
            .map(str::to_owned);

        let title = html_product
            .select(&scraper::Selector::parse("div.list__ul__li__a__txt__ti").unwrap())
            .next()
            .map(|div| div.text().collect::<String>());

        let enterprise = html_product
            .select(&scraper::Selector::parse("div.list__ul__li__a__txt__ent").unwrap())
            .next()
            .map(|div| div.text().collect::<String>());

        let type_of_contract = html_product
            .select(&scraper::Selector::parse("span.list__ul__li__a__txt__reg__contrat").unwrap())
            .next()
            .map(|span| span.text().collect::<String>());

        let created_at = html_product
            .select(&scraper::Selector::parse("span.list__ul__li__tools__date").unwrap())
            .next()
            .map(|span| span.text().collect::<String>());

        let job = JobList {
            url,
            title,
            enterprise,
            type_of_contract,
            created_at,
        };

        job_list.push(job);
    }

    let path = std::path::Path::new("jobs.csv");
    let mut writer = csv::Writer::from_path(path).unwrap();
    writer
        .write_record(&[
            "url",
            "title",
            "enterprise",
            "created_at",
            "type of contract",
        ])
        .unwrap();

    for job in job_list {
        let url = match job.url {
            Some(value) => value,
            None => String::from(""),
        };

        println!("url => {:?}", url);
        let title = match job.title {
            Some(value) => value,
            None => String::from(""),
        };
        println!("title => {:?} \n", title);
        let enterprise = match job.enterprise {
            Some(value) => value,
            None => String::from(""),
        };
        let created_at = match job.created_at{
            Some(value) => value,
            None => String::from(""),
        };
        let type_of_contract = match job.type_of_contract {
            Some(value) => value,
            None => String::from(""),
        };
        writer
            .write_record(&[url, title, enterprise, created_at, type_of_contract])
            .unwrap();
    }
    writer.flush().unwrap();
}
