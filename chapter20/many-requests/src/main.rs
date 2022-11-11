use async_std::task;

pub async fn many_requests(urls: &[String]) -> Vec<Result<String, surf::Exception>> {
    let client = surf::Client::new();

    let mut handles = vec![];
    for url in urls {
        // `recv_string()` is an async method
        // that returns a `Send +'static` future
        // which we can then send to `spawn()`
        let request = client.get(&url).recv_string();
        handles.push(task::spawn(request));
    }

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await);
    }

    results
}

fn main() {
    let requests = &[
        "http://example.com".to_owned(),
        "https://www.red-bean.com".to_owned(),
        "https://en.wikipedia.org/wiki/Main_Page".to_owned(),
    ];

    let results = async_std::task::block_on(many_requests(requests));
    for result in results {
        match result {
            Ok(response) => println!("{}", response),
            Err(err) => eprintln!("error: {}", err),
        }
    }
}
