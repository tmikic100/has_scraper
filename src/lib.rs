/*
    Functions:
    save
    CLI_Question
    get_years
    get_ages
    get_categories
    get_data
    get_category
    get_category_data
    format_data
    search
 */
use scraper::{Html, Selector};
use std::fs;
use std::io;
pub fn get_years() -> Vec<String> {
    let res = reqwest::blocking::get("https://www.has.hr/images/stories/HAS/tabsez").unwrap();
    let year = res.text_with_charset("windows-1250").unwrap();
    let fragment = Html::parse_fragment(&year.to_string());
    let selector = Selector::parse("a").unwrap();
    let mut years = Vec::new();
    for y in fragment.select(&selector) {
        let txt = y.text().collect::<Vec<_>>();
        if txt[0].len() == 5 {
            years.push(txt[0].to_string().replace("/",""));
        }
    }
    years
}
pub fn get_ages(year:String) -> Vec<String>{
    let url = "https://www.has.hr/images/stories/HAS/tabsez/".to_owned() + &year;
    let res = reqwest::blocking::get(&url).unwrap();
    let age = res.text_with_charset("windows-1250").unwrap();
    let fragment = Html::parse_fragment(&age.to_string());
    let selector = Selector::parse("a").unwrap();
    let mut ages = Vec::new();
    for y in fragment.select(&selector) {
        let txt = y.text().collect::<Vec<_>>();
        if txt[0].find(".html").unwrap_or(99) != 99 || txt[0].find(".htm").unwrap_or(99) != 99 {
            ages.push(txt[0].to_string());
                /*.replace(".html", "")
                .replace("ddm", "Kadeti")
                .replace("ddw", "Kadetkinje")
                .replace("ddm", "Kadeti")
                .replace("jjm", "Juniori")
                .replace("jjw", "Juniorke")
                .replace("mdm", "Mlađi kadeti")
                .replace("mdw", "Mlađe kadetkinje")
                .replace("mjm", "Mlađi juniori")
                .replace("mjw", "Mlađe juniorke")
                .replace("msm", "Mlađi seniori")
                .replace("msw", "Mlađe seniorke")
                .replace("ssm", "Seniori")
                .replace("ssw", "Seniorke")
                .replace(&year[2..4], "&")
                .replace("&d", " dvorana")
                .replace("&", ""));*/
        }
    }
    ages
}
pub fn get_categories(data:Vec<Vec<Vec<String>>>) -> Vec<String>{
    let mut cat = Vec::new();
    for x in 0..data.len(){
        cat.push(data[x][0][0].clone());
    }
    cat
}
pub fn CLI_input(question:&str) -> String {
    let mut ans:String= "".to_string();
    println!("{}", question);
    io::stdin()
        .read_line(&mut ans)
        .expect("Failed to read line");

    let ans:String = ans.split_whitespace().collect();
    ans
}
pub fn CLI_question(question:&str, answers:Vec<String>) -> String{
    let mut ans:String= "".to_string();
    println!("{}", question);
    if answers.len() == 2{
        println!("{} or {}",answers[0],answers[1]);
    }else{
        for x in 0..answers.len(){
            println!("{} -> {}",x,answers[x]);
        }
    }
    let mut response = "".to_string();
    'main:loop{
        let mut ans = "".to_string();
        io::stdin()
            .read_line(&mut ans)
            .expect("Failed to read line");

        let ans:String = ans.split_whitespace().collect();
        for x in 0..answers.len(){
            let comp:String = answers[x].split_whitespace().collect();
            if ans == comp{
                return ans;
            }
        }
        println!("Invaild answer");
    }
}
pub fn CLI_question_alias(question:&str, answers:Vec<String>, alias:Vec<String>) -> String{
    let mut ans= "".to_string();
    let mut mode = 0;
    println!("{}", question);
    if answers.len() == 2{
        println!("{} or {}",answers[0],answers[1]);
        mode = 0;
    }else{
        for x in 0..answers.len(){
            println!("{} -> {}",x,answers[x]);
        }
        mode = 1;
    }
    'main:loop{
        io::stdin()
            .read_line(&mut ans)
            .expect("Failed to read line");

        let mut ans:String = ans.split_whitespace().collect();
        for x in 0..answers.len(){
            if ans == answers[x]{
                ans = alias[x].clone();
                break 'main;
            }
        }
        println!("Invaild answer");
    }
    ans
}
pub fn get_category(data:Vec<Vec<Vec<String>>>, category:String) -> Vec<Vec<Vec<String>>> {
    let mut cat = Vec::new();
    for x in 0..data.len(){
        if data[x][0][0].replace(" ","").find(&category.replace(" ","")).unwrap_or(99) != 99 {
            cat.push(data[x].clone());
        }
    }
    cat
}
pub fn get_data(url:String) -> Vec<String>{
    let res = reqwest::blocking::get(&url).unwrap();
    let body =  res.text_with_charset("windows-1250").unwrap();
    let fragment = Html::parse_fragment(&body.to_string());
    let selector = Selector::parse("b").unwrap();
    let mut catagory = Vec::new();
    for cat in fragment.select(&selector) {
        let txt = cat.text().collect::<Vec<_>>();
        if txt[0].len() > 1 {
            catagory.push(txt[0]);
        }
    }
    let selector = Selector::parse(r#"font[face="Courier New, Courier, mono"]"#).unwrap();
    let mut text = String::new();
    for body in fragment.select(&selector) {
        let txt = body.text().collect::<Vec<_>>();
        for t in txt{
            text += t;
        }
    }
    let body = text;
    catagory.remove(1);
    let mut results = Vec::new();
    for x in 3..catagory.len() {
        let start = body.find(catagory[x-1]).unwrap_or(0);
        let end = body.find(catagory[x]).unwrap_or(body.len());
        if start != end{
            let result = &body[(start+catagory[x-1].len())..(end)];
            results.push(result);
        }
    }

    let start = body.find(catagory[catagory.len()-1]).unwrap_or(0);
    results.push(&body[(start+catagory[catagory.len()-1].len())..(body.len())]);
    let mut catagories = Vec::new();
    for cat in catagory {
        let disp = cat.split("- ").collect::<Vec<&str>>();
        if disp.len() > 1{
            catagories.push(disp[1]);
        }
    }
    let mut info = Vec::new();

    let mut index = 0;
    for result in &results {
        info.push("Start");
        info.push(catagories[index]);
        let mut row = 0;
        for time in result.split("\n"){
            info.push("New");
            let mut found = (false, false);
            let mut once = false;
            for detail in time.split(" "){
                if detail != ""{
                    if catagories[index].find("x").unwrap_or(99) == 99{
                        if !detail.chars().all(char::is_alphabetic) && (found == (true,false) || found == (true,true)) && detail != "/"{
                            info.push("End");
                            found = (false, true);
                        }
                        if detail.len() > 1{
                            if detail == "uz" || detail == "nema"{
                                info.push("Wind");
                                found = (true, true);
                            }else if detail.chars().all(char::is_alphabetic){
                                if found == (false,false){
                                    info.push("Name");
                                    found = (true,false);
                                }else if found == (false,true){
                                    info.push("Club");
                                    found = (true, true);
                                }
                            }
                        }
                    }
                    info.push(detail);
                    if detail == "assisted" || detail == "information"{
                        info.push("End");
                    }

                }
            }
        }
        index += 1;
    }
    info.iter().map(|s| s.to_string()).collect()
}
pub fn format_data(info:Vec<String>) -> Vec<Vec<Vec<String>>> {
    let mut cur = 0;
    let mut name = 0;
    let mut vals:Vec<String> = Vec::new();
    let mut table = Vec::new();
    let mut data_array = Vec::new();
    loop{
        if info[cur] == "Start" {
            table.push(vals.clone());
            data_array.push(table.clone());
            table.clear();
            vals.clear();
        } else if info[cur] == "New"{
            table.push(vals.clone());
            vals.clear();
        } else{
            let valb = info[cur].find("(").unwrap_or(99);
            let valrb = info[cur].find(")").unwrap_or(99);
            let valc = info[cur].find("(cesta)").unwrap_or(99);
            if (valb == 99 && valrb == 99) || valc != 99{
                if !(info[cur] == "Name" || info[cur] == "Club" || info[cur] == "Wind")  && name == 0{
                    vals.push((&info[cur]).to_string());
                }else if info[cur] == "End"{
                    if name == 2{
                        vals.push((&info[cur-1]).to_string());
                    }else{
                        let mut string = String::new();
                        for x in 0..(name-2){
                            string += info[cur - (name - 1) + x].as_str();
                            string += " ";
                        }
                        string += info[cur - 1].as_str();
                        let mut temp = Vec::new();
                        if info[cur-name] == "Name" || info[cur-name] == "Club" {
                            for el in string.split(" "){
                                temp.push(el);
                            }
                            vals.push(temp[0].to_string());
                            temp.remove(0);
                            let mut string = String::new();
                            if temp.len() != 1{
                                for x in 0..temp.len()-1{
                                    string += &temp[x].to_string();
                                    string += " ";
                                }
                                string += &temp[temp.len()-1].to_string();
                                vals.push(string);
                            }else{
                                vals.push(temp[temp.len()-1].to_string());
                            }
                        }else if info[cur-name] == "Wind"{
                            vals.push(string);
                        }
                    }
                    name = 0;
                }else{
                    name += 1;
                }
            }
        }
        cur += 1;

        if cur == info.len() {
            data_array.push(table.clone());
            break;
        }
    }
    data_array.remove(0);
    for x in 0..data_array.len(){
        let mut remove = Vec::new();
        for y in 0..data_array[x].len(){
            if data_array[x][y].is_empty(){
                remove.push(y);
            }else{
                let font = data_array[x][y][0].find("<font").unwrap_or(99);
                if font != 99{
                    remove.push(y);
                }
            }
        }
        let mut removed = 0;
        for el in &remove{
            data_array[x].remove(*el-removed);
            removed += 1;
        }
    }
    for x in 0..data_array.len(){
        let mut wind = false;
        let mut noWindInfo = false;
        for y in 1..data_array[x].len(){
            if wind == true{
                data_array[x][y].insert(0, "Wind assisted".parse().unwrap());
            }
            if noWindInfo == true && wind == false{
                if data_array[x][y].len() > 1{
                    data_array[x][y].insert(0, "No wind information".parse().unwrap());
                    data_array[x][y].insert(2, "0,0".parse().unwrap());
                }
            }
            if data_array[x][y][0].find("wind").unwrap_or(99) == 99 && data_array[x][y][0].find("nema").unwrap_or(99) == 99 && wind == false && noWindInfo == false{
                data_array[x][y].insert(0, y.to_string());
            }else{
                if data_array[x][y][0].find("uz").unwrap_or(99) != 99  && data_array[x][y][0].find("nema").unwrap_or(99) == 99{
                    wind = true;
                }else{
                    noWindInfo = true;
                }
            }
        }
    }
    data_array
}
pub fn search(mut data:Vec<Vec<Vec<String>>>, key:String, exact:bool) -> Vec<Vec<Vec<String>>>{
    let mut matches = Vec::new();
    let key:String = key.split_whitespace().collect();
    for x in 0..data.len(){
        for y in 0..data[x].len(){
            for z in 0..data[x][y].len(){
                if exact == false{
                    if data[x][y][z].to_lowercase().find(&key.to_lowercase()).unwrap_or(99) != 99{
                        matches.push((x,y));
                    }
                }else{
                    if data[x][y][z].to_lowercase() == key.to_lowercase(){
                        matches.push((x,y));
                    }
                }
            }
        }
    }
    let mut lx = 0;
    let mut row = Vec::new();
    let mut collum = Vec::new();
    for el in matches{
        let(x,y) = el;
        if lx != x {
            collum.push((lx,row.clone()));
            row.clear();
            row.push(y);
            lx = x;
        }else{
            row.push(y);
        }
    }
    collum.push((lx,row.clone()));
    let mut xes = Vec::new();
    for el in collum{
        let (x,array) = el;
        let mut removed = 0;
        for y in 1..data[x].len(){
            let mut found = false;
            for data in &array{
                if y == *data{
                    found = true;
                    break;
                }
            }
            if found == false{
                data[x].remove(y - removed);
                removed += 1;
            }
        }
        xes.push(x);
    }
    let mut removed = 0;
    for y in 0..data.len(){
        let mut found = false;
        for x in &xes{
            if *x == y{
                found = true;
                break;
            }
        }
        if found == false{
            data.remove(y - removed);
            removed += 1;
        }
    }
    data
}
pub fn save(data_array:Vec<Vec<Vec<String>>>){
    let mut data:String = String::new();
    for x in 0..data_array.len(){
        for y in 0..data_array[x].len(){
            if data_array[x][y][0].find("x").unwrap_or(99) == 99 && y == 1{
                if data_array[x][y].len() == 8{
                    data += "Result~Wind~Name~Surname~Birthday~Club~City~Date\n";
                }else if data_array[x][y].len() == 7{
                    data += "Result~Name~Surname~Birthday~Club~City~Date\n";
                }
            }
            if data_array[x][y].len() != 1 {
                for z in 0..data_array[x][y].len()-1{
                    data += &(data_array[x][y][z].to_string() +"~");
                }
                data += &(data_array[x][y][data_array[x][y].len()-1].to_string()+"\n");
            }else{
                data += &(data_array[x][y][0].to_string()+"\n");
            }
        }
    }
    fs::write("", data).expect("Unable to write file");
}