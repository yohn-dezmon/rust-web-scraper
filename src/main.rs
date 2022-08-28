use scraper;
use reqwest;

fn main() {
    // get the source code (HTML) of the page
    let response = reqwest::blocking::get(
            "https://www.imdb.com/search/title/?groups=top_100&sort=user_rating,desc&count=100",
    )
    .unwrap()
    .text()
    .unwrap();
    // a reference is used when referring to response so we can continue to reference response 
    // after this function call within the scope of main()
    // 
    let document = scraper::Html::parse_document(&response);
    // this is the query that can be used to extract the Movie Titles
    let title_selector = scraper::Selector::parse("h3.lister-item-header>a").unwrap();
    // I guess .inner_html() exists because title_selector is a Selector type...
    // titles is now an ITERATOR 
    // I had to use `mut` when I wanted to only access the first element 
    // since I was in effect removing it from the iterator
    // i.e. let mut titles ... 
    let titles = document.select(&title_selector).map(|x| x.inner_html());

    // how can i get the first value of an iterator? I want to see what the raw inner_html data 
    // looks like before we use zip/for_each 
    // first here is Option<String> type
    // let first = titles.next();
    // // This converts Option<String> into a String
    // let first_str = first.as_ref().map(String::as_str).unwrap();
    // println!("{}", first_str);

    // first we `zip` the title list from 1-100
    // I guess we do this to get a subset of the potential iterator result values.
    // where is the `number` value coming from? 
    // ** The number values come from the range we provided 1..101 !! 
    // the item values come from the titles iterator 
    // thus we have merged the range iterator with the titles iterator, and output a tuple for each value.
    titles 
        .zip(1..101)
        .for_each(|(item, number)| println!("{}. {}", number, item));

}
