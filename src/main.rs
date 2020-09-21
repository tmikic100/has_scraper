//! Somewhat complex example of usage of structopt.

use structopt::StructOpt;
use term_table::table_cell::TableCell;
use term_table::row::Row;
use indicatif::ProgressBar;

#[derive(StructOpt, Debug)]
#[structopt(name = "Has scraper")]
/// An example of StructOpt usage.
struct Opt {
	// Needed parameter, the first on the command line.
	#[structopt(short="m", long, default_value = "1")]
	/**
    This is a doc comment

    0 - munual

    1 - guided

    */
	mode:String,
	#[structopt(short="y", long, required_if("mode", "0"))]
	/**
    This is a doc comment

    Hello!
    */
	year: Option<String>,
	#[structopt(short="a", long, requires("year"))]
	/**
    This is a doc comment

    Hello!\n lalalala
    */
	age: Option<String>,
	#[structopt(short="c", long,requires("age"))]
	/**
    This is a doc comment

    Hello!
    */
	category: Option<Vec<String>>,
	#[structopt(short = "e", long, requires("year"))]
	/**
    This is a doc comment

    Hello!
    */
	find_exact: Option<String>,
	#[structopt(short = "p", long, requires("year"))]
	/**
    This is a doc comment

    Hello!
    */
	find_partial: Option<String>,
}

fn main() {
	let opt = Opt::from_args();
	guided();
}

fn guided(){
	/*
		1) year
		2) age
		3) category
		4) filter
	 */
	let year = has::CLI_question("Choose a year",has::get_years());
	let age:String;
	let category:String;
	let ans = has::CLI_question("Do you want to see all ages", vec!("yes".to_string(),"no".to_string()));
	let mut data:Vec<Vec<Vec<String>>> = vec!(vec!(vec!("".to_string())));
	if ans == "yes"{
		let bar = ProgressBar::new(has::get_ages(year.clone()).len() as u64);
		for x in has::get_ages(year.clone()){
			data.append(&mut has::format_data(has::get_data("https://www.has.hr/images/stories/HAS/tabsez/".to_owned() + &year.clone() + "/" + &x.clone()),format!("{} {}",x.clone(),year.clone())));
			bar.inc(1);
		}
		bar.finish();
	}else{
		let mut alias:Vec<Vec<String>> = vec![];
		for x in has::get_ages(year.clone()){
			alias.push(has::get_age_alias(x, year.clone()));
		}
		age = has::CLI_question_alias("Choose a age",has::get_ages(year.clone()), alias);
		data = has::format_data(has::get_data("https://www.has.hr/images/stories/HAS/tabsez/".to_owned()+&year.clone()+"/"+&age.clone()), "".parse().unwrap());
		let ans = has::CLI_question("Do you want to see all categories", vec!("yes".to_string(),"no".to_string()));
		if ans == "no"{
			let mut alias:Vec<Vec<String>> = vec![];
			for x in has::get_categories(data.clone()){
				alias.push(has::get_category_alias(x));
			}
			category = has::CLI_question_alias("Choose a category", has::get_categories(data.clone()),alias);
			data = has::get_category(data.clone(), category);
		}
		let ans = has::CLI_question("Do you want filter the results", vec!("yes".to_string(),"no".to_string()));
		if ans == "yes"{
			let ans = has::CLI_question("How do you want to filter the results", vec!("exact".to_string(),"partial".to_string()));
			let key = has::CLI_input("What do you want to search for");
			if ans == "exact"{
				data = has::search(data,key,true);
			}else{
				data = has::search(data,key,false);
			}
		}
	}

	let ans = has::CLI_question("Do you want to save the result", vec!("yes".to_string(),"no".to_string()));

	if ans == "yes"{
		has::save(data);
	}else{
		for x in 0..data.len(){
			let mut table = term_table::Table::new();
			table.max_column_width = 40;
			table.style = term_table::TableStyle::extended();
			for y in 0..data[x].len(){
				let mut rows = Vec::new();

					for z in 0..data[x][y].len(){
						if data[x][y].len() != 1 {
							rows.push(TableCell::new(data[x][y][z].clone()));
						}else{
							rows.push(TableCell::new_with_alignment(data[x][y][z].clone(), data[x][1].len(), term_table::table_cell::Alignment::Center));
						}
					}
				table.add_row(Row::new(rows));
			}
			println!("{}",table.render());
		}
	}

}